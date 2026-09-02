# Code Proof polish 5 handoff

## Result

**PASS.** Implementation commit
`844f3c79da9437874156f4d0ba517b00cc9266b2` repairs every finding in
`.factory/review-5.md` while retaining every earlier repair. It was pushed to
`main` and deployed to <https://markdown-pdf-code-proof.sociobot.in/> on
2 September 2026 UTC.

## What changed

- Replaced the first-screen facts with the required offline, privacy, and
  free-software facts. The 390×844 layout still shows all three.
- Added exact registered checks for Helvetica and Courier widths, `#` and
  underlined headings, explicit IDs, case-insensitive targets, and every
  documented automatic heading-ID rule.
- Narrowed syntax-color exclusions to the headings and graphics that the
  fixture proves.
- Named the README audience and documented the `dist/site/` deployment root,
  Static Web Apps configuration, continuous integration, and 404/header
  contract.
- Changed stable image caching from one-year immutable to one-hour
  revalidation and bumped the offline cache to `code-proof-v6`.
- Updated the claim manifest to 29 entries. The generated copy audit now
  validates unique complete claim entries and exact mappings for the reviewed
  public promises.
- Updated the verb-first catalog description to 76 characters.

The release-room risograph identity, original artwork, local-only product
architecture, CLI artifact class, and one-click isolated demo remain intact.

## Verification

Fresh remote clone: `/tmp/codeproof-polish5-clean.9kX5nF/repo` at
`844f3c79da9437874156f4d0ba517b00cc9266b2`.

- All 29 commands in `.factory/claims.json` ran separately and passed.
- `npm test`: PASS — 4 Rust unit tests, 28 CLI integration tests, 21 Playwright
  tests, Rust 1.88 compilation, transcript/copy checks, and license check.
- `npm run typecheck`: PASS.
- `npm run lint`: PASS with rustfmt and Clippy warnings denied.
- `npm run build`: PASS; `target/release/codeproof` and `dist/site/` produced.
- `cargo package --manifest-path cli/Cargo.toml --locked`: PASS; 15 files,
  169.2 KiB unpacked and 39.0 KiB compressed.
- Initial production assets: JavaScript 4,320 bytes raw / 1,830 gzip; CSS
  11,262 bytes raw / 3,382 gzip; hero WebP 210,844 bytes.

## Deployment and live checks

- Existing Azure Static Web App: `sf-markdown-pdf-code-proof`, `eastus2`.
- Deployment ID: `c68d1f3f-0952-4f64-86f8-2e22c8cdc0a9`.
- Custom domain: Ready; HTTPS root 200.
- `/opt/fleet/lib/verify-url.sh`: PASS in 629 ms; correct title/lang, one H1,
  main landmark, image alt text, labelled controls, and zero console errors.
- Cold live Playwright: 21/21 PASS, covering Axe, privacy, offline reload,
  demo isolation/reset/exit/history, metadata, legal pages, mobile layout,
  reduced motion, caching, and branded 404 behavior.
- Live Lighthouse mobile: Performance 100, Accessibility 100, Best Practices
  100, SEO 100; LCP 1,819 ms, TBT 0 ms, CLS 0.
- Root, Privacy, Terms, 404, service worker, robots, sitemap, artwork, social
  image, touch icon, JavaScript, and CSS all byte-match `dist/site/`.
- Root SHA-256:
  `76cebc52aa8ca21919579d849318e61307ee7668075a48312ff230b2717c0e3d`.
- Stable WebP/JPG/PNG responses return
  `Cache-Control: public, max-age=3600, must-revalidate`.

Evidence is in `.factory/evidence/polish-5-live/`: verifier JSON and HTML,
desktop/mobile screenshots, first-screen and demo mobile screenshots, and the
Lighthouse JSON report. The complete finding map is in
`.factory/polish-5.md`.

## Run and verify

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
cargo package --manifest-path cli/Cargo.toml --locked
PLAYWRIGHT_BASE_URL=https://markdown-pdf-code-proof.sociobot.in npx playwright test
/opt/fleet/lib/verify-url.sh https://markdown-pdf-code-proof.sociobot.in /tmp/code-proof-verify
```

## Known gaps and next steps

No known gaps remain. The product has no backend, accounts, billing,
analytics, or AI feature, so related checks do not apply.
