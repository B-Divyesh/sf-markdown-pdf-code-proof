# Adversarial first-read review 5 — FAIL

- Product: Code Proof (`markdown-pdf-code-proof`)
- Candidate: `88ed74524be9b461067777d3b308736d7f95ebfd`
- Live URL: <https://markdown-pdf-code-proof.sociobot.in/>
- Reviewed: 2 September 2026 UTC
- Verdict: **FAIL**

There are 14 findings: nine high, four medium, and one minor. There are no
blocking findings. The landing page, demo, routes, accessibility checks, and
all 25 registered claim commands work. The product still fails because the
README publishes behavior that is absent from, or broader than, the claim
manifest. PASS requires zero findings and no untested claim.

## Cold first read

Fresh Chromium contexts opened the live root at 390×844 and 1440×900 before
scrolling.

- What does this do? It catches broken code, page overflow, and internal-link
  defects in a finished Markdown PDF before release.
- For whom? Engineers and technical writers producing code-heavy manuals.
- What should I click first? **Try it with sample data**.

The exact first-screen copy that supplied those answers was:

> Catch PDF bugs before release.
>
> For engineers and technical writers, Code Proof catches broken code, page
> overflow, and internal links in the final PDF.
>
> TRY IT WITH SAMPLE DATA
>
> See a sample PDF defect and failed check.

This comprehension check passes on both viewports. At 390 px, the headline,
audience sentence, primary action, outcome note, install alternative, and all
three current fact tiles appear before the first scroll. There is no horizontal
page overflow.

## Findings

### High — unlisted or overstated claims

#### F-5-1 — The “base fonts” claim is unlisted and broader than the implementation

- Exact quote/location: README, **Checks**: “Standard PDF font metrics cover
  base fonts.”
- Evidence: `font-metrics` promises and tests embedded widths and text
  transforms. Its exact test does not exercise a standard base font. The
  separate unregistered `helvetica_metrics_detect_wide_glyph_overflow_without_narrow_false_positive`
  test covers Helvetica only. In `cli/src/pdf.rs`, standard glyph widths exist
  for Helvetica and Courier, but not Times, Symbol, or ZapfDingbats.
- Why this misleads: “base fonts” reads as coverage of the standard PDF base
  fonts, while the implementation uses a one-em fallback for several of them.
- Concrete fix: remove the sentence, or narrow it to the exact supported fonts
  and add a registered claim test for every named family. Plain rewrite:
  “Built-in width tables cover Helvetica and Courier.” Use that only after a
  claim test exercises both.

#### F-5-2 — ATX heading support is not in the heading claim

- Exact quote/location: README, **Heading fragments**: “Code Proof parses
  CommonMark ATX (`# Heading`) and Setext headings.”
- Evidence: `heading-fragments` claims and tests Setext headings and Pandoc
  explicit IDs. Its integration test does not include a plain ATX heading.
  A library unit test is not the exact registered sandbox command.
- Why this matters: claim-only verification can pass if ATX behavior regresses.
- Concrete fix: add ATX input to the registered integration test and include
  ATX in the claim text. Plain rewrite: “Code Proof recognizes `# Heading` and
  underlined Markdown headings.”

#### F-5-3 — Case-insensitive fragment matching is unlisted

- Exact quote/location: README, **Heading fragments**: “Fragment matching is
  case-insensitive.”
- Evidence: no claim entry says this, and the `heading-fragments` fixture uses
  the same lowercase spelling in Markdown, the annotation, and the named PDF
  destination.
- Why this matters: users may rely on links whose letter case differs, but the
  claim gate never proves that path.
- Concrete fix: add a `case-insensitive-fragments` claim and an integration
  fixture with mixed-case source, annotation, and destination values, or remove
  the sentence. Plain rewrite: “Link target matching ignores letter case.”

#### F-5-4 — Pandoc automatic-ID compatibility is unlisted

- Exact quote/location: README, **Heading fragments**: “Without an explicit
  ID, Code Proof follows Pandoc's automatic identifier rules.”
- Evidence: `heading-fragments` tests one plain Setext heading and one explicit
  ID. It does not compare generated identifiers with Pandoc across the rule set.
- Why this matters: “follows Pandoc's rules” is a compatibility promise, not a
  description of one example.
