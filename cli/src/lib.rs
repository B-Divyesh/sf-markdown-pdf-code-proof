//! The analysis library behind the `codeproof` command.
//!
//! The public surface is intentionally small: parse Markdown source, inspect a
//! PDF, and render a standalone proof sheet.

pub mod demo;
pub mod markdown;
pub mod pdf;
pub mod render;
pub mod report;
pub mod sandbox;

pub use markdown::{parse_markdown, Fence, SourceContract};
pub use pdf::inspect_pdf;
pub use report::{Finding, Report, Severity};
