use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;

use lopdf::content::Content;
use lopdf::{Dictionary, Document, Encoding, Object, ObjectId};

use crate::markdown::SourceContract;
use crate::report::{Finding, Severity};

pub struct PdfInspection {
    pub findings: Vec<Finding>,
    pub pages: usize,
    pub link_annotations: usize,
}

pub fn inspect_pdf(
    path: &Path,
    source: &SourceContract,
    overflow_tolerance: f64,
    check_highlighting: bool,
) -> Result<PdfInspection, String> {
    let document =
        Document::load(path).map_err(|e| format!("could not read PDF {}: {e}", path.display()))?;
    if document.is_encrypted() {
        return Err("the PDF is encrypted; provide an unlocked release artifact".into());
    }
    let pages = document.get_pages();
    if pages.is_empty() {
        return Err("the PDF has no pages".into());
    }

    let mut findings = Vec::new();
    let mut link_annotations = 0;
    let mut has_color = false;
    let mut all_text = String::new();
    let mut visual_lines = Vec::new();

    for (number, id) in &pages {
        link_annotations += link_annotations_on_page(&document, *id).len();
        match document.extract_text(&[*number]) {
            Ok(text) => {
                all_text.push_str(&text);
                all_text.push('\n');
            }
            Err(error) => findings.push(
                Finding::new(
                    "pdf.text-extraction",
                    Severity::Warning,
                    format!("Text could not be decoded on PDF page {number}: {error}"),
                    "Confirm the renderer embeds a usable text encoding.",
                    None,
                )
                .on_page(*number),
            ),
        }
        if let Ok(bytes) = document.get_page_content(*id) {
            if let Ok(content) = Content::decode(&bytes) {
                visual_lines.extend(extract_visual_lines(&document, *id, &content));
                let (colored, overflows) =
                    inspect_operations(&content, page_box(&document, *id), overflow_tolerance);
                has_color |= colored;
                for overflow in overflows {
                    findings.push(Finding::new(
                        "geometry.text-overflow",
                        Severity::Error,
                        format!(
                            "Painted text reaches {}={:.1}pt past the {} page boundary at {}={:.1}pt",
                            overflow.axis,
                            overflow.coordinate,
                            overflow.side,
                            overflow.axis,
                            overflow.boundary
                        ),
                        "Move, wrap, or shorten the affected text, then render again.",
                        None,
                    ).on_page(*number));
                }
            }
        }
    }

    inspect_internal_links(&document, &pages, source, &mut findings);

    let normalized_pdf = normalize(&all_text);
    for fence in &source.fences {
        let useful: Vec<&str> = fence
            .lines
            .iter()
            .map(String::as_str)
            .filter(|line| !line.trim().is_empty())
            .collect();
        let missing: Vec<&str> = useful
            .iter()
            .copied()
            .filter(|line| !normalized_pdf.contains(&normalize(line)))
            .collect();
        if !missing.is_empty() {
            findings.push(Finding::new(
                "code.content-missing",
                Severity::Error,
                format!(
                    "Code block on line {} lost {} of {} non-empty lines",
                    fence.line,
                    missing.len(),
                    useful.len()
                ),
                format!("First missing line: {}", truncate(missing[0], 90)),
                Some(fence.line),
            ));
        } else if !useful.is_empty() && !preserves_line_flow(&useful, &visual_lines) {
            findings.push(Finding::new(
                "code.flow-changed",
                Severity::Error,
                format!(
                    "Code block on line {} is present but its line flow changed",
                    fence.line
                ),
                "Restore the source line boundaries and order, then render again.",
                Some(fence.line),
            ));
        }
    }

    let expects_highlight = source
        .fences
        .iter()
        .any(|f| f.language.is_some() && !f.lines.is_empty());
    if check_highlighting && expects_highlight && !has_color {
        findings.push(Finding::new(
            "code.highlight-not-detected",
            Severity::Warning,
            "Language-tagged code exists, but no non-default color operation was found",
            "Confirm syntax highlighting visually or configure the renderer's highlight style.",
            source
                .fences
                .iter()
                .find(|f| f.language.is_some())
                .map(|f| f.line),
        ));
    }

    Ok(PdfInspection {
        findings,
        pages: pages.len(),
        link_annotations,
    })
}