- Concrete fix: register a `pandoc-auto-identifiers` claim and run a table of
  Pandoc-produced fixtures. If exact compatibility is not intended, replace the
  sentence with the narrower rules Code Proof actually implements.

#### F-5-5 — Punctuation removal is an unlisted identifier claim

- Exact quote/location: README, **Heading fragments**: “Formatting and most
  punctuation are removed.”
- Evidence: no registered test exercises formatted heading text or a stated
  punctuation set.
- Why this matters: “most” is also imprecise; a user cannot know which link
  target to write.
- Concrete fix: name the retained characters, register the behavior, and test
  emphasis, inline code, periods, underscores, and punctuation. Example:
  “Formatting is ignored. Letters, numbers, underscores, hyphens, and periods
  remain.”

#### F-5-6 — Space-to-hyphen conversion is unlisted

- Exact quote/location: README, **Heading fragments**: “Spaces become
  hyphens.”
- Evidence: no claim entry states this transformation, and the registered
  heading integration does not isolate it against a precomputed expected ID.
- Why this matters: this rule determines whether internal links pass.
- Concrete fix: add it to the `heading-fragments` claim and assert an exact
  multiword generated target.

#### F-5-7 — Lowercasing is unlisted

- Exact quote/location: README, **Heading fragments**: “Letters become
  lowercase.”
- Evidence: no registered claim or exact mixed-case fixture covers this rule.
- Why this matters: this statement is part of the public link-resolution
  contract.
- Concrete fix: add a mixed-case generated-heading fixture to a registered
  claim, or remove the sentence.

#### F-5-8 — Leading-character removal is unlisted

- Exact quote/location: README, **Heading fragments**: “Leading non-letters
  are removed.”
- Evidence: no registered claim test starts a heading with digits or
  punctuation and asserts the generated target.
- Why this matters: a visitor can construct a documented heading form that
  the claim gate never checks.
- Concrete fix: register and test examples such as `## 2. Retry` and
  `## — Retry`, including the exact link target produced.

#### F-5-9 — The syntax-color exclusions exceed their test

- Exact quote/location: README, **Checks**: “Colored headings, links, logos,
  and graphics do not count.”
- Evidence: `syntax-color` uses a fixture with a blue rectangle and blue
  heading before black code. It does not contain a colored link or logo. The
  implementation matches text tokens and does not classify semantic “logo” or
  “link” objects.
- Why this matters: the sentence promises four independently observable
  exclusions while the registered sandbox covers two.
- Concrete fix: either add colored-link and colored-logo cases to the exact
  registered test, or narrow the copy to “Unrelated colored headings and
  graphics do not count.”

### Medium — first-screen and documentation clarity

#### F-5-10 — The three first-screen facts omit offline use and price

- Exact location: first-screen tiles: “SAMPLE INCLUDED / No files needed to
  try it”, “SITE PRIVACY / No tracking data”, and “MIT LICENSE / Read the
  license”.
- Why this fails the required shape: the mandated facts are privacy, offline
  use, and price. “Sample included” repeats the primary action, and “Read the
  license” is an instruction without a link or a price statement.
- Concrete fix: use three facts such as “Works offline / after the first
  visit”, “Site privacy / no tracking”, and “Free software / MIT licensed”.
  Keep “Sample included” beside the demo action.

#### F-5-11 — The README does not state who the product is for

- Exact location: README opening: “Code Proof checks code-heavy Markdown
  manuals before PDF release.”
- Why this matters: the repository contract requires the README to say what
  the product is and who it is for. The landing names engineers and technical
  writers; the README does not.
- Concrete rewrite: “Code Proof helps engineers and technical writers check
  code-heavy Markdown manuals before PDF release.”

#### F-5-12 — The README does not explain how to deploy the site

- Exact location: README, **Develop and verify**: “Build the static deployment
  with `npm run build:site`.”
- Why this matters: this creates the site but does not identify the deployment
  directory or the required static-host fallback/header configuration. The
  repository contract requires run, test, and deploy instructions.
- Concrete fix: state that `dist/site/` is the deploy root and that
  `staticwebapp.config.json` must be served with it. Keep provider operations
  outside this repository.

#### F-5-13 — “CI” is not expanded in user-facing copy

- Exact quotes: landing, “Save the JSON report in CI”; README, “Write a JSON
  report for CI”.
- Why this slows a first read: “CI” is an unexplained abbreviation. Technical
  readers may know it, but the copy does not need the shortcut.
