# Verify Markdown PDFs before release — review 6

- Live URL: <https://markdown-pdf-code-proof.sociobot.in/>
- Reviewed candidate: `ccf8b385a075e130f04787e5771b539749065051`
- Live product payload: `844f3c79da9437874156f4d0ba517b00cc9266b2`
- Documentation head: `2e8b7765fca263c6a493219f9e56b0aa878ea879`
- Date: 5 September 2026 UTC
- Verdict: **PASS**

**PASS — 0 findings and 0 untested claims.**

`ccf8b385` adds a copy-audit regression only. `2e8b7765` adds reports only.
The material live files match the clean build from the reviewed candidate; the
last deployed product payload is `844f3c79`.

## First read

Fresh Chromium desktop (1440×900) and iPhone 13 contexts opened the live root
at scroll position zero. Before scrolling, both state:

- Job: catch broken code, page overflow, and internal-link defects before a
  PDF release.
- Audience: engineers and technical writers producing code-heavy manuals.
- First action: **Try it with sample data**. The adjacent copy says it shows a
  sample PDF defect and failed check.

The phone first screen keeps the action and all three facts visible: works
offline after the first visit, no tracking data, and MIT-licensed free
software. It has no horizontal overflow. Screenshots are in
`/work/.evidence/review-6-live/desktop-first-screen.png` and
`/work/.evidence/review-6-live/phone-first-screen.png`.

## Live product review

The one-click sample opens `/?demo=1#demo`, changes the title to `Demo — Code
Proof`, and displays the persistent label **Demo — sample data, nothing is
saved**. It shows realistic populated output from `codeproof demo`:

```text
DEMO HOLD — do not release — 1 expected defect found
  Error [code.flow-changed] Code fence on line 7 is present but its line flow changed
Sample workspace: /tmp/codeproof-demo-…
HTML proof sheet: /tmp/codeproof-demo-…/proof/index.html
```

Reset replays the sample, retains focus on **Reset demo**, and keeps the label
visible. The fresh desktop and phone contexts had no `demo:` or `real:` local
storage entries after the flow. The live 21-test browser suite additionally
proved real local/session-storage and cookie sentinels remain unchanged, only
same-origin requests occur, Back/Forward restore focus, and reduced-motion
keyboard reset works.

The normal, invalid, boundary, and recovery CLI paths pass: a valid supplied
PDF produces PASS JSON and a proof sheet; a missing source returns exit 2; the
wrapped bundled sample returns exit 1 and a populated HOLD proof; renderer
no-shell, denied-network, fail-closed, and timeout paths are covered by exact
fixtures. A clean public-Git consumer install completed and the built demo
wrote a non-empty HOLD proof with `code.flow-changed`.

The site has route-specific titles and one H1 on root, demo, Privacy, Terms,
and the designed 404. The deliberate unknown-route HTTP 404 has a working
**Return home** link and is expected, not a defect. Root headers include CSP,
`nosniff`, referrer, and permissions policies. `robots.txt`, sitemap, legal
pages, external GitHub usage link, original image cache policy, offline reload,
service-worker update, keyboard focus, touch targets, privacy, and console
behavior pass the live suite.

`/opt/fleet/lib/verify-url.sh` passed with `lang=en`, title, one H1, main,
complete image alternatives, labelled buttons, and no console errors. The
Playwright Axe integration found no serious or critical issue in landing, demo,
mobile, legal, and 404 states. The standalone `npx @axe-core/cli` wrapper could
not locate a system Chrome executable in this worker; this is an environment
limitation, not an untested claim, because the required Playwright Axe checks
ran against its installed Chromium and passed. The latest independent
verification records Lighthouse mobile 100/100/100/100.

## Declared claims

All 29 manifest commands were run separately from a clean clone at
`2e8b7765`. Every command passed. `npm test` then passed again: 4 unit, 28 CLI
integration, and 21 browser tests. No public claim found on the landing page,
runtime text, README, legal pages, or metadata lacks a manifest entry.

| Claim | Result |
| --- | --- |
| single-line-wrap; code-lines-merge; page-bounds; code-content; internal-links | PASS |
| syntax-color; font-metrics; standard-font-widths | PASS |
| heading-fragments; fragment-case; automatic-heading-ids | PASS |
| existing-pdf; local-cli-files; input-unchanged | PASS |
| renderer-no-shell; renderer-network; renderer-fail-closed; renderer-timeout | PASS |
| html-proof; json-report; exit-codes; sample-demo | PASS |
| demo-transcript; private-site; offline-reload; static-routing | PASS |
| rust-msrv; install-from-git; mit-license | PASS |

The clean-clone quality commands also passed: `npm ci`, `npm run typecheck`,
`npm run lint`, `npm run build`, and `cargo package --manifest-path
cli/Cargo.toml --locked`. The generated crate is 39,901 bytes. The static build
contains 4,320 bytes raw JavaScript (1.81 KiB gzip), 11,262 bytes CSS (3.37 KiB
gzip), and a 210,844-byte hero image.

## Deployment identity

The following clean build files byte-match the live responses: root, Privacy,
Terms, 404, service worker, robots, sitemap, WebP/JPG/PNG artwork, SVG mark,
and the hashed CSS and JavaScript assets. This confirms the live runtime is the
reviewed implementation, not a later report-only commit.

## Earlier findings

All earlier review and verification reports were read, including their minor
findings. Their current dispositions are proven below.

| Earlier source | Current disposition and proof |
| --- | --- |
| Verifications 1–4 | Wrong destinations, renderer networking, worker installation, flattened/wrapped code, page edges, Rust version, accessibility, and narrow footer targets are covered by the exact current CLI, Rust, live Axe, and 390 px tests. All pass. |
| Verifications 5–8 | Their accepted package, link, sandbox, browser, privacy, offline, and update paths pass in the clean and live suites. |
| Verification 9 | The unrelated-color, glyph-width, and valid-heading regressions pass their current syntax-color, font-metric, and heading claim commands. |
| Verification 10 | Stable artwork responses use `public, max-age=3600, must-revalidate`, not immutable caching. |
| Reviews 1–3 | Demo lifecycle/output, install, source integrity, terminology, metadata, accessibility, and copy-audit issues remain covered by the transcript, privacy, route, copy-audit, and exact CLI claims. |
| Review 4, F-4-1–F-4-6 | Merged code, reset focus, workflow/color/output wording, and result-heading issues pass `code-lines-merge`, live reduced-motion reset, transcript, and copy tests. |
| Review 5, F-5-1–F-5-14 | Font, heading-ID, case, automatic-ID, syntax-color, first-screen facts, README audience/deploy instructions, CI wording, and image caching are present and covered by current claims or live checks. |

No earlier finding has regressed. There is no backend, tenant, database,
payment, rate-limit, or restart-persistence surface because this product is a
local CLI with a static documentation site.

