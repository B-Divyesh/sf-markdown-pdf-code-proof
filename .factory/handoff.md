# Code Proof v0.1 handoff

## Shipped

- A Rust 2021 single-binary CLI, `codeproof`, with a small library surface and
  a documented `check` command.
- Existing-PDF, built-in Pandoc, and custom-renderer workflows. Custom commands
  use placeholder substitution and direct process execution (no implicit
  shell), a clean environment, a temporary workspace, and a configurable
  deadline.
- Markdown contract parsing for headings, fragment links, fenced code,
  languages, empty input, and malformed fences.
- PDF inspection for internal GoTo annotations, source/code text survival and
  flow, page-bound text geometry, and non-default color operations.
- Stable exits (`0` pass, `1` contract defect, `2` operational failure), JSON
  output, warning promotion, and a responsive self-contained HTML proof sheet.
- Vite/vanilla TypeScript documentation in `dist/site`, including a recorded
  proof run, CLI reference, security boundary, privacy and terms pages, offline
  service worker, CSP/cache configuration, and original risograph artwork.
- README-first API documentation, MIT license, changelog, and publish metadata.

## Verification

Run from a clean clone:

```sh
npm ci
npm test
npm run build
cargo clippy --workspace --all-targets -- -D warnings
cargo package --manifest-path cli/Cargo.toml
npm audit
```

Verified on 2026-08-27:

- Rust: 3 unit tests + 6 CLI integration tests passed. These cover the README
  existing-PDF flow, a directly executed custom renderer, JSON/proof output,
  help, source contract failure, placeholder validation, and operational error
  exits.
- Browser: 6 Playwright tests passed against the production build. Axe found no
  serious/critical issues; the suite also checks console errors, semantic
  structure, the recorded run live announcement, a 390x844 layout, offline
  reload, and legal pages.
- Worker URL verifier: HTTP 200; title and `lang` present; one h1; main present;
  zero missing alt attributes; zero unlabeled buttons; zero console errors;
  measured load 556 ms locally.
- Mobile Lighthouse: Performance 99, Accessibility 100, Best Practices 100,
  SEO 100. LCP 2.1 s, CLS 0, TBT 50 ms. Lab INP was not emitted because the
  Lighthouse trace contained no interaction; the replay interaction is covered
  by Playwright.
- Production payload: 2.08 KB JS (0.95 KB gzip), 10.18 KB CSS (3.16 KB gzip),
  206 KB WebP hero, no font payload. The build script enforces 200/50/300 KB
  budgets respectively.
- `npm audit`: 0 vulnerabilities. Clippy passes with warnings denied.
- `cargo package`: verified; 23.2 KB compressed crate. Do not publish from the
  worker. The factory can publish with the same command (without
  `--allow-dirty`) after release metadata is final.
- Release binary: `target/release/codeproof` (1.9 MB in this Linux build).
- Static deployment root: `dist/site/` with `index.html` at that root.

## Known gaps and next steps

- Pandoc was not installed in this worker image, so the adapter could not be
  exercised against a live Pandoc/LaTeX pipeline. Its fixed command construction
  is covered indirectly; add a pinned Pandoc backend to release CI for a golden
  fixture.
- PDF text bounds are estimated from content-stream font size and advances.
  Highly transformed or custom-encoded fonts can still yield false positives or
  negatives. The finding includes the page and coordinate so a reviewer can
  confirm it.
- Highlight detection is deliberately a warning and document-wide heuristic;
  PDF color operators do not consistently preserve source-block identity.
- Internal PDF links are validated by source target and internal GoTo annotation
  counts. A future adapter-specific pass can resolve each named destination for
  exact one-to-one proof across more PDF producers.
- Next: add golden PDFs from Pandoc, Quarto, and browser-print engines; publish
  signed binaries for Linux/macOS/Windows; feed pilot false-positive reports
  into engine-specific geometry profiles.
