# Adversarial first-read review 4 — FAIL

- Product: Code Proof (`markdown-pdf-code-proof`)
- Candidate: `dfc16a75cd20fb222d78460d56f62e3c2ef42fb0`
- Live URL: <https://markdown-pdf-code-proof.sociobot.in/>
- Reviewed: 2 September 2026 UTC
- Verdict: **FAIL**

There are seven findings: one blocking, one high, four medium, and one minor.
The landing page is immediately understandable, the sandbox is isolated, all
21 registered claim entries pass, and the live deployment byte-matches the
candidate. The product still fails because its purported terminal recording is
not the output of the real CLI. That mismatch reopens earlier finding F-1-26.

## Cold first read

Fresh Chromium contexts opened the live root at 390×844 and 1440×900 before
scrolling.

My first-screen answers were:

- What does this do? It checks a finished Markdown PDF for broken code, page
  overflow, and internal-link defects before release.
- For whom? Engineers and technical writers producing code-heavy manuals.
- What should I click first? **Try it with sample data**.

The exact text that supplied those answers was:

> Catch PDF bugs before release.
>
> For engineers and technical writers, Code Proof catches broken code, page
> overflow, and internal links in the final PDF.
>
> TRY IT WITH SAMPLE DATA
>
> See a sample PDF defect and failed check.

This part passes. On the phone, the headline, audience, primary action, outcome
note, install alternative, and all three facts appeared before the first
scroll. Desktop showed the same information and the original artwork. Neither
viewport had horizontal page overflow.

## Findings

### Blocking

#### F-1-26 — The recorded demo and README do not match the CLI output

- Earlier status: review 1 required stable output terms, review 2 repeated the
  finding as blocking, and all three polish reports marked it fixed.
- Live location: `/?demo=1#demo`, element labelled “Recorded Code Proof
  terminal output.”
- Live quote: “DEMO HOLD — do not release — 1 expected defect found” and
  “ERROR [code.flow-changed] source line 7”.
- README quote: “It returns exit `1` with `HOLD — do not release`.”
- Real release-binary output from a fresh temporary directory:

  ```text
  DEMO HOLD — 1 expected defect found
    Error [code.flow-changed] Code fence on line 7 is present but its line flow changed
  ```

- Code evidence: `site/index.html` hard-codes the first version;
  `cli/tests/cli.rs` explicitly expects the second version. The generated HTML
  proof sheet also uses the heading “HOLD,” not “HOLD — do not release.”
- Why this blocks: the only one-click demo identifies itself as recorded CLI
  output, but it is an edited simulation. A first-time visitor cannot use the
  demo to learn what the installed command actually reports. This is an
  output-term regression under the same earlier finding ID, so the work order
  requires it to be blocking.
- Concrete fix: make the real CLI, browser recording, README, proof sheet, and
  terminology table use one exact failed-decision phrase. Preserve the real
  diagnostic rather than shortening it. Generate the browser transcript from
  a checked fixture, and add a test that normalizes temporary paths and
  compares the displayed transcript with `codeproof demo` output.

### High

#### F-4-1 — “Merge” is an unlisted product claim

- Location: landing page, **Missing or wrapped code**.
- Exact quote: “Flags code fence lines that disappear, merge, or wrap in the
  PDF.”
- Evidence: `claims.json` registers disappearing text as `code-content` and a
  single line wrapping as `single-line-wrap`. Neither claim states or invokes
  the separate two-lines-becoming-one behavior. The repository has a suitable
  test, `flattened_code_lines_fail_the_release_contract`, but no claim entry
  selects it.
- Why this matters: claim-only verification can pass while this advertised
  behavior regresses.
- Concrete fix: add a `code-lines-merge` claim whose exact test is
  `cargo test --test cli flattened_code_lines_fail_the_release_contract -- --exact`,
  or remove “merge” from the landing sentence.

### Medium

#### F-4-2 — Resetting the demo loses keyboard focus

- Location: live demo, **Reset demo**.
- Evidence: after the first run settled, I focused Reset and pressed Space.
  The button immediately became disabled and `document.activeElement` became
  `BODY`. Focus was still on `BODY` after the replay completed and the button
  was enabled again.
