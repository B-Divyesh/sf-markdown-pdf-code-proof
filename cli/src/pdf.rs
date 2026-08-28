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
                let (colored, overflow) =
                    inspect_operations(&content, page_width(&document, *id), overflow_tolerance);
                has_color |= colored;
                if let Some((x, width)) = overflow {
                    findings.push(Finding::new(
                        "geometry.text-overflow",
                        Severity::Error,
                        format!("Painted text reaches x={x:.1}pt beyond the {width:.1}pt page boundary"),
                        "Wrap or shorten the affected line, then render again.",
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
        } else if useful.len() > 1 && !preserves_line_flow(&useful, &visual_lines) {
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

fn page_width(document: &Document, page_id: ObjectId) -> f64 {
    let Ok(page) = document.get_dictionary(page_id) else {
        return 612.0;
    };
    let box_value = page.get(b"CropBox").or_else(|_| page.get(b"MediaBox"));
    let Ok(array) = box_value.and_then(Object::as_array) else {
        return 612.0;
    };
    if array.len() != 4 {
        return 612.0;
    }
    number(&array[2]) - number(&array[0])
}

fn number(value: &Object) -> f64 {
    match value {
        Object::Integer(v) => *v as f64,
        Object::Real(v) => *v as f64,
        _ => 0.0,
    }
}

fn inspect_operations(
    content: &Content,
    page_width: f64,
    tolerance: f64,
) -> (bool, Option<(f64, f64)>) {
    let mut has_color = false;
    let mut font_size = 12.0;
    let mut text_x = 0.0;
    let mut scale_x = 1.0;
    let mut scale_stack = Vec::new();
    let mut worst = None;
    for op in &content.operations {
        match op.operator.as_str() {
            "rg" | "RG" | "k" | "K" | "sc" | "SC" | "scn" | "SCN" => {
                if op.operands.iter().any(|v| number(v).abs() > f64::EPSILON) {
                    has_color = true;
                }
            }
            "cm" if op.operands.len() >= 6 => scale_x *= number(&op.operands[0]).abs().max(0.01),
            "q" => scale_stack.push(scale_x),
            "Q" => scale_x = scale_stack.pop().unwrap_or(1.0),
            "Tf" if op.operands.len() >= 2 => font_size = number(&op.operands[1]).abs(),
            "Td" | "TD" if op.operands.len() >= 2 => text_x += number(&op.operands[0]),
            "Tm" if op.operands.len() >= 6 => text_x = number(&op.operands[4]),
            "Tj" | "'" | "\"" => {
                if let Some(value) = op.operands.last() {
                    let advance = string_len(value) as f64 * font_size * 0.58 * scale_x;
                    let edge = text_x + advance;
                    if edge > page_width + tolerance && worst.map(|(x, _)| edge > x).unwrap_or(true)
                    {
                        worst = Some((edge, page_width));
                    }
                    text_x += advance;
                }
            }
            "TJ" => {
                if let Some(Object::Array(values)) = op.operands.first() {
                    let advance = values
                        .iter()
                        .map(|v| string_len(v) as f64 * font_size * 0.58 * scale_x)
                        .sum::<f64>();
                    let edge = text_x + advance;
                    if edge > page_width + tolerance && worst.map(|(x, _)| edge > x).unwrap_or(true)
                    {
                        worst = Some((edge, page_width));
                    }
                    text_x += advance;
                }
            }
            "BT" => {
                text_x = 0.0;
                scale_x = 1.0;
            }
            _ => {}
        }
    }
    (has_color, worst)
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
        let (color, overflow) = inspect_operations(&content, 100.0, 0.0);
        assert!(color);
        assert!(overflow.is_some());
    }
}
