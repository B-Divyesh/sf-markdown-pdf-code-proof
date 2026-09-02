# Adversarial first-read review 3 — FAIL

- Product: Code Proof (`markdown-pdf-code-proof`)
- Candidate: `f3aaaae31e2cab889eda6a6a4c68175350ab6812`
- Live URL: <https://markdown-pdf-code-proof.sociobot.in>
- Reviewed: 2 September 2026 UTC
- Verdict: **FAIL**

There are two findings: one medium and one minor. The cold landing, CLI demo,
registered claims, accessibility, routes, privacy behavior, and earlier repair
set otherwise verify. PASS requires zero findings.

## Cold first read

Fresh Chromium contexts opened the live URL at 390×844 and 1440×960, before
scrolling. My answers were:

- What does this do? It checks a finished Markdown PDF for broken code, page
  overflow, and incorrect internal links before release.
- For whom? Engineers and technical writers producing code-heavy manuals.
- What should I click first? **Try it with sample data**.

The exact first-screen text was:

> Catch PDF bugs before release.
>
> For engineers and technical writers, Code Proof catches broken code, page
> overflow, and internal links in the final PDF.
>
> TRY IT WITH SAMPLE DATA
>
> See a sample PDF defect and failed check.

This passes. At 390 px the headline, audience sentence, primary action,
outcome note, install alternative, and all three facts appear without
horizontal overflow. The visual system is a distinct, coherent two-ink proof
sheet rather than a generic SaaS layout.

## Findings

### Medium

#### F-3-1 — README scope promise is not a registered claim

- Location: `README.md`, opening.
- Exact quote: “It is a verifier, not an editor.”
- Why this is a finding: a visitor may rely on this to decide whether running
  the CLI can change their Markdown. It is a concrete product behavior, but
  `.factory/claims.json` has no entry or observable test for it. The claims
  policy requires a test for every claim-like sentence.
- Concrete fix: either remove this sentence as redundant after the opening
  description, or add an `input-unchanged` claim. Its test should run both
  `check --pdf` and a custom-renderer flow against a fixture, then assert that
  the Markdown file bytes are unchanged. Plain replacement copy: “It checks
  PDFs; it does not edit your Markdown.”

### Minor

#### F-3-2 — The checked-in copy audit is incomplete and has incorrect counts

- Location: `.factory/copy-audit.md`.
- Evidence: it is titled “Landing-page copy audit,” so it omits the required
  README audit. Several stated counts are wrong: “See a sample PDF defect and
  failed check.” is 8 words, not 10; “Check an existing PDF, or use a custom
  renderer command.” is 9, not 14; and “Renderer arguments never pass through
  a shell.” is 7, not 9.
- Why this is a finding: the plain-words proof is meant to catch long or
  unclear copy before handoff. An incomplete and inaccurate audit cannot serve
  that purpose, even though the current visible copy is short.
- Concrete fix: regenerate `.factory/copy-audit.md` from the current landing
  and README, include every sentence with a whitespace-delimited count, and
  keep a terminology table. Make the audit scriptable or add a regression test
  so copy changes cannot silently stale it.

## Copy audit

Counts below use whitespace-delimited words; command tokens and versions count
as one word. No visible landing or README sentence exceeds 22 words. No banned
marketing adjective appears. F-3-1 is the only claim-registration flag.

### Landing page

| Words | Sentence | Flag |
| ---: | --- | --- |
| 5 | Catch PDF bugs before release. | — |
| 19 | For engineers and technical writers, Code Proof catches broken code, page overflow, and internal links in the final PDF. | Registered defect claims |
| 8 | See a sample PDF defect and failed check. | — |
| 6 | Check the source against the PDF. | — |
| 10 | Code Proof compares your Markdown with the finished PDF. | Existing-PDF and defect claims |
| 9 | It writes an HTML proof sheet for review. | `html-proof` |
| 9 | Check an existing PDF, or use a custom renderer command. | `existing-pdf`, `renderer-no-shell` |
| 7 | Renderer arguments never pass through a shell. | `renderer-no-shell` |
| 11 | Check links, code colors, and text that runs outside the page. | Defect claims |
| 7 | Match each code fence with the PDF. | Code-content claims |
| 19 | Open the HTML proof sheet, save a JSON report in CI, and use exit codes to stop a broken release. | Output claims |
| 4 | Sample failed release check. | — |
| 4 | Find release-breaking PDF defects. | — |
| 4 | Missing or wrapped code. | — |
| 12 | Flags code fence lines that disappear, merge, or wrap in the PDF. | `single-line-wrap`, `code-content` |
| 8 | Flags text that runs outside a page edge. | `page-bounds` |
| 10 | Flags Markdown fragments with a missing or wrong PDF destination. | `internal-links` |
| 11 | Warns when a language-tagged code fence has no detectable syntax color. | `syntax-color` |
| 6 | Add Code Proof to your build. | — |
| 9 | Install from the public repository with Rust 1.88 or newer. | `install-from-git`, `rust-msrv` |
| 9 | Then check a Markdown and PDF pair. | Defect claims |
| 7 | Linux renderer commands cannot use network sockets. | `renderer-network` |
| 10 | Code Proof runs a renderer only after Linux sandbox setup succeeds. | `renderer-fail-closed` |
| 8 | Checking an existing PDF does not start a renderer. | `existing-pdf` |
| 5 | Check Markdown PDFs before release. | — |
| 7 | The docs and recorded proof still work. | `offline-reload` |
| 3 | Proof run started. | — |
| 7 | Proof run complete: one expected defect found. | — |
| 2 | Demo opened. | — |
| 9 | Sample data is active and nothing is saved. | `private-site`, `sample-demo` |
| 3 | Install commands opened. | — |
| 2 | Copy unavailable. | — |

