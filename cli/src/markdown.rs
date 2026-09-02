use std::collections::HashSet;
use std::ops::Range;
use std::path::Path;

use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};

use crate::report::{Finding, Severity};

#[derive(Debug, Clone)]
pub struct Fence {
    pub line: usize,
    pub language: Option<String>,
    pub lines: Vec<String>,
}

#[derive(Debug, Default)]
pub struct SourceContract {
    pub headings: HashSet<String>,
    pub internal_links: Vec<(usize, String)>,
    pub fences: Vec<Fence>,
    pub findings: Vec<Finding>,
}

pub fn parse_markdown(path: &Path, source: &str) -> SourceContract {
    let mut contract = SourceContract::default();
    if source.trim().is_empty() {
        contract.findings.push(Finding::new(
            "source.empty",
            Severity::Error,
            "Markdown source is empty",
            "Add content before producing a release PDF.",
            None,
        ));
        return contract;
    }

    let mut options = Options::empty();
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    let mut active_heading: Option<(Option<String>, String)> = None;
    let mut active_fence: Option<(usize, Option<String>, String)> = None;

    for (event, range) in Parser::new_ext(source, options).into_offset_iter() {
        match event {
            Event::Start(Tag::Heading { id, .. }) => {
                active_heading = Some((id.map(|value| value.into_string()), String::new()));
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some((explicit_id, text)) = active_heading.take() {
                    let target = explicit_id.unwrap_or_else(|| pandoc_slugify(&text));
                    if !target.is_empty() {
                        contract.headings.insert(normalize_fragment(&target));
                    }
                }
            }
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(info))) => {
                let language = info
                    .split_whitespace()
                    .next()
                    .filter(|value| !value.is_empty())
                    .map(str::to_owned);
                active_fence = Some((line_number(source, range), language, String::new()));
            }
            Event::Text(text) if active_fence.is_some() => {
                if let Some((_, _, body)) = &mut active_fence {
                    body.push_str(&text);
                }
            }
            Event::End(TagEnd::CodeBlock) => {
                if let Some((line, language, body)) = active_fence.take() {
                    let mut lines = body.lines().map(str::to_owned).collect::<Vec<_>>();
                    if body.ends_with('\n') && lines.last().is_some_and(String::is_empty) {
                        lines.pop();
                    }
                    contract.fences.push(Fence {
                        line,
                        language,
                        lines,
                    });
                }
            }
            Event::Start(Tag::Link { dest_url, .. }) => {
                if let Some(target) = dest_url.strip_prefix('#').filter(|value| !value.is_empty()) {
                    contract
                        .internal_links
                        .push((line_number(source, range), normalize_fragment(target)));
                }
            }
            Event::Text(text) | Event::Code(text) if active_heading.is_some() => {
                if let Some((_, heading)) = &mut active_heading {
                    heading.push_str(&text);
                }
            }
            _ => {}
        }
    }

    if let Some(line) = unclosed_fence_line(source) {
        contract.findings.push(Finding::new(
            "source.unclosed-fence",
            Severity::Error,
            format!("Code fence opened on line {line} is not closed"),
            "Close the fence before rendering the PDF.",
            Some(line),
        ));
    }

    for (line, target) in &contract.internal_links {
        if !contract.headings.contains(target) {
            contract.findings.push(Finding::new(
                "link.missing-source-target",
                Severity::Error,
                format!("Internal link #{target} has no matching Markdown heading"),
                "Fix the fragment or add the missing heading.",
                Some(*line),
            ));
        }
    }

    if contract.fences.is_empty() {
        contract.findings.push(Finding::new(
            "source.no-code-fences",
            Severity::Info,
            format!("{} contains no code fences", path.display()),
            "Link and page-bound checks will still run.",
            None,
        ));
    }
    contract
}

fn line_number(source: &str, range: Range<usize>) -> usize {
    source[..range.start.min(source.len())]
        .bytes()
        .filter(|byte| *byte == b'\n')
        .count()
        + 1
}

/// Pandoc's `auto_identifiers` rule: keep letters, digits, `_`, `-`, and `.`,
/// turn whitespace into `-`, lowercase, then discard everything before the
/// first letter. Explicit `{#id}` attributes bypass this derivation.
fn pandoc_slugify(value: &str) -> String {
    let mut out = String::new();
    let mut dash = false;
    for ch in value.to_lowercase().chars() {
        if ch.is_alphanumeric() || ch == '_' || ch == '-' || ch == '.' {
            if dash && !out.is_empty() && !out.ends_with('-') {
                out.push('-');
            }
            dash = false;
            out.push(ch);
        } else if ch.is_whitespace() {
            dash = true;
        }
    }
    let out = out.trim_matches('-');
    out.char_indices()
        .find(|(_, ch)| ch.is_alphabetic())
        .map(|(index, _)| out[index..].to_owned())
        .unwrap_or_else(|| "section".to_owned())
}

fn normalize_fragment(value: &str) -> String {
    value.to_lowercase()
}

fn unclosed_fence_line(source: &str) -> Option<usize> {
    let mut open: Option<(usize, char, usize)> = None;
    for (offset, raw) in source.lines().enumerate() {
        let indent = raw.len() - raw.trim_start_matches(' ').len();
        if indent > 3 {
            continue;
        }
        let trimmed = &raw[indent..];
        if let Some((_, marker, width)) = open {
            let closing = trimmed.chars().take_while(|ch| *ch == marker).count();
            if closing >= width && trimmed[closing..].trim().is_empty() {
                open = None;
            }
        } else {
            let marker = trimmed.chars().next().unwrap_or(' ');
            let width = trimmed.chars().take_while(|ch| *ch == marker).count();
            if (marker == '`' || marker == '~') && width >= 3 {
                open = Some((offset + 1, marker, width));
            }
        }
    }
    open.map(|(line, _, _)| line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_fences_headings_and_links() {
        let source = "# Start here\n[Jump](#start-here)\n```rust\nfn main() {}\n```\n";
        let contract = parse_markdown(Path::new("manual.md"), source);
        assert!(contract
            .findings
            .iter()
            .all(|f| f.severity != Severity::Error));
        assert_eq!(contract.fences[0].language.as_deref(), Some("rust"));
        assert_eq!(contract.fences[0].line, 3);
        assert!(contract.headings.contains("start-here"));
    }

    #[test]
    fn reports_source_contract_errors() {
        let source = "[Missing](#nowhere)\n```js\nalert(1)\n";
        let contract = parse_markdown(Path::new("manual.md"), source);
        assert_eq!(
            contract
                .findings
                .iter()
                .filter(|f| f.severity == Severity::Error)
                .count(),
            2
        );
    }

    #[test]
    fn parses_commonmark_setext_and_pandoc_heading_ids() {
        let source = "Retry policy\n------------\n[Setext](#retry-policy)\n\n## Retry behavior {#retry-explicit}\n[Explicit](#retry-explicit)\n";
        let contract = parse_markdown(Path::new("manual.md"), source);
        assert!(contract
            .findings
            .iter()
            .all(|finding| finding.severity != Severity::Error));
        assert!(contract.headings.contains("retry-policy"));
        assert!(contract.headings.contains("retry-explicit"));
    }
}
