# Adversarial first-read review 2 — FAIL

- Product: Code Proof (`markdown-pdf-code-proof`)
- Candidate: `f1474e5871a1c5c28d4e9967c8f9476a41f20a79`
- Live URL: <https://markdown-pdf-code-proof.sociobot.in>
- Reviewed: 2 September 2026 UTC
- Verdict: **FAIL**

There are eight findings: one blocking, three high, and four medium. The
product works, the demo is useful, the live deployment matches the candidate,
and all 20 registered claims pass. It still fails this review because an
earlier terminology finding is only partly fixed. The README also contains
three usable promises without claim entries, and four phrases do not meet the
plain-words standard.

## Cold first read

Fresh Chromium contexts opened the live site at 390×844 and 1440×900 before
scrolling. My answers were:

- What does it do? It checks a finished Markdown PDF for broken code, page
  overflow, and internal-link errors before release.
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

This part passes. At 390 px the headline, audience sentence, primary action,
outcome note, install alternative, all three facts, and the top of the original
art fit without scrolling. The page had no horizontal overflow.

## Findings

### Blocking

#### F-1-26 — Output and code-region terminology is still inconsistent

- Exact quote/location: live landing page, **Syntax color**: “Warns when
  language-tagged **blocks** produce no detectable non-default PDF color.”
  README **Checks** calls the same thing a “language-tagged **code fence**.”
- Exact quote/location: live demo heading: “Sample HOLD **report**.” The
  established output terms are “HTML proof sheet” and “JSON report.” This
  heading does not say which output it names; the displayed object is recorded
  terminal output, not the JSON report.
- Code: `site/index.html:80,104`; README `README.md:71`.
- Why this fails: F-1-26 required one term for each concept, and
  `.factory/polish-1.md` says the site was standardized on “code fence,” “HTML
  proof sheet,” and “JSON report.” The live candidate still makes a new reader
  decide whether a block differs from a code fence and whether this report is
  the JSON report. The work order makes every half-fixed earlier finding
  blocking.
- Concrete fix: change the check copy to “Warns when a language-tagged code
  fence has no detectable syntax color.” Rename the demo heading “Sample
  failed check” or “Sample terminal result.” Reserve “HTML proof sheet” and
  “JSON report” for those two artifacts. Update the terminology audit and add a
  copy regression that rejects “block” and bare “report” in user-facing output
  terminology.

### High

#### F-2-1 — The checkout installation path is an unlisted claim

- Exact quote/location: README **Install**: “For a checkout, run
  `cargo install --path cli` from the repository root.”
- Why this matters: this is a second installation promise. The
  `install-from-git` claim tests only `cargo install --git ...`; no
  `.factory/claims.json` entry covers the path install.
- Concrete fix: add a `checkout-install` claim whose test installs `cli` with
  `--locked` into an empty temporary root and asserts the installed binary's
  exact version. Alternatively remove this extra installation path.

#### F-2-2 — The stated `npm test` coverage is an unlisted claim

- Exact quote/location: README **Develop and verify**: “`npm test` runs Rust
  unit/integration tests and browser checks.”
- Why this matters: the sentence promises specific coverage, but there is no
  claim entry for it. The command passed in this review; that does not register
  the promise for future claim-only verification.
- Concrete fix: rewrite this as the non-claim instruction “Verify the project:
  `npm test`.” Otherwise add a non-recursive contract test that inspects the
  test script and proves both Rust and browser suites are selected.

#### F-2-3 — The stated build outputs are an unlisted claim

- Exact quote/location: README **Develop and verify**: “`npm run build`
  creates `target/release/codeproof` and `dist/site/`.”
- Why this matters: both artifacts exist in this review, but no claim entry
  makes that promised result part of the required claim gate.
- Concrete fix: add a `build-artifacts` claim with a clean build test that
  asserts the binary and deployable site exist. Alternatively use the
  instruction “Build the CLI and site: `npm run build`” without promising
  paths.

### Medium

#### F-2-4 — The focused demo heading uses unexplained jargon

