# Verify Markdown PDFs before release — independent verification 11

- Live URL: <https://markdown-pdf-code-proof.sociobot.in/>
- Candidate commit: `ccf8b385a075e130f04787e5771b539749065051`
- Live implementation payload commit: `844f3c79da9437874156f4d0ba517b00cc9266b2`
- Incoming documentation commit: `328bfebe6ea3cd87305596891b5295abc7d7139b`
- Verified: 5 September 2026 UTC
- Verdict: **PASS**

There are zero findings of every severity and zero untested claims. The two
commits after `844f3c79` add verification evidence and a copy-audit regression;
they do not change the deployed site or CLI implementation. A clean build at
the candidate commit byte-matches every material live site file.

## First read before scrolling

Fresh Chromium contexts opened the live root at 1440×900 and a Pixel 5 phone
viewport. Before scrolling, the page states:

- Job: catch broken code, page overflow, and internal-link defects in a
  finished PDF.
- Audience: engineers and technical writers producing code-heavy manuals.
- First action: **Try it with sample data**. The adjacent text says it will
  show a sample PDF defect and failed check.

The phone first screen also shows all three facts: offline after the first
visit, no tracking data, and free MIT-licensed software. There is no horizontal
page overflow. Screenshots are in
`/work/.evidence/verification-11/live/desktop-first-screen.png` and
`phone-first-screen.png`.

## Live demo and site

The one-click sample opens `/?demo=1#demo`, changes the title to
`Demo — Code Proof`, and focuses **Sample failed release check**. The persistent
label reads **Demo — sample data, nothing is saved**. The populated terminal
shows the real bundled sample, expected exit 1, `code.flow-changed`, and
`DEMO HOLD — do not release — 1 expected defect found`.

Reset replays the sample, keeps keyboard focus, and announces completion.
**View install commands** leaves demo mode, removes the banner, changes the URL
and title, and focuses the install heading. Back and Forward restore the right
state and focus. Real local-storage, session-storage, and cookie sentinels were
unchanged throughout the sample, reset, and exit flow. The flow made only
same-origin requests and added no tracking data.

The 21-test live browser suite passed after the local CLI build was warm. It
covers the sample, direct demo route, transcript equality, reset, history,
keyboard use, focus, reduced motion, mobile controls, privacy, offline reload,
service-worker update, metadata, legal routes, caching, and the 404 page. Axe
found no serious or critical issue. The fleet URL verifier found one H1, `lang`,
`main`, complete image alternatives, labelled buttons, and zero console errors.

The first cold live sweep recorded 20 passes and one transcript-test timeout.
The local Rust build consumed that test's global 30-second limit before its
browser assertion. The exact declared command builds before Playwright and
passed from the clean clone; the isolated live rerun passed in 4.0 seconds, and
the complete live rerun passed 21/21 in 19.9 seconds. This was a harness setup
timeout, not a product or claim failure. Both logs are retained.

The designed missing route returns HTTP 404 and the branded page with one H1,
its own title, and a working **Return home** link. It is an expected 404, not a
defect. Root, demo, Privacy, Terms, GitHub, and the GitHub Usage anchor all
respond as expected. Every route has its own title and complete navigation.

The installed documentation works offline after its first visit. Its service
worker activates, controls a reload, reports offline state, updates without a
waiting worker, and removes old product cache versions. Stable-name images
revalidate after one hour; hashed assets remain immutable.

## Declared claims: 29 / 29 passed

Each manifest command ran separately from the clean remote clone at the exact
candidate commit. The numbered evidence file contains that command's complete
output.

