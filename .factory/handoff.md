# Code Proof review 3 handoff

## Result: FAIL

Reviewer-only work completed at candidate
`f3aaaae31e2cab889eda6a6a4c68175350ab6812`. No product code was changed.
The committed review is `.factory/review-3.md`.

## Verified

- Cold live checks at 390×844 and 1440×960; landing clarity, mobile layout,
  demo, storage isolation, routes, links, metadata, headers, accessibility,
  visual identity, and earlier-finding closure.
- Fresh clone at `/tmp/codeproof-review3.DxkCOg/repo`; `npm ci`; every one of
  the 20 exact registered claim commands; `npm test`; `npm run typecheck`;
  `npm run lint`; `npm run build`; and locked `cargo package`.
- A real release `codeproof demo` run from a fresh temporary directory returned
  expected exit 1 and printed its isolated proof-sheet workspace.

## Remaining work

1. Register and test, or remove, README’s “It is a verifier, not an editor.”
   promise (F-3-1).
2. Regenerate `.factory/copy-audit.md` with all landing and README sentences
   and correct word counts; prevent staleness (F-3-2).

After those corrections, rerun `npm test` and the commands listed in
`.factory/claims.json` before requesting another review.
