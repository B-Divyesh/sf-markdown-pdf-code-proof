use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Duration;

use clap::{Args, Parser, Subcommand};
use codeproof::render;
use codeproof::report::{write_html, Report, Severity};
use codeproof::{inspect_pdf, parse_markdown};

#[derive(Parser)]
#[command(
    name = "codeproof",
    version,
    about = "Catch broken code and links before a Markdown PDF ships",
    long_about = None,
    after_help = "Examples:\n  codeproof check manual.md --engine pandoc --out proof\n  codeproof check manual.md --pdf dist/manual.pdf --json\n\nExit status: 0 passed, 1 defects found, 2 command could not complete."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Render and inspect a Markdown/PDF release pair
    Check(CheckArgs),
    /// Run an isolated proof with bundled sample data
    Demo(DemoArgs),
}

#[derive(Args)]
struct DemoArgs {
    /// Keep the sample Markdown, PDF, and proof in this directory
    #[arg(long, value_name = "DIR")]
    out: Option<PathBuf>,
}

#[derive(Args)]
struct CheckArgs {
    /// Markdown source that defines the release contract
    #[arg(value_name = "MARKDOWN")]
    source: PathBuf,

    /// Inspect this existing PDF instead of running a renderer
    #[arg(long, value_name = "FILE", conflicts_with_all = ["engine", "engine_command"])]
    pdf: Option<PathBuf>,

    /// Built-in renderer adapter
    #[arg(long, value_name = "NAME", default_value = "pandoc")]
    engine: String,

    /// Custom renderer command with literal {input} and {output} placeholders
    #[arg(long, value_name = "COMMAND", conflicts_with = "pdf")]
    engine_command: Option<String>,

    /// Directory for the self-contained HTML proof sheet
    #[arg(long, value_name = "DIR", default_value = "proof")]
    out: PathBuf,

    /// Print the JSON report instead of the terminal summary
    #[arg(long)]
    json: bool,

    /// Treat warning findings as a failed contract
    #[arg(long)]
    deny_warnings: bool,

    /// Renderer deadline in seconds
    #[arg(long, value_name = "SECONDS", default_value_t = 60, value_parser = clap::value_parser!(u64).range(1..=3600))]
    timeout: u64,

    /// Allowed distance beyond the PDF page edge in points
    #[arg(
        long,
        value_name = "POINTS",
        default_value_t = 2.0,
        allow_hyphen_values = false
    )]
    overflow_tolerance: f64,

    /// Skip the color-operator heuristic for syntax highlighting
    #[arg(long)]
    no_highlight_check: bool,
}

fn main() -> ExitCode {
    match run() {
        Ok(passed) if passed => ExitCode::SUCCESS,
        Ok(_) => ExitCode::from(1),
        Err(message) => {
            eprintln!("codeproof: {message}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<bool, String> {
    let Cli { command } = Cli::parse();
    match command {
        Commands::Check(args) => check(args),
        Commands::Demo(args) => demo(args),
    }
}

fn demo(args: DemoArgs) -> Result<bool, String> {
    let workspace = match args.out {
        Some(path) => {
            std::fs::create_dir_all(&path).map_err(|error| {
                format!(
                    "could not create demo directory {}: {error}",
                    path.display()
                )
            })?;
            path
        }
        None => tempfile::Builder::new()
            .prefix("codeproof-demo-")
            .tempdir()
            .map_err(|error| format!("could not create demo directory: {error}"))?
            .keep(),
    };
    let source_path = workspace.join("sample-manual.md");
    let pdf_path = workspace.join("sample-manual.pdf");
    let proof_path = workspace.join("proof");
    std::fs::write(&source_path, codeproof::demo::SAMPLE_MARKDOWN)
        .map_err(|error| format!("could not write demo Markdown: {error}"))?;
    codeproof::demo::write_sample_pdf(&pdf_path)?;

    let source = parse_markdown(&source_path, codeproof::demo::SAMPLE_MARKDOWN);
    let inspection = inspect_pdf(&pdf_path, &source, 2.0, true)?;
    let report = Report::new(
        &source_path,
        &pdf_path,
        "bundled-demo".into(),
        inspection.findings,
        inspection.pages,
        source.fences.len(),
        source.internal_links.len(),
        inspection.link_annotations,
        false,
    );
    write_html(&report, &proof_path)?;
    println!(
        "DEMO {} — {} expected defect{} found",
        if report.summary.passed {
            "PASS"
        } else {
            "HOLD"
        },
        report.summary.errors,
        if report.summary.errors == 1 { "" } else { "s" }
    );
    for finding in &report.findings {
        println!(
            "  {:?} [{}] {}",
            finding.severity, finding.code, finding.message
        );
    }
    println!("Sample workspace: {}", workspace.display());
    println!(
        "HTML proof sheet: {}",
        proof_path.join("index.html").display()
    );
    Ok(report.summary.passed)
}

fn check(args: CheckArgs) -> Result<bool, String> {
    if !args.source.is_file() {
        return Err(format!(
            "Markdown source not found: {}",
            args.source.display()
        ));
    }
    if args.overflow_tolerance < 0.0 || !args.overflow_tolerance.is_finite() {
        return Err("--overflow-tolerance must be a finite non-negative number".into());
    }
    let markdown = std::fs::read_to_string(&args.source).map_err(|e| {
        format!(
            "could not read {} as UTF-8 Markdown: {e}",
            args.source.display()
        )
    })?;
    let source = parse_markdown(&args.source, &markdown);
    if source
        .findings
        .iter()
        .any(|f| f.severity == Severity::Error)
    {
        let report = Report::new(
            &args.source,
            args.pdf
                .as_deref()
                .unwrap_or_else(|| std::path::Path::new("not-rendered.pdf")),
            "not-run".into(),
            source.findings,
            0,
            source.fences.len(),
            source.internal_links.len(),
            0,
            args.deny_warnings,
        );
        write_html(&report, &args.out)?;
        print_report(&report, args.json)?;
        return Ok(report.summary.passed);
    }

    let rendered = if let Some(pdf) = &args.pdf {
        render::existing(pdf)?
    } else {
        render::render(
            &args.source,
            &args.engine,
            args.engine_command.as_deref(),
            Duration::from_secs(args.timeout),
        )?
    };
    let inspection = inspect_pdf(
        &rendered.path,
        &source,
        args.overflow_tolerance,
        !args.no_highlight_check,
    )?;
    let mut findings = source.findings;
    findings.extend(inspection.findings);
    let report = Report::new(
        &args.source,
        &rendered.path,
        rendered.engine,
        findings,
        inspection.pages,
        source.fences.len(),
        source.internal_links.len(),
        inspection.link_annotations,
        args.deny_warnings,
    );
    write_html(&report, &args.out)?;
    print_report(&report, args.json)?;
    Ok(report.summary.passed)
}

fn print_report(report: &Report, json: bool) -> Result<(), String> {
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(report).map_err(|e| e.to_string())?
        );
        return Ok(());
    }
    println!(
        "{} — {} pages, {} code fences, {} errors, {} warnings",
        if report.summary.passed {
            "PASS"
        } else {
            "HOLD"
        },
        report.summary.pages,
        report.summary.code_fences,
        report.summary.errors,
        report.summary.warnings
    );
    for finding in &report.findings {
        println!(
            "  {:?} [{}] {}",
            finding.severity, finding.code, finding.message
        );
    }
    println!("HTML proof sheet: index.html");
    Ok(())
}
