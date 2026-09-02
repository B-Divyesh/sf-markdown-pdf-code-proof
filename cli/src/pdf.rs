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
                let overflows = inspect_operations(
                    &document,
                    *id,
                    &content,
                    page_box(&document, *id),
                    overflow_tolerance,
                );
                for overflow in overflows {
                    findings.push(
                        Finding::new(
                            "geometry.text-overflow",
                            Severity::Error,
                            format!(
                                "Text reaches {}={:.1}pt past the {} page boundary at {}={:.1}pt",
                                overflow.axis,
                                overflow.coordinate,
                                overflow.side,
                                overflow.axis,
                                overflow.boundary
                            ),
                            "Move, wrap, or shorten the affected text, then render again.",
                            None,
                        )
                        .on_page(*number),
                    );
                }
            }
        }
    }

    inspect_internal_links(&document, &pages, source, &mut findings);

    let normalized_pdf = normalize(&all_text);
    let mut visual_cursor = 0;
    let fence_visual_ranges = source
        .fences
        .iter()
        .map(|fence| {
            let useful = fence
                .lines
                .iter()
                .map(String::as_str)
                .filter(|line| !line.trim().is_empty())
                .collect::<Vec<_>>();
            let found = matching_visual_line_range(&useful, &visual_lines, visual_cursor);
            if let Some(range) = &found {
                visual_cursor = range.end;
            }
            found
        })
        .collect::<Vec<_>>();

    for (index, fence) in source.fences.iter().enumerate() {
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
                    "Code fence on line {} lost {} of {} non-empty lines",
                    fence.line,
                    missing.len(),
                    useful.len()
                ),
                format!("First missing line: {}", truncate(missing[0], 90)),
                Some(fence.line),
            ));
        } else if !useful.is_empty() && fence_visual_ranges[index].is_none() {
            findings.push(Finding::new(
                "code.flow-changed",
                Severity::Error,
                format!(
                    "Code fence on line {} is present but its line flow changed",
                    fence.line
                ),
                "Restore the source line boundaries and order, then render again.",
                Some(fence.line),
            ));
        }
    }

    if check_highlighting {
        for (index, fence) in source
            .fences
            .iter()
            .enumerate()
            .filter(|(_, fence)| fence.language.is_some())
        {
            let useful = fence
                .lines
                .iter()
                .map(String::as_str)
                .filter(|line| !line.trim().is_empty())
                .collect::<Vec<_>>();
            let Some(range) = &fence_visual_ranges[index] else {
                continue;
            };
            let lines = &visual_lines[range.clone()];
            let has_syntax_color = useful
                .iter()
                .zip(lines)
                .any(|(source, line)| matching_token_color(source, line).unwrap_or(false));
            if !useful.is_empty() && !has_syntax_color {
                findings.push(Finding::new(
                    "code.highlight-not-detected",
                    Severity::Warning,
                    format!(
                        "Language-tagged code fence on line {} has no non-black text",
                        fence.line
                    ),
                    "Confirm syntax highlighting visually or configure the renderer's highlight style.",
                    Some(fence.line),
                ));
            }
        }
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
#[derive(Debug)]
struct VisualLine {
    tokens: Vec<VisualToken>,
}

#[derive(Debug)]
struct VisualToken {
    text: String,
    has_non_black_text: bool,
}

#[derive(Clone, Copy, Debug, Default)]
struct PaintState {
    fill_non_black: bool,
    stroke_non_black: bool,
    render_mode: u8,
}

impl PaintState {
    fn text_is_non_black(self) -> bool {
        matches!(self.render_mode, 0 | 2 | 4 | 6) && self.fill_non_black
            || matches!(self.render_mode, 1 | 2 | 5 | 6) && self.stroke_non_black
    }
}

fn extract_visual_lines(
    document: &Document,
    page_id: ObjectId,
    content: &Content,
) -> Vec<VisualLine> {
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
    let mut current_line = Vec::new();
    let mut lines = Vec::new();
    let mut baseline = None;
    let mut font_size = 12.0_f64;
    let mut paint = PaintState::default();
    let mut paint_stack = Vec::new();

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
            "q" => paint_stack.push(paint),
            "Q" => paint = paint_stack.pop().unwrap_or_default(),
            "g" if !operation.operands.is_empty() => {
                paint.fill_non_black = gray_is_non_black(&operation.operands)
            }
            "G" if !operation.operands.is_empty() => {
                paint.stroke_non_black = gray_is_non_black(&operation.operands)
            }
            "rg" if operation.operands.len() >= 3 => {
                paint.fill_non_black = rgb_is_non_black(&operation.operands)
            }
            "RG" if operation.operands.len() >= 3 => {
                paint.stroke_non_black = rgb_is_non_black(&operation.operands)
            }
            "k" if operation.operands.len() >= 4 => {
                paint.fill_non_black = cmyk_is_non_black(&operation.operands)
            }
            "K" if operation.operands.len() >= 4 => {
                paint.stroke_non_black = cmyk_is_non_black(&operation.operands)
            }
            "sc" | "scn" => paint.fill_non_black = generic_color_is_non_black(&operation.operands),
            "SC" | "SCN" => {
                paint.stroke_non_black = generic_color_is_non_black(&operation.operands)
            }
            "Tr" if !operation.operands.is_empty() => {
                paint.render_mode = number(&operation.operands[0]).clamp(0.0, 7.0) as u8
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
                        paint.text_is_non_black(),
                        decode_text_operands(encoding, &operation.operands),
                    );
                }
            }
            "Tj" | "TJ" => {
                if let Some(encoding) = encodings.get(&current_font) {
                    append_decoded_text(
                        &mut lines,
                        &mut current_line,
                        paint.text_is_non_black(),
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
    lines: &mut Vec<VisualLine>,
    current_line: &mut Vec<(char, bool)>,
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

fn append_decoded_text(
    lines: &mut Vec<VisualLine>,
    current_line: &mut Vec<(char, bool)>,
    has_non_black_text: bool,
    text: String,
) {
    for character in text.chars() {
        if character == '\n' {
            finish_visual_line(lines, current_line);
        } else {
            current_line.push((character, has_non_black_text));
        }
    }
}

fn finish_visual_line(lines: &mut Vec<VisualLine>, current_line: &mut Vec<(char, bool)>) {
    let mut tokens = Vec::new();
    let mut token = String::new();
    let mut token_has_non_black_text = false;
    for (character, colored) in current_line.drain(..) {
        if character.is_whitespace() {
            finish_visual_token(&mut tokens, &mut token, &mut token_has_non_black_text);
        } else {
            token.push(character);
            token_has_non_black_text |= colored;
        }
    }
    finish_visual_token(&mut tokens, &mut token, &mut token_has_non_black_text);
    if !tokens.is_empty() {
        lines.push(VisualLine { tokens });
    }
}

fn finish_visual_token(
    tokens: &mut Vec<VisualToken>,
    token: &mut String,
    has_non_black_text: &mut bool,
) {
    if !token.is_empty() {
        tokens.push(VisualToken {
            text: std::mem::take(token),
            has_non_black_text: *has_non_black_text,
        });
    }
    *has_non_black_text = false;
}

fn matching_visual_line_range(
    source_lines: &[&str],
    visual_lines: &[VisualLine],
    start: usize,
) -> Option<std::ops::Range<usize>> {
    if source_lines.is_empty() {
        return Some(start..start);
    }
    visual_lines[start..]
        .windows(source_lines.len())
        .position(|window| {
            source_lines
                .iter()
                .zip(window.iter())
                .all(|(source, painted)| matching_token_color(source, painted).is_some())
        })
        .map(|offset| {
            let first = start + offset;
            first..first + source_lines.len()
        })
}

fn matching_token_color(source: &str, visual: &VisualLine) -> Option<bool> {
    let source = source.split_whitespace().collect::<Vec<_>>();
    if source.is_empty() {
        return Some(false);
    }
    visual
        .tokens
        .windows(source.len())
        .find(|window| {
            source
                .iter()
                .zip(window.iter())
                .all(|(source, painted)| *source == painted.text)
        })
        .map(|window| window.iter().any(|token| token.has_non_black_text))
}

fn gray_is_non_black(operands: &[Object]) -> bool {
    number(&operands[0]).abs() > f64::EPSILON
}

fn rgb_is_non_black(operands: &[Object]) -> bool {
    operands
        .iter()
        .take(3)
        .any(|value| number(value).abs() > f64::EPSILON)
}

fn cmyk_is_non_black(operands: &[Object]) -> bool {
    operands
        .iter()
        .take(3)
        .any(|value| number(value).abs() > f64::EPSILON)
        || (number(&operands[3]) - 1.0).abs() > f64::EPSILON
}

fn generic_color_is_non_black(operands: &[Object]) -> bool {
    operands.iter().any(|value| match value {
        Object::Name(_) => true,
        _ => number(value).abs() > f64::EPSILON,
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
struct FontBox {
    bottom: f64,
    top: f64,
}

#[derive(Clone, Debug)]
struct FontMetrics {
    widths: HashMap<u32, f64>,
    default_width: f64,
    code_bytes: usize,
    base_font: String,
    bbox: FontBox,
}

impl Default for FontMetrics {
    fn default() -> Self {
        Self {
            widths: HashMap::new(),
            // Unknown glyphs are treated conservatively as one em. A PDF's
            // /Widths, /W, /MissingWidth, or standard-font metrics replace it.
            default_width: 1000.0,
            code_bytes: 1,
            base_font: String::new(),
            bbox: FontBox {
                bottom: -200.0,
                top: 800.0,
            },
        }
    }
}

impl FontMetrics {
    fn text_width(&self, bytes: &[u8]) -> (f64, usize, usize) {
        let mut width = 0.0;
        let mut glyphs = 0;
        let mut spaces = 0;
        if self.code_bytes == 2 {
            for pair in bytes.chunks(2) {
                let code = if pair.len() == 2 {
                    u32::from(u16::from_be_bytes([pair[0], pair[1]]))
                } else {
                    u32::from(pair[0])
                };
                width += self.width(code);
                glyphs += 1;
            }
        } else {
            for byte in bytes {
                let code = u32::from(*byte);
                width += self.width(code);
                glyphs += 1;
                spaces += usize::from(*byte == b' ');
            }
        }
        (width, glyphs, spaces)
    }

    fn width(&self, code: u32) -> f64 {
        self.widths
            .get(&code)
            .copied()
            .or_else(|| standard_glyph_width(&self.base_font, code))
            .unwrap_or(self.default_width)
    }
}

fn page_font_metrics(document: &Document, page_id: ObjectId) -> BTreeMap<Vec<u8>, FontMetrics> {
    document
        .get_page_fonts(page_id)
        .map(|fonts| {
            fonts
                .into_iter()
                .map(|(name, font)| (name, font_metrics(document, font)))
                .collect()
        })
        .unwrap_or_default()
}

fn font_metrics(document: &Document, font: &Dictionary) -> FontMetrics {
    let mut metrics = FontMetrics {
        base_font: font
            .get(b"BaseFont")
            .and_then(Object::as_name)
            .map(|name| String::from_utf8_lossy(name).into_owned())
            .unwrap_or_default(),
        ..FontMetrics::default()
    };

    if matches!(font.get(b"Subtype").and_then(Object::as_name), Ok(b"Type0")) {
        metrics.code_bytes = 2;
        if let Some(descendant) = font
            .get(b"DescendantFonts")
            .ok()
            .and_then(|value| deref_array(document, value).ok())
            .and_then(|fonts| fonts.first())
            .and_then(|value| deref_dict(document, value))
        {
            metrics.default_width = descendant
                .get(b"DW")
                .ok()
                .map(number)
                .filter(|width| *width > 0.0)
                .unwrap_or(1000.0);
            if let Ok(widths) = descendant.get(b"W") {
                parse_cid_widths(document, widths, &mut metrics.widths);
            }
            metrics.bbox = font_bbox(document, descendant).unwrap_or(metrics.bbox);
        }
        return metrics;
    }

    let first_char = font.get(b"FirstChar").ok().map(number).unwrap_or(0.0) as u32;
    if let Some(widths) = font
        .get(b"Widths")
        .ok()
        .and_then(|value| deref_array(document, value).ok())
    {
        for (offset, value) in widths.iter().enumerate() {
            metrics
                .widths
                .insert(first_char + offset as u32, number(value));
        }
    }
    if let Some(descriptor) = font
        .get(b"FontDescriptor")
        .ok()
        .and_then(|value| deref_dict(document, value))
    {
        metrics.default_width = descriptor
            .get(b"MissingWidth")
            .ok()
            .map(number)
            .filter(|width| *width > 0.0)
            .unwrap_or(metrics.default_width);
    }
    metrics.bbox =
        font_bbox(document, font).unwrap_or_else(|| standard_font_box(&metrics.base_font));
    metrics
}

fn parse_cid_widths(document: &Document, value: &Object, output: &mut HashMap<u32, f64>) {
    let Ok(values) = deref_array(document, value) else {
        return;
    };
    let mut index = 0;
    while index < values.len() {
        let start = number(&values[index]) as u32;
        index += 1;
        let Some(next) = values.get(index) else {
            break;
        };
        if let Ok(widths) = deref_array(document, next) {
            for (offset, width) in widths.iter().enumerate() {
                output.insert(start + offset as u32, number(width));
            }
            index += 1;
        } else if let (Some(end), Some(width)) = (values.get(index), values.get(index + 1)) {
            let end = number(end) as u32;
            let width = number(width);
            for code in start..=end {
                output.insert(code, width);
            }
            index += 2;
        } else {
            break;
        }
    }
}

fn font_bbox(document: &Document, font: &Dictionary) -> Option<FontBox> {
    let descriptor = font
        .get(b"FontDescriptor")
        .ok()
        .and_then(|value| deref_dict(document, value));
    let value = descriptor
        .and_then(|dict| dict.get(b"FontBBox").ok())
        .or_else(|| font.get(b"FontBBox").ok())?;
    let values = deref_array(document, value).ok()?;
    if values.len() != 4 {
        return None;
    }
    Some(FontBox {
        bottom: number(&values[1]),
        top: number(&values[3]),
    })
}

fn standard_font_box(base_font: &str) -> FontBox {
    let name = base_font.rsplit('+').next().unwrap_or(base_font);
    if name.starts_with("Helvetica") {
        FontBox {
            bottom: -225.0,
            top: 931.0,
        }
    } else if name.starts_with("Times") {
        FontBox {
            bottom: -218.0,
            top: 898.0,
        }
    } else if name.starts_with("Courier") {
        FontBox {
            bottom: -250.0,
            top: 805.0,
        }
    } else {
        FontMetrics::default().bbox
    }
}

fn standard_glyph_width(base_font: &str, code: u32) -> Option<f64> {
    let name = base_font.rsplit('+').next().unwrap_or(base_font);
    if name.starts_with("Courier") && (32..=255).contains(&code) {
        return Some(600.0);
    }
    if !name.starts_with("Helvetica") {
        return None;
    }
    let width = match code {
        32 => 278,
        33 => 278,
        34 => 355,
        35..=36 => 556,
        37 => 889,
        38 => 667,
        39 => 191,
        40..=41 => 333,
        42 => 389,
        43 => 584,
        44 => 278,
        45 => 333,
        46..=47 => 278,
        48..=57 => 556,
        58..=59 => 278,
        60..=62 => 584,
        63 => 556,
        64 => 1015,
        65..=66 => 667,
        67..=68 => 722,
        69 => 667,
        70 => 611,
        71 => 778,
        72 => 722,
        73 => 278,
        74 => 500,
        75 => 667,
        76 => 556,
        77 => 833,
        78 => 722,
        79 => 778,
        80 => 667,
        81 => 778,
        82 => 722,
        83 => 667,
        84 => 611,
        85 => 722,
        86 => 667,
        87 => 944,
        88..=89 => 667,
        90 => 611,
        91..=93 => 278,
        94 => 469,
        95 => 556,
        96 => 333,
        97..=98 => 556,
        99 => 500,
        100..=101 => 556,
        102 => 278,
        103..=104 => 556,
        105..=106 => 222,
        107 => 500,
        108 => 222,
        109 => 833,
        110..=113 => 556,
        114 => 333,
        115 => 500,
        116 => 278,
        117 => 556,
        118 => 500,
        119 => 722,
        120..=122 => 500,
        123 | 125 => 334,
        124 => 260,
        126 => 584,
        _ => return None,
    };
    Some(f64::from(width))
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

#[derive(Clone, Debug)]
struct TextState {
    ctm: Matrix,
    font_name: Vec<u8>,
    font_size: f64,
    horizontal_scale: f64,
    character_spacing: f64,
    word_spacing: f64,
    rise: f64,
    leading: f64,
}

impl Default for TextState {
    fn default() -> Self {
        Self {
            ctm: Matrix::IDENTITY,
            font_name: Vec::new(),
            font_size: 12.0,
            horizontal_scale: 1.0,
            character_spacing: 0.0,
            word_spacing: 0.0,
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
    document: &Document,
    page_id: ObjectId,
    content: &Content,
    bounds: PageBox,
    tolerance: f64,
) -> Vec<BoundsOverflow> {
    let fonts = page_font_metrics(document, page_id);
    let mut state = TextState::default();
    let mut state_stack = Vec::new();
    let mut text_matrix = Matrix::IDENTITY;
    let mut line_matrix = Matrix::IDENTITY;
    let mut worst: [Option<BoundsOverflow>; 4] = [None; 4];
    for op in &content.operations {
        match op.operator.as_str() {
            "cm" if op.operands.len() >= 6 => {
                state.ctm = state.ctm.concat(Matrix::from_operands(&op.operands));
            }
            "q" => state_stack.push(state.clone()),
            "Q" => state = state_stack.pop().unwrap_or_default(),
            "BT" => {
                text_matrix = Matrix::IDENTITY;
                line_matrix = Matrix::IDENTITY;
            }
            "Tf" if op.operands.len() >= 2 => {
                if let Ok(name) = op.operands[0].as_name() {
                    state.font_name.clear();
                    state.font_name.extend_from_slice(name);
                }
                state.font_size = number(&op.operands[1]).abs().max(1.0);
            }
            "Tc" if !op.operands.is_empty() => state.character_spacing = number(&op.operands[0]),
            "Tw" if !op.operands.is_empty() => state.word_spacing = number(&op.operands[0]),
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
                        &state,
                        fonts.get(&state.font_name),
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
                        &state,
                        fonts.get(&state.font_name),
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
                        &state,
                        fonts.get(&state.font_name),
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
                                &state,
                                fonts.get(&state.font_name),
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
    worst.into_iter().flatten().collect()
}

fn paint_text(
    value: &Object,
    text_matrix: &mut Matrix,
    state: &TextState,
    font: Option<&FontMetrics>,
    bounds: PageBox,
    tolerance: f64,
    worst: &mut [Option<BoundsOverflow>; 4],
) {
    let Some(bytes) = string_bytes(value) else {
        return;
    };
    let fallback = FontMetrics::default();
    let font = font.unwrap_or(&fallback);
    let (width, glyphs, spaces) = font.text_width(bytes);
    let spacing = glyphs as f64 * state.character_spacing + spaces as f64 * state.word_spacing;
    let advance = (width / 1000.0 * state.font_size + spacing) * state.horizontal_scale;
    if advance <= f64::EPSILON {
        return;
    }

    let placement = state.ctm.concat(*text_matrix);
    let bottom = state.rise + font.bbox.bottom / 1000.0 * state.font_size;
    let top = state.rise + font.bbox.top / 1000.0 * state.font_size;
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

fn string_bytes(value: &Object) -> Option<&[u8]> {
    match value {
        Object::String(bytes, _) => Some(bytes),
        _ => None,
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
    fn transformed_text_geometry_detects_overflow() {
        let mut document = Document::with_version("1.5");
        let font = document.add_object(lopdf::dictionary! {
            "Type" => "Font", "Subtype" => "Type1", "BaseFont" => "Helvetica"
        });
        let resources = document.add_object(lopdf::dictionary! {
            "Font" => lopdf::dictionary! { "F1" => font }
        });
        let content = Content {
            operations: vec![
                Operation::new(
                    "cm",
                    vec![2.into(), 0.into(), 0.into(), 1.into(), 0.into(), 0.into()],
                ),
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec![Object::Name(b"F1".to_vec()), 12.into()]),
                Operation::new(
                    "Tm",
                    vec![1.into(), 0.into(), 0.into(), 1.into(), 90.into(), 20.into()],
                ),
                Operation::new("Tj", vec![Object::string_literal("some very long line")]),
            ],
        };
        let pages = document.new_object_id();
        let stream = document.add_object(lopdf::Stream::new(
            lopdf::dictionary! {},
            content.encode().unwrap(),
        ));
        let page = document.add_object(lopdf::dictionary! {
            "Type" => "Page", "Parent" => pages, "Contents" => stream,
            "Resources" => resources, "MediaBox" => vec![0.into(), 0.into(), 100.into(), 100.into()]
        });
        document.objects.insert(
            pages,
            Object::Dictionary(lopdf::dictionary! {
                "Type" => "Pages", "Kids" => vec![page.into()], "Count" => 1
            }),
        );
        let overflow = inspect_operations(
            &document,
            page,
            &content,
            PageBox {
                left: 0.0,
                bottom: 0.0,
                right: 100.0,
                top: 100.0,
            },
            0.0,
        );
        assert!(overflow.iter().any(|item| item.side == "right"));
    }
}