- Why this matters: a keyboard or screen-reader user loses their place after
  using the required reset control. The current reduced-motion test checks the
  status text but does not assert focus.
- Concrete fix: keep the control focusable while preventing duplicate runs,
  or move focus to the demo heading/status and announce that move. Add an
  assertion for the active element immediately after Reset and after replay.

#### F-4-3 — The workflow heading uses an inconsistent, ambiguous term

- Location: landing page, **How it works**.
- Exact quote: “Check the source against the PDF.”
- Evidence: the checked-in terminology table says the source document is
  called “Markdown,” and the next sentence does use “your Markdown.”
- Why this matters: heard alone in a heading list, “the source” could mean
  source code, the Markdown file, or a source PDF.
- Concrete rewrite: “Check Markdown against the finished PDF.”

#### F-4-4 — The syntax-color check has three names

- Locations and exact quotes: landing workflow, “Check links, **code colors**,
  and text that runs outside the page”; landing check, “no detectable
  **syntax color**”; README Checks, “no **non-default PDF color** is found.”
- Why this matters: “non-default PDF color” is implementation jargon, while
  the three terms make one feature appear to be three different checks.
- Concrete rewrite: use “syntax color” throughout. For the README: “A
  language-tagged code fence warns when Code Proof cannot detect syntax
  color.”

#### F-4-5 — One workflow sentence contains three separate instructions

- Location: landing page, **Review the result**.
- Exact quote, 20 words: “Open the HTML proof sheet, save a JSON report in CI,
  and use exit codes to stop a broken release.”
- Why this matters: it is under the 22-word cap, but it violates the one-idea
  rule and makes three distinct outputs/actions harder to scan.
- Concrete rewrite: “Open the HTML proof sheet. Save the JSON report in CI.
  Use exit codes to stop a broken release.”

### Minor

#### F-4-6 — “Review the result” is not a standalone section name

- Location: landing workflow step 03 heading.
- Exact quote: “Review the result”.
- Why this matters: the heading does not identify which result a screen-reader
  heading list refers to and could appear unchanged in any product.
- Concrete rewrite: “Review the HTML proof sheet”.

## Copy audit

Counts are whitespace-delimited. No sentence exceeds 22 words and no banned
marketing adjective appears. The semantic and terminology flags are shown in
the final column.

### Landing page sentences and statement-like copy

| Words | Copy | Result |
| ---: | --- | --- |
| 5 | PDF checks for code-heavy manuals | Pass |
| 5 | Catch PDF bugs before release. | Pass |
| 19 | For engineers and technical writers, Code Proof catches broken code, page overflow, and internal links in the final PDF. | Pass |
| 8 | See a sample PDF defect and failed check. | Pass |
| 6 | No files needed to try it | Pass |
| 3 | No tracking data | `private-site` |
| 3 | Read the license | `mit-license` |
| 6 | Check the source against the PDF. | **F-4-3** |
| 9 | Code Proof compares your Markdown with the finished PDF. | Pass |
| 8 | It writes an HTML proof sheet for review. | `html-proof` |
| 10 | Check an existing PDF, or use a custom renderer command. | `existing-pdf`, `renderer-no-shell` |
| 7 | Renderer arguments never pass through a shell. | `renderer-no-shell` |
| 11 | Check links, code colors, and text that runs outside the page. | **F-4-4** |
| 7 | Match each code fence with the PDF. | Registered defect checks |
| 20 | Open the HTML proof sheet, save a JSON report in CI, and use exit codes to stop a broken release. | **F-4-5** |
| 7 | Demo — sample data, nothing is saved | `private-site`; transcript fails **F-1-26** |
| 11 | DEMO HOLD — do not release — 1 expected defect found | **F-1-26** |
| 5 | ERROR [code.flow-changed] source line 7 | **F-1-26** |
| 3 | Sample workspace: /tmp/codeproof-demo-… | `sample-demo` |
| 4 | HTML proof sheet: /tmp/codeproof-demo-…/proof/index.html | `sample-demo` |
| 4 | Find release-breaking PDF defects. | Pass |
| 12 | Flags code fence lines that disappear, merge, or wrap in the PDF. | **F-4-1** |
| 8 | Flags text that runs outside a page edge. | `page-bounds` |
| 10 | Flags Markdown fragments with a missing or wrong PDF destination. | `internal-links` |
| 11 | Warns when a language-tagged code fence has no detectable syntax color. | `syntax-color`; **F-4-4** terminology |
| 6 | Add Code Proof to your build. | Pass |
| 10 | Install from the public repository with Rust 1.88 or newer. | `install-from-git`, `rust-msrv` |
| 7 | Then check a Markdown and PDF pair. | `existing-pdf` |
| 7 | Linux renderer commands cannot use network sockets. | `renderer-network` |
| 11 | Code Proof runs a renderer only after Linux sandbox setup succeeds. | `renderer-fail-closed` |
| 9 | Checking an existing PDF does not start a renderer. | `existing-pdf` |
| 5 | Check Markdown PDFs before release. | Pass |
| 7 | The docs and recorded proof still work. | `offline-reload` |
| 3 | Proof run started. | Pass |
| 7 | Proof run complete: one expected defect found. | Pass |
| 2 | Demo opened. | Pass |
| 8 | Sample data is active and nothing is saved. | `private-site` |
| 3 | Install commands opened. | Pass |
| 2 | Copy unavailable. | Pass |
| 4 | Select this command: [value] | Pass |