/// Extract text grouped by painted PDF baselines. `Document::extract_text`
/// intentionally concatenates every `Tj`/`TJ` operation until `ET`, which
/// loses the distinction between a code block and the same words flattened
/// into a paragraph. This small text-state walker keeps that evidence.
fn extract_visual_lines(document: &Document, page_id: ObjectId, content: &Content) -> Vec<String> {
    let encodings: BTreeMap<Vec<u8>, Encoding<'_>> = document
        .get_page_fonts(page_id)
        .map(|fonts| {
            fonts
                .into_iter()
                .filter_map(|(name, font)| {
                    font.get_font_encoding(document)
                        .ok()
                        .map(|encoding| (name, encoding))
                })
                .collect()
        })
        .unwrap_or_default();
    let mut current_font = Vec::new();
    let mut current_line = String::new();
    let mut lines = Vec::new();
    let mut baseline = None;
    let mut font_size = 12.0_f64;

    for operation in &content.operations {
        match operation.operator.as_str() {
            "BT" => {
                finish_visual_line(&mut lines, &mut current_line);
                baseline = None;
            }
            "ET" => {
                finish_visual_line(&mut lines, &mut current_line);
                baseline = None;
            }
            "Tf" if operation.operands.len() >= 2 => {
                if let Ok(name) = operation.operands[0].as_name() {
                    current_font.clear();
                    current_font.extend_from_slice(name);
                }
                font_size = number(&operation.operands[1]).abs().max(1.0);
            }
            "Tm" if operation.operands.len() >= 6 => {
                move_to_baseline(
                    number(&operation.operands[5]),
                    font_size,
                    &mut baseline,
                    &mut lines,
                    &mut current_line,
                );
            }
            "Td" | "TD" if operation.operands.len() >= 2 => {
                let next = baseline.unwrap_or(0.0) + number(&operation.operands[1]);
                move_to_baseline(
                    next,
                    font_size,
                    &mut baseline,
                    &mut lines,
                    &mut current_line,
                );
            }
            "T*" => finish_visual_line(&mut lines, &mut current_line),
            "'" | "\"" => {
                finish_visual_line(&mut lines, &mut current_line);
                if let Some(encoding) = encodings.get(&current_font) {
                    append_decoded_text(
                        &mut lines,
                        &mut current_line,
                        decode_text_operands(encoding, &operation.operands),
                    );
                }
            }
            "Tj" | "TJ" => {
                if let Some(encoding) = encodings.get(&current_font) {
                    append_decoded_text(
                        &mut lines,
                        &mut current_line,
                        decode_text_operands(encoding, &operation.operands),
                    );
                }
            }
            _ => {}
        }
    }
    finish_visual_line(&mut lines, &mut current_line);
    lines
}

fn move_to_baseline(
    next: f64,
    font_size: f64,
    baseline: &mut Option<f64>,
    lines: &mut Vec<String>,
    current_line: &mut String,
) {
    // Renderers round text matrices differently. A font-relative tolerance
    // ignores sub-point positioning noise while keeping actual code baselines
    // (normally at least one em apart) distinct.
    let tolerance = (font_size * 0.2).max(1.0);
    if baseline.is_some_and(|current| (current - next).abs() > tolerance) {
        finish_visual_line(lines, current_line);
    }
    *baseline = Some(next);
}

fn decode_text_operands(encoding: &Encoding<'_>, operands: &[Object]) -> String {
    let mut output = String::new();
    for operand in operands {
        match operand {
            Object::String(bytes, _) => {
                if let Ok(text) = Document::decode_text(encoding, bytes) {
                    output.push_str(&text);
                }
            }
            Object::Array(values) => output.push_str(&decode_text_operands(encoding, values)),
            Object::Integer(adjustment) if *adjustment < -100 => output.push(' '),
            _ => {}
        }
    }
    output
}

fn append_decoded_text(lines: &mut Vec<String>, current_line: &mut String, text: String) {
    let mut parts = text.split('\n').peekable();
    while let Some(part) = parts.next() {
        current_line.push_str(part);
        if parts.peek().is_some() {
            finish_visual_line(lines, current_line);
        }
    }
}

fn finish_visual_line(lines: &mut Vec<String>, current_line: &mut String) {
    let line = normalize(current_line);
    if !line.is_empty() {
        lines.push(line);
    }
    current_line.clear();
}

fn preserves_line_flow(source_lines: &[&str], visual_lines: &[String]) -> bool {
    let source_lines = source_lines
        .iter()
        .map(|line| normalize(line))
        .collect::<Vec<_>>();
    visual_lines.windows(source_lines.len()).any(|window| {
        source_lines
            .iter()
            .zip(window)
            .all(|(source, painted)| painted.contains(source))
    })
}

