# Polish 2 — cumulative finding closure

- Reviewed candidate: `f1474e5871a1c5c28d4e9967c8f9476a41f20a79`
- Repair commits: `0a18c6a83db473b2ca129a4b297b6882f807cc08` and
  `18510ca669597dfbb6a8fe2159d9a05998242db4`
- Final clean clone: `/tmp/codeproof-polish2-final.sMJ5XD/repo`
- Live URL: <https://markdown-pdf-code-proof.sociobot.in>
- Live screenshots: `evidence/polish-2-live/screenshot-mobile.png`,
  `evidence/polish-2-live/screenshot-desktop.png`, and
  `evidence/polish-2-live/demo-mobile.png`

The live browser suite ran with
`PLAYWRIGHT_BASE_URL=https://markdown-pdf-code-proof.sociobot.in npx playwright test`.
All 13 tests passed. “Live suite” below refers to that cold production run.

## Review 1 finding map

| Finding | Change retained or made | Evidence |
| --- | --- | --- |
| F-1-1 | Demo lines remain hidden with `visibility`, so no low-contrast transition state exists. | `@claim:private-site` runs Axe at demo start and completion; live suite; live demo screenshot. |
| F-1-2 | `?demo=1` has its own title, banner, focus target, announcement, reset, and clean exit to `/#install`; Back and Forward restore state. | `sample demo is one click away and reports completion`; live suite; live demo screenshot. |
| F-1-3 | The only public install path is the tested Git command. | `npm run test:install` installed Git commit `18510ca6` from an empty root and ran `codeproof 0.1.0`; live landing screenshot. |
| F-1-4 | File-handling copy stays limited to supplied paths and local HTML proof sheet output. | `documented_existing_pdf_flow_passes_and_writes_proof`; live `/privacy/`. |
| F-1-5 | Unprovable account, daemon, and CLI telemetry promises remain removed. | `@claim:private-site`; live suite observed one origin and empty cookies/Web Storage. |
| F-1-6 | Renderer placeholders remain individual arguments without shell input. | `custom_renderer_runs_without_a_shell_and_is_checked`. |
| F-1-7 | Public safety copy claims only tested socket isolation. | `renderer_sandbox_denies_network_connections`; live Renderer safety section. |
| F-1-8 | Renderer launch still fails closed when sandbox setup fails. | `renderer_refuses_to_start_when_sandbox_setup_fails`. |
| F-1-9 | Untested Pandoc sanitizing copy remains removed. | `user-facing copy keeps one plain term for each output and check`; README audit. |
| F-1-10 | Timeout copy remains narrow and directly tested; the embedded-script promise stays removed. | `renderer_deadline_stops_a_long_running_command`. |
| F-1-11 | Rust 1.88 compiles the locked workspace. | `npm run test:msrv` in the clean clone. |
| F-1-12 | Existing-PDF mode stays documented without an “everywhere” claim. | `documented_existing_pdf_flow_passes_and_writes_proof`. |
| F-1-13 | Public copy documents only the tested existing-PDF and custom-renderer paths. | `documented_existing_pdf_flow_passes_and_writes_proof`; `custom_renderer_runs_without_a_shell_and_is_checked`. |
| F-1-14 | Low-level PDF implementation inventory remains replaced by visible defect outcomes. | Live Checks section; live landing screenshot. |
| F-1-15 | HTML proof sheet, JSON report, and exit codes stay separate registered claims. | `documented_existing_pdf_flow_passes_and_writes_proof`; `json_report_and_exit_codes_are_observable`. |
| F-1-16 | Subjective diagnostic-matrix wording remains removed. | README copy audit. |
| F-1-17 | MIT packaging and site references remain verified. | `npm run test:license`; live `/terms/`. |
| F-1-18 | Exit meanings remain three short sentences. | `.factory/copy-audit.md`; README Usage. |
| F-1-19 | Link behavior remains two short sentences. | `.factory/copy-audit.md`; README Checks. |
| F-1-20 | Code-fence wrapping remains two short sentences. | `.factory/copy-audit.md`; README Checks. |
| F-1-21 | HOLD is defined only inside the result as “HOLD — do not release.” | `sample demo is one click away and reports completion`; live demo screenshot. |
| F-1-22 | Public copy continues to use Markdown, PDF, and HTML proof sheet instead of internal metaphors. | `user-facing copy keeps one plain term for each output and check`; live landing screenshot. |
| F-1-23 | The workflow continues to name links, syntax color, and page overflow as outcomes. | Live Checks section and screenshot. |
| F-1-24 | Safety wording names the tested Linux socket restriction. | `renderer_sandbox_denies_network_connections`; live Renderer safety section. |
| F-1-25 | “Renderer” remains the single public term. | Copy regression; README and live landing. |
| F-1-26 | Standardized the final stragglers: “code fence,” “HTML proof sheet,” and “JSON report.” The demo is a “failed release check.” CLI diagnostics, HTML output, terminal labels, site, README, Terms, and JSON `code_fences` now agree. | `user-facing copy keeps one plain term for each output and check`; `documented_existing_pdf_flow_passes_and_writes_proof`; `demo_uses_bundled_sample_data_and_writes_an_isolated_proof`; live demo screenshot and live suite. |
| F-1-27 | Headings stay task-specific; the demo heading is now “Sample failed release check.” | Copy regression; live demo screenshot. |
| F-1-28 | Copy and demo-exit controls retain result-naming labels. | Live suite; live landing and demo screenshots. |
| F-1-29 | Social image, Apple icon, route metadata, and demo sitemap entry remain present. | `npm run build:site`; live hash comparison and live suite. |
| F-1-30 | Every route retains the same Home, Privacy, Terms, and labeled external GitHub footer. | Live suite on `/`, `/privacy/`, `/terms/`, and 404. |
| F-1-31 | All three facts remain inside the 390×844 first viewport. | `390px layout keeps primary paths available`; live mobile screenshot. |