| # | Claim | Result | Evidence |
| ---: | --- | --- | --- |
| 1 | `single-line-wrap` | PASS | `/work/.evidence/verification-11/claims/01-single-line-wrap.log` |
| 2 | `code-lines-merge` | PASS | `/work/.evidence/verification-11/claims/02-code-lines-merge.log` |
| 3 | `page-bounds` | PASS | `/work/.evidence/verification-11/claims/03-page-bounds.log` |
| 4 | `code-content` | PASS | `/work/.evidence/verification-11/claims/04-code-content.log` |
| 5 | `internal-links` | PASS | `/work/.evidence/verification-11/claims/05-internal-links.log` |
| 6 | `syntax-color` | PASS | `/work/.evidence/verification-11/claims/06-syntax-color.log` |
| 7 | `font-metrics` | PASS | `/work/.evidence/verification-11/claims/07-font-metrics.log` |
| 8 | `standard-font-widths` | PASS | `/work/.evidence/verification-11/claims/08-standard-font-widths.log` |
| 9 | `heading-fragments` | PASS | `/work/.evidence/verification-11/claims/09-heading-fragments.log` |
| 10 | `fragment-case` | PASS | `/work/.evidence/verification-11/claims/10-fragment-case.log` |
| 11 | `automatic-heading-ids` | PASS | `/work/.evidence/verification-11/claims/11-automatic-heading-ids.log` |
| 12 | `existing-pdf` | PASS | `/work/.evidence/verification-11/claims/12-existing-pdf.log` |
| 13 | `local-cli-files` | PASS | `/work/.evidence/verification-11/claims/13-local-cli-files.log` |
| 14 | `input-unchanged` | PASS | `/work/.evidence/verification-11/claims/14-input-unchanged.log` |
| 15 | `renderer-no-shell` | PASS | `/work/.evidence/verification-11/claims/15-renderer-no-shell.log` |
| 16 | `renderer-network` | PASS | `/work/.evidence/verification-11/claims/16-renderer-network.log` |
| 17 | `renderer-fail-closed` | PASS | `/work/.evidence/verification-11/claims/17-renderer-fail-closed.log` |
| 18 | `renderer-timeout` | PASS | `/work/.evidence/verification-11/claims/18-renderer-timeout.log` |
| 19 | `html-proof` | PASS | `/work/.evidence/verification-11/claims/19-html-proof.log` |
| 20 | `json-report` | PASS | `/work/.evidence/verification-11/claims/20-json-report.log` |
| 21 | `exit-codes` | PASS | `/work/.evidence/verification-11/claims/21-exit-codes.log` |
| 22 | `sample-demo` | PASS | `/work/.evidence/verification-11/claims/22-sample-demo.log` |
| 23 | `demo-transcript` | PASS | `/work/.evidence/verification-11/claims/23-demo-transcript.log` |
| 24 | `private-site` | PASS | `/work/.evidence/verification-11/claims/24-private-site.log` |
| 25 | `offline-reload` | PASS | `/work/.evidence/verification-11/claims/25-offline-reload.log` |
| 26 | `static-routing` | PASS | `/work/.evidence/verification-11/claims/26-static-routing.log` |
| 27 | `rust-msrv` | PASS | `/work/.evidence/verification-11/claims/27-rust-msrv.log` |
| 28 | `install-from-git` | PASS | `/work/.evidence/verification-11/claims/28-install-from-git.log` |
| 29 | `mit-license` | PASS | `/work/.evidence/verification-11/claims/29-mit-license.log` |

The landing page, runtime messages, terminal transcript, legal copy, metadata,
and README were cross-checked against the manifest. No missing, broader, false,
or untested public claim remains. The candidate's copy audit also binds the
review-5 wording to exact claim IDs and commands.

## Clean build and installed CLI

Clean clone: `/tmp/codeproof-verification11-clean.tT6VCq/repo`.

| Check | Result |
| --- | --- |
| `npm ci` | PASS — 22 packages, 0 vulnerabilities |
| `npm test` | PASS — 4 unit, 28 integration, 21 browser tests |
| `npm run typecheck` | PASS |
| `npm run lint` | PASS — rustfmt and Clippy with warnings denied |
| `npm run build` | PASS — release binary and `dist/site/` produced |
| `cargo package --manifest-path cli/Cargo.toml --locked` | PASS — 15 files, 169.2 KiB unpacked, 39.0 KiB compressed |

A fresh consumer installed the verified packaged source into an empty Cargo
root. `codeproof --version` returned `0.1.0`, and `--help` documented both
commands and exit codes. The installed binary then exercised:

- Normal: a valid Markdown/PDF pair returned 0, emitted JSON with
  `passed: true`, and wrote a non-empty PASS HTML proof sheet.
- Invalid: a missing Markdown path returned 2 with a direct recovery message.
- Boundary: the bundled wrapped-code sample returned 1 and wrote a populated
  HOLD proof with `code.flow-changed`.
- Recovery and safety: the exact tests covered missing content, wrong links,
  every page edge, merged and wrapped code, warning denial, renderer timeout,
  no-shell arguments, denied sockets, and fail-closed sandbox setup.

The public Git install command independently installed commit `ccf8b385` from
an empty root and ran the binary. No interactive prompt appeared. Product
state is local files only; there is no backend, tenant, account, payment,
database, rate-limit, or restart-persistence surface to test.

## Earlier findings

Every earlier review and verification report was inspected, including minor
and low items. Current disposition is below; `.factory/polish-5.md` retains the
full finding-by-finding repair history.

