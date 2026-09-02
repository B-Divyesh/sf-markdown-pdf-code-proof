# Code Proof polish 3 handoff

## Result: PASS

Product repair commit: `66287975257c0f76c8b2cfff403348a17f3a6e15`.
It is pushed to `origin/main`; the configured static deployment was then
checked cold at <https://markdown-pdf-code-proof.sociobot.in/>.

## Done

- Registered README's verifier/not-editor behavior as the `input-unchanged`
  claim. The exact Rust integration test checks byte-for-byte source integrity
  after both existing-PDF and custom-renderer flows.
- Added `scripts/audit-copy.mjs` and `npm run test:copy-audit`. The generated
  audit covers the landing page, runtime feedback, accessibility labels, and
  README prose with exact whitespace-delimited counts. It rejects stale output,
  overlong copy, banned marketing words, and a missing exact source-integrity
  claim test.
- Rewrote the catalog description as a verb-first, 86-character sentence.
- Preserved all prior demo, privacy, accessibility, routing, metadata, mobile,
  terminology, and CLI fixes. No visual system was replaced.

## Verification

Fresh remote clone:
`/tmp/codeproof-polish3-clean.uhWU49/repo` at `6628797`.

- `npm ci` completed with zero reported vulnerabilities.
- All 21 exact commands in `.factory/claims.json` passed separately. This
  includes `input-unchanged`, both dedicated browser claim contexts, Rust 1.88,
  and a public Git installation from an empty root at revision `66287975`.
- `npm test` passed: copy-audit freshness, Rust 1.88, 3 Rust unit tests, 22 CLI
  integration tests, 13 Playwright browser/accessibility tests, and license.
- `npm run typecheck`, `npm run lint`, `npm run build`, and
  `cargo package --manifest-path cli/Cargo.toml --locked` passed. The verified
  crate is 33.4 KiB; the release binary and `dist/site/` were produced.
- Cold live `verify-url.sh` passed; the live Playwright suite passed 13/13.
  It includes Axe, privacy request/storage checks, offline reload, reduced
  motion, keyboard focus, routes, legal pages, 404, and 390 px layout.
- Live mobile Lighthouse: 100 Performance, 100 Accessibility, 100 Best
  Practices, 100 SEO; LCP 1804 ms, TBT 0 ms, CLS 0.

Evidence is in `evidence/polish-3-live/`, including desktop, mobile, and
one-click demo screenshots, verifier JSON, live HTML, and Lighthouse JSON.
The detailed finding map is [polish-3.md](polish-3.md).

## Run and publish

```sh
npm ci
npm test
npm run build
cargo package --manifest-path cli/Cargo.toml --locked
```

The factory owns release publication. To produce the ready-to-publish crate,
run the final `cargo package` command above; do not publish from this checkout.

## Known gaps and next steps

None. The current review set has zero unresolved findings.
