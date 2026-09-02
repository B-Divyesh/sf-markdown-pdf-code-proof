# Polish 4 — cumulative zero-finding closure

- Reviewed candidate: `dfc16a75cd20fb222d78460d56f62e3c2ef42fb0`
- Repair commit: `7adcd759514d4a01b54e529cedd9483a7c96c954`
- Clean remote clone: `/tmp/codeproof-polish4-claims.ADF5zK/repo`
- Live URL: <https://markdown-pdf-code-proof.sociobot.in/>
- Live evidence: `evidence/polish-4-live/`

The live browser suite passed 19/19. All 23 claim commands ran separately from
the clean clone and passed. Screenshot references below are
`landing-desktop.png`, `landing-mobile.png`, and `demo-mobile.png` inside the
live evidence directory.

## Review 1 finding map

| Finding | Change retained or made | Evidence |
| --- | --- | --- |
| F-1-1 | Demo lines stay hidden with `visibility`; visible text never fades below contrast. | `@claim:private-site` runs Axe before and after reveal; live 19/19; `demo-mobile.png`. |
| F-1-2 | Direct `?demo=1`, title, canonical/social metadata, banner, focus, announcements, Reset, exit, Back, and Forward work as real route state. | `sample demo is one click away…`, `direct demo query…`, `keyboard and reduced-motion…`; live demo URL; `demo-mobile.png`. |
| F-1-3 | The sole public install command remains the working Git command. | `@claim:install-from-git` installed remote `7adcd759` from an empty root; live root; `landing-desktop.png`. |
| F-1-4 | File copy remains limited to supplied paths and local artifacts. | `@claim:local-cli-files`; live `/privacy/`; `landing-mobile.png`. |
| F-1-5 | Unproved CLI-wide service claims remain absent; site privacy is observable. | `@claim:private-site` preserved pre-seeded real storage and saw one request origin; live root. |
| F-1-6 | Renderer placeholders remain separate arguments without a shell. | `@claim:renderer-no-shell`; live workflow; `landing-desktop.png`. |
| F-1-7 | The untested filesystem promise remains absent; only socket isolation is stated. | `@claim:renderer-network`; live Renderer safety section. |
| F-1-8 | Sandbox setup failure still prevents renderer execution. | `@claim:renderer-fail-closed`; live Renderer safety section. |
| F-1-9 | Untested Pandoc-sanitizing copy remains absent. | `npm run test:copy-audit`; live/README copy review. |
| F-1-10 | Timeout remains separately registered; script-execution copy remains absent. | `@claim:renderer-timeout`; README Renderer safety. |
| F-1-11 | The locked workspace compiles with Rust 1.88. | `@claim:rust-msrv` from the clean clone; live Install copy. |
| F-1-12 | Existing-PDF mode remains registered and starts no renderer. | `@claim:existing-pdf`; live workflow. |
| F-1-13 | Public copy retains only tested existing-PDF and custom-renderer paths. | `@claim:existing-pdf`, `@claim:renderer-no-shell`; live root. |
| F-1-14 | Public copy names visible defects instead of PDF internals. | Generated copy audit; live Checks section; `landing-desktop.png`. |
| F-1-15 | HTML proof sheet, JSON report, and exit codes remain separate claims. | `@claim:html-proof`, `@claim:json-report`, `@claim:exit-codes`; live workflow. |
| F-1-16 | Subjective diagnostic-matrix wording remains absent. | `npm run test:copy-audit`; README review. |
| F-1-17 | MIT references remain consistent in the repository, crate, landing page, and Terms. | `@claim:mit-license`; live `/terms/`. |
| F-1-18 | Exit meanings remain three short sentences. | Generated README copy audit; `@claim:exit-codes`. |
| F-1-19 | Internal-link behavior remains two short sentences. | Generated README copy audit; `@claim:internal-links`. |
| F-1-20 | Code-fence flow behavior remains two short sentences. | Generated README copy audit; `@claim:single-line-wrap`. |
| F-1-21 | The primary action explains the failed sample; HOLD is defined only as `HOLD — do not release`. | `sample demo is one click away…`; live demo; `demo-mobile.png`. |
| F-1-22 | Public copy continues to use Markdown, PDF, code fence, and HTML proof sheet. | Copy terminology regression; live root; `landing-desktop.png`. |
| F-1-23 | Workflow copy names links, syntax color, and page overflow outcomes. | Copy regression; live workflow; `landing-desktop.png`. |
| F-1-24 | Safety copy states the tested Linux socket restriction. | `@claim:renderer-network`; live Renderer safety section. |
| F-1-25 | Renderer remains the sole public component term. | Copy regression; README and live root. |
| F-1-26 | CLI terminal output, browser transcript, README, and HTML proof sheet now use `HOLD — do not release`; the full real diagnostic is preserved. The site transcript is generated from the CLI. | `@claim:demo-transcript`, `@claim:sample-demo`, `npm run test:demo-transcript`; live demo; `demo-mobile.png`. |
| F-1-27 | Workflow and demo headings identify the PDF, Markdown, and failed check when read alone. | Heading assertions; live root/demo; both screenshots. |
| F-1-28 | Controls still name their result. | Live browser suite; `landing-mobile.png`, `demo-mobile.png`. |
| F-1-29 | Social image, touch icon, per-route metadata, demo sitemap URL, and 404 metadata remain complete; `og:url` was added everywhere. | Route metadata tests; live `/`, `?demo=1`, legal routes, and 404. |
| F-1-30 | All routes retain Home, Privacy, Terms, and labelled external GitHub links. | `has route metadata and complete legal navigation`; live routes. |
| F-1-31 | All three facts remain above 844 px at 390 px without horizontal page overflow. | `390px layout keeps primary paths available`; `landing-mobile.png`. |