- Exact quote/location: live `/?demo=1#demo` heading, “Sample HOLD report.”
- Why this matters: demo entry moves keyboard focus to this heading before the
  animated terminal defines “HOLD — do not release.” A heading must make sense
  when heard alone.
- Concrete fix: use “Sample failed release check.” Keep “HOLD — do not
  release” inside the result where it is defined.

#### F-2-5 — “Code flow” does not name the defect out of context

- Exact quote/location: live landing **Checks** heading, “Code flow.”
- Why this matters: in a heading list, “flow” could mean program control flow,
  text wrapping, or source order. The useful meaning appears only in the next
  sentence.
- Concrete fix: rename it “Missing or wrapped code.”

#### F-2-6 — The README uses unexplained PDF implementation terms

- Exact quote/location: README **Checks**: “Painted text is checked against all
  four MediaBox or CropBox edges.”
- Why this matters: “painted text,” “MediaBox,” and “CropBox” require PDF
  internals knowledge. The sentence can state the tested result without making
  users decode the implementation.
- Concrete fix: “Text is checked against every visible PDF page edge.” If the
  box distinction is necessary, add: “This includes the PDF MediaBox and
  CropBox.”

#### F-2-7 — The README names Linux controls without explaining them

- Exact quote/location: README **Renderer safety**: “On Linux, renderer
  commands start only after Landlock and seccomp setup succeeds.”
- Why this matters: Landlock and seccomp are unexplained implementation names.
  The next sentence explains only the socket effect, not what Landlock does.
- Concrete fix: “On Linux, Code Proof applies file-access limits and blocks
  network sockets before a renderer starts.” Publish only the file-access part
  after adding a behavior test for it; otherwise say “Code Proof applies its
  Linux sandbox before a renderer starts.”

## Copy audit

Counts use whitespace-delimited words. Inline commands count as one unit. The
landing page averages 7.9 words across its static and runtime sentences; the
README averages 7.6 words. No sentence exceeds 22 words and no banned
marketing adjective appears.

### Landing-page sentences

| Words | Sentence | Flag |
| ---: | --- | --- |
| 5 | Catch PDF bugs before release. | — |
| 19 | For engineers and technical writers, Code Proof catches broken code, page overflow, and internal links in the final PDF. | Registered defect claims |
| 8 | See a sample PDF defect and failed check. | — |
| 6 | Check the source against the PDF. | — |
| 9 | Code Proof compares your Markdown with the finished PDF. | Registered inspection claims |
| 8 | It writes an HTML proof sheet for review. | `html-proof` |
| 10 | Check an existing PDF, or use a custom renderer command. | `existing-pdf`, `renderer-no-shell` |
| 7 | Renderer arguments never pass through a shell. | `renderer-no-shell` |
| 11 | Check links, code colors, and text that runs outside the page. | Registered defect claims |
| 7 | Match each code fence with the PDF. | `single-line-wrap`, `code-content` |
| 20 | Open the HTML proof sheet, save a JSON report in CI, and use exit codes to stop a broken release. | `html-proof`, `json-report`, `exit-codes` |
| 4 | Find release-breaking PDF defects. | — |
| 12 | Flags code fence lines that disappear, merge, or wrap in the PDF. | `single-line-wrap`, `code-content`; F-1-26 elsewhere |
| 8 | Flags text that runs outside a page edge. | `page-bounds` |
| 10 | Flags Markdown fragments with a missing or wrong PDF destination. | `internal-links` |
| 10 | Warns when language-tagged blocks produce no detectable non-default PDF color. | F-1-26 |
| 6 | Add Code Proof to your build. | — |
| 10 | Install from the public repository with Rust 1.88 or newer. | `install-from-git`, `rust-msrv` |
| 7 | Then check a Markdown and PDF pair. | Registered inspection claims |
| 7 | Linux renderer commands cannot use network sockets. | `renderer-network` |
| 11 | Code Proof runs a renderer only after Linux sandbox setup succeeds. | `renderer-fail-closed` |
| 9 | Checking an existing PDF does not start a renderer. | `existing-pdf` |
| 5 | Check Markdown PDFs before release. | Registered inspection claims |
| 1 | Offline. | `offline-reload` |
| 7 | The docs and recorded proof still work. | `offline-reload` |
| 3 | Proof run started. | — |
| 8 | Proof run complete: hold, with one expected error. | F-2-4 |
| 2 | Demo opened. | — |
| 9 | Sample data is active and nothing is saved. | `private-site`, `sample-demo` |
| 3 | Install commands opened. | — |
| 2 | Copy unavailable. | Error says what happened; the following status names the command to select. |

