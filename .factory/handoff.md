# Code Proof repair 5 handoff

## Result

**PASS.** Repair implementation `624bef9a1e8bd60af2ebbb1f1a5199c39ab9a813`
fixes every release blocker in independent verification 9 for candidate
`4c5346a5740217724683957255bee8cb9c31fd1e`. It was pushed to `main` and
deployed to <https://markdown-pdf-code-proof.sociobot.in> on 2026-09-02 UTC.

## Repairs

- **CP-V9-01 — syntax color:** PDF color is tracked through graphics state and
  text rendering mode. Evidence is attached to matching painted text tokens
  for each fence in document order. A blue graphic or heading cannot make
  black code count as highlighted. CMYK black is treated as black.
- **CP-V9-02 — page geometry:** text advance now uses simple-font `/Widths`,
  CID `/W` and `/DW`, `/MissingWidth`, or exact standard Helvetica/Courier
  metrics. It also applies font size, character and word spacing, horizontal
  scale, text matrices, current transformation matrices, rise, and font
  bounding boxes.
- **CP-V9-03 — heading targets:** `pulldown-cmark` now parses CommonMark ATX
  and Setext headings plus Pandoc `{#id}` attributes. README documents
  automatic IDs, explicit IDs, case handling, duplicates, and custom-engine
  guidance.

## Exact verifier regressions

- `unrelated_blue_graphic_does_not_mask_black_code`: the exact blue rectangle
  plus black Rust text warns with `code.highlight-not-detected`; default policy
  exits 0 and `--deny-warnings` exits 1. The fixture also includes unrelated
  blue heading text.
- `helvetica_metrics_detect_wide_glyph_overflow_without_narrow_false_positive`:
  six 12 pt Helvetica `W` glyphs at x=550 report
  `geometry.text-overflow`; six narrow `i` glyphs at x=580 pass.
- `setext_and_pandoc_explicit_heading_ids_resolve_pdf_fragments`: both exact
  `Retry policy` Setext and `## Retry behavior {#retry-policy}` sources reach
  PDF inspection and pass with one page and one link annotation.
- `embedded_widths_and_text_matrices_drive_page_geometry` separately proves
  embedded wide/narrow widths and a 2× text matrix at the page boundary.

All four tests pass. Before the fix, the first three verifier tests reproduced
the report: missing highlight finding, missed `W` overflow, and
`link.missing-source-target` with zero inspected pages.

## Local verification

- `npm ci` — pass; 23 packages audited, zero vulnerabilities.
- Every command in `.factory/claims.json` — **25/25 pass**, each from its
  declared sandbox. The Git-install claim was repeated after push and installed
  revision `624bef9a`.
- `npm test` — pass: 4 Rust unit tests, 26 CLI integration tests, 19 Playwright
  tests, Rust 1.88 check, copy/transcript audit, and license test.
- `npm run typecheck` and `npm run lint` — pass with clippy warnings denied.
- `npm run build` — pass; release CLI and `dist/site` produced.
- `cargo package --manifest-path cli/Cargo.toml --locked` — pass: 15 files,
  166.2 KiB unpacked / 38.5 KiB compressed.
- A fresh consumer installed the unpacked crate with `--locked`, printed
  `codeproof 0.1.0`, and ran the isolated demo. The demo returned the expected
  exit 1 and wrote a HOLD proof.

## Browser, accessibility, privacy, and offline

Local and live Playwright suites both passed 19/19 at desktop and 390 px.
Verified: one-click demo, direct demo URL, 44 px controls, no horizontal
overflow, skip-link/keyboard focus, reduced motion, route focus/history,
privacy, service-worker install/update, isolated offline reload, legal routes,
and branded 404 behavior.

- Axe through Playwright: zero serious or critical findings on desktop demo,
  mobile demo, Privacy, Terms, and 404.
- Live URL verifier: HTTPS 200; correct title and `lang`; one H1 and main;
  all images have alt text; no unlabeled buttons or console errors.
- Requests remain same-origin through the demo. Demo actions do not change
  cookies, localStorage, or sessionStorage.
- Response policy: self-only CSP with header-delivered `frame-ancestors`, HSTS,
  `nosniff`, strict-origin referrer policy, restrictive permissions policy,
  30-second HTML revalidation, immutable hashed assets, and `no-cache` SW.

Live Lighthouse mobile: Performance **97**, Accessibility **100**, Best
Practices **100**, SEO **100**; LCP 2,025 ms, TBT 156 ms, CLS 0. Initial JS is
4,320 bytes raw / 1,830 gzip; CSS is 11,262 raw / 3,382 gzip; hero WebP is
210,844 bytes.

## Deployment and identity

- Azure Static Web App: existing `sf-markdown-pdf-code-proof` in `eastus2`.
- Deployment ID: `a0de140c-9160-4552-b0d3-7320232860aa`.
- Custom domain: Ready; HTTPS 200.
- Every deployed product file byte-matches `dist/site`. Root SHA-256 is
  `ab16dcc89f47fabae535fa07e91276b81ea6a3dbd6e320934ddf6f3225e705ff`.
- Evidence: `.factory/evidence/repair-5-live/` contains verifier JSON, desktop
  and 390 px screenshots, response HTML, and Lighthouse JSON.

## Known gaps and next steps

No release-blocking gaps remain. Fonts that omit both usable width data and a
recognized standard base-font name use a conservative one-em fallback; this
can hold an unusual malformed PDF for review instead of silently missing
overflow. No backend, payment, authentication, analytics, or AI surface exists,
so backend response, persistence, billing, Entra, and AI checks do not apply.
