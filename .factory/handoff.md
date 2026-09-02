# Code Proof polish 4 handoff

## Result

PASS. Repair commit `7adcd759514d4a01b54e529cedd9483a7c96c954` closes every
finding in reviews 1–4. It was pushed to `origin/main` and deployed to
<https://markdown-pdf-code-proof.sociobot.in/> as Azure Static Web Apps
deployment `51082c09-a8b2-4d00-b44f-ca1f1070abef`.

The site keeps the release-room risograph design and the product remains a
Rust CLI with static Vite documentation. No backend, analytics, AI service,
payment integration, or new infrastructure was added.

## What changed

- The failed decision is exactly `HOLD — do not release` in CLI summaries,
  README prose, and generated HTML proof sheets.
- `scripts/generate-demo-transcript.mjs` runs the real bundled CLI demo during
  every site build. It normalizes only the temporary path and generates the
  four browser transcript lines in `site/src/demo-transcript.ts`.
- `@claim:demo-transcript` runs the CLI again in a fresh directory and compares
  every displayed line. The full diagnostic is no longer shortened.
- Reset demo remains focusable during replay, blocks duplicate activation with
  `aria-disabled`, exposes `aria-busy`, and retains focus through completion.
- Added the `code-lines-merge` claim and its exact existing CLI regression.
- Replaced the remaining “source,” “code colors,” “non-default PDF color,” and
  generic result wording with Markdown, syntax color, and HTML proof sheet.
- Added dynamic demo description/Open Graph metadata, route metadata
  regressions, stronger legal/footer checks, direct `?demo=1` coverage, and a
  real-storage sentinel test for demo isolation.
- Updated the copy audit, demo documentation, catalog line, changelog, and
  service-worker cache version.

## Verification

Fresh remote clone: `/tmp/codeproof-polish4-claims.ADF5zK/repo`, exact SHA
`7adcd759514d4a01b54e529cedd9483a7c96c954`.

- Every `.factory/claims.json` command ran separately: 23/23 passed. The Git
  install claim installed remote revision `7adcd759` into an empty root.
- `npm test`: passed; 3 Rust unit tests, 22 CLI integration tests, 19 browser
  tests, copy/transcript freshness, Rust 1.88, and MIT checks.
- `npm run typecheck`: passed.
- `npm run lint`: passed with rustfmt and Clippy warnings denied.
- `npm run build`: passed; JS 4.32 kB raw / 1.81 kB gzip, CSS 11.26 kB raw /
  3.37 kB gzip, and the hero WebP is 210,844 bytes.
- `cargo package --manifest-path cli/Cargo.toml --locked`: passed and verified
  a 33.5 KiB compressed crate.
- Cold live Playwright: 19/19 passed, including Axe during demo transitions,
  direct demo routing, Reset focus, privacy sentinels, offline reload, mobile,
  route metadata, legal navigation, and the designed 404.
- Fleet URL verifier: passed at 791 ms with no console errors, one H1, `lang`,
  main landmark, alt text, and labelled buttons.
- Live `index.html` SHA-256 matches the local build:
  `ab16dcc89f47fabae535fa07e91276b81ea6a3dbd6e320934ddf6f3225e705ff`.
- Mobile Lighthouse: Performance 99, Accessibility 100, Best Practices 100,
  SEO 100; LCP 1,814 ms, TBT 0 ms, CLS 0.

Evidence is in `evidence/polish-4-live/`: desktop/mobile landing screenshots,
the direct demo screenshot, URL-verifier output, and Lighthouse JSON. The full
finding map is `.factory/polish-4.md`.

## Run and verify

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
cargo package --manifest-path cli/Cargo.toml --locked
```

Run the isolated CLI sample with `target/release/codeproof demo`. Open the
isolated browser sample at
<https://markdown-pdf-code-proof.sociobot.in/?demo=1>.

## Known gaps and next steps

None. All recorded findings are closed and all required gates pass locally,
from a clean clone, and on the deployed site.
