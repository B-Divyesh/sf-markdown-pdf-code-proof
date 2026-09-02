# Code Proof handoff — adversarial first-read review 2

## Result: FAIL

- Candidate: `f1474e5871a1c5c28d4e9967c8f9476a41f20a79`
- Live URL: <https://markdown-pdf-code-proof.sociobot.in>
- Reviewed: 2026-09-02 UTC
- Product code changed: no

The full report is `.factory/review-2.md`. It records eight findings: one
blocking, three high, and four medium. The blocker is inherited F-1-26: the
live copy still uses “blocks” for “code fence” and a bare “report” label after
the previous repair claimed those terms were standardized.

The live first screen, one-click browser demo, real CLI demo, privacy behavior,
routing, metadata, links, accessibility, visual identity, and all 20 registered
claims otherwise passed. The live deployment matched the clean candidate build
byte for byte for all checked site files.

## Verification run

From a clean clone at the candidate commit:

```sh
npm ci
# Every test command in .factory/claims.json, separately: 20/20 passed
npm test
npm run typecheck
npm run lint
npm run build
cargo package --manifest-path cli/Cargo.toml --locked
```

The real `codeproof demo` command also ran from a fresh temporary directory. It
returned the intentional exit 1 and wrote its bundled Markdown, generated PDF,
and HOLD proof sheet under a separate `/tmp/codeproof-demo-*` workspace.
`/opt/fleet/lib/verify-url.sh` passed against the live root. Live Axe checks at
390 px and desktop, including the demo transition, found no violations.

## Required next work

Fix the eight findings in `.factory/review-2.md`, especially F-1-26. Add claim
coverage or remove the three unlisted README promises. Then rerun the entire
cold review rather than only the changed copy.
