use std::path::Path;

use serde::Serialize;

pub const FAILED_DECISION: &str = "HOLD — do not release";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize)]
pub struct Finding {
    pub code: String,
    pub severity: Severity,
    pub message: String,
    pub help: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_line: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

impl Finding {
    pub fn new(
        code: impl Into<String>,
        severity: Severity,
        message: impl Into<String>,
        help: impl Into<String>,
        source_line: Option<usize>,
    ) -> Self {
        Self {
            code: code.into(),
            severity,
            message: message.into(),
            help: help.into(),
            source_line,
            page: None,
        }
    }

    pub fn on_page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }
}

#[derive(Debug, Serialize)]
pub struct Summary {
    pub passed: bool,
    pub errors: usize,
    pub warnings: usize,
    pub info: usize,
    pub pages: usize,
    pub code_fences: usize,
    pub internal_links: usize,
    pub pdf_link_annotations: usize,
}

#[derive(Debug, Serialize)]
pub struct Report {
    pub schema_version: &'static str,
    pub source: String,
    pub pdf: String,
    pub engine: String,
    pub summary: Summary,
    pub findings: Vec<Finding>,
}

impl Report {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        source: &Path,
        pdf: &Path,
        engine: String,
        findings: Vec<Finding>,
        pages: usize,
        code_fences: usize,
        internal_links: usize,
        pdf_link_annotations: usize,
        deny_warnings: bool,
    ) -> Self {
        let errors = findings
            .iter()
            .filter(|f| f.severity == Severity::Error)
            .count();
        let warnings = findings
            .iter()
            .filter(|f| f.severity == Severity::Warning)
            .count();
        let info = findings
            .iter()
            .filter(|f| f.severity == Severity::Info)
            .count();
        Self {
            schema_version: "1",
            source: source.display().to_string(),
            pdf: pdf.display().to_string(),
            engine,
            summary: Summary {
                passed: errors == 0 && (!deny_warnings || warnings == 0),
                errors,
                warnings,
                info,
                pages,
                code_fences,
                internal_links,
                pdf_link_annotations,
            },
            findings,
        }
    }
}

pub fn write_html(report: &Report, output: &Path) -> Result<(), String> {
    std::fs::create_dir_all(output)
        .map_err(|e| format!("could not create {}: {e}", output.display()))?;
    let status = if report.summary.passed {
        "PASS"
    } else {
        FAILED_DECISION
    };
    let findings = if report.findings.is_empty() {
        "<li class=\"empty\"><strong>No defects found.</strong><span>The PDF contract passed every enabled check.</span></li>".to_owned()
    } else {
        report.findings.iter().map(|f| {
            let location = match (f.source_line, f.page) {
                (Some(line), Some(page)) => format!("Source line {line} · PDF page {page}"),
                (Some(line), None) => format!("Source line {line}"),
                (None, Some(page)) => format!("PDF page {page}"),
                _ => "Document-wide".into(),
            };
            format!("<li class=\"{}\"><div><span class=\"tag\">{}</span><span class=\"where\">{}</span></div><strong>{}</strong><p>{}</p><code>{}</code></li>",
                severity_name(f.severity), severity_name(f.severity), escape(&location), escape(&f.message), escape(&f.help), escape(&f.code))
        }).collect::<Vec<_>>().join("\n")
    };
    let html = format!(
        r#"<!doctype html>
<html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>{status} — Code Proof HTML proof sheet</title><style>
:root{{--paper:#f3e9d2;--sheet:#fff9ea;--ink:#171817;--muted:#55534d;--blue:#184e9e;--red:#a52f25;--green:#28613f;--amber:#75400e}}
*{{box-sizing:border-box}}body{{margin:0;background:var(--paper);color:var(--ink);font:16px/1.55 system-ui,sans-serif}}main{{max-width:1080px;margin:auto;padding:48px 24px 96px}}header{{border-bottom:3px solid;padding-bottom:24px;display:flex;justify-content:space-between;gap:24px;align-items:end}}h1{{font:700 clamp(40px,8vw,80px)/.92 Georgia,serif;margin:8px 0}}.eyebrow,.tag,code{{font:700 12px/1.2 ui-monospace,monospace;text-transform:uppercase;letter-spacing:.08em}}.stamp{{border:4px solid;padding:12px;transform:rotate(-2deg);color:{status_color};font:800 24px ui-monospace,monospace}}dl{{display:grid;grid-template-columns:repeat(4,1fr);gap:16px;margin:32px 0}}dt{{color:var(--muted)}}dd{{font:700 30px Georgia,serif;margin:4px 0}}ul{{padding:0;list-style:none}}li{{background:var(--sheet);border:2px solid;margin:16px 0;padding:20px;box-shadow:4px 4px 0 var(--ink)}}li strong{{display:block;font-size:19px;margin-top:12px}}li p{{margin:6px 0 12px;max-width:70ch}}.tag{{display:inline-block;color:#fff;padding:5px 8px;background:var(--blue)}}.error .tag{{background:var(--red)}}.warning .tag{{background:var(--amber)}}.where{{margin-left:12px;color:var(--muted)}}.paths{{overflow-wrap:anywhere}}@media(max-width:650px){{main{{padding:32px 16px 64px}}header{{align-items:start;flex-direction:column}}dl{{grid-template-columns:1fr 1fr}}.where{{display:block;margin:8px 0 0}}}}
</style></head><body><main><header><div><div class="eyebrow">Code Proof / release evidence</div><h1>{status}</h1><div class="paths">{source}<br>{pdf}</div></div><div class="stamp">{errors} errors<br>{warnings} warnings</div></header>
<dl><div><dt>Pages</dt><dd>{pages}</dd></div><div><dt>Code fences</dt><dd>{fences}</dd></div><div><dt>Source links</dt><dd>{links}</dd></div><div><dt>PDF links</dt><dd>{annotations}</dd></div></dl><h2>Inspection log</h2><ul>{findings}</ul></main></body></html>"#,
        status_color = if report.summary.passed {
            "var(--green)"
        } else {
            "var(--red)"
        },
        source = escape(&report.source),
        pdf = escape(&report.pdf),
        errors = report.summary.errors,
        warnings = report.summary.warnings,
        pages = report.summary.pages,
        fences = report.summary.code_fences,
        links = report.summary.internal_links,
        annotations = report.summary.pdf_link_annotations
    );
    std::fs::write(output.join("index.html"), html)
        .map_err(|e| format!("could not write proof sheet: {e}"))
}

fn severity_name(value: Severity) -> &'static str {
    match value {
        Severity::Error => "error",
        Severity::Warning => "warning",
        Severity::Info => "info",
    }
}

fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