fn inspect_internal_links(
    document: &Document,
    pages: &std::collections::BTreeMap<u32, ObjectId>,
    source: &SourceContract,
    findings: &mut Vec<Finding>,
) {
    if source.internal_links.is_empty() {
        return;
    }

    let named_destinations = named_destinations(document, pages);
    let mut annotations_by_target: HashMap<String, usize> = HashMap::new();
    for page_id in pages.values() {
        for destination in link_annotations_on_page(document, *page_id) {
            if let Some(target) = named_destination_target(document, destination) {
                *annotations_by_target.entry(target).or_default() += 1;
            }
        }
    }

    let mut seen_by_target: HashMap<&str, usize> = HashMap::new();
    for (line, target) in &source.internal_links {
        let seen = seen_by_target.entry(target).or_default();
        *seen += 1;
        let annotation_count = annotations_by_target.get(target).copied().unwrap_or(0);

        if annotation_count < *seen {
            findings.push(Finding::new(
                "link.destination-missing",
                Severity::Error,
                format!("Markdown link #{target} has no matching PDF link annotation"),
                "Preserve this fragment as a named PDF destination and link to it in the rendered PDF.",
                Some(*line),
            ));
        } else if !named_destinations.get(target).copied().unwrap_or(false) {
            findings.push(Finding::new(
                "link.destination-unresolved",
                Severity::Error,
                format!("PDF link destination #{target} does not resolve to a PDF page"),
                "Emit a named destination for this heading that points to a page in the PDF.",
                Some(*line),
            ));
        }
    }
}

fn link_annotations_on_page(document: &Document, page_id: ObjectId) -> Vec<&Object> {
    let Ok(page) = document.get_dictionary(page_id) else {
        return Vec::new();
    };
    let Ok(annots) = page.get(b"Annots") else {
        return Vec::new();
    };
    let Ok(items) = deref_array(document, annots) else {
        return Vec::new();
    };
    items
        .iter()
        .filter_map(|item| {
            let dict = deref_dict(document, item)?;
            if !matches!(dict.get(b"Subtype").and_then(Object::as_name), Ok(b"Link")) {
                return None;
            }
            if let Ok(destination) = dict.get(b"Dest") {
                return Some(destination);
            }
            let action = dict
                .get(b"A")
                .ok()
                .and_then(|value| deref_dict(document, value))?;
            if !matches!(action.get(b"S").and_then(Object::as_name), Ok(b"GoTo")) {
                return None;
            }
            action.get(b"D").ok()
        })
        .collect()
}

fn named_destinations(
    document: &Document,
    pages: &std::collections::BTreeMap<u32, ObjectId>,
) -> HashMap<String, bool> {
    let Ok(root) = document.trailer.get(b"Root") else {
        return HashMap::new();
    };
    let Some(catalog) = deref_dict(document, root) else {
        return HashMap::new();
    };

    let mut entries = HashMap::new();
    if let Ok(value) = catalog.get(b"Dests") {
        if let Some(destinations) = deref_dict(document, value) {
            for (name, destination) in destinations.iter() {
                entries.insert(normalize_destination_name(name), destination.clone());
            }
        }
    }
    if let Some(names) = catalog
        .get(b"Names")
        .ok()
        .and_then(|value| deref_dict(document, value))
    {
        if let Some(tree) = names
            .get(b"Dests")
            .ok()
            .and_then(|value| deref_dict(document, value))
        {
            collect_name_tree(document, tree, &mut entries, 0);
        }
    }

    let page_ids: HashSet<ObjectId> = pages.values().copied().collect();
    entries
        .into_iter()
        .map(|(name, destination)| {
            let resolves = destination_page(document, &destination)
                .is_some_and(|page| page_ids.contains(&page));
            (name, resolves)
        })
        .collect()
}

fn collect_name_tree(
    document: &Document,
    tree: &Dictionary,
    entries: &mut HashMap<String, Object>,
    depth: usize,
) {
    if depth > 32 {
        return;
    }
    if let Ok(value) = tree.get(b"Names") {
        if let Ok(names) = deref_array(document, value) {
            let (pairs, _) = names.as_chunks::<2>();
            for pair in pairs {
                if let Some(name) = object_destination_name(document, &pair[0]) {
                    entries.insert(name, pair[1].clone());
                }
            }
        }
    }
    if let Ok(value) = tree.get(b"Kids") {
        if let Ok(kids) = deref_array(document, value) {
            for kid in kids {
                if let Some(child) = deref_dict(document, kid) {
                    collect_name_tree(document, child, entries, depth + 1);
                }
            }
        }
    }
}