### README sentences

| Words | Copy | Result |
| ---: | --- | --- |
| 9 | Code Proof checks code-heavy Markdown manuals before PDF release. | Registered defect checks |
| 14 | It compares your Markdown with a finished PDF and writes an HTML proof sheet. | `existing-pdf`, `html-proof` |
| 9 | Code Proof does not edit the supplied Markdown source. | `input-unchanged` |
| 3 | Live docs: https://markdown-pdf-code-proof.sociobot.in | Pass |
| 10 | Install from the public repository with Rust 1.88 or newer: | `install-from-git`, `rust-msrv` |
| 8 | Run a complete check without your own files: | `sample-demo` |
| 14 | The command creates an isolated temporary workspace and prints its HTML proof sheet path. | `sample-demo` |
| 9 | Its bundled sample contains a wrapped code fence line. | `sample-demo` |
| 10 | It returns exit `1` with `HOLD — do not release`. | **F-1-26** |
| 6 | Keep artifacts in a chosen directory: | `sample-demo` |
| 8 | Check an existing PDF without starting a renderer: | `existing-pdf` |
| 6 | Use a compatible custom renderer command. | `renderer-no-shell` |
| 9 | `{input}` and `{output}` become individual arguments, never shell input: | `renderer-no-shell` |
| 6 | Write a JSON report for CI: | `json-report` |
| 4 | Exit `0` means pass. | `exit-codes` |
| 4 | Exit `1` means defects. | `exit-codes` |
| 8 | Exit `2` means the check could not finish. | `exit-codes` |
| 6 | Warnings do not fail by default. | `syntax-color` |
| 5 | Add `--deny-warnings` to fail warnings. | `syntax-color` |
| 9 | Each Markdown fragment must match one PDF link destination. | `internal-links` |
| 10 | That destination must open a page in the finished PDF. | `internal-links` |
| 11 | Code fence text must remain present and keep its line breaks. | `code-content` |
| 10 | One source line fails if it wraps in the PDF. | `single-line-wrap` |
| 10 | Text is checked against every visible PDF page edge. | `page-bounds` |
| 12 | A language-tagged code fence warns when no non-default PDF color is found. | **F-4-4** |
| 7 | Run `codeproof check --help` for command options. | Instruction |
| 10 | Code Proof applies its Linux sandbox before a renderer starts. | `renderer-fail-closed` |
| 6 | Renderer commands cannot create network sockets. | `renderer-network` |
| 7 | Existing-PDF checks do not start a renderer. | `existing-pdf` |
| 8 | A renderer has a deadline set by `--timeout`. | `renderer-timeout` |
| 5 | Verify the project: `npm test`. | Instruction |
| 8 | Build the CLI and site: `npm run build`. | Instruction |
| 7 | Create the publishable Rust package without publishing: | Instruction |
| 7 | The site uses Vite and vanilla TypeScript. | Repository detail |
| 7 | Run it locally with `npm run dev`. | Instruction |
| 8 | Build the static deployment with `npm run build:site`. | Instruction |
| 2 | Version 0.1.0. | Metadata |
| 2 | See CHANGELOG.md. | Instruction |
| 4 | See the MIT License. | `mit-license` |

