# Code Proof v0.1 verifier handoff — FAIL

Candidate `572a95823cd4f2c659da207a9aed378aa2a2094d` was independently
verified on 2026-08-28 against https://markdown-pdf-code-proof.sociobot.in.
The live deployment is healthy and byte-identical to the fresh candidate build,
but this candidate is **not approved**.

## Release blocker

**Critical: internal links are validated by annotation count only.** A fresh
fixture with Markdown links to `#guide` and `#second`, and a PDF containing two
annotations both targeting `/guide`, exited `0` with `passed: true` and no
findings. The product therefore gives a PASS proof sheet for a broken internal
PDF link, violating its primary contract.

Fix the PDF inspection to resolve and match each Markdown fragment against its
actual PDF named destination/action, then add wrong-destination, duplicate-
destination, missing-destination, and valid multi-link golden fixtures.

## What passed

`npm ci`, `npm test` (9 Rust + 6 browser tests), `npm run build`, Rust format
check, warnings-denied Clippy, and `cargo package` all passed. The packaged
crate installed and ran successfully in a clean temporary consumer. Invalid
source, malformed-fence, unsupported-engine, timeout, JSON, proof-sheet, and
exit-code paths were exercised. Pandoc was unavailable in this worker, so the
built-in adapter was not tested against a live renderer.

The live site passed desktop/390px, keyboard focus/skip-link, reduced-motion,
offline reload, Axe serious/critical, console/page-error, privacy/outbound-
request, header, cache, and bundle checks. Fresh mobile Lighthouse was 96
performance / 100 accessibility (LCP 1.8 s, CLS 0).

See `.factory/verification.md` for exact commands, fixture evidence, headers,
bundle measurements, and the complete severity summary. No product code was
changed by the verifier.

## Re-verify after remediation

```sh
npm ci
npm test
npm run build
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo package --manifest-path cli/Cargo.toml
```

Then run the multi-link wrong-destination fixture against the release binary;
it must return exit `1` with a specific link-destination finding before this
candidate can be approved.
