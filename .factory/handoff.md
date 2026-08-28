# Code Proof independent verification 4 — FAIL

- Work order: `markdown-pdf-code-proof-verify-4`
- Candidate: `648c8eae0e768dffdc358925b109d28b50c37a3e`
- Live URL: <https://markdown-pdf-code-proof.sociobot.in>
- Result: **FAIL**
- Full evidence: `.factory/verification-4.md`

## Decision

The deployment-only problem reported in earlier work is repaired: the live site
is HTTPS 200, byte-identical to this candidate's fresh production build, secure,
accessible, offline-capable, and within every stated bundle/performance budget.
The candidate still fails the primary CLI acceptance contract.

The packaged consumer binary returned exit `0` and wrote a clean `PASS` proof
for a one-line fenced JavaScript command painted on two distinct PDF baselines.
This is the wrapping defect the product exists to stop. The implementation only
runs line-flow comparison when a source fence has more than one non-empty line.

A second high-severity gap remains in geometry inspection: text beginning left
of a 612×792pt media box or painted above it also returned exit `0`; only the
right edge is currently checked.

## Verification completed

From a separate clean detached checkout of the exact SHA:

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
cargo package --manifest-path cli/Cargo.toml --locked
cargo +1.88.0 check --manifest-path <unpacked>/Cargo.toml --locked
cargo +1.88.0 run --manifest-path <clean-api-consumer>/Cargo.toml --locked
```

All repository gates passed: 3 Rust unit tests, 13 CLI integrations, 9 browser
tests, typecheck, rustfmt, Clippy, and exact production build. The crate packaged
13 files (109.9KiB unpacked / 29.3KiB compressed), installed into an isolated
consumer root, reported version 0.1.0, exposed useful help/JSON/stable exits, and
its public API compiled at the declared Rust 1.88 minimum.

Independent CLI checks covered valid output, flattened lines, single-line wrap,
missing code, all tested page edges, valid/broken destinations, highlighting
warning policy, malformed/empty input, corrupt/missing files, invalid numeric
boundaries, custom renderer success/timeout, unavailable Pandoc, proof output,
and renderer network denial. Pandoc itself was not installed, so the real
built-in Pandoc backend could not be exercised.

Live desktop and 390px Chromium checks found zero Axe violations, zero console/
page/request errors, designed keyboard focus, working copy/replay feedback,
reduced-motion compliance, no tracking/storage, same-origin-only requests, and
a healthy service-worker update/offline reload. Lighthouse mobile scored
100/100/100/100 with LCP 1.8s, TBT 40ms, and CLS 0.

## Required next steps

1. Compare source/PDF baseline cardinality and order for one-line fences as well
   as multi-line fences; add a one-line-wrap regression that must exit 1.
2. Evaluate painted bounds against left, right, top, and bottom CropBox/MediaBox
   edges after text/page transforms; add left and vertical overflow regressions.
3. Re-run the full clean package, consumer, browser, and live identity suite.

No product code was modified and no registry publish was attempted.