### Headings, actions, and terminology

- **F-4-6** flags “Review the result.” All other headings identify their
  section when read in the page outline. “How it works” is the required
  standard-skeleton section label.
- Every action names its result: Try it with sample data, Copy install/check/CI
  command, Reset demo, View install commands, Read the full CLI reference, and
  Return home. No generic Submit, Go, or Continue control appears.
- **F-4-3** and **F-4-4** identify the remaining term drift. Markdown, PDF,
  code fence, renderer, HTML proof sheet, and JSON report are otherwise
  consistent.
- No metaphor, mood heading, generic slogan, or banned marketing adjective was
  found.

## Demo and sandbox

- One click from the cold first screen opened `/?demo=1#demo`, set the title to
  “Demo — Code Proof,” focused “Sample failed release check,” and displayed
  the banner “Demo — sample data, nothing is saved.”
- The first viewport after the click showed `codeproof demo`, a
  `code.flow-changed` defect, the sample workspace, the proof path, and expected
  exit 1. It is realistic sample data, but its transcript is inaccurate as
  recorded in blocking F-1-26.
- Reset replayed the result and did not add storage. Its keyboard focus defect
  is F-4-2. “View install commands” removed demo mode, hid the banner, focused
  the Install heading, and Back/Forward restored the corresponding state.
- A fresh context with pre-seeded `real:sentinel` localStorage,
  `real:session` sessionStorage, and `real_sentinel` cookie retained every
  value unchanged through entry, Reset, and exit. Demo mode added no value.
- The full browser request log contained only
  `https://markdown-pdf-code-proof.sociobot.in` requests. There were no console
  errors.
- The release binary ran `codeproof demo` from a fresh
  `/tmp/codeproof-review4-cli.*` directory. It exited 1 and created only a new
  `/tmp/codeproof-demo-*` workspace containing the bundled Markdown, generated
  PDF, and `proof/index.html`. It did not invoke a renderer or read a supplied
  user file.

## Registered claim results

Every one of the 21 entries in `.factory/claims.json` was run separately from
the clean clone `/tmp/codeproof-review4.kUwOVw/repo`. Duplicate commands were
rerun for each entry rather than inferred from another claim.

| Claim | Exact command | Result |
| --- | --- | --- |
| `single-line-wrap` | `cargo test --test cli wrapped_single_code_line_fails_the_release_contract -- --exact` | PASS |
| `page-bounds` | `cargo test --test cli page_bounds_cover_every_media_and_crop_edge -- --exact` | PASS |
| `code-content` | `cargo test --test cli missing_code_content_fails_the_release_contract -- --exact` | PASS |
| `internal-links` | `cargo test --test cli wrong_pdf_destination_cannot_satisfy_a_fragment -- --exact` | PASS |
| `syntax-color` | `cargo test --test cli missing_syntax_color_warns_and_respects_warning_policy -- --exact` | PASS |
| `existing-pdf` | `cargo test --test cli documented_existing_pdf_flow_passes_and_writes_proof -- --exact` | PASS |
| `local-cli-files` | `cargo test --test cli documented_existing_pdf_flow_passes_and_writes_proof -- --exact` | PASS |
| `input-unchanged` | `cargo test --test cli input_files_remain_unchanged_in_existing_pdf_and_custom_renderer_checks -- --exact` | PASS |
| `renderer-no-shell` | `cargo test --test cli custom_renderer_runs_without_a_shell_and_is_checked -- --exact` | PASS |
| `renderer-network` | `cargo test --test cli renderer_sandbox_denies_network_connections -- --exact` | PASS |
| `renderer-fail-closed` | `cargo test --test cli renderer_refuses_to_start_when_sandbox_setup_fails -- --exact` | PASS |
| `renderer-timeout` | `cargo test --test cli renderer_deadline_stops_a_long_running_command -- --exact` | PASS |
| `html-proof` | `cargo test --test cli documented_existing_pdf_flow_passes_and_writes_proof -- --exact` | PASS |
| `json-report` | `cargo test --test cli json_report_and_exit_codes_are_observable -- --exact` | PASS |
| `exit-codes` | `cargo test --test cli json_report_and_exit_codes_are_observable -- --exact` | PASS |
| `sample-demo` | `cargo test --test cli demo_uses_bundled_sample_data_and_writes_an_isolated_proof -- --exact` | PASS |
| `private-site` | `npm run test:site -- --grep @claim:private-site` | PASS |
| `offline-reload` | `npm run test:site -- --grep @claim:offline-reload` | PASS |
| `rust-msrv` | `npm run test:msrv` | PASS |
| `install-from-git` | `npm run test:install` | PASS |
| `mit-license` | `npm run test:license` | PASS |

