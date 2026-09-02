# Code Proof adversarial review 4 handoff

## Result: FAIL

Reviewed candidate: `dfc16a75cd20fb222d78460d56f62e3c2ef42fb0`.
Live URL: <https://markdown-pdf-code-proof.sociobot.in/>.

The review found seven issues: one blocking, one high, four medium, and one
minor. Blocking finding F-1-26 is a regression: the live element labelled
“Recorded Code Proof terminal output” and the README do not match the release
binary's actual demo output. The remaining findings cover an unlisted merge
claim, Reset focus loss, and four plain-language issues.

See [review-4.md](review-4.md) for the exact quotes, reproduction evidence,
complete landing/README copy audit, all claim results, and the finding-by-
finding history check.

## Verification performed

- Fresh 390×844 and 1440×900 live first reads and screenshots.
- Browser demo entry, Reset, exit, Back/Forward, request log, console log, and
  pre-seeded real-storage sentinel check.
- Real release-binary `codeproof demo` from a fresh temporary directory.
- All 21 `claims.json` entries run separately from a clean clone: 21 PASS.
- `npm test`, `npm run typecheck`, `npm run lint`, `npm run build`, and locked
  `cargo package`: PASS.
- Live Playwright suite: 13/13 PASS.
- Fleet URL verifier: PASS.
- Route metadata, 404, headers, sitemap, robots, asset dimensions, all internal
  fragments, and all linked destinations checked.
- Live HTML, JavaScript, and CSS hashes match the clean production build.

## Changes made

Only `.factory/review-4.md` and this handoff were written. Product code,
deployment, infrastructure, DNS, billing, and external resources were not
modified.

## Next steps

Fix F-1-26 first by deriving the browser transcript from real CLI output and
testing the match. Then address F-4-1 through F-4-6 and rerun the complete
review. Do not mark the candidate accepted until a fresh round has zero
findings.