The first-screen facts (“No files needed to try it,” “No tracking data,” and
“Read the license”) are short fragments, not sentences. They are respectively
covered by `sample-demo`, `private-site`, and `mit-license`.

### README

| Words | Sentence | Flag |
| ---: | --- | --- |
| 9 | Code Proof checks code-heavy Markdown manuals before PDF release. | Defect claims |
| 14 | It compares your Markdown with a finished PDF and writes an HTML proof sheet. | `existing-pdf`, `html-proof` |
| 7 | It is a verifier, not an editor. | **F-3-1** |
| 9 | Install from the public repository with Rust 1.88 or newer. | `install-from-git`, `rust-msrv` |
| 8 | Run a complete check without your own files. | `sample-demo` |
| 14 | The command creates an isolated temporary workspace and prints its HTML proof sheet path. | `sample-demo` |
| 9 | Its bundled sample contains a wrapped code fence line. | `sample-demo` |
| 9 | It returns exit `1` with `HOLD — do not release`. | `sample-demo`, `exit-codes` |
| 6 | Keep artifacts in a chosen directory. | `sample-demo` |
| 8 | Check an existing PDF without starting a renderer. | `existing-pdf` |
| 6 | Use a compatible custom renderer command. | `renderer-no-shell` |
| 9 | `{input}` and `{output}` become individual arguments, never shell input. | `renderer-no-shell` |
| 6 | Write a JSON report for CI. | `json-report` |
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
| 12 | A language-tagged code fence warns when no non-default PDF color is found. | `syntax-color` |
| 5 | Run `codeproof check --help` for command options. | Instruction |
| 10 | Code Proof applies its Linux sandbox before a renderer starts. | `renderer-fail-closed` |
| 6 | Renderer commands cannot create network sockets. | `renderer-network` |
| 7 | Existing-PDF checks do not start a renderer. | `existing-pdf` |
| 8 | A renderer has a deadline set by `--timeout`. | `renderer-timeout` |
| 4 | Verify the project: `npm test`. | Instruction |
| 6 | Build the CLI and site: `npm run build`. | Instruction |
| 7 | Create the publishable Rust package without publishing. | Instruction |
| 6 | The site uses Vite and vanilla TypeScript. | Repository detail |
| 7 | Run it locally with `npm run dev`. | Instruction |
| 6 | Build the static deployment with `npm run build:site`. | Instruction |
| 2 | Version 0.1.0. | Metadata |
| 2 | See CHANGELOG.md. | Instruction |
| 4 | See the MIT License. | `mit-license` |

Headings name their sections; controls name the result (Try, Copy, Reset,
View, Return); no metaphor or mood heading remains. Terms stay consistent:
Markdown, PDF, code fence, renderer, HTML proof sheet, JSON report, and
HOLD — do not release.

## Demo and sandbox

The CLI demo and browser recording pass the required behavior.

- One click opened `/?demo=1#demo`, changed the title to “Demo — Code Proof,”
  focused “Sample failed release check,” and showed the persistent “Demo —
  sample data, nothing is saved” banner.
- The first demo screen showed the actual `codeproof demo` command, a realistic
  `code.flow-changed` defect, an expected exit 1, and a proof-sheet path.
- Reset set the live status to “Proof run started.” and replayed the recording.
  “View install commands” opened `/#install`, hid the banner, restored the
  landing title, and focused the Install heading.
- A fresh browser context made only same-origin requests. A pre-seeded
  `real:sentinel` localStorage value was unchanged; demo use added no stored
  state, cookie, or tracking request.
- In a fresh temporary working directory, the built `codeproof demo` returned
  its intended exit 1, created a separate `/tmp/codeproof-demo-*` workspace,
  and printed the generated HTML proof-sheet path.

## Claims and quality gates

From a fresh clone at the candidate SHA, `npm ci` completed and every exact
`test` command in `.factory/claims.json` completed successfully: all 20
registered claims passed. This included the dedicated browser contexts for
`private-site` and `offline-reload`, Rust 1.88 compilation, public Git
installation, and the bundled CLI demo.