The registered suite has no failing or untested entry. F-4-1 is outside that
suite because the landing claim has no entry. F-1-26 is outside it because the
CLI and browser tests separately assert contradictory transcripts.

## Earlier finding verification

I read all three earlier reviews, all three polish reports, and the handoff.
The matrix below records a live-and-code check for every unique earlier
finding. “Fixed” means both deployed behavior and repository evidence agree.

| Earlier finding | Current result |
| --- | --- |
| F-1-1 demo transition contrast | Fixed: unrevealed lines use visibility; Axe at start and completion passes. |
| F-1-2 demo lifecycle/title/focus | Fixed for routing: entry, exit, Back, Forward, title, banner, and destination focus pass. Reset focus has the new F-4-2 defect. |
| F-1-3 unusable install paths | Fixed: the sole Git command installed the binary from an empty directory. |
| F-1-4 overbroad local-file wording | Fixed: broad device/upload wording is absent; supplied-path behavior is registered. |
| F-1-5 unlisted service/telemetry claims | Fixed: CLI telemetry wording is absent; site privacy is registered and request-logged. |
| F-1-6 unlisted no-shell claim | Fixed: `renderer-no-shell` passed. |
| F-1-7 unlisted filesystem limits | Fixed: the filesystem-limit promise remains removed. |
| F-1-8 unlisted fail-closed behavior | Fixed: `renderer-fail-closed` passed. |
| F-1-9 unlisted Pandoc sanitizing | Fixed: the claim remains absent. |
| F-1-10 timeout/script claims | Fixed: timeout is registered and passes; script-execution copy is absent. |
| F-1-11 Rust 1.88 proof | Fixed: locked workspace compiled with Rust 1.88. |
| F-1-12 existing-PDF availability | Fixed: registered flow passed without a renderer. |
| F-1-13 unlisted renderer modes | Fixed: only the tested existing-PDF and custom-command paths remain. |
| F-1-14 PDF-internals inventory | Fixed: the landing describes user-visible outcomes. |
| F-1-15 HTML/JSON/exit-code claims | Fixed: all three registered contracts passed. |
| F-1-16 subjective diagnostic matrix | Fixed: the promise remains absent. |
| F-1-17 MIT claim | Fixed: package, site, and license test agree. |
| F-1-18 long exit-code sentence | Fixed: three four-to-eight-word sentences. |
| F-1-19 long link sentence | Fixed: two sentences of 9 and 10 words. |
| F-1-20 long code-flow sentence | Fixed: two sentences of 11 and 10 words. |
| F-1-21 unexplained HOLD action copy | Fixed at the primary action; the transcript mismatch is F-1-26. |
| F-1-22 metaphor/internal terms | Fixed: the earlier source-contract and tactile-proof wording is absent. |
| F-1-23 PDF implementation inventory | Fixed: the workflow names observable checks. |
| F-1-24 vague locked-down wording | Fixed: the page names the Linux socket restriction. |
| F-1-25 engine/renderer drift | Fixed: public prose uses renderer. |
| F-1-26 output/code-region terms | **Regressed / BLOCKING:** the browser, README, binary, and HTML proof disagree on the failed-decision phrase and terminal output. |
| F-1-27 unclear workflow/sample headings | Fixed for the original headings; the separate generic heading is F-4-6. |
| F-1-28 generic controls | Fixed: visible controls name their results. |
| F-1-29 social/install metadata | Fixed: 1200×630 social image, 180×180 touch icon, route metadata, and demo sitemap entry are present live. |
| F-1-30 footer/external-link inconsistency | Fixed: all routes share Home, Privacy, Terms, and labelled GitHub links. |
| F-1-31 third mobile fact below fold | Fixed: all three facts are above 844 px at 390 px width. |
| F-2-1 checkout-install claim | Fixed: the redundant path-install instruction is absent. |
| F-2-2 test-coverage claim | Fixed: README gives a direct command without promising its coverage. |
| F-2-3 build-output claim | Fixed: README gives a direct build command without promised paths. |
| F-2-4 unexplained demo heading | Fixed: focused heading is “Sample failed release check.” |
| F-2-5 vague “Code flow” heading | Fixed: heading is “Missing or wrapped code.” |
| F-2-6 PDF implementation terms | Fixed for page bounds; the separate color term drift is F-4-4. |
| F-2-7 unexplained Linux controls | Fixed: public copy says “Linux sandbox.” |
| F-3-1 unregistered non-editor promise | Fixed: `input-unchanged` passed both required flows. |
| F-3-2 incomplete/inaccurate copy audit | Fixed: the generated landing/README audit is current and its regression passed. |

