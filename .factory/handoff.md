# Code Proof adversarial review 5 handoff

## Result

**FAIL.** Review 5 is recorded in `.factory/review-5.md` for candidate
`88ed74524be9b461067777d3b308736d7f95ebfd`. It contains 14 findings: nine
high, four medium, and one minor. No product code was changed.

The live first screen, one-click demo, sandbox isolation, registered claims,
routes, accessibility checks, and build gates pass. The failure is driven by
unlisted or overstated README behavior, incomplete first-screen/docs content,
and the known immutable-cache issue for stable-name images.

## Verification performed

- Fresh mobile (390×844) and desktop (1440×900) cold loads.
- One-click demo, direct demo URL, Reset, exit, Back/Forward, title, focus,
  browser storage, request log, and real CLI demo in `/tmp`.
- All 25 exact `.factory/claims.json` commands from a fresh clone: PASS.
- Clean `npm test`, `npm run typecheck`, `npm run lint`, `npm run build`, and
  verified `cargo package`: PASS.
- Live Playwright: 18/19 on the first cold-cache run because the transcript
  test exhausted its total timeout while compiling the CLI; the exact claim
  command passed first in a second clean clone and the isolated live rerun
  passed.
- `/opt/fleet/lib/verify-url.sh`: PASS.
- Live route/metadata/Axe/link crawl, security headers, 200% text smoke check,
  and local/live asset hash comparison.
- Every finding in reviews 1–4 was checked in live behavior and source.

## Remaining work

Resolve F-5-1 through F-5-14 in `.factory/review-5.md`, then rerun the entire
review. The highest-priority work is to make every README behavior correspond
to an exact registered claim and test, especially base-font coverage and
heading identifier rules.

## Reproduce

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
cargo package --manifest-path cli/Cargo.toml --locked
PLAYWRIGHT_BASE_URL=https://markdown-pdf-code-proof.sociobot.in npx playwright test
/opt/fleet/lib/verify-url.sh https://markdown-pdf-code-proof.sociobot.in /tmp/code-proof-review-5-url
```