## Review 2 finding map

| Finding | Change | Evidence |
| --- | --- | --- |
| F-1-26 | Replaced “blocks,” “Code flow,” “Sample HOLD report,” bare “Proof sheet,” and other public stragglers across the site and CLI. Added a regression that rejects every retired phrase. | `user-facing copy keeps one plain term for each output and check`; live suite; `evidence/polish-2-live/demo-mobile.png`. |
| F-2-1 | Removed the redundant checkout-install sentence. The public Git install command is now the sole installation promise. | Copy regression; `npm run test:install` from the clean clone. |
| F-2-2 | Replaced the test-coverage promise with the instruction “Verify the project: `npm test`.” | Copy regression; README Develop and verify. |
| F-2-3 | Replaced promised output paths with the instruction “Build the CLI and site: `npm run build`.” | Copy regression; clean-clone `npm run build` passed. |
| F-2-4 | Renamed the focused heading to “Sample failed release check.” | `sample demo is one click away and reports completion`; live demo screenshot. |
| F-2-5 | Renamed “Code flow” to “Missing or wrapped code.” | Copy regression; live landing screenshot. |
| F-2-6 | Rewrote the README result as “Text is checked against every visible PDF page edge.” CLI diagnostics now say “Text” too. | `page_bounds_cover_every_media_and_crop_edge`; copy regression. |
| F-2-7 | Replaced unexplained kernel names with “Code Proof applies its Linux sandbox before a renderer starts.” | `renderer_refuses_to_start_when_sandbox_setup_fails`; copy regression. |

## Verification summary

- Final clean clone SHA: `18510ca669597dfbb6a8fe2159d9a05998242db4`.
- Claims: 20 of 20 commands passed separately.
- Full clean suite: 3 Rust unit tests, 21 CLI integration tests, 13 browser
  tests, Rust 1.88 compile, license, typecheck, rustfmt, Clippy, build, and
  verified `cargo package` all passed.
- Real CLI demo: exit 1, isolated source/PDF/HTML proof sheet, expected
  `code.flow-changed` finding.
- Live suite: 13 of 13 passed with Axe, privacy, offline, mobile, focus,
  routing, 404, and legal checks.
- Live Lighthouse: Performance 100, Accessibility 100, Best Practices 100,
  SEO 100; LCP 1.8 s, TBT 0 ms, CLS 0.
- Local and live `index.html` SHA-256 matched:
  `e37af9d0830b0ff36058c9f824e831055f59389f1490eaef3a3562f4c8e5c7d6`.

No review finding remains open.
