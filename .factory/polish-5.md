# Polish 5 — cumulative zero-finding closure

- Reviewed release candidate: `0fff412476781d63482d2d540adc8de2caea8c94`
- Adversarial review commit: `bfbedad80679a091668aaf4e1292a7779ac714ba`
- Repair implementation: `844f3c79da9437874156f4d0ba517b00cc9266b2`
- Clean remote clone: `/tmp/codeproof-polish5-clean.9kX5nF/repo`
- Live URL: <https://markdown-pdf-code-proof.sociobot.in/>
- Live evidence: `.factory/evidence/polish-5-live/`

All 14 review-5 findings were repaired. Every one of the 29 claim commands
passed separately from the clean remote clone. The clean full suite and the
21-test cold live browser suite also passed.

## Review 5 finding map

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-5-1 | Narrowed the README promise to Helvetica and Courier, added both font families to one boundary fixture, and registered `standard-font-widths`. | `helvetica_and_courier_width_tables_drive_page_geometry`; clean claim run PASS. |
| F-5-2 | Replaced the ATX jargon with “`# Heading`,” added an ATX case beside underlined and explicit-ID cases, and updated `heading-fragments`. | `atx_setext_and_pandoc_explicit_heading_ids_resolve_pdf_fragments`; PASS. |
| F-5-3 | Reworded the promise as “Link target matching ignores letter case,” added mixed-case Markdown/PDF inputs, and registered `fragment-case`. | `fragment_matching_ignores_letter_case`; PASS. |
| F-5-4 | Removed the broad Pandoc automatic-ID compatibility promise and documented only the implemented rules. | `automatic_heading_ids_follow_documented_rules`; PASS. |
| F-5-5 | Replaced “most punctuation” with the exact retained set and a formatting-mark rule. | `automatic_heading_ids_follow_documented_rules` checks emphasis, inline code, periods, underscores, and hyphens; PASS. |
| F-5-6 | Kept the space-to-hyphen statement and registered an exact multiword target. | `automatic_heading_ids_follow_documented_rules`; `Retry Policy` resolves to `retry-policy`; PASS. |
| F-5-7 | Kept the lowercase statement and covered mixed-case source text. | `automatic_heading_ids_follow_documented_rules`; PASS. |
| F-5-8 | Reworded the rule as dropping characters before the first letter and covered numeric and punctuation prefixes. | `automatic_heading_ids_follow_documented_rules`; `2. Retry` and `— Retry` resolve to `retry`; PASS. |
| F-5-9 | Narrowed the exclusion to unrelated colored headings and graphics and expanded the `syntax-color` claim text to match. | `unrelated_blue_graphic_does_not_mask_black_code`; PASS. |
| F-5-10 | First-screen facts are now “Works offline,” “Site privacy,” and “Free software,” with explanatory second lines. | `@claim:offline-reload`, `@claim:private-site`, `npm run test:license`, `390px layout keeps primary paths available`; live `landing-mobile-first-screen.png`. |
| F-5-11 | The README opening now names engineers and technical writers. | `npm run test:copy-audit`; README source review. |
| F-5-12 | README now identifies `dist/site/` as the deploy root and requires its Static Web Apps configuration. Registered the resulting header/404 contract. | `@claim:static-routing`; local and live PASS. |
| F-5-13 | Expanded continuous integration on first use in the landing page and README, and removed the unexplained Terms abbreviation. | `npm run test:copy-audit`; live root and Terms checks. |
| F-5-14 | Stable WebP, JPG, and PNG responses now revalidate after one hour; only hashed build assets remain one-year immutable. Bumped the service-worker cache to v6. | `stable-name images use short revalidating cache policies`; live headers for all three named images. |

## Earlier finding regression map

Every earlier finding was rechecked through the clean full suite and cold live
suite. These rows identify the retained fix and its current evidence.

