use std::path::Path;

use lopdf::content::Content;
use lopdf::{Dictionary, Document, Object, ObjectId};

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

    for (number, id) in &pages {
        link_annotations += count_link_annotations(&document, *id);
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

    if link_annotations < source.internal_links.len() {
        findings.push(Finding::new(
            "link.annotations-missing",
            Severity::Error,
            format!(
                "Found {link_annotations} PDF link annotations for {} internal Markdown links",
                source.internal_links.len()
            ),
            "Enable link preservation in the renderer and inspect the proof PDF.",
            source.internal_links.first().map(|(line, _)| *line),
        ));
    }

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
        } else if useful.len() > 1 {
            let joined = useful
                .iter()
                .map(|line| normalize(line))
                .collect::<Vec<_>>()
                .join(" ");
            if !normalized_pdf.contains(&joined) {
                findings.push(Finding::new(
                    "code.flow-changed",
                    Severity::Warning,
                    format!("Code block on line {} is present but its line flow changed", fence.line),
                    "Review this block in the proof sheet; wrapping or reordering may have occurred.",
                    Some(fence.line),
                ));
            }
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

fn count_link_annotations(document: &Document, page_id: ObjectId) -> usize {
    let Ok(page) = document.get_dictionary(page_id) else {
        return 0;
    };
    let Ok(annots) = page.get(b"Annots") else {
        return 0;
    };
    let Ok(items) = deref_array(document, annots) else {
        return 0;
    };
    items
        .iter()
        .filter(|item| {
            let Some(dict) = deref_dict(document, item) else {
                return false;
            };
            matches!(dict.get(b"Subtype").and_then(Object::as_name), Ok(b"Link"))
        })
        .count()
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
    let mut worst = None;
    for op in &content.operations {
        match op.operator.as_str() {
            "rg" | "RG" | "k" | "K" | "sc" | "SC" | "scn" | "SCN" => {
                if op.operands.iter().any(|v| number(v).abs() > f64::EPSILON) {
                    has_color = true;
                }
            }
            "cm" if op.operands.len() >= 6 => scale_x *= number(&op.operands[0]).abs().max(0.01),
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
