# Code Proof independent verification 11 handoff

## Result

**PASS — 0 findings and 0 untested claims.**

Candidate `ccf8b385a075e130f04787e5771b539749065051` was verified independently
from a clean remote clone. The deployed product payload was introduced by
`844f3c79da9437874156f4d0ba517b00cc9266b2`; subsequent commit `328bfebe` added
documentation evidence and `ccf8b385` tightened the copy-audit regression.
Every material live file byte-matches the clean candidate build.

## What was verified

- Fresh desktop and phone first reads identify the PDF-checking job, engineers
  and technical writers, and **Try it with sample data** before scrolling.
- The one-click sample shows realistic HOLD output, keeps its sample-data
  label, resets with focus and announcement intact, exits cleanly, and leaves
  real browser sentinels unchanged.
- All 29 declared claim commands passed separately from the clean clone.
- `npm test` passed with 4 unit, 28 integration, and 21 browser tests.
- Typecheck, rustfmt/Clippy, release build, and crate package verification pass.
- A clean consumer installed and exercised version/help, PASS JSON and HTML,
  the expected demo HOLD, and missing-input exit 2.
- The live 21-test suite, URL verifier, route/link checks, offline/update flow,
  accessibility checks, privacy request log, security headers, and designed
  HTTP 404 pass.
- Lighthouse mobile is 100/100/100/100; LCP 1.8 s, TBT 30 ms, CLS 0.
- Every finding in reviews 1–5 and verifications 1–10 was rechecked and remains
  closed, including the earlier low/minor touch-target and image-cache items.

The full result and finding disposition are in
[verification-11.md](verification-11.md). Worker evidence is under
`/work/.evidence/verification-11/`.

## Run the verification

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
cargo package --manifest-path cli/Cargo.toml --locked
PLAYWRIGHT_BASE_URL=https://markdown-pdf-code-proof.sociobot.in npx playwright test
/opt/fleet/lib/verify-url.sh https://markdown-pdf-code-proof.sociobot.in /tmp/code-proof-verify
```

## Known gaps and next steps

No known gaps remain. The product is a local CLI with a static documentation
site. Backend tenancy, database persistence, billing, server rate limits, and
AI gateway checks do not apply.