- Concrete rewrite: first use “Save the JSON report in continuous integration
  (CI).” The README can then use “CI” later.

### Minor

#### F-5-14 — Stable-name images remain immutable for one year

- Location: live responses and `site/public/staticwebapp.config.json`.
- Exact affected files: `code-proof-press.webp`, `code-proof-social.jpg`, and
  `apple-touch-icon.png` return `Cache-Control: public, max-age=31536000,
  immutable` despite stable URLs.
- Why this matters: a returning visitor can retain old artwork or social/icon
  assets for a year after those files change. This was already disclosed as
  `CP-V10-01` in the incoming handoff and remains present in live and source.
- Concrete fix: content-hash these filenames and update their references, or
  give stable-name images a short revalidating cache policy.

## Copy audit

Counts are whitespace-delimited. The inventory covers every sentence and
statement-like fragment in the landing document, its runtime feedback, the
recorded terminal output, and the README. No item exceeds 22 words. No banned
marketing adjective appears. Findings F-5-1 through F-5-9 and F-5-13 are the
claim/jargon flags; headings and controls are checked after the tables.

### Landing page

| Words | Copy |
| ---: | --- |
| 3 | Skip to content |
| 1 | CP |
| 2 | Code Proof |
| 4 | release inspector / 0.1 |
| 1 | Demo |
| 1 | Checks |
| 1 | Install |
| 5 | PDF checks for code-heavy manuals |
| 5 | Catch PDF bugs before release. |
| 19 | For engineers and technical writers, Code Proof catches broken code, page overflow, and internal links in the final PDF. |
| 5 | Try it with sample data |
| 8 | See a sample PDF defect and failed check. |
| 3 | Copy install command |
| 2 | Sample included |
| 6 | No files needed to try it |
| 2 | Site privacy |
| 3 | No tracking data |
| 2 | MIT license |
| 3 | Read the license |
| 2 | Original artwork |
| 4 | Code and page inspection |
| 2 | Markdown source |
| 2 | Your renderer |
| 2 | Code Proof |
| 2 | Release PDF |
| 3 | How it works |
| 6 | Check Markdown against the finished PDF. |
| 9 | Code Proof compares your Markdown with the finished PDF. |
| 8 | It writes an HTML proof sheet for review. |
| 5 | Create or choose the PDF |
| 10 | Check an existing PDF, or use a custom renderer command. |
| 7 | Renderer arguments never pass through a shell. |
| 4 | Check the finished PDF |
| 11 | Check links, syntax color, and text that runs outside the page. |
| 7 | Match each code fence with the PDF. |
| 5 | Review the HTML proof sheet |
| 5 | Open the HTML proof sheet. |
| 6 | Save the JSON report in CI. |
| 8 | Use exit codes to stop a broken release. |
| 7 | Demo — sample data, nothing is saved |
| 2 | Reset demo |
| 3 | View install commands |
| 2 | Sample check |
| 4 | Sample failed release check |
| 2 | codeproof demo |
| 2 | local sample |
| 11 | DEMO HOLD — do not release — 1 expected defect found |
| 14 | Error [code.flow-changed] Code fence on line 7 is present but its line flow changed |
| 3 | Sample workspace: /tmp/codeproof-demo-… |
| 4 | HTML proof sheet: /tmp/codeproof-demo-…/proof/index.html |
| 3 | Expected exit 1 |
| 3 | Bundled sample only |
| 4 | Checks in version 0.1 |
| 4 | Find release-breaking PDF defects. |
| 4 | Missing or wrapped code |
| 12 | Flags code fence lines that disappear, merge, or wrap in the PDF. |
| 2 | Page bounds |
| 8 | Flags text that runs outside a page edge. |
| 2 | Internal links |
| 10 | Flags Markdown fragments with a missing or wrong PDF destination. |
| 2 | Syntax color |
| 11 | Warns when a language-tagged code fence has no detectable syntax color. |
| 1 | Install |
| 6 | Add Code Proof to your build. |
| 10 | Install from the public repository with Rust 1.88 or newer. |
| 7 | Then check a Markdown and PDF pair. |
| 5 | Read the full CLI reference |
| 3 | Copy install command |
| 3 | Copy check command |
| 3 | Copy CI command |
| 2 | Renderer safety |
| 7 | Linux renderer commands cannot use network sockets. |
| 11 | Code Proof runs a renderer only after Linux sandbox setup succeeds. |
| 9 | Checking an existing PDF does not start a renderer. |
| 5 | Check Markdown PDFs before release. |
| 6 | Built by Param Factory · v0.1.0 |
| 1 | Home |
| 1 | Privacy |
| 1 | Terms |
| 2 | GitHub ↗ |

