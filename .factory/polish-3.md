# Polish 3 — zero-finding closure

- Repaired source commit: `66287975257c0f76c8b2cfff403348a17f3a6e15`
- Clean remote clone: `/tmp/codeproof-polish3-clean.uhWU49/repo`
- Live URL: <https://markdown-pdf-code-proof.sociobot.in/>
- Live evidence: `evidence/polish-3-live/`

The repaired commit was pushed to `origin/main`, then checked from a fresh
remote clone and against the cold live URL. Every one of the 21 registered
claim commands passed verbatim. The clean full suite passed with 3 Rust unit
tests, 22 CLI integration tests, and 13 browser tests.

## Review 1 finding map

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | Demo lines remain `visibility: hidden` until revealed; text opacity is never animated. | `@claim:private-site` runs Axe during demo start; live `/?demo=1#demo`; `evidence/polish-3-live/demo-first-viewport.png`. |
| F-1-2 | The demo URL, route title, banner, reset, exit, focus, live announcement, Back, and Forward behavior remain isolated and tested. | `sample demo is one click away and reports completion`; live Playwright 13/13; `demo-mobile.png`. |
| F-1-3 | The sole public installation command remains the tested Git install command. | `@claim:install-from-git` installed remote revision `66287975` from an empty root. |
| F-1-4 | Public privacy wording stays limited to supplied paths and local HTML proof output. | `@claim:local-cli-files`; live `/privacy/`. |
| F-1-5 | Unprovable account, daemon, and CLI telemetry promises remain absent; the documentation-demo privacy promise is tested. | `@claim:private-site`; live request/storage check. |
| F-1-6 | Custom renderer placeholders stay individual arguments rather than shell input. | `@claim:renderer-no-shell`. |
| F-1-7 | The untested renderer filesystem-limit promise remains removed; the tested Linux socket claim remains narrow. | `@claim:renderer-network`; live Renderer safety section. |
| F-1-8 | Sandbox setup refusal still prevents a renderer from starting. | `@claim:renderer-fail-closed`. |
| F-1-9 | Untested Pandoc sanitizer/fixed-argument copy remains removed. | `npm run test:copy-audit`; README audit. |
| F-1-10 | The tested timeout claim remains separate and the untested Markdown-script promise remains removed. | `@claim:renderer-timeout`. |
| F-1-11 | Rust 1.88 compilation is an exact, locked test. | `@claim:rust-msrv`. |
| F-1-12 | Existing-PDF checking remains registered and does not start a renderer. | `@claim:existing-pdf`. |
| F-1-13 | Public docs retain only the exercised existing-PDF and custom-renderer modes. | `@claim:existing-pdf`; `@claim:renderer-no-shell`. |
| F-1-14 | PDF-internals inventory remains replaced by user-visible defect outcomes. | `npm run test:copy-audit`; live landing screenshot. |
| F-1-15 | HTML proof sheet, JSON report, and exit-code contracts stay separately registered. | `@claim:html-proof`; `@claim:json-report`; `@claim:exit-codes`. |
| F-1-16 | The subjective diagnostic-matrix promise remains removed. | `npm run test:copy-audit`; README audit. |
| F-1-17 | MIT references remain matched across package and site. | `@claim:mit-license`; live `/terms/`. |
| F-1-18 | Exit meanings remain three short, registered sentences. | `@claim:exit-codes`; generated copy audit. |
| F-1-19 | Internal-link wording remains two plain sentences. | `@claim:internal-links`; generated copy audit. |
| F-1-20 | Code-fence wrap wording remains two plain sentences. | `@claim:single-line-wrap`; generated copy audit. |
| F-1-21 | The primary action explains the failed sample; HOLD is defined in the terminal result. | `sample demo is one click away and reports completion`; `demo-first-viewport.png`. |
| F-1-22 | Public terms remain Markdown, PDF, code fence, and HTML proof sheet. | `user-facing copy keeps one plain term for each output and check`; generated copy audit. |
| F-1-23 | Workflow text continues to name links, syntax color, and page overflow outcomes. | Live landing check; `screenshot-desktop.png`. |
| F-1-24 | Renderer safety copy stays limited to tested Linux network-socket behavior. | `@claim:renderer-network`; live landing check. |
| F-1-25 | Public prose uses renderer consistently. | `user-facing copy keeps one plain term for each output and check`; generated copy audit. |
| F-1-26 | Code fence, HTML proof sheet, JSON report, and sample failed release check remain the sole product terms. | Copy regression; live `/?demo=1#demo`; `demo-first-viewport.png`. |
| F-1-27 | Workflow and demo headings remain standalone task descriptions. | Playwright demo route test; live screenshots. |
| F-1-28 | Visible controls continue to name their results. | Live Playwright suite; `demo-first-viewport.png`. |
| F-1-29 | Required social/touch metadata, demo sitemap entry, and styled 404 remain in the production build. | `npm run build`; live Playwright route checks. |
| F-1-30 | Header/footer and labelled external GitHub link remain consistent on all routes. | Live Playwright suite on root, legal routes, and 404. |
| F-1-31 | All three first-screen facts remain visible without horizontal overflow at 390×844. | `390px layout keeps primary paths available`; `screenshot-mobile.png`. |

