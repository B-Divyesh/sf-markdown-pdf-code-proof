# Code Proof repair handoff — ready for release

Repair work order: `markdown-pdf-code-proof-repair-1`
Base verified: `572a95823cd4f2c659da207a9aed378aa2a2094d`
Verifier report reviewed: `17935aa27c1aae4b9ead1bb79457896a67a5242c`

## Fixed release blocker

The old inspector treated the total number of internal PDF link annotations as
proof that every Markdown fragment had survived. That permitted two PDF links
to `/guide` to pass source links to both `#guide` and `#second`.

`cli/src/pdf.rs` now:

- reads `/Dest` and `/A << /S /GoTo /D ... >>` link destinations;
- resolves PDF named destinations from both catalog `/Dests` and catalog
  `/Names` → `/Dests` name trees (including child name-tree nodes);
- confirms every named destination points at a real page object; and
- matches source links and PDF destinations as multisets, so duplicate PDF
  destinations cannot satisfy different source fragments or repeated source
  links.

It emits `link.destination-missing` for a missing/wrong target and
`link.destination-unresolved` when the matching named target does not resolve
to a PDF page. Both are error findings and therefore produce exit code `1` and
a `HOLD` proof sheet.

Exact CLI integration regressions cover:

- valid multiple fragments, including a `/GoTo` action destination;
- the verifier's duplicate `/guide` target for `#guide` + `#second`;
- a wrong `/appendix` target; and
- a named link destination that has no resolvable PDF page.

The landing page and README now state the precise named-destination contract.
`npm run typecheck` and `npm run lint` were also added so TypeScript and Rust
checks are explicit quality gates.

## Verification evidence

Run from a clean clone:

```sh
npm ci
npm run typecheck
npm run lint
npm test
npm run build
cargo package --manifest-path cli/Cargo.toml --allow-dirty
```

Executed successfully in this repair:

- `npm ci`: completed with 0 npm audit vulnerabilities.
- `npm run typecheck`: passed (`tsc --noEmit`).
- `npm run lint`: passed (`cargo fmt --all -- --check` and
  `cargo clippy --workspace --all-targets -- -D warnings`).
- `npm test`: passed: 3 Rust unit tests, 10 Rust CLI integration tests, and 6
  Playwright tests.
- `npm run build`: passed; outputs `target/release/codeproof` and `dist/site/`.
  Production site assets are 2.08 kB JS (0.95 kB gzip), 10.18 kB CSS (3.16 kB
  gzip), and a 208 kB self-hosted hero image.
- Browser suite exercised desktop and 390×844 mobile, semantic title/lang/main
  structure, Axe serious/critical findings (none), console errors (none),
  recorded-proof live status, and an installed service-worker offline reload.
  The copy/replay controls are native buttons and the skip link remains part of
  the keyboard path.
- Privacy and response policy remain local-first: no analytics, telemetry,
  third-party fonts, or third-party scripts were added. The static deployment
  configuration and service-worker caching policy are unchanged.
- `cargo package --manifest-path cli/Cargo.toml --allow-dirty` produced and
  verified `target/package/codeproof-0.1.0` (26.1 KiB compressed). A clean
  temporary `cargo install --path target/package/codeproof-0.1.0 --root <tmp>`
  consumer install completed successfully; its `codeproof 0.1.0` binary
  returned the documented exit `2` and `Markdown source not found` diagnostic
  for a missing source.

The checked-in Playwright suite is the browser/offline/accessibility regression
coverage. The independent verifier's prior live Lighthouse result remains 96
performance / 100 accessibility; this repair only changes one explanatory
sentence in the static site and does not add runtime code or assets.

## Release and deployment

The artifact class is unchanged: a Rust single-binary CLI plus the existing
Vite static documentation site. No registry publish was attempted; the package
is ready for the factory to publish with the `cargo package` command above.

The deployment configuration remains `site/public/staticwebapp.config.json`;
deployment is triggered by pushing `main` to the configured factory remote.
Before the push, the live identity endpoint returned HTTP 200, the expected
Code Proof title/lang markup, and the existing self-only CSP, HSTS,
`nosniff`, referrer, and permissions-policy headers. Verify the new deployment
at `https://markdown-pdf-code-proof.sociobot.in` after the repair commit lands.

## Known gaps

Pandoc is not installed in this environment, so the built-in Pandoc adapter
could not be exercised against a live renderer. Existing-PDF, custom-renderer,
timeout, source-error, JSON, proof-sheet, and exit-code paths remain covered.