| Finding | Retained change | Current evidence |
| --- | --- | --- |
| F-1-1 | Demo text is hidden with `visibility`, never low opacity. | `@claim:private-site` transition Axe checks; live suite. |
| F-1-2 | Demo entry/exit updates URL, title, banner, focus, announcement, Back, and Forward. | `sample demo is one click away and reports completion`; live suite. |
| F-1-3 | The public Git install command works from an empty directory. | `@claim:install-from-git` installed commit `844f3c79`; PASS. |
| F-1-4 | File-handling wording is scoped to supplied paths and local output. | `@claim:local-cli-files`; live Privacy. |
| F-1-5 | Unproved CLI service/telemetry wording remains absent; site privacy is registered. | `@claim:private-site`; PASS. |
| F-1-6 | Renderer placeholders remain separate arguments, without a shell. | `@claim:renderer-no-shell`; PASS. |
| F-1-7 | The unproved filesystem-limits promise remains absent. | Copy audit; README review. |
| F-1-8 | Sandbox setup failure prevents renderer execution. | `@claim:renderer-fail-closed`; PASS. |
| F-1-9 | Unproved Pandoc sanitizing wording remains absent. | Copy audit; README review. |
| F-1-10 | Timeout is registered; Markdown-script wording remains absent. | `@claim:renderer-timeout`; PASS. |
| F-1-11 | Rust 1.88 performs a real locked workspace compile. | `@claim:rust-msrv`; PASS. |
| F-1-12 | Existing-PDF mode starts no renderer. | `@claim:existing-pdf`; PASS. |
| F-1-13 | Public copy documents only exercised existing-PDF and custom-renderer paths. | `@claim:existing-pdf`, `@claim:renderer-no-shell`; PASS. |
| F-1-14 | Public copy names outcomes instead of PDF implementation details. | Generated copy audit; live landing. |
| F-1-15 | HTML proof, JSON, and exit-code contracts remain separately registered. | `@claim:html-proof`, `@claim:json-report`, `@claim:exit-codes`; PASS. |
| F-1-16 | Subjective diagnostic-matrix wording remains absent. | Generated copy audit. |
| F-1-17 | Free-software and MIT statements match the package and license. | `@claim:mit-license`; PASS. |
| F-1-18 | Exit meanings remain three short sentences. | Generated copy audit. |
| F-1-19 | Internal-link behavior remains two short sentences. | Generated copy audit; `@claim:internal-links`. |
| F-1-20 | Code-wrap behavior remains two short sentences. | Generated copy audit; `@claim:single-line-wrap`. |
| F-1-21 | The primary action describes the sample; HOLD is defined in the terminal result. | Live one-click demo and screenshot. |
| F-1-22 | Markdown, PDF, code fence, and HTML proof sheet remain the public terms. | Copy terminology regression. |
| F-1-23 | Workflow copy names links, syntax color, and page overflow outcomes. | Live landing and copy audit. |
| F-1-24 | Safety copy names the tested Linux socket restriction. | `@claim:renderer-network`; PASS. |
| F-1-25 | Public prose consistently uses “renderer.” | Copy terminology regression. |
| F-1-26 | Browser, README, CLI, and proof use the same code-fence/output/HOLD terms. | `@claim:demo-transcript`, `@claim:sample-demo`; PASS. |
| F-1-27 | Workflow and sample headings name their subjects. | Live heading assertions. |
| F-1-28 | Every visible control names its result. | Live browser suite and Axe. |
| F-1-29 | Social/touch metadata, canonical URLs, titles, and demo sitemap remain complete. | Route metadata tests; live suite. |
| F-1-30 | All routes retain Home, Privacy, Terms, and labelled GitHub links. | Route navigation tests; live suite. |
| F-1-31 | All three required facts fit in the 390×844 first viewport. | Mobile layout test; `landing-mobile-first-screen.png`. |
| F-2-1 | The redundant checkout-install promise remains absent. | Copy audit; `@claim:install-from-git`. |
| F-2-2 | README gives a verification command without a coverage promise. | Generated copy audit. |
| F-2-3 | README gives a build command without an unregistered output promise. | Generated copy audit; clean build. |
| F-2-4 | Demo focus target remains “Sample failed release check.” | Direct demo focus test. |
| F-2-5 | The check heading remains “Missing or wrapped code.” | Copy regression; live root. |
| F-2-6 | Page-bound copy remains in user-facing terms. | `@claim:page-bounds`; copy audit. |
| F-2-7 | Safety copy says “Linux sandbox,” without kernel jargon. | `@claim:renderer-fail-closed`; copy audit. |
| F-3-1 | Source integrity remains registered across existing-PDF and renderer flows. | `@claim:input-unchanged`; PASS. |
| F-3-2 | The generated audit covers landing, runtime, transcript, and README copy. | `npm run test:copy-audit`; PASS. |
| F-4-1 | Merged source lines have a separate release-failure claim. | `@claim:code-lines-merge`; PASS. |
| F-4-2 | Reset keeps keyboard focus and announces both states. | Reduced-motion/keyboard browser test; live suite. |
| F-4-3 | Workflow heading names Markdown and the finished PDF. | Copy regression; live root. |
| F-4-4 | Workflow, check, and README use “syntax color.” | Copy regression; `@claim:syntax-color`. |
| F-4-5 | The three output actions remain separate sentences. | Generated copy audit. |
| F-4-6 | Step 03 remains “Review the HTML proof sheet.” | Heading regression; live root. |

## Verification evidence

- Clean clone SHA: `844f3c79da9437874156f4d0ba517b00cc9266b2`.
- All 29 exact `.factory/claims.json` commands: PASS, rerun separately.
- Clean `npm test`: 4 unit, 28 CLI integration, and 21 browser tests passed.
- Clean `npm run typecheck`, `npm run lint`, `npm run build`, and verified
  `cargo package --manifest-path cli/Cargo.toml --locked`: PASS.
- Live Playwright: 21/21 passed. Axe found no serious or critical issue.
- Fleet verifier: PASS in 629 ms with no console errors.
- Live Lighthouse mobile: Performance 100, Accessibility 100, Best Practices
  100, SEO 100; LCP 1,819 ms, TBT 0 ms, CLS 0.
- Live image responses: `public, max-age=3600, must-revalidate`.
- Every material deployed file byte-matches `dist/site/`; root SHA-256 is
  `76cebc52aa8ca21919579d849318e61307ee7668075a48312ff230b2717c0e3d`.
- Live route status: root/demo/privacy/terms 200; unknown route 404 with the
  byte-matched branded 404 document.

No finding of any severity remains open.