Runtime feedback and metadata:

| Words | Copy |
| ---: | --- |
| 8 | Code Proof — inspect Markdown PDFs before release |
| 15 | Check a finished Markdown PDF for broken code, page overflow, and internal links before release. |
| 4 | Demo — Code Proof |
| 12 | Run the bundled Code Proof sample and review its expected PDF defect. |
| 3 | Copied to clipboard |
| 2 | Copied: [value] |
| 3 | Copy install command |
| 2 | Copy command |
| 2 | Copy unavailable. |
| 4 | Select this command: [value] |
| 3 | Proof run started. |
| 7 | Proof run complete: one expected defect found. |
| 2 | Demo opened. |
| 8 | Sample data is active and nothing is saved. |
| 3 | Install commands opened. |
| 1 | Offline. |
| 7 | The docs and recorded proof still work. |

### README

| Words | Copy |
| ---: | --- |
| 2 | Code Proof |
| 9 | Code Proof checks code-heavy Markdown manuals before PDF release. |
| 14 | It compares your Markdown with a finished PDF and writes an HTML proof sheet. |
| 9 | Code Proof does not edit the supplied Markdown source. |
| 3 | Live docs: https://markdown-pdf-code-proof.sociobot.in |
| 1 | Install |
| 10 | Install from the public repository with Rust 1.88 or newer: |
| 4 | Try the bundled sample |
| 8 | Run a complete check without your own files: |
| 14 | The command creates an isolated temporary workspace and prints its HTML proof sheet path. |
| 9 | Its bundled sample contains a wrapped code fence line. |
| 17 | It returns exit `1` and prints `DEMO HOLD — do not release — 1 expected defect found`. |
| 6 | Keep artifacts in a chosen directory: |
| 1 | Usage |
| 8 | Check an existing PDF without starting a renderer: |
| 6 | Use a compatible custom renderer command. |
| 9 | `{input}` and `{output}` become individual arguments, never shell input: |
| 6 | Write a JSON report for CI: |
| 4 | Exit `0` means pass. |
| 4 | Exit `1` means defects. |
| 8 | Exit `2` means the check could not finish. |
| 6 | Warnings do not fail by default. |
| 5 | Add `--deny-warnings` to fail warnings. |
| 1 | Checks |
| 9 | Each Markdown fragment must match one PDF link destination. |
| 10 | That destination must open a page in the finished PDF. |
| 11 | Code fence text must remain present and keep its line breaks. |
| 10 | One source line fails if it wraps in the PDF. |
| 17 | Text is checked against every visible PDF page edge using the PDF font's widths and text transforms. |
| 7 | Standard PDF font metrics cover base fonts. |
| 14 | Each language-tagged code fence warns when its matching PDF text has no syntax color. |
| 9 | Colored headings, links, logos, and graphics do not count. |
| 2 | Heading fragments |
| 10 | Code Proof parses CommonMark ATX (`# Heading`) and Setext headings. |
| 13 | Pandoc explicit IDs such as `## Retry behavior {#retry-policy}` define the fragment directly. |
| 4 | Fragment matching is case-insensitive. |
| 11 | Without an explicit ID, Code Proof follows Pandoc's automatic identifier rules. |
| 6 | Formatting and most punctuation are removed. |
| 3 | Spaces become hyphens. |
| 3 | Letters become lowercase. |
| 4 | Leading non-letters are removed. |
| 15 | Use explicit IDs for repeated headings or when a custom renderer uses different fragment rules. |
| 13 | The final check still requires the same PDF link annotation and named destination. |
| 7 | Run `codeproof check --help` for command options. |
| 2 | Renderer safety |
| 10 | Code Proof applies its Linux sandbox before a renderer starts. |
| 6 | Renderer commands cannot create network sockets. |
| 7 | Existing-PDF checks do not start a renderer. |
| 8 | A renderer has a deadline set by `--timeout`. |
| 3 | Develop and verify |
| 5 | Verify the project: `npm test`. |
| 8 | Build the CLI and site: `npm run build`. |
| 7 | Create the publishable Rust package without publishing: |
| 7 | The site uses Vite and vanilla TypeScript. |
| 7 | Run it locally with `npm run dev`. |
| 8 | Build the static deployment with `npm run build:site`. |
| 2 | Project status |
| 2 | Version 0.1.0. |
| 2 | See CHANGELOG.md. |
| 4 | See the MIT License. |