### README sentences

| Words | Sentence | Flag |
| ---: | --- | --- |
| 9 | Code Proof checks code-heavy Markdown manuals before PDF release. | Registered defect claims |
| 14 | It compares your Markdown with a finished PDF and writes an HTML proof sheet. | `existing-pdf`, `html-proof` |
| 7 | It is a verifier, not an editor. | Scope statement |
| 10 | Install from the public repository with Rust 1.88 or newer. | `install-from-git`, `rust-msrv` |
| 9 | For a checkout, run `cargo install --path cli` from the repository root. | F-2-1 |
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
| 11 | Painted text is checked against all four MediaBox or CropBox edges. | `page-bounds`; F-2-6 |
| 12 | A language-tagged code fence warns when no non-default PDF color is found. | `syntax-color` |
| 5 | Run `codeproof check --help` for command options. | Instruction |
| 12 | On Linux, renderer commands start only after Landlock and seccomp setup succeeds. | `renderer-fail-closed`; F-2-7 |
| 6 | Renderer commands cannot create network sockets. | `renderer-network` |
| 7 | Existing-PDF checks do not start a renderer. | `existing-pdf` |
| 8 | A renderer has a deadline set by `--timeout`. | `renderer-timeout` |
| 8 | `npm test` runs Rust unit/integration tests and browser checks. | F-2-2 |
| 5 | `npm run build` creates `target/release/codeproof` and `dist/site/`. | F-2-3 |
| 7 | Create the publishable Rust package without publishing. | Instruction |
| 7 | The site uses Vite and vanilla TypeScript. | Repository description |
| 5 | Run it locally with `npm run dev`. | Instruction |
| 6 | Build the static deployment with `npm run build:site`. | Instruction |
| 2 | Version 0.1.0. | Package metadata |
| 2 | See CHANGELOG.md. | Instruction |
| 4 | See the MIT License. | `mit-license` |

### Headings, fragments, and controls

- F-2-4 flags “Sample HOLD report”; F-2-5 flags “Code flow.” All other
  headings name their section when read alone.
- F-1-26 flags “language-tagged blocks” and bare “report.” Other product terms
  consistently use Markdown, PDF, code fence, renderer, HTML proof sheet, and
  JSON report.
- F-2-6 and F-2-7 flag unexplained implementation jargon in the README.
- Every button starts with a result-naming verb: Try, Copy, Reset, View, or
  Return. Navigation links name their destinations.
- “Original artwork” is a provenance caption backed by `.factory/design.md`,
  not a generic slogan. No metaphor or mood heading remains.

## Demo and sandbox

The demo behavior passes:

- One click from the cold page opens `/?demo=1#demo` with title “Demo — Code
  Proof” and focus on the sample heading.
- The persistent banner reads “Demo — sample data, nothing is saved” and
  exposes **Reset demo** and **View install commands**.
- The first demo viewport contains the real `codeproof demo` command and a
  realistic `code.flow-changed` result with expected exit 1, workspace, and
  proof paths. Reset changes the live status to “Proof run started” and then
  completes again.
- A seeded `real:sentinel` localStorage value remained unchanged. The demo
  added no cookies, localStorage, or sessionStorage state. All requests stayed
  on `https://markdown-pdf-code-proof.sociobot.in`.
- Leaving demo mode opened `/#install`, hid the banner, and focused the install
  heading. Back restored the demo URL, title, banner, and heading focus.
- The real CLI ran from a new temporary working directory. It returned the
  intentional exit 1, created a distinct `/tmp/codeproof-demo-*` directory,
  and wrote only `sample-manual.md`, `sample-manual.pdf`, and
  `proof/index.html`. The proof contained HOLD and `code.flow-changed`.