fn named_destination_target(document: &Document, destination: &Object) -> Option<String> {
    object_destination_name(document, destination)
}

fn object_destination_name(document: &Document, object: &Object) -> Option<String> {
    let (_, object) = document.dereference(object).ok()?;
    match object {
        Object::Name(name) | Object::String(name, _) => Some(normalize_destination_name(name)),
        _ => None,
    }
}

fn normalize_destination_name(name: &[u8]) -> String {
    String::from_utf8_lossy(name).to_lowercase()
}

fn destination_page(document: &Document, destination: &Object) -> Option<ObjectId> {
    let (_, destination) = document.dereference(destination).ok()?;
    let destination = match destination {
        Object::Dictionary(dict) => dict.get(b"D").ok()?,
        value => value,
    };
    let (_, destination) = document.dereference(destination).ok()?;
    let Object::Array(values) = destination else {
        return None;
    };
    values.first()?.as_reference().ok()
}

fn deref_array<'a>(document: &'a Document, object: &'a Object) -> Result<&'a Vec<Object>, ()> {
    match object {
        Object::Array(values) => Ok(values),
        Object::Reference(id) => document
            .get_object(*id)
            .ok()
            .and_then(|o| o.as_array().ok())
            .ok_or(()),
        _ => Err(()),
    }
}

fn deref_dict<'a>(document: &'a Document, object: &'a Object) -> Option<&'a Dictionary> {
    match object {
        Object::Dictionary(dict) => Some(dict),
        Object::Reference(id) => document.get_object(*id).ok()?.as_dict().ok(),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug)]
struct PageBox {
    left: f64,
    bottom: f64,
    right: f64,
    top: f64,
}

impl Default for PageBox {
    fn default() -> Self {
        Self {
            left: 0.0,
            bottom: 0.0,
            right: 612.0,
            top: 792.0,
        }
    }
}

fn page_box(document: &Document, page_id: ObjectId) -> PageBox {
    let mut current = Some(page_id);
    let mut visited = HashSet::new();
    let mut crop_box = None;
    let mut media_box = None;

    while let Some(id) = current {
        if !visited.insert(id) || visited.len() > 32 {
            break;
        }
        let Ok(node) = document.get_dictionary(id) else {
            break;
        };
        if crop_box.is_none() {
            crop_box = node
                .get(b"CropBox")
                .ok()
                .and_then(|value| parse_page_box(document, value));
        }
        if media_box.is_none() {
            media_box = node
                .get(b"MediaBox")
                .ok()
                .and_then(|value| parse_page_box(document, value));
        }
        current = node.get(b"Parent").and_then(Object::as_reference).ok();
    }

    crop_box.or(media_box).unwrap_or_default()
}

fn parse_page_box(document: &Document, value: &Object) -> Option<PageBox> {
    let (_, value) = document.dereference(value).ok()?;
    let array = value.as_array().ok()?;
    if array.len() != 4 {
        return None;
    }
    let x1 = number(&array[0]);
    let y1 = number(&array[1]);
    let x2 = number(&array[2]);
    let y2 = number(&array[3]);
    Some(PageBox {
        left: x1.min(x2),
        bottom: y1.min(y2),
        right: x1.max(x2),
        top: y1.max(y2),
    })
}

fn number(value: &Object) -> f64 {
    match value {
        Object::Integer(v) => *v as f64,
        Object::Real(v) => *v as f64,
        _ => 0.0,
    }
}

#[derive(Clone, Copy, Debug)]
struct Matrix {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    f: f64,
}

impl Matrix {
    const IDENTITY: Self = Self {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        e: 0.0,
        f: 0.0,
    };

    fn from_operands(operands: &[Object]) -> Self {
        Self {
            a: number(&operands[0]),
            b: number(&operands[1]),
            c: number(&operands[2]),
            d: number(&operands[3]),
            e: number(&operands[4]),
            f: number(&operands[5]),
        }
    }

    fn translation(x: f64, y: f64) -> Self {
        Self {
            e: x,
            f: y,
            ..Self::IDENTITY
        }
    }