### Headings, terminology, and actions

- All landing headings identify their section when read alone. “Heading
  fragments” is understandable to the target technical audience, but its
  paragraph uses the unexplained abbreviations “ATX” and “CI”; F-5-2 and
  F-5-13 provide plain rewrites.
- Product terms are otherwise consistent: Markdown, PDF, code fence, renderer,
  HTML proof sheet, JSON report, syntax color, and HOLD — do not release.
- Buttons and action links name a result: Try, Copy, Reset, View, Read, and
  Return. No Submit, Go, or Continue control appears.
- No metaphor, mood heading, generic slogan, or banned marketing adjective was
  found.

## Demo and sandbox

The demo itself passes.

- One click from the cold first screen opens `/?demo=1#demo`, changes the title
  to “Demo — Code Proof,” focuses “Sample failed release check,” and shows the
  persistent “Demo — sample data, nothing is saved” banner.
- The first screen after the click already shows `codeproof demo` and the real
  `code.flow-changed` diagnostic. The complete transcript includes expected
  exit 1 and the sample/proof paths. It exactly matches the CLI after only its
  temporary path is normalized.
- Reset works with Space, retains focus, and announces start and completion.
  “View install commands” is a clearer result-naming equivalent to “Start for
  real”: it exits demo mode, hides the banner, and focuses the Install heading.
  Back and Forward restore the matching title, banner, URL, and focus.
- A fresh context seeded with real-namespaced localStorage, sessionStorage, and
  a cookie retained those values unchanged. Demo use added no storage. Its
  request log contained only the product origin.
- The real CLI command ran from a temporary directory, exited 1 by design, and
  created only `sample-manual.md`, `sample-manual.pdf`, and
  `proof/index.html` under a new `/tmp/codeproof-demo-*` workspace. The proof
  contains `HOLD — do not release` and `code.flow-changed`.

## Registered claim results

Every entry was run as its exact `test` command from a fresh local clone.
Duplicate commands were rerun for each claim.

| Claim | Result | Exact command |
| --- | --- | --- |
| `single-line-wrap` | PASS | `cargo test --test cli wrapped_single_code_line_fails_the_release_contract -- --exact` |
| `code-lines-merge` | PASS | `cargo test --test cli flattened_code_lines_fail_the_release_contract -- --exact` |
| `page-bounds` | PASS | `cargo test --test cli page_bounds_cover_every_media_and_crop_edge -- --exact` |
| `code-content` | PASS | `cargo test --test cli missing_code_content_fails_the_release_contract -- --exact` |
| `internal-links` | PASS | `cargo test --test cli wrong_pdf_destination_cannot_satisfy_a_fragment -- --exact` |
| `syntax-color` | PASS | `cargo test --test cli unrelated_blue_graphic_does_not_mask_black_code -- --exact` |
| `font-metrics` | PASS | `cargo test --test cli embedded_widths_and_text_matrices_drive_page_geometry -- --exact` |
| `heading-fragments` | PASS | `cargo test --test cli setext_and_pandoc_explicit_heading_ids_resolve_pdf_fragments -- --exact` |
| `existing-pdf` | PASS | `cargo test --test cli documented_existing_pdf_flow_passes_and_writes_proof -- --exact` |
| `local-cli-files` | PASS | `cargo test --test cli documented_existing_pdf_flow_passes_and_writes_proof -- --exact` |
| `input-unchanged` | PASS | `cargo test --test cli input_files_remain_unchanged_in_existing_pdf_and_custom_renderer_checks -- --exact` |
| `renderer-no-shell` | PASS | `cargo test --test cli custom_renderer_runs_without_a_shell_and_is_checked -- --exact` |
| `renderer-network` | PASS | `cargo test --test cli renderer_sandbox_denies_network_connections -- --exact` |
| `renderer-fail-closed` | PASS | `cargo test --test cli renderer_refuses_to_start_when_sandbox_setup_fails -- --exact` |
| `renderer-timeout` | PASS | `cargo test --test cli renderer_deadline_stops_a_long_running_command -- --exact` |
| `html-proof` | PASS | `cargo test --test cli documented_existing_pdf_flow_passes_and_writes_proof -- --exact` |
| `json-report` | PASS | `cargo test --test cli json_report_and_exit_codes_are_observable -- --exact` |
| `exit-codes` | PASS | `cargo test --test cli json_report_and_exit_codes_are_observable -- --exact` |
| `sample-demo` | PASS | `cargo test --test cli demo_uses_bundled_sample_data_and_writes_an_isolated_proof -- --exact` |
| `demo-transcript` | PASS | `npm run test:site -- --grep @claim:demo-transcript` |
| `private-site` | PASS | `npm run test:site -- --grep @claim:private-site` |
| `offline-reload` | PASS | `npm run test:site -- --grep @claim:offline-reload` |
| `rust-msrv` | PASS | `npm run test:msrv` |
| `install-from-git` | PASS | `npm run test:install` |
| `mit-license` | PASS | `npm run test:license` |