## Review 2 finding map

| Finding | Change made | Evidence |
| --- | --- | --- |
| Review-2 F-1-26 | Terminology repair remains enforced against retired “blocks” and bare-report wording. | `user-facing copy keeps one plain term for each output and check`; generated audit. |
| F-2-1 | The redundant checkout-install claim remains removed. | Copy regression; `@claim:install-from-git`. |
| F-2-2 | README uses the direct verification instruction rather than promising test coverage. | `npm run test:copy-audit`. |
| F-2-3 | README uses the direct build instruction rather than promising output paths. | `npm run test:copy-audit`; clean `npm run build`. |
| F-2-4 | The demo focus target remains “Sample failed release check.” | Live demo focus check; `demo-first-viewport.png`. |
| F-2-5 | The check heading remains “Missing or wrapped code.” | Copy regression; live landing check. |
| F-2-6 | README keeps the user-facing “visible PDF page edge” wording. | `@claim:page-bounds`; generated audit. |
| F-2-7 | README keeps “Linux sandbox,” not unexplained implementation names. | `@claim:renderer-fail-closed`; generated audit. |

## Review 3 finding map

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-3-1 | Replaced the ambiguous scope sentence with “Code Proof does not edit the supplied Markdown source,” registered `input-unchanged`, and added an exact test that byte-compares the Markdown fixture after both existing-PDF and custom-renderer checks. | `@claim:input-unchanged` → `input_files_remain_unchanged_in_existing_pdf_and_custom_renderer_checks`; clean clone PASS. |
| F-3-2 | Replaced the stale manual audit with `scripts/audit-copy.mjs`. It extracts landing HTML, runtime feedback, accessibility labels, and README prose; counts whitespace-delimited words; rejects over-22-word and banned-word copy; and byte-compares generated output in `npm run test:copy-audit`. | Clean `npm run test:copy-audit` PASS; [copy audit](copy-audit.md). |

## Deployment and live verification

- `git push origin main` published `6628797`; the cold live root matched the
  freshly built `index.html` SHA-256
  `e37af9d0830b0ff36058c9f824e831055f59389f1490eaef3a3562f4c8e5c7d6`.
- `/opt/fleet/lib/verify-url.sh` passed: HTTPS 200, `lang=en`, one H1, main
  landmark, image alt text, no unlabeled buttons, and no console errors.
- `PLAYWRIGHT_BASE_URL=https://markdown-pdf-code-proof.sociobot.in npx playwright test`
  passed 13/13, including Axe, demo lifecycle, offline reload, privacy,
  focus, mobile, routing, 404, and legal routes.
- Mobile Lighthouse: Performance 100, Accessibility 100, Best Practices 100,
  SEO 100; LCP 1804 ms, TBT 0 ms, CLS 0. Report:
  `evidence/polish-3-live/lighthouse-mobile.json`.

No review finding remains open.
