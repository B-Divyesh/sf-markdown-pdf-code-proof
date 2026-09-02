# Code Proof independent verification 9 handoff

## Result

**FAIL.** Candidate `4c5346a5740217724683957255bee8cb9c31fd1e` was tested
locally and at <https://markdown-pdf-code-proof.sociobot.in> on 2026-09-02.
The live site byte-matches the candidate build and its quality gates pass, but
the CLI can miss two release defects in its advertised core checks.

## Release blockers

1. **High — syntax-color false negative.** A blue non-code rectangle anywhere
   in a PDF makes completely black code count as highlighted. The fresh
   fixture returned exit 0 with zero warnings, including under
   `--deny-warnings`.
2. **High — page-bounds false negative.** Width is estimated as a fixed
   `0.58em` per byte. Six 12 pt Helvetica `W` glyphs at x=550 reach x=617.968
   on a 612 pt page, but Code Proof returned exit 0 with no findings.
3. **Medium — valid Markdown headings rejected.** CommonMark Setext headings
   and Pandoc `{#id}` heading IDs return `link.missing-source-target` before a
   PDF with the correct named destination is inspected.

Exact reproductions, expected/actual results, and source locations are in
`.factory/verification-9.md`.

## What passed

- Every `.factory/claims.json` command after `npm ci`: 23/23.
- `npm test`: 3 unit, 22 CLI integration, and 19 browser tests.
- `npm run typecheck`, `npm run lint`, and `npm run build`.
- Verified Cargo package (15 files, 33.5 KiB compressed), fresh consumer
  install, `--version`, help, demo, normal PASS flow, JSON/HTML output, input
  integrity, and invalid-input recovery.
- Live 19-test Playwright run, cold first-read/demo gate, keyboard/mobile,
  reduced motion, zero serious/critical axe findings, privacy request log,
  service-worker update and offline reload, routes, links, and headers.
- Mobile Lighthouse: Performance 98, Accessibility 100, Best Practices 100,
  SEO 100; LCP 1,974 ms, TBT 128 ms, CLS 0.
- Local/live byte comparison for all deployed product files; root SHA-256
  `ab16dcc89f47fabae535fa07e91276b81ea6a3dbd6e320934ddf6f3225e705ff`.

Evidence is in `evidence/verification-9-live/` and
`evidence/verification-9-cli/`. No product code was changed.

## Required next steps

- Scope syntax-color detection to text belonging to each code fence, then add
  a regression containing unrelated colored graphics/text and black code.
- Use embedded font widths and text transforms for geometry, then add
  wide/narrow glyph boundary regressions with default tolerance.
- Parse the supported Markdown dialect with a real parser, including Setext
  headings and explicit IDs, and document engine-specific fragment rules.
- Rerun every claim, all local gates, packaged-consumer exercise, and live QA
  before reconsidering release.
