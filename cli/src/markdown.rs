use std::collections::HashSet;
use std::path::Path;

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

struct ActiveFence {
    line: usize,
    marker: char,
    width: usize,
    language: Option<String>,
    lines: Vec<String>,
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

    let mut active: Option<ActiveFence> = None;
    for (offset, raw) in source.lines().enumerate() {
        let line_no = offset + 1;
        let trimmed = raw.trim_start();
        if let Some(open) = &mut active {
            let closing = trimmed.chars().take_while(|c| *c == open.marker).count();
            if closing >= open.width && trimmed[closing..].trim().is_empty() {
                contract.fences.push(Fence {
                    line: open.line,
                    language: open.language.take(),
                    lines: std::mem::take(&mut open.lines),
                });
                active = None;
            } else {
                open.lines.push(raw.to_owned());
            }
            continue;
        }

        let marker = trimmed.chars().next().unwrap_or(' ');
        if marker == '`' || marker == '~' {
            let width = trimmed.chars().take_while(|c| *c == marker).count();
            if width >= 3 {
                let info = trimmed[width..].trim();
                let language = info
                    .split_whitespace()
                    .next()
                    .filter(|s| !s.is_empty())
                    .map(str::to_owned);
                active = Some(ActiveFence {
                    line: line_no,
                    marker,
                    width,
                    language,
                    lines: Vec::new(),
                });
                continue;
            }
        }

        if let Some(text) = heading_text(trimmed) {
            contract.headings.insert(slugify(text));
        }
        extract_fragment_links(raw, line_no, &mut contract.internal_links);
    }

    if let Some(open) = active {
        contract.findings.push(Finding::new(
            "source.unclosed-fence",
            Severity::Error,
            format!("Code fence opened on line {} is not closed", open.line),
            "Close the fence before rendering the PDF.",
            Some(open.line),
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

fn heading_text(line: &str) -> Option<&str> {
    let marks = line.chars().take_while(|c| *c == '#').count();
    if (1..=6).contains(&marks) && line.as_bytes().get(marks) == Some(&b' ') {
        Some(line[marks + 1..].trim_end_matches('#').trim())
    } else {
        None
    }
}

fn slugify(value: &str) -> String {
    let mut out = String::new();
    let mut dash = false;
    for ch in value.to_lowercase().chars() {
        if ch.is_alphanumeric() || ch == '_' || ch == '-' {
            if dash && !out.is_empty() && !out.ends_with('-') {
                out.push('-');
            }
            dash = false;
            out.push(ch);
        } else if ch.is_whitespace() {
            dash = true;
        }
    }
    out.trim_matches('-').to_owned()
}

fn extract_fragment_links(line: &str, line_no: usize, output: &mut Vec<(usize, String)>) {
    let mut rest = line;
    while let Some(start) = rest.find("](#") {
        let target_start = start + 3;
        if let Some(end) = rest[target_start..].find(')') {
            let raw = &rest[target_start..target_start + end];
            if !raw.is_empty() {
                output.push((line_no, raw.to_lowercase()));
            }
            rest = &rest[target_start + end + 1..];
        } else {
            break;
        }
    }
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
}
