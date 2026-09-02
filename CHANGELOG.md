# Changelog

## 0.1.0 — unreleased repair

- Reject one-line code fences painted across multiple PDF baselines.
- Check transformed text against the left, right, top, and bottom CropBox or
  MediaBox edges.
- Add `codeproof demo` with bundled sample data and an isolated proof workspace.
- Sandbox renderer subprocesses with Linux Landlock filesystem allowlists and
  seccomp network denial; refuse renderer execution without those controls.
- Make service-worker precaching production-safe by excluding Static Web Apps'
  deployment-only configuration file and claiming updated clients promptly.

All notable changes follow Keep a Changelog. This project uses Semantic Versioning.

## [Unreleased]

### Fixed

- Generate the browser sample transcript from the real bundled CLI demo and
  keep the failed release wording identical in terminal and HTML output.
- Preserve keyboard focus while Reset demo replays the sample.
- Register the two-lines-merged PDF failure as a release claim.
- Use Markdown, syntax color, and HTML proof sheet consistently in public copy.
- Preserve painted PDF baselines when checking fenced code, so source lines
  flattened into a paragraph are a failing `code.flow-changed` defect.
- Align the documented Rust minimum with the locked dependency floor and
  enforce it in the repository test/CI gates.
- Make mobile horizontal scroll regions explicitly keyboard-focusable, align
  the brand's accessible name with its visible label, and keep every footer
  target at least 44×44 CSS pixels.
- Verify every Markdown fragment against its matching PDF named destination and
  resolved page, rather than treating a count of link annotations as proof.

## [0.1.0] - 2026-08-27

### Added

- Initial CLI, PDF checks, proof sheet, and static documentation site.