`npm test` also passed: 3 Rust unit tests, 21 CLI integration tests, and 13
Playwright tests. `npm run typecheck`, `npm run lint`, `npm run build`, and
`cargo package --manifest-path cli/Cargo.toml --locked` passed; the release
binary and package artifact were present.

## Earlier finding verification

Each prior finding was retested on the live site and checked against the
candidate source. The first two review rounds are fixed as follows.

| Earlier finding | Current verification |
| --- | --- |
| F-1-1 | Fixed: demo hides unrevealed lines without low-contrast opacity; Axe passes. |
| F-1-2 | Fixed: demo route, title, banner, reset, exit, Back/Forward, focus, and live announcement work. |
| F-1-3 | Fixed: the public Git install command installs from an empty directory. |
| F-1-4 | Fixed: broad local-only promise removed; local-path behavior is registered. |
| F-1-5 | Fixed: unprovable CLI telemetry/account promises removed; website privacy is registered. |
| F-1-6 | Fixed: no-shell behavior is registered and tested. |
| F-1-7 | Fixed: untested filesystem-limit promise removed from public copy. |
| F-1-8 | Fixed: forced Linux sandbox setup failure prevents renderer start. |
| F-1-9 | Fixed: untested Pandoc sanitizing promise removed. |
| F-1-10 | Fixed: timeout is registered; embedded-script promise removed. |
| F-1-11 | Fixed: locked workspace compiles under Rust 1.88. |
| F-1-12 | Fixed: existing-PDF flow is registered and avoids renderer start. |
| F-1-13 | Fixed: public copy contains only tested renderer modes. |
| F-1-14 | Fixed: PDF-internals inventory replaced with visible outcomes. |
| F-1-15 | Fixed: HTML proof, JSON, and exit codes have separate claims. |
| F-1-16 | Fixed: subjective diagnostic-matrix promise removed. |
| F-1-17 | Fixed: MIT references and package metadata are tested. |
| F-1-18 | Fixed: exit meanings are three short sentences. |
| F-1-19 | Fixed: link requirement is two short sentences. |
| F-1-20 | Fixed: code-wrap requirement is two short sentences. |
| F-1-21 | Fixed: action copy explains the sample; HOLD is defined in the result. |
| F-1-22 | Fixed: internal metaphors were removed. |
| F-1-23 | Fixed: workflow names user-visible outcomes. |
| F-1-24 | Fixed: safety copy names the tested socket restriction. |
| F-1-25 | Fixed: public prose consistently says renderer. |
| F-1-26 | Fixed: code fence, HTML proof sheet, JSON report, and sample failed release check are consistent. |
| F-1-27 | Fixed: workflow and sample headings make sense alone. |
| F-1-28 | Fixed: visible controls name their results. |
| F-1-29 | Fixed: social image, touch icon, metadata, and demo sitemap entry are present. |
| F-1-30 | Fixed: header/footer treatment and labeled external GitHub link are consistent. |
| F-1-31 | Fixed: all three first-screen facts are present at 390×844. |
| F-2-1 | Fixed: redundant checkout-install promise removed. |
| F-2-2 | Fixed: test coverage sentence is now a direct instruction. |
| F-2-3 | Fixed: build-output promise is now a direct instruction. |
| F-2-4 | Fixed: focused demo heading is “Sample failed release check.” |
| F-2-5 | Fixed: “Code flow” is now “Missing or wrapped code.” |
| F-2-6 | Fixed: page-edge wording replaces PDF implementation terms. |
| F-2-7 | Fixed: README uses “Linux sandbox,” not unexplained kernel-control names. |

## Structure and missed leverage

The live root, demo, Privacy, Terms, and designed 404 have route-specific
titles, descriptions, canonical URLs, social metadata, favicon/touch icon,
one H1, one main landmark, and consistent navigation/footer treatment. Root,
Privacy, and Terms return 200; an unknown route returns the styled 404 with
status 404; all first-party links and the labeled external GitHub link resolve.
The deployed CSP, referrer policy, nosniff header, robots file, sitemap,
offline service-worker behavior, keyboard skip route, visible focus, mobile
layout, reduced-motion behavior, and Axe checks verified. The 404 navigation
causes the browser's expected failed-resource console message because its HTTP
status is 404; normal routes had no console error.

No missing AI feature is expected: this local PDF-verification CLI has no
user-facing task that benefits from inference, and adding one would be
decorative. Import/export is already covered by supplied Markdown/PDF paths,
HTML proof sheets, and JSON output.

## What would make this perfect

Register or remove the README non-editor promise, then regenerate the complete
copy audit with correct counts and an automated stale-audit check. Re-run the
claim suite. With those two corrections, there is no remaining product, demo,
structure, accessibility, or visual finding from this review.