    /// Compose a local-space transform after this transform.
    fn concat(self, local: Self) -> Self {
        Self {
            a: self.a * local.a + self.c * local.b,
            b: self.b * local.a + self.d * local.b,
            c: self.a * local.c + self.c * local.d,
            d: self.b * local.c + self.d * local.d,
            e: self.a * local.e + self.c * local.f + self.e,
            f: self.b * local.e + self.d * local.f + self.f,
        }
    }

    fn point(self, x: f64, y: f64) -> (f64, f64) {
        (
            self.a * x + self.c * y + self.e,
            self.b * x + self.d * y + self.f,
        )
    }
}

#[derive(Clone, Copy, Debug)]
struct TextState {
    ctm: Matrix,
    font_size: f64,
    horizontal_scale: f64,
    rise: f64,
    leading: f64,
}

impl Default for TextState {
    fn default() -> Self {
        Self {
            ctm: Matrix::IDENTITY,
            font_size: 12.0,
            horizontal_scale: 1.0,
            rise: 0.0,
            leading: 0.0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct BoundsOverflow {
    axis: &'static str,
    side: &'static str,
    coordinate: f64,
    boundary: f64,
}

fn inspect_operations(
    content: &Content,
    bounds: PageBox,
    tolerance: f64,
) -> (bool, Vec<BoundsOverflow>) {
    let mut has_color = false;
    let mut state = TextState::default();
    let mut state_stack = Vec::new();
    let mut text_matrix = Matrix::IDENTITY;
    let mut line_matrix = Matrix::IDENTITY;
    let mut worst: [Option<BoundsOverflow>; 4] = [None; 4];
    for op in &content.operations {
        match op.operator.as_str() {
            "rg" | "RG" | "k" | "K" | "sc" | "SC" | "scn" | "SCN" => {
                if op.operands.iter().any(|v| number(v).abs() > f64::EPSILON) {
                    has_color = true;
                }
            }
            "cm" if op.operands.len() >= 6 => {
                state.ctm = state.ctm.concat(Matrix::from_operands(&op.operands));
            }
            "q" => state_stack.push(state),
            "Q" => state = state_stack.pop().unwrap_or_default(),
            "BT" => {
                text_matrix = Matrix::IDENTITY;
                line_matrix = Matrix::IDENTITY;
            }
            "Tf" if op.operands.len() >= 2 => {
                state.font_size = number(&op.operands[1]).abs().max(1.0);
            }
            "Tz" if !op.operands.is_empty() => {
                state.horizontal_scale = number(&op.operands[0]).abs() / 100.0;
            }
            "Ts" if !op.operands.is_empty() => state.rise = number(&op.operands[0]),
            "TL" if !op.operands.is_empty() => state.leading = number(&op.operands[0]),
            "TD" if op.operands.len() >= 2 => {
                state.leading = -number(&op.operands[1]);
                line_matrix = line_matrix.concat(Matrix::translation(
                    number(&op.operands[0]),
                    number(&op.operands[1]),
                ));
                text_matrix = line_matrix;
            }
            "Td" if op.operands.len() >= 2 => {
                line_matrix = line_matrix.concat(Matrix::translation(
                    number(&op.operands[0]),
                    number(&op.operands[1]),
                ));
                text_matrix = line_matrix;
            }
            "Tm" if op.operands.len() >= 6 => {
                text_matrix = Matrix::from_operands(&op.operands);
                line_matrix = text_matrix;
            }
            "T*" => {
                line_matrix = line_matrix.concat(Matrix::translation(0.0, -state.leading));
                text_matrix = line_matrix;
            }
            "Tj" => {
                if let Some(value) = op.operands.first() {
                    paint_text(
                        value,
                        &mut text_matrix,
                        state,
                        bounds,
                        tolerance,
                        &mut worst,
                    );
                }
            }
            "'" => {
                line_matrix = line_matrix.concat(Matrix::translation(0.0, -state.leading));
                text_matrix = line_matrix;
                if let Some(value) = op.operands.first() {
                    paint_text(
                        value,
                        &mut text_matrix,
                        state,
                        bounds,
                        tolerance,
                        &mut worst,
                    );
                }
            }
            "\"" => {
                line_matrix = line_matrix.concat(Matrix::translation(0.0, -state.leading));
                text_matrix = line_matrix;
                if let Some(value) = op.operands.get(2) {
                    paint_text(
                        value,
                        &mut text_matrix,
                        state,
                        bounds,
                        tolerance,
                        &mut worst,
                    );
                }
            }
            "TJ" => {
                if let Some(Object::Array(values)) = op.operands.first() {
                    for value in values {
                        if matches!(value, Object::String(_, _)) {
                            paint_text(
                                value,
                                &mut text_matrix,
                                state,
                                bounds,
                                tolerance,
                                &mut worst,
                            );
                        } else {
                            let adjustment =
                                -number(value) / 1000.0 * state.font_size * state.horizontal_scale;
                            text_matrix = text_matrix.concat(Matrix::translation(adjustment, 0.0));
                        }
                    }
                }
            }
            _ => {}
        }
    }
    (has_color, worst.into_iter().flatten().collect())
}

fn paint_text(
    value: &Object,
    text_matrix: &mut Matrix,
    state: TextState,
    bounds: PageBox,
    tolerance: f64,
    worst: &mut [Option<BoundsOverflow>; 4],
) {
    let advance = string_len(value) as f64 * state.font_size * 0.58 * state.horizontal_scale;
    if advance <= f64::EPSILON {
        return;
    }

    let placement = state.ctm.concat(*text_matrix);
    let bottom = state.rise - state.font_size * 0.2;
    let top = state.rise + state.font_size * 0.8;
    let corners = [
        placement.point(0.0, bottom),
        placement.point(advance, bottom),
        placement.point(0.0, top),
        placement.point(advance, top),
    ];
    let min_x = corners
        .iter()
        .map(|(x, _)| *x)
        .fold(f64::INFINITY, f64::min);
    let max_x = corners
        .iter()
        .map(|(x, _)| *x)
        .fold(f64::NEG_INFINITY, f64::max);
    let min_y = corners
        .iter()
        .map(|(_, y)| *y)
        .fold(f64::INFINITY, f64::min);
    let max_y = corners
        .iter()
        .map(|(_, y)| *y)
        .fold(f64::NEG_INFINITY, f64::max);

    update_overflow(
        &mut worst[0],
        min_x < bounds.left - tolerance,
        BoundsOverflow {
            axis: "x",
            side: "left",
            coordinate: min_x,
            boundary: bounds.left,
        },
    );
    update_overflow(
        &mut worst[1],
        max_x > bounds.right + tolerance,
        BoundsOverflow {
            axis: "x",
            side: "right",
            coordinate: max_x,
            boundary: bounds.right,
        },
    );
    update_overflow(
        &mut worst[2],
        min_y < bounds.bottom - tolerance,
        BoundsOverflow {
            axis: "y",
            side: "bottom",
            coordinate: min_y,
            boundary: bounds.bottom,
        },
    );
    update_overflow(
        &mut worst[3],
        max_y > bounds.top + tolerance,
        BoundsOverflow {
            axis: "y",
            side: "top",
            coordinate: max_y,
            boundary: bounds.top,
        },
    );

    *text_matrix = text_matrix.concat(Matrix::translation(advance, 0.0));
}

fn update_overflow(current: &mut Option<BoundsOverflow>, outside: bool, candidate: BoundsOverflow) {
    if !outside {
        return;
    }
    let distance = (candidate.coordinate - candidate.boundary).abs();
    if current
        .map(|value| distance > (value.coordinate - value.boundary).abs())
        .unwrap_or(true)
    {
        *current = Some(candidate);
    }
}

fn string_len(value: &Object) -> usize {
    match value {
        Object::String(bytes, _) if bytes.starts_with(&[0xfe, 0xff]) => {
            bytes.len().saturating_sub(2) / 2
        }
        Object::String(bytes, _) => bytes.len(),
        _ => 0,
    }
}

fn normalize(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn truncate(value: &str, max: usize) -> String {
    if value.chars().count() <= max {
        value.to_owned()
    } else {
        format!("{}…", value.chars().take(max).collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::content::Operation;

    #[test]
    fn detects_color_and_overflow() {
        let content = Content {
            operations: vec![
                Operation::new("rg", vec![0.into(), 0.into(), 1.into()]),
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec![Object::Name(b"F1".to_vec()), 12.into()]),
                Operation::new(
                    "Tm",
                    vec![1.into(), 0.into(), 0.into(), 1.into(), 90.into(), 20.into()],
                ),
                Operation::new("Tj", vec![Object::string_literal("some very long line")]),
            ],
        };
        let (color, overflow) = inspect_operations(
            &content,
            PageBox {
                left: 0.0,
                bottom: 0.0,
                right: 100.0,
                top: 100.0,
            },
            0.0,
        );
        assert!(color);
        assert!(overflow.iter().any(|item| item.side == "right"));
    }
}