## Registered claims

Every command in `.factory/claims.json` ran from the clean clone
`/tmp/codeproof-review2.ktoLUY/repo` at the candidate commit.

| Claim | Result | Observable evidence |
| --- | --- | --- |
| `single-line-wrap` | PASS | Wrapped source line returned `code.flow-changed`. |
| `page-bounds` | PASS | All MediaBox/CropBox edges were rejected. |
| `code-content` | PASS | Missing fence text returned `code.content-missing`. |
| `internal-links` | PASS | Wrong PDF destination failed. |
| `syntax-color` | PASS | Warning and denied-warning paths passed. |
| `existing-pdf` | PASS | Existing PDF produced PASS proof without a renderer. |
| `local-cli-files` | PASS | Supplied local paths produced local proof output. |
| `renderer-no-shell` | PASS | Placeholder arguments reached the fixture without a shell. |
| `renderer-network` | PASS | Renderer could not reach the loopback listener. |
| `renderer-fail-closed` | PASS | Forced sandbox failure prevented renderer start. |
| `renderer-timeout` | PASS | Two-second renderer stopped at a one-second deadline. |
| `html-proof` | PASS | Successful check wrote a PASS HTML proof sheet. |
| `json-report` | PASS | Successful check emitted `passed: true` JSON. |
| `exit-codes` | PASS | Pass, defect, and operational paths returned 0, 1, and 2. |
| `sample-demo` | PASS | Bundled sample wrote an isolated HOLD proof. |
| `private-site` | PASS | Demo stayed same-origin and left tracking stores empty. |
| `offline-reload` | PASS | Dedicated browser context reloaded the shell offline. |
| `rust-msrv` | PASS | Rust 1.88 compiled the locked workspace. |
| `install-from-git` | PASS | Empty install root built the public Git revision and ran it. |
| `mit-license` | PASS | Repository, crate, landing page, and Terms identify MIT. |

The registered claims are tested. F-2-1 through F-2-3 are the remaining
claim-like README sentences with no entries.

## Earlier finding verification

Every F-1 finding was checked on the byte-matched live deployment and in the
candidate source. One remains half-fixed.

| Earlier finding | Current result |
| --- | --- |
| F-1-1 demo contrast | Fixed. Live Axe at 0, 250, 500, 750, and 1,000 ms found zero violations. Hidden lines use visibility, not low opacity. |
| F-1-2 demo route/title/focus | Fixed. Live entry, exit, Back, title, banner, and focus all behave as specified. |
| F-1-3 public installation | Fixed. `npm run test:install` installed public Git commit `f1474e58` into an empty root. |
| F-1-4 overbroad local-only copy | Fixed. The broad upload/device promise is gone; the narrower local-path claim passes. |
| F-1-5 no-service/telemetry copy | Fixed. The CLI-wide promises are gone; site privacy remains registered. |
| F-1-6 no-shell claim | Fixed. Claim is registered and its exact test passes. |
| F-1-7 filesystem-limit claim | Fixed at the public-copy location. The untested filesystem-limit promise was removed. |
| F-1-8 fail-closed renderer | Fixed. Forced setup refusal prevents process start. |
| F-1-9 Pandoc sanitizing claim | Fixed. The unregistered claim was removed. |
| F-1-10 timeout/script promise | Fixed. Timeout is registered and tested; script-execution copy is gone. |
| F-1-11 Rust 1.88 | Fixed. The exact locked workspace compiled under Rust 1.88. |
| F-1-12 existing PDF | Fixed. Registered test passes and avoids renderer start. |
| F-1-13 renderer modes | Fixed. Unverified Pandoc marketing is gone; custom mode has an exact test. |
| F-1-14 PDF implementation inventory | Fixed. Landing copy now names user-visible outcomes. |
| F-1-15 HTML/JSON/exits | Fixed. All three contracts have entries and passing tests. |
| F-1-16 diagnostic matrix | Fixed. The subjective matrix claim is gone. |
| F-1-17 MIT claim | Fixed. License entry and test pass. |
| F-1-18 long exit sentence | Fixed. It is three sentences of 4, 4, and 8 words. |
| F-1-19 long link sentence | Fixed. It is two sentences of 9 and 10 words. |
| F-1-20 long code-flow sentence | Fixed. It is two sentences of 11 and 10 words. |
| F-1-21 unexplained HOLD action copy | Fixed at the prior locations. The action now says “sample PDF defect and failed check,” and README defines HOLD. F-2-4 covers the remaining heading problem. |
| F-1-22 metaphor/internal terminology | Fixed. “Source contract,” “fixed artifact,” and “tactile” copy are gone. |
| F-1-23 implementation-inventory sentence | Fixed. It now names link, color, and page-edge outcomes. |
| F-1-24 vague lock-down copy | Fixed. Landing safety copy names blocked network sockets. |
| F-1-25 engine/renderer drift | Fixed. Public prose uses renderer; `--engine-command` remains the CLI option name. |
| F-1-26 output/code-region drift | **Not fixed.** See blocking finding F-1-26. |
| F-1-27 vague workflow headings | Fixed. The three workflow headings name their actions. |
| F-1-28 generic controls | Fixed. Copy and demo-exit controls name their results. |
| F-1-29 social/install metadata | Fixed. Social image is 1200×630, touch icon is 180×180, all routes have social metadata, and demo is in the sitemap. |
| F-1-30 footer/external links | Fixed. Header/footer links are consistent and GitHub is labeled external. |
| F-1-31 third mobile fact | Fixed. All three facts start above 844 px at 390 px width. |

