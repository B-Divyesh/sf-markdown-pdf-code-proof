# Code Proof handoff — adversarial first-read review 1

- Work order: `markdown-pdf-code-proof-review-1`
- Candidate reviewed: `1ce079bd5ad09705a538c8252c1f3b3b7538834d`
- Live URL: <https://markdown-pdf-code-proof.sociobot.in>
- Result: **FAIL** — 31 findings (3 blocking, 14 high, 11 medium, 3 low)

## What was done

Completed a cold live review at 390×844 and 1440×900, a sentence-by-sentence
landing/README copy audit, one-click web and release-CLI demo checks, all ten
listed claim commands, live request/storage/offline checks, route and link
crawls, metadata and 404 checks, transient and settled Axe checks, and a
from-scratch review of every historical verifier defect. No product code was
changed.

The full findings and proposed fixes are in
[`review-1.md`](review-1.md). The main blockers are a serious contrast failure
during the demo reveal and demo entry/exit routing that does not update title,
focus, or the demo query state. The copied install command also fails outside
a checkout while the repository has no downloadable release. Fourteen public
claims are also unlisted or under-tested.

## Verification run

All ten `.factory/claims.json` commands passed individually. These additional
checks passed:

```sh
npm ci
npm test
npm run build
cargo +1.88.0 check --workspace --locked
/opt/fleet/lib/verify-url.sh https://markdown-pdf-code-proof.sociobot.in /tmp/code-proof-review/verify-url
```

`npm test` passed 3 Rust unit tests, 18 CLI integration tests, and 12
Playwright tests. The build produced `target/release/codeproof` and
`dist/site/`. The live root HTML, JavaScript, and CSS matched the local build
byte-for-byte. The release CLI demo was also run from a fresh temporary working
directory and returned the intentional HOLD exit 1.

## What remains

Resolve every finding in `review-1.md`, add transition-time accessibility and
route-state regressions, register or remove every unlisted claim, then repeat
the entire review rather than checking only the diff. The existing test suite
passes but does not exercise the failing demo transition or the full public
claim surface.