The full live Playwright run initially completed 18 of 19 tests; the transcript
case reached its 30-second total timeout after a cold local CLI compile. Its
observable state remained “Proof run started.” The exact declared command was
then run first in a second fresh clone and passed, and the isolated live rerun
passed in 3.8 seconds. This is recorded as environment-sensitive test setup,
not a failed claim, because the declared clean-clone command and its live
observable result both pass.

## Earlier finding verification

All four earlier reviews, all four polish reports, and the incoming handoff
were read. Each earlier review finding was rechecked against the live site and
current source.

| Finding | Current result |
| --- | --- |
| F-1-1 | Fixed: demo lines use `visibility`; live transition/completion Axe checks have no serious or critical issue. |
| F-1-2 | Fixed: demo entry/exit, titles, banner, focus, announcements, Back, and Forward work. |
| F-1-3 | Fixed: the public Git install claim passes from an empty installation root. |
| F-1-4 | Fixed: broad device/upload wording is absent; local-path behavior is registered and passes. |
| F-1-5 | Fixed: unproved CLI telemetry/account wording is absent; the site privacy request/storage test passes. |
| F-1-6 | Fixed: the no-shell claim is registered and passes. |
| F-1-7 | Fixed: the untested renderer filesystem-limit promise remains absent. |
| F-1-8 | Fixed: forced Linux sandbox setup failure prevents renderer start. |
| F-1-9 | Fixed: untested Pandoc sanitizing and fixed-argument wording remains absent. |
| F-1-10 | Fixed: timeout is registered and the script-execution promise remains absent. |
| F-1-11 | Fixed: the locked workspace compiles with Rust 1.88. |
| F-1-12 | Fixed: existing-PDF mode is registered and does not start a renderer. |
| F-1-13 | Fixed: public copy documents only exercised existing-PDF and custom-renderer modes. |
| F-1-14 | Fixed: landing copy names observable defects rather than a PDF-internals inventory. |
| F-1-15 | Fixed: HTML proof, JSON, and exit-code contracts are separately registered and pass. |
| F-1-16 | Fixed: the subjective diagnostic-matrix promise remains absent. |
| F-1-17 | Fixed: repository, crate, landing, and Terms agree on MIT. |
| F-1-18 | Fixed: exit meanings remain three short sentences. |
| F-1-19 | Fixed: internal-link requirements remain two short sentences. |
| F-1-20 | Fixed: code-wrap behavior remains two short sentences. |
| F-1-21 | Fixed: the primary action explains the sample; HOLD is defined in the result. |
| F-1-22 | Fixed: the old source-contract/fixed-artifact metaphors remain absent. |
| F-1-23 | Fixed: workflow copy names visible checks. |
| F-1-24 | Fixed: safety copy names the tested network-socket restriction. |
| F-1-25 | Fixed: public prose consistently calls the PDF-producing component a renderer. |
| F-1-26 | Fixed: CLI, browser transcript, README, and proof use the same code-fence/output/HOLD terms. |
| F-1-27 | Fixed: workflow and sample headings identify their subject. |
| F-1-28 | Fixed: visible actions name their result; demo exit opens install commands. |
| F-1-29 | Fixed: route metadata, 1200×630 social image, and 180×180 touch icon are present. |
| F-1-30 | Fixed: all routes share Home, Privacy, Terms, and labelled external GitHub links. |
| F-1-31 | Fixed: all three current facts appear above 844 px at 390 px. F-5-10 concerns their required content, not placement. |
| F-2-1 | Fixed: the unregistered checkout-install instruction remains absent. |
| F-2-2 | Fixed: README gives a direct `npm test` instruction without a coverage promise. |
| F-2-3 | Fixed: README gives a direct build instruction without claiming output paths. |
| F-2-4 | Fixed: the focused demo heading is “Sample failed release check.” |
| F-2-5 | Fixed: the check heading is “Missing or wrapped code.” |
| F-2-6 | Fixed: public page-bound wording avoids MediaBox/CropBox. F-5-1 covers the new base-font statement. |
| F-2-7 | Fixed: public safety copy uses “Linux sandbox,” not kernel-control names. |
| F-3-1 | Fixed: source integrity is registered and byte-compared through both required flows. |
| F-3-2 | Fixed: the generated audit is current, covers landing/README text, and passes its freshness regression. |
| F-4-1 | Fixed: line merging has its own claim and exact passing test. |
| F-4-2 | Fixed: Reset keeps focus immediately and after completion. |
| F-4-3 | Fixed: the heading is “Check Markdown against the finished PDF.” |
| F-4-4 | Fixed: public check terminology uses “syntax color.” |
| F-4-5 | Fixed: the workflow uses three separate sentences. |
| F-4-6 | Fixed: the heading is “Review the HTML proof sheet.” |