## Structure, accessibility, links, and identity

- `/`, `/?demo=1`, `/privacy/`, and `/terms/` returned 200. An unknown path
  returned the designed Code Proof 404 with HTTP 404 and a route home.
- Titles are “Code Proof — inspect Markdown PDFs before release,” “Demo — Code
  Proof,” “Privacy — Code Proof,” “Terms — Code Proof,” and “Page not found —
  Code Proof.” Each document has `lang=en`, one H1, one main landmark, a plain
  description, canonical, Open Graph/Twitter metadata, favicon, and 180×180
  touch icon.
- The social image is 1200×630. Live `robots.txt` and `sitemap.xml` match the
  source and include every canonical public route, including the query demo.
- Every internal fragment exists. Every crawled product route, asset, GitHub
  repository link, and README anchor returned 200. Header/footer structure is
  consistent across root, legal pages, and 404.
- The deployed response sends CSP `frame-ancestors` as a header, nosniff,
  referrer policy, and permissions policy. Normal routes produced no console
  errors.
- The live Playwright suite passed 13/13, including Axe at demo transition,
  legal pages and 404, 390 px targets/overflow, reduced motion, skip link,
  offline reload, and route focus. The fleet URL verifier also passed with no
  console errors, one H1, `lang=en`, main, alt text, and labelled controls.
- The release-room risograph palette, local typography, hard ink offsets,
  crop marks, continuous galley, and original press artwork are distinct and
  match `.factory/design.md`. This is not a generic SaaS template.
- Production output is small: initial JavaScript is 3.00 kB raw / 1.29 kB
  gzip. The live HTML, JavaScript, and CSS SHA-256 hashes match the clean build.

## Quality gates

From the clean clone:

- `npm test`: PASS — copy audit, Rust 1.88, 3 unit tests, 22 CLI integration
  tests, 13 browser tests, and license check.
- `npm run typecheck`: PASS.
- `npm run lint`: PASS.
- `npm run build`: PASS; release CLI and `dist/site/` produced.
- `cargo package --manifest-path cli/Cargo.toml --locked`: PASS.
- Live `PLAYWRIGHT_BASE_URL=… npx playwright test`: PASS, 13/13.
- `/opt/fleet/lib/verify-url.sh <url> <temp-evidence-dir>`: PASS.

## Missed leverage

No AI feature is warranted. This is a deterministic, reproducible release
check; model output would not improve its core decision. The brief's useful
import/export surface already exists through Markdown/PDF inputs, HTML proof
sheets, and JSON reports. No account or sync workflow is implied for this local
CLI.

## What would make this perfect

Resolve all seven findings. Most importantly, make the one-click recording a
literal, tested transcript of the real CLI and close F-1-26 again. Register the
merge check, retain focus after Reset, use Markdown and syntax color
consistently, split the three-action workflow sentence, and name the proof
sheet in step 03. Then rerun every claim entry, the real demo, the live route
suite, and this full checklist. A passing round requires zero findings.