## Structure, accessibility, links, and identity

The structural checks pass:

- Root title is 49 characters and follows “Product — what it does.” Demo,
  Privacy, Terms, and 404 use distinct route titles.
- Each route has `lang=en`, one `h1`, one `main`, a plain description,
  canonical URL, Open Graph/Twitter data, SVG favicon, and 180×180 touch icon.
  The social image is 1200×630.
- Unknown paths return the designed risograph 404 with a route home. The 404
  document's own skip link resolves to its existing `#main`; the document
  correctly retains HTTP 404.
- All other discovered same-origin links and both GitHub destinations returned
  200. Demo deep links, exit, Back, and focus restoration work.
- Axe found zero violations on root, Privacy, Terms, 404, and the completed
  demo at mobile and desktop sizes. Transition-time Axe checks also found zero.
  The live root had no console or page errors. `/opt/fleet/lib/verify-url.sh`
  passed.
- The response includes self-only CSP, header-delivered `frame-ancestors`,
  HSTS, `nosniff`, referrer policy, and permissions policy. Reduced motion and
  keyboard paths pass in the repository suite.
- The live root, legal pages, 404, assets, artwork, metadata files, and service
  worker match the clean production build byte for byte. Initial JavaScript is
  3,003 bytes raw and 1,312 bytes gzip.
- The two-ink release-room risograph, editorial type, hard registration
  offsets, proof marks, and original art match `.factory/design.md`. The site
  is visually distinct from a generic SaaS template.

## Quality gates

The following passed in the clean clone:

```text
npm test
npm run typecheck
npm run lint
npm run build
cargo package --manifest-path cli/Cargo.toml --locked
```

This covered Rust 1.88, 3 unit tests, 21 CLI integration tests, 12 Playwright
tests, formatting, Clippy with warnings denied, TypeScript, the production CLI,
`dist/site/`, and a verified 33.3 KiB crate package.

## Missed leverage

No AI feature is justified. This product verifies deterministic release
artifacts; model output would weaken reproducibility. HTML and JSON exports,
existing-PDF input, and custom renderer input already cover the obvious import
and export needs. A sync feature is not implied for a local CLI.

## What would make this perfect

Resolve F-1-26 everywhere, replace the two unclear headings, explain or remove
the two implementation terms, and register or rewrite all three remaining
README claims. Then rerun the claim-only gate and this complete cold review.
A passing round must have no inherited terminology drift, no unlisted promise,
and no heading that needs its following paragraph to be understood.