The incoming handoff's low `CP-V10-01` cache gap is not fixed; it is reported
as F-5-14.

## Structure, accessibility, privacy, and quality gates

Verified passes:

- `/`, `/?demo=1`, `/privacy/`, and `/terms/` return 200. An unknown route
  returns the designed risograph 404 with HTTP 404 and a route home.
- Every public route has `lang=en`, one H1, one main landmark, a plain title,
  description, canonical URL, Open Graph/Twitter metadata, SVG favicon, and
  180×180 touch icon. The social image is 1200×630.
- Every discovered internal fragment exists. Every discovered product route,
  asset, and GitHub destination resolves; no dead link was found.
- Header/footer treatment is consistent. The skip link is first in keyboard
  order. Demo entry, Reset, exit, Back, and Forward preserve the expected
  focus. At 390 px, controls are at least 44×44 px and the page has no
  horizontal overflow. A 200% text-size smoke test also retained the page
  width, primary action, and footer.
- Axe found zero serious or critical issues on root, demo, Privacy, Terms, and
  404. Normal routes produced no console or page errors. The 404 document
  produced only Chromium's expected failed-document console message.
- Requests during landing, demo entry, and Reset stayed same-origin. Seeded
  real browser storage remained unchanged and demo mode added no storage.
- The service worker supports an offline reload and shows the offline status.
  CSP, header-delivered `frame-ancestors`, `nosniff`, referrer policy, and
  permissions policy are present.
- The release-room risograph layout, two-ink palette, editorial/monospace type,
  crop marks, hard offsets, and original artwork match `.factory/design.md` and
  are not a generic SaaS template.
- `/opt/fleet/lib/verify-url.sh`: PASS. The local production build byte-matches
  all deployed site files checked. Initial JavaScript is 4,320 bytes raw and
  1.81 KiB gzip.
- Clean `npm test`, `npm run typecheck`, `npm run lint`, `npm run build`, and
  `cargo package --manifest-path cli/Cargo.toml --locked` all pass. The suite
  has 4 Rust unit tests, 26 CLI integration tests, and 19 Playwright tests.

## Missed leverage

No AI feature is justified. This is a deterministic release verifier, so model
output would weaken reproducibility. The brief's obvious import/export needs
are covered by Markdown/PDF inputs, HTML proof sheets, and JSON reports. No
account or sync workflow is implied for this local CLI.

## What would make this perfect

Register and test every heading-normalization, font, and syntax-color exclusion
statement—or remove/narrow those statements. Replace the first-screen facts
with privacy, offline, and price facts; name the intended README audience; add
deploy instructions; expand “CI” once; and stop serving stable-name images as
immutable. Then rerun every claim command from a fresh clone and repeat the
cold live review. At that point, there should be nothing left to change.