| Source | Findings rechecked | Current proof |
| --- | --- | --- |
| Initial verification | Wrong PDF destinations falsely passed | `internal-links` rejects the wrong destination; valid installed flow passes. |
| Verification 2 | Renderer network access; disappearing service worker | `renderer-network` and `renderer-fail-closed` pass; fresh install, offline reload, and update pass live. |
| Verification 3 | Flattened code; false Rust minimum; serious accessibility; narrow footer target | `code-lines-merge` and Rust 1.88 compile pass; Axe/Lighthouse pass; every visible 390 px control is at least 44×44. |
| Verification 4 | Wrapped one-line code; missed page edges | `single-line-wrap` and `page-bounds` pass, including every MediaBox edge and a non-default CropBox. |
| Verifications 5–8 | No findings reported | Their accepted link, sandbox, package, browser, privacy, and offline paths all pass the current clean and live suites. |
| Verification 9 | Unrelated color false pass; glyph-width overflow; valid heading forms rejected | `syntax-color`, both font-metric claims, and all three heading claims pass. |
| Verification 10 | Stable image names cached immutable | Live WebP/JPG/PNG responses use `public, max-age=3600, must-revalidate`. |
| Review 1 — F-1-1, F-1-2, F-1-21, F-1-26, F-1-28, F-1-31 | Demo visibility/lifecycle, HOLD wording, term agreement, control labels, mobile facts | Fresh phone/desktop demo, transcript equality, history/focus suite, copy audit, and 390 px target checks pass. |
| Review 1 — F-1-3, F-1-11, F-1-12 | Install and Rust/existing-PDF availability | Public Git install at `ccf8b385`, locked Rust 1.88 compile, and installed existing-PDF PASS all succeed. |
| Review 1 — F-1-4–F-1-10, F-1-13–F-1-17, F-1-24–F-1-25 | Overbroad privacy/sandbox/mode/output/license claims and terminology | Exact local-file, no-shell, network, fail-closed, timeout, output, exit, privacy, and license claims pass; retired wording remains absent. |
| Review 1 — F-1-18–F-1-20, F-1-22–F-1-23, F-1-27, F-1-29–F-1-30 | Long or unclear copy, terminology, headings, metadata, routes, footer | Current copy audit passes; live route titles, social/touch metadata, sitemap, navigation, headings, and links pass. |
| Review 2 — F-2-1–F-2-3 | Unproved install/test/build wording | Retired promises remain absent; documented commands pass from the clean clone. |
| Review 2 — F-2-4–F-2-7 | Demo/check headings, page-edge wording, Linux terminology | Exact copy regressions and live focused demo pass; `page-bounds` and sandbox claims pass. |
| Review 3 — F-3-1–F-3-2 | Source integrity and incomplete copy audit | `input-unchanged` covers both supported flows; generated audit is current and now binds review-5 promises. |
| Review 4 — F-4-1–F-4-6 | Merged code, reset focus, workflow/color/output terminology, vague result heading | `code-lines-merge`, reduced-motion keyboard reset, transcript, and current copy tests pass. |
| Review 5 — F-5-1–F-5-9 | Font, heading-ID, case, punctuation, and color claims | The nine corresponding exact CLI claim commands pass independently. |
| Review 5 — F-5-10–F-5-13 | First-screen facts, audience, deploy docs, expanded CI | Fresh first-read evidence and the current copy audit pass. |
| Review 5 — F-5-14 | Stable image caching | Source and live header tests pass; all three images revalidate within one hour. |

No earlier finding has regressed.

## Accessibility, privacy, performance, and deployment identity

- Lighthouse mobile: Performance 100, Accessibility 100, Best Practices 100,
  SEO 100; LCP 1.8 s, TBT 30 ms, CLS 0.
- JavaScript: 4,320 bytes raw / 1.81 kB gzip. CSS: 11,262 bytes raw /
  3.37 kB gzip. Hero WebP: 210,844 bytes.
- Focus starts on the skip link; Enter moves to `main`. Demo focus, Reset with
  Space, live announcements, scrollable command regions, reduced motion, and
  44 px phone targets pass.
- CSP, nosniff, referrer, permissions, and cache headers are live. No console
  error occurred. No third-party script, font, analytics, or tracking request
  was observed.
- Root SHA-256 is
  `76cebc52aa8ca21919579d849318e61307ee7668075a48312ff230b2717c0e3d`.
  Root, Privacy, Terms, 404, service worker, robots, sitemap, original artwork,
  social image, touch icon, SVG mark, CSS, and JavaScript all byte-match the
  candidate build.

Evidence is under `/work/.evidence/verification-11/`. The required report copy
is `/work/.evidence/qa-report.md` and the machine result is
`/work/.evidence/qa-result.json`.

## Final result

**PASS — 0 findings and 0 untested claims.**