## Review 2 finding map

| Finding | Change retained or made | Evidence |
| --- | --- | --- |
| F-1-26 | Retired code-region/output terms remain rejected; round 4 also binds the recording to real CLI output. | Copy regression plus `@claim:demo-transcript`; live demo; `demo-mobile.png`. |
| F-2-1 | Redundant checkout-install wording remains absent. | Copy regression; `@claim:install-from-git`; live Install section. |
| F-2-2 | README gives the direct `npm test` instruction without a coverage promise. | Generated README copy audit. |
| F-2-3 | README gives the direct build instruction without promising output paths. | Generated README copy audit; clean `npm run build`. |
| F-2-4 | The demo focus target remains “Sample failed release check.” | Direct demo/focus tests; live demo; `demo-mobile.png`. |
| F-2-5 | The check heading remains “Missing or wrapped code.” | Copy regression; live Checks section. |
| F-2-6 | Page-edge copy remains user-facing and free of PDF implementation terms. | `@claim:page-bounds`; copy regression; live Checks section. |
| F-2-7 | Public safety copy says “Linux sandbox,” without unexplained kernel names. | `@claim:renderer-fail-closed`; copy regression; live root. |

## Review 3 finding map

| Finding | Change retained or made | Evidence |
| --- | --- | --- |
| F-3-1 | The source-integrity claim remains registered and checks both existing-PDF and custom-renderer flows byte-for-byte. | `@claim:input-unchanged`; README opening. |
| F-3-2 | The generated audit includes landing HTML, runtime feedback, generated CLI transcript, and README; it rejects stale output, long copy, banned words, and retired terminology. | `npm run test:copy-audit`; `.factory/copy-audit.md`. |

## Review 4 finding map

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-26 | Added a build-time CLI transcript generator, canonical failed-decision constant, exact full diagnostic, and browser-to-CLI comparison. | `@claim:demo-transcript`, `@claim:sample-demo`; live `?demo=1`; `demo-mobile.png`. |
| F-4-1 | Registered the separate two-source-lines-to-one-PDF-line behavior. | `@claim:code-lines-merge` selects `flattened_code_lines_fail_the_release_contract`; clean clone PASS. |
| F-4-2 | Reset remains focusable and uses `aria-disabled` while replaying; focus is asserted immediately and after completion. | `keyboard and reduced-motion users receive demo feedback`; live demo; `demo-mobile.png`. |
| F-4-3 | Replaced “Check the source…” with “Check Markdown against the finished PDF.” | Copy/heading regression; live workflow; `landing-desktop.png`. |
| F-4-4 | Standardized the workflow, check, and README on “syntax color.” | Copy regression and generated audit; live Checks section. |
| F-4-5 | Split the three workflow actions into three sentences. | Generated copy audit; live workflow; `landing-desktop.png`. |
| F-4-6 | Renamed step 03 to “Review the HTML proof sheet.” | Heading regression; live workflow; `landing-desktop.png`. |

## Final verification

- Clean clone claims: 23/23 passed separately.
- Clean `npm test`: 3 unit, 22 CLI integration, and 19 browser tests passed.
- Clean typecheck, rustfmt, Clippy, release build, and verified package passed.
- Cold live Playwright: 19/19 passed with Axe, privacy, offline, mobile, focus,
  metadata, legal pages, and 404 coverage.
- Fleet verifier: PASS, 791 ms, zero console errors.
- Lighthouse mobile: Performance 99, Accessibility 100, Best Practices 100,
  SEO 100; LCP 1,814 ms, TBT 0 ms, CLS 0.
- Live/local root SHA-256:
  `ab16dcc89f47fabae535fa07e91276b81ea6a3dbd6e320934ddf6f3225e705ff`.

No finding of any severity remains open.
