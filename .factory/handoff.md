# Independent verification handoff — FAIL

- Work order: `markdown-pdf-code-proof-verify-3`
- Candidate: `23ef1657b140c5b38617a7d4f9d0ba7c0bd48ae8`
- Live URL: <https://markdown-pdf-code-proof.sociobot.in>
- Result: **FAIL**
- Full evidence: `.factory/verification-3.md`

## Why it fails

1. **Critical:** a two-line fenced block flattened into one colored PDF text
   run returns exit 0, zero findings, and a PASS proof sheet. This reproduces
   the brief's exact lost-newlines failure mode.
2. **High:** the packaged crate cannot be checked/installed by the documented
   Rust 1.79 minimum because locked dependencies require edition 2024 and up to
   Rust 1.88.
3. **High:** the deployed 390px page has a serious repository-pinned Axe
   finding across four scroll regions, and Lighthouse's newer audit reports a
   serious accessible-name mismatch on the brand link.
4. **Low:** the “Terms” footer target measures 42×44 CSS px, below the 44×44
   contract.

## What passed

`npm ci`, `npm test`, `npm run typecheck`, `npm run lint`, exact `npm run build`,
and `cargo package --manifest-path cli/Cargo.toml --locked` all passed. The
package installed and its public CLI/API were exercised in a clean consumer;
normal links, wrong links, missing code, overflow, warning promotion, malformed
inputs, timeout, sandbox network denial, and concurrent runs were checked.

The live deployment is not suffering the earlier deployment-only failure. It
is HTTP 200 and byte-identical to this candidate's fresh site build. Desktop
and 390px rendering, keyboard controls, visible focus, clipboard recovery,
reduced motion, privacy/outbound requests, response security and caching,
service-worker update/offline reload, and legal pages were checked. Lighthouse
mobile scored 96 performance / 100 accessibility / 100 best practices / 100
SEO with LCP 1.8s and CLS 0; JS, CSS, fonts, and hero image remain within budget.

## Re-run

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
cargo package --manifest-path cli/Cargo.toml --locked
cargo +1.79.0 check --manifest-path <unpacked-crate>/Cargo.toml --locked
```

After repairing code-flow detection, add a regression PDF that contains two
source lines in one PDF text operation and require exit 1. Run Axe at 390px in
the repository suite, align the brand's accessible name with its visible text,
enforce the claimed MSRV in CI, then repeat live byte-identity and PWA checks.

No product code was modified during verification. Pandoc was unavailable, so
the built-in adapter was not exercised against a real Pandoc PDF backend.
