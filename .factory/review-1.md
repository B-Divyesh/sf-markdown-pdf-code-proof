# Adversarial first-read review 1 — FAIL

- Product: Code Proof (`markdown-pdf-code-proof`)
- Candidate: `1ce079bd5ad09705a538c8252c1f3b3b7538834d`
- Live URL: <https://markdown-pdf-code-proof.sociobot.in>
- Reviewed: 30 August 2026 UTC
- Verdict: **FAIL**

There are 31 findings: 3 blocking, 14 high, 11 medium, and 3 low. The
first screen is clear and all ten listed claim commands pass, but the product
cannot pass with a serious live demo contrast failure, broken demo-route
semantics, unlisted claims, and unresolved copy and metadata issues.

## Cold first read

Fresh Chromium contexts opened the live site at 390×844 and 1440×900 before
scrolling.

My first-screen answers were:

- What does it do? It checks a finished Markdown PDF for broken code, page
  overflow, and internal-link problems before release.
- For whom? Engineers and technical writers making code-heavy manuals.
- What should I click first? **Try it with sample data**.

The exact copy that supplied those answers was:

> Catch PDF bugs before release.
>
> For engineers and technical writers, Code Proof catches broken code, page
> overflow, and internal links in the final PDF.
>
> TRY IT WITH SAMPLE DATA

This portion passes. At 390 px the headline, audience sentence, primary action,
its outcome note, install alternative, and two of the three facts were visible
without scrolling. At desktop width all three facts and the original artwork
were visible. There was no horizontal overflow.

## Findings

### Blocking

#### F-1-1 — The running demo has a serious contrast failure

- Location: live `/?demo=1#demo`; `site/src/style.css:93-94`.
- Exact affected text: “DEMO HOLD — 1 expected defect found”, “ERROR
  [code.flow-changed] source line 7”, and “Sample workspace:
  /tmp/codeproof-demo-…”.
- Evidence: fresh Axe 4.10.2 runs at both 390 px and 1440 px report serious
  `color-contrast` failures while `.terminal.replaying` fades unrevealed text
  through `opacity: 0`. Axe measured contrast as low as 1.02:1 against
  `#161a19`; the requirement is 4.5:1. The repository test waits for the
  animation to finish before running Axe, so it misses the failing state.
- Why this fails a first visit: the primary action immediately enters this
  state. Required result text temporarily becomes unreadable while remaining
  exposed to accessibility checks.
- Concrete fix: do not animate text opacity. Reveal lines with `visibility` or
  `display`, or use a transform-only transition while keeping every visible
  text state at 4.5:1. Add an Axe assertion immediately after demo start and
  during reset, not only after completion.

#### F-1-2 — “Start for real” does not leave demo mode, and the demo route does not manage title or focus

- Location: live demo banner and `site/index.html:57`; route behavior in
  `site/src/main.ts`.
- Exact control: “START FOR REAL”.
- Evidence: after entering `/?demo=1#demo`, activating the control produces
  `/?demo=1#install`, not `/#install`. The demo banner remains present. The
  title remains “Code Proof — inspect Markdown PDFs before release” instead of
  “Demo — Code Proof”. On entry, exit, and Back, `document.activeElement` is
  `BODY`; focus never moves to the destination heading and no route change is
  announced.
- Why this fails a first visit: the control says demo use is over while the URL
  still says demo. Keyboard and screen-reader users receive no destination
  context. This is broken demo lifecycle and broken route behavior.
- Concrete fix: make the link `/#install` (or clear `demo=1` with
  `replaceState`), hide the demo-mode banner after exit, give the demo route the
  title “Demo — Code Proof”, focus its heading on entry, focus the install
  heading on exit, and announce both changes. Add Playwright assertions for
  URL, title, banner state, active element, Back, and Forward.

#### F-1-3 — Neither public installation path works for a cold visitor

- Location: landing control “Copy install command”; README Install, “Download
  the binary for your platform from a release, or build it with Rust 1.88 or
  newer”; copied command `cargo install --path cli`.
- Evidence: from a fresh temporary directory, the copied command fails with
  “`cli` is not a directory. --path must point to a directory containing a
  Cargo.toml file.” The page never tells the visitor to clone the repository
  and enter its root. GitHub’s releases API returned an empty array, and
  `/releases/latest` redirected to the empty releases page, so there is no
  binary to download either.
- Why this blocks the real job: the demo can be watched, but a first-time
  visitor cannot install the CLI using either instruction presented.
- Concrete fix: publish platform binaries or replace the copied command with a
  tested install-from-Git command that works in an empty directory. If local
  source installation remains, show the clone and `cd` steps before
  `cargo install --path cli`. Add a clean-temporary-directory installation
  smoke test and register the availability claim.

### High — unlisted or under-tested claims

Each item below is a claim-like sentence or fragment on the landing page or in
the README with no matching `.factory/claims.json` entry. Passing unrelated
tests is not a substitute for registering the claim and its exact sandbox
test.

#### F-1-4 — Local-only file handling is unlisted and overbroad

- Quote/location: landing facts, “RUNS LOCALLY / Your files stay on this
  device”; demo footer, “NO FILES UPLOADED”; README opening, “Code Proof is a
  local, single-binary release check for engineers and technical writers.”
- Problem: `private-site` tests the static website, not CLI file handling. The
  privacy page also concedes that a custom renderer has its own behavior, so
  “Your files stay” is broader than what this process controls.
- Fix: rewrite as “Code Proof uploads no files. Review your custom renderer’s
  network behavior.” Add a CLI network-observation claim test for both the
  existing-PDF and renderer paths, or remove the claim.

#### F-1-5 — No-service and no-telemetry claims are unlisted

- Quote/location: landing install, “No account, upload, daemon, or telemetry.”;
  README Install, “Code Proof has no runtime service and sends no telemetry.”
- Problem: `private-site` observes browser storage and requests only. It does
  not exercise the CLI.
- Fix: add a CLI claim entry and a clean-process network/request probe, or
  narrow the copy to an implementation fact that the test can prove.

#### F-1-6 — The no-shell security claim is unlisted

- Quote/location: landing workflow, “Arguments never pass through a shell.”;
  README Usage, “`{input}` and `{output}` are substituted as individual
  arguments, never through a shell.”
- Problem: the integration test
  `custom_renderer_runs_without_a_shell_and_is_checked` exists, but no claim
  entry connects this public security promise to that test.
- Fix: add a `renderer-no-shell` claim pointing to that exact test.

#### F-1-7 — Renderer filesystem limits are unlisted

- Quote/location: landing safety, “On Linux, Code Proof blocks renderer network
  access and limits file access.”; README Renderer safety, “It can read the
  Markdown directory and required runtime files.” and “It can write only to
  its private proof workspace.”
- Problem: `renderer-network` proves socket denial only. It never attempts to
  read an out-of-scope file or write outside the proof workspace.
- Fix: split network and filesystem promises. Add a renderer fixture that
  attempts both forbidden reads and writes and asserts denial.

#### F-1-8 — Fail-closed renderer behavior is unlisted

- Quote/location: landing safety, “It refuses to render when those controls
  are unavailable.”; README, “Code Proof refuses to launch a renderer without
  those kernel controls.”
- Problem: the listed network test runs where the controls are available; it
  does not force setup failure.
- Fix: add an injectable sandbox-setup failure test that asserts exit 2 before
  the renderer starts, then list it in `claims.json`.

#### F-1-9 — Pandoc sanitizing and fixed-argument behavior are unlisted

- Quote/location: README Renderer safety, “The Pandoc adapter also disables raw
  HTML and uses fixed arguments.”
- Problem: no listed claim test records the Pandoc arguments or verifies that
  raw HTML is disabled.
- Fix: run a fake Pandoc executable that records argv, assert the raw HTML/TeX
  disabling flags and absence of shell parsing, and register the claim.

#### F-1-10 — Timeout and non-execution promises are unlisted

- Quote/location: README Renderer safety, “Code Proof enforces a timeout and
  never executes Markdown scripts.”
- Problem: neither part has a claims entry. The sentence also combines two
  independent security promises.
- Fix: split it into two sentences and add separate timeout and embedded-script
  tests, or remove whichever promise cannot be observed.

#### F-1-11 — The Rust 1.88 installation claim is stronger than its listed test

- Quote/location: README Install, “Download the binary for your platform from a
  release, or build it with Rust 1.88 or newer”.
- Problem: `rust-msrv` only compares declared dependency metadata. It does not
  compile with Rust 1.88. A separate manual `cargo +1.88.0 check --workspace
  --locked` passed in this review, but that protection is absent from the
  listed test and normal suite.
- Fix: make the claim test install/use Rust 1.88 and compile the locked package,
  or change the copy to “The package declares Rust 1.88 support.”

#### F-1-12 — Existing-PDF availability is unlisted

- Quote/location: README Install, “A renderer is only needed when Code Proof is
  asked to create the PDF.” and “Existing PDFs can be checked directly.”;
  README safety, “Checking an existing PDF remains available everywhere.”
- Problem: an integration test covers one existing-PDF path, but no claim
  entry covers it, and “everywhere” is an untested platform-wide promise.
- Fix: add a platform-scoped `existing-pdf` claim/test and replace “everywhere”
  with the tested operating systems.

#### F-1-13 — Renderer-mode support is unlisted

- Quote/location: landing workflow, “Use the built-in Pandoc adapter, a custom
  command, or an already-built PDF.”; landing install, “Build the 0.1 binary
  with Rust, then point it at a source/PDF pair.”; README Usage, “Render with
  Pandoc, audit the result, and create `proof/index.html`” and “Use a compatible
  custom renderer.”
- Problem: these are usable input-mode claims without entries. Pandoc was not
  installed or exercised in this review.
- Fix: add one claim/test per mode using sandbox fixtures, including a real
  Pandoc smoke test, or remove unverified modes from public copy.

#### F-1-14 — Low-level PDF inspection behavior is unlisted

- Quote/location: landing workflow, “Read PDF annotations, content streams,
  color operations, and painted text bounds. Match code lines back to their
  source fence.”; README opening, “It uses your existing engine and inspects
  PDF links, page geometry, and fenced code.”
- Problem: individual defect tests cover selected outcomes, but no entry maps
  this broader implementation promise to observable evidence.
- Fix: replace the implementation inventory with the already listed user
  outcomes, or add a table of narrowly worded claims and exact tests.

#### F-1-15 — HTML, JSON, and exit-code output promises are unlisted

- Quote/location: landing workflow, “Open the tactile HTML proof, archive JSON
  in CI, and use stable exit codes to stop a broken manual.”; README opening,
  “It writes a self-contained HTML proof sheet.”; README Usage, “Emit the same
  report as JSON for CI.” and the 25-word exit-code sentence.
- Problem: `sample-demo` proves one HOLD proof only. It does not register the
  general HTML/JSON formats or all three stable exit meanings.
- Fix: split these into `html-proof`, `json-report`, and `exit-codes` claims and
  test success, defect, and operational-error paths from clean directories.

#### F-1-16 — The diagnostic matrix is unlisted

- Quote/location: README Checks, “empty source, malformed fences,
  encrypted/unreadable PDF files, renderer errors, and timeouts produce
  actionable diagnostics.”
- Problem: “actionable” is subjective, and the listed claims do not exercise
  this matrix.
- Fix: name the exact error and next action for each case, then add table-driven
  tests and a claims entry; otherwise delete “actionable”.

#### F-1-17 — The price/license claim is unlisted

- Quote/location: landing fact, “FREE / MIT licensed”; README status, “Code
  Proof is free software under the MIT License.”
- Problem: the repository has a license, but no claim entry verifies that the
  distributed crate and site expose the same license and no paid gate.
- Fix: add a simple package/site license claim test or present only the linked
  license without the unregistered “Free” promise.

### Medium — copy

#### F-1-18 — The exit-code sentence exceeds 22 words

- Quote/location: README Usage, 25 words: “Exit codes are stable: `0` means the
  PDF contract passed, `1` means defects were found, and `2` means the command
  or renderer could not complete.”
- Fix: “Exit `0` means pass. Exit `1` means defects. Exit `2` means the check
  could not finish.”

#### F-1-19 — The internal-link bullet exceeds 22 words

- Quote/location: README Checks, 32 words: “every Markdown fragment link
  resolves to a heading and maps one-for-one to a PDF link annotation with the
  same named destination; that destination must resolve to a page in the final
  PDF”.
- Fix: “Each Markdown fragment must match one PDF link destination. That
  destination must open a page in the final PDF.”

#### F-1-20 — The code-flow bullet exceeds 22 words

- Quote/location: README Checks, 24 words: “code fence text remains present and
  keeps its source line flow in the PDF, including a one-line fence that wraps
  onto multiple painted baselines”.
- Fix: “Fenced code must remain present and keep its line breaks. A one-line
  fence fails if it wraps in the PDF.”

#### F-1-21 — “HOLD” is unexplained jargon at the primary action

- Quote/location: landing action note, “See the bundled HOLD case.”; README
  demo, “its proof shows an expected HOLD result.”
- Why it slows a first read: a new visitor has not yet learned that HOLD means
  a failed release check.
- Fix: “See a sample PDF defect and failed check.” Define `HOLD` only in the
  result itself: “HOLD — do not release”.

#### F-1-22 — The workflow uses metaphor and internal terminology

- Quotes/location: landing, “source contract”, “fixed PDF”, “fixed artifact”,
  and “tactile HTML proof”.
- Why it slows a first read: these terms describe the product’s internal model
  or visual mood, not the visitor’s task.
- Fix: “Code Proof compares your Markdown with the finished PDF and writes an
  HTML report for review.” Replace “Inspect the fixed artifact” with “Check the
  finished PDF” and “tactile HTML proof” with “HTML report”.

#### F-1-23 — The workflow sentence is an implementation inventory

- Quote/location: landing, “Read PDF annotations, content streams, color
  operations, and painted text bounds.”
- Why it slows a first read: “content streams”, “color operations”, and
  “painted text bounds” require PDF internals knowledge but do not state the
  result.
- Fix: “Check links, code colors, and text that runs outside the page.”

#### F-1-24 — “Locked-down process” is vague

- Quote/location: landing heading, “Renderers run in a locked-down process.”
- Why it fails plain words: “locked-down” does not name the actual restriction.
- Fix: “Linux renderers cannot use the network or write outside the proof
  folder.” Keep only restrictions backed by claim tests.

#### F-1-25 — The same renderer is called an “engine” and a “renderer”

- Quote/location: README opening, “existing engine”; landing and README later,
  “renderer”, “Pandoc adapter”, and “custom command”.
- Why it causes friction: a visitor must infer whether these are different
  components.
- Fix: use **renderer** for the component everywhere; call Pandoc a built-in
  renderer option and a custom command a custom renderer command.

#### F-1-26 — Output and code-region terms drift

- Quote/location: “fenced code”, “fence”, “fenced blocks”, and “code block”;
  “HTML proof”, “proof sheet”, “proof”, and “report”.
- Why it causes friction: the README and terminal appear to name the same
  concepts differently.
- Fix: use **code fence**, **HTML proof sheet**, and **JSON report** consistently.

#### F-1-27 — Three headings do not name their sections plainly

- Quote/location: landing headings “Render the real job”, “Inspect the fixed
  artifact”, and “See what a release check returns.”
- Why they fail out of context: “job” and “artifact” are vague; the last one
  does not say this is a failed sample report.
- Fix: “Create or choose the PDF”, “Check the finished PDF”, and “Sample HOLD
  report”.

#### F-1-28 — Four controls do not name their result

- Quote/location: three install controls display only “Copy”; demo control
  displays “Start for real”.
- Why it causes uncertainty: the generic labels make the surrounding layout do
  the work, and “Start for real” does not say it opens installation.
- Fix: display “Copy build command”, “Copy check command”, “Copy CI command”,
  and “View install commands”. The accessible names already contain most of
  this information; make the visible labels equally specific.

### Low — structure and metadata

#### F-1-29 — Social and install metadata are incomplete

- Location: all HTML documents and `site/public/code-proof-press.webp`.
- Evidence: the declared Open Graph image is 1200×800, not the required
  1200×630. The landing page uses the SVG favicon as `apple-touch-icon` with no
  180×180 raster asset; privacy and terms omit the Apple icon. The designed 404
  has no Open Graph metadata. The demo URL is absent from `sitemap.xml`.
- Fix: add a product-art 1200×630 social image and a real 180×180 Apple icon to
  every page, add appropriate 404 social metadata, and list the canonical demo
  route in the sitemap after implementing its distinct title/canonical.

#### F-1-30 — Footer navigation and external-link treatment are inconsistent

- Location: live `/`, `/privacy/`, `/terms/`.
- Evidence: the home footer has Privacy, Terms, and GitHub. Privacy has only
  Home and Terms; Terms has only Home and Privacy. The external GitHub links do
  not expose “external” or “opens in a new site” in their accessible names.
- Fix: use the same footer on every route with Home, Privacy, Terms, and an
  explicitly identified GitHub link.

#### F-1-31 — The mobile first screen hides the third required fact

- Location: live root at 390×844 before scrolling.
- Evidence: “Runs locally” and “Network blocked” are visible, but the third
  fact, “Free / MIT licensed”, starts below the viewport.
- Why it matters: the required first-screen structure calls for all three short
  privacy/offline/price facts. A phone visitor must scroll to finish that scan.
- Fix: reduce mobile hero spacing/type size or arrange the three facts more
  compactly so all three are visible at 390×844 without hiding the primary
  action or its outcome note.

## Copy audit

Counts treat hyphenated terms, versions, paths, and code tokens without spaces
as one word. Headings and UI fragments are audited separately after the
sentence tables.

### Landing-page sentences

| Words | Sentence | Flag |
| ---: | --- | --- |
| 5 | Catch PDF bugs before release. | — |
| 19 | For engineers and technical writers, Code Proof catches broken code, page overflow, and internal links in the final PDF. | Covered by listed defect claims |
| 5 | See the bundled HOLD case. | F-1-21 |
| 6 | Check the source against the PDF. | — |
| 22 | Code Proof compares the source contract with the fixed PDF and leaves a proof sheet your team can review in a browser. | F-1-22 |
| 12 | Use the built-in Pandoc adapter, a custom command, or an already-built PDF. | F-1-13, F-1-25 |
| 6 | Arguments never pass through a shell. | F-1-6 |
| 11 | Read PDF annotations, content streams, color operations, and painted text bounds. | F-1-14, F-1-23 |
| 8 | Match code lines back to their source fence. | F-1-14 |
| 19 | Open the tactile HTML proof, archive JSON in CI, and use stable exit codes to stop a broken manual. | F-1-15, F-1-22 |
| 6 | See what a release check returns. | F-1-27 |
| 4 | Find release-breaking PDF defects. | — |
| 12 | Flags source lines that disappear or stop following their original fenced order. | Covered by `single-line-wrap` and `code-content` |
| 12 | Estimates painted text width from PDF content operations and reports the page. | Covered by `page-bounds` |
| 18 | Matches every Markdown fragment to its PDF named destination and confirms that destination resolves to a real page. | Covered by `internal-links` |
| 10 | Warns when language-tagged blocks produce no detectable non-default PDF color. | Covered by `syntax-color` |
| 6 | Add Code Proof to your build. | — |
| 15 | Build the 0.1 binary with Rust, then point it at a source/PDF pair. | F-1-13 |
| 6 | No account, upload, daemon, or telemetry. | F-1-5 |
| 6 | Renderers run in a locked-down process. | F-1-24 |
| 12 | On Linux, Code Proof blocks renderer network access and limits file access. | `renderer-network` covers only the first half; F-1-7 |
| 9 | It refuses to render when those controls are unavailable. | F-1-8 |
| 5 | Inspect Markdown PDFs before release. | — |
| 7 | The docs and recorded proof still work. | Covered by `offline-reload` |

No landing sentence exceeds 22 words or uses a banned marketing adjective.

### README sentences

| Words | Sentence | Flag |
| ---: | --- | --- |
| 13 | Code Proof is a local, single-binary release check for engineers and technical writers. | F-1-4 |
| 8 | It checks code-heavy Markdown manuals before PDF release. | Covered by listed defect claims |
| 14 | It uses your existing engine and inspects PDF links, page geometry, and fenced code. | F-1-14, F-1-25 |
| 7 | It writes a self-contained HTML proof sheet. | F-1-15 |
| 9 | It is a verifier, not another editor or renderer. | — |
| 17 | Download the binary for your platform from a release, or build it with Rust 1.88 or newer. | F-1-11, F-1-3 |
| 10 | Code Proof has no runtime service and sends no telemetry. | F-1-5 |
| 14 | A renderer is only needed when Code Proof is asked to create the PDF. | F-1-12 |
| 6 | Existing PDFs can be checked directly. | F-1-12 |
| 13 | Run a complete check without installing a renderer or using your own files. | Covered by `sample-demo` |
| 20 | The command creates an isolated temporary workspace, checks the bundled sample, and prints the path to its HTML proof sheet. | Covered by `sample-demo` |
| 15 | The sample contains one wrapped code line, so its proof shows an expected HOLD result. | F-1-21; behavior covered by `sample-demo` |
| 12 | Keep the artifacts in a chosen directory with `codeproof demo --out demo-proof`. | Covered by `sample-demo` |
| 12 | The demo exits `1` because its sample intentionally contains a release defect. | Covered by `sample-demo` |
| 9 | Render with Pandoc, audit the result, and create `proof/index.html`. | F-1-13, F-1-15 |
| 8 | Audit an existing PDF without running a renderer. | F-1-12 |
| 5 | Use a compatible custom renderer. | F-1-13 |
| 12 | `{input}` and `{output}` are substituted as individual arguments, never through a shell. | F-1-6 |
| 8 | Emit the same report as JSON for CI. | F-1-15 |
| 25 | Exit codes are stable: `0` means the PDF contract passed, `1` means defects were found, and `2` means the command or renderer could not complete. | F-1-15, F-1-18 |
| 8 | Warnings do not fail the build by default. | Covered by `syntax-color` |
| 9 | A fence with no detectable color is one example. | Covered by `syntax-color` |
| 6 | Add `--deny-warnings` to make warnings fail. | Covered by `syntax-color` |
| 32 | Every Markdown fragment link resolves to a heading and maps one-for-one to a PDF link annotation with the same named destination; that destination must resolve to a page in the final PDF. | F-1-19; covered by `internal-links` |
| 24 | Code fence text remains present and keeps its source line flow in the PDF, including a one-line fence that wraps onto multiple painted baselines. | F-1-20; covered by `single-line-wrap` and `code-content` |
| 14 | Painted text stays within all four transformed media/crop box edges, with a configurable tolerance. | Covered by `page-bounds` |
| 10 | Fenced blocks contain non-default color operators when highlighting is expected. | Covered by `syntax-color` |
| 14 | Empty source, malformed fences, encrypted/unreadable PDF files, renderer errors, and timeouts produce actionable diagnostics. | F-1-16 |
| 8 | Run `codeproof check --help` for all engine-specific controls. | — |
| 12 | On Linux, every renderer is contained with kernel-enforced Landlock and seccomp rules. | F-1-7, F-1-8 |
| 10 | It can read the Markdown directory and required runtime files. | F-1-7 |
| 9 | It can write only to its private proof workspace. | F-1-7 |
| 7 | It cannot create or use network sockets. | Covered by `renderer-network` |
| 11 | Code Proof refuses to launch a renderer without those kernel controls. | F-1-8 |
| 7 | Checking an existing PDF remains available everywhere. | F-1-12 |
| 11 | The Pandoc adapter also disables raw HTML and uses fixed arguments. | F-1-9 |
| 10 | Code Proof enforces a timeout and never executes Markdown scripts. | F-1-10 |
| 9 | `npm test` runs Rust unit/integration tests and site checks. | Verified locally |
| 15 | `npm run build` creates the release binary in `target/release/codeproof` and the deployable site in `dist/site/`. | Verified locally |
| 8 | To create the publishable Rust package without publishing. | — |
| 7 | The site is Vite + vanilla TypeScript. | Verified in source |
| 7 | Run it locally with `npm run dev`. | — |
| 8 | Build the static deployment with `npm run build:site`. | — |
| 15 | Deploy the generated `dist/site/` directory to the configured Static Web App; no server is required. | — |
| 2 | Version 0.1.0. | Verified in package metadata |
| 2 | See CHANGELOG.md. | — |
| 9 | Code Proof is free software under the MIT License. | F-1-17 |

The three sentences over 22 words are findings F-1-18 through F-1-20. No
README sentence uses a banned marketing adjective.

### Headings, fragments, and controls

All headings and controls were checked out of context. The flagged headings
are “Render the real job”, “Inspect the fixed artifact”, and “See what a
release check returns” (F-1-27). The flagged controls are three visible “Copy”
buttons and “Start for real” (F-1-28). The hero’s “Copy install command” path
does not work from a cold directory (F-1-3). “See the bundled HOLD case” is jargon
(F-1-21). “Runs locally / Your files stay on this device”, “No files uploaded”,
and “Free / MIT licensed” are unlisted claims (F-1-4 and F-1-17). Other
headings and controls name their destination or result plainly.

Terminology should be reduced to this table:

| Concept | One term |
| --- | --- |
| Program that creates the PDF | renderer |
| Markdown code region | code fence |
| Finished document | PDF |
| Browser-readable result | HTML proof sheet |
| Machine-readable result | JSON report |
| Failed release result | HOLD — do not release |

## Demo and sandbox evidence

- One click from the cold landing page opened `/?demo=1#demo`.
- The first settled viewport showed the banner, Reset, Start for real, the
  realistic `code.flow-changed` defect, sample path, proof path, and expected
  exit 1. This is a useful CLI recording, not lorem ipsum.
- Reset changed the live status from “Proof run started.” to “Proof run
  complete: hold, with one expected error.”
- The live browser flow made same-origin requests only. Cookies,
  `localStorage`, and `sessionStorage` remained empty before and after demo use.
- The release binary was run from a new `/tmp/codeproof-cli-cwd.*` directory.
  It used the bundled sample, returned the intentional exit 1, created a new
  `/tmp/codeproof-demo-*` workspace, and printed the proof path. It did not
  read a user file or invoke a renderer.
- The demo still fails overall because of F-1-1 and F-1-2.

## Claims test results

Every command listed in `.factory/claims.json` was run separately from the
initially clean checkout.

| Claim | Result | Evidence |
| --- | --- | --- |
| `single-line-wrap` | PASS | Exact Rust test returned 1 passing test |
| `page-bounds` | PASS | All four MediaBox edges and CropBox case passed |
| `code-content` | PASS | Missing fenced content produced the expected defect |
| `internal-links` | PASS | Wrong named destination produced the expected defect |
| `syntax-color` | PASS | Default warning and denied-warning paths passed |
| `renderer-network` | PASS | Loopback listener received no connection |
| `sample-demo` | PASS | Bundled sample, PDF, HOLD proof, and isolation passed |
| `private-site` | PASS | Same-origin requests and empty browser storage passed |
| `offline-reload` | PASS | Dedicated context reloaded the installed shell offline |
| `rust-msrv` | PASS | Locked dependency declarations require no newer than 1.88 |

The full `npm test` also passed 3 Rust unit tests, 18 CLI integration tests,
and 12 Playwright tests. `npm run build` produced the release binary and
`dist/site/`. A separate real `cargo +1.88.0 check --workspace --locked`
passed. These successes do not cure the unlisted claims in F-1-4 through
F-1-17.

## Privacy and offline evidence

- Live demo requests stayed on
  `https://markdown-pdf-code-proof.sociobot.in`; there were no failed requests,
  cookies, or Web Storage entries.
- A fresh live context installed and activated service worker
  `code-proof-v3`. After going offline, reload returned 200, retained the
  headline, and displayed “Offline. The docs and recorded proof still work.”
- The fresh release CLI demo used only its bundled source and generated local
  temporary artifacts.

The observable web privacy and offline claims pass. The broader CLI privacy
and sandbox claims remain unlisted or only partly tested as specified above.

## Earlier findings checked from scratch

There are no earlier `.factory/review-*.md` or `.factory/polish-*.md` files.
The handoff and five verification reports contain the earlier defects below.
The live HTML, JS, and CSS hashes match the current local production build.

| Earlier defect | Live/code result now |
| --- | --- |
| Wrong or unresolved PDF destination could pass | Fixed: exact destination regression passes |
| Renderer could reach the network | Fixed: real loopback-denial regression passes |
| Production service worker disappeared | Fixed: fresh live worker activates and serves offline reload |
| Two code lines flattened into one could pass | Fixed: `flattened_code_lines_fail_the_release_contract` passes |
| Declared Rust minimum did not compile | Fixed: declared minimum is 1.88 and a real 1.88 locked check passes |
| Mobile overflow regions were not explicitly focusable | Fixed: regression suite passes and live Axe no longer reports it |
| Brand accessible name replaced visible text | Fixed: accessible name contains the visible label |
| Terms touch target was 42 px | Fixed: repository mobile regression asserts at least 44×44 px |
| One source line wrapped onto two PDF baselines could pass | Fixed: exact claim test passes |
| Left/top/bottom page overflow could pass | Fixed: exact four-edge/CropBox claim test passes |

The current demo contrast failure F-1-1 is a new state-specific regression; it
does not reuse an earlier finding ID.

## Structure, links, and visual identity

Passing checks:

- `/`, `/privacy/`, and `/terms/` return 200. An unknown route returns the
  designed Code Proof 404 with a path home.
- Every tested page has `lang=en`, one `h1`, one `main`, a description,
  favicon, and a plain title. Root, privacy, and terms have canonical and Open
  Graph metadata.
- All crawled internal routes/assets and both GitHub destinations returned 200.
- Header navigation is consistent, skip links work, Back restores the demo
  URL, CSP/security headers are present, reduced motion is supported, and the
  initial JavaScript is 2,151 bytes (1.00 KiB gzip).
- The release-room risograph palette, typography, hard ink offsets, proof
  marks, and original artwork are distinctive and match `.factory/design.md`.
  The page is not a generic centered-gradient/feature-card template.

Failures are recorded in F-1-2 and F-1-29 through F-1-31.

## Missed leverage

No extra AI feature is justified. The brief is a deterministic release
verifier, and model output would weaken reproducibility. The expected useful
exports already exist as HTML and JSON, and the tool supports an existing PDF,
Pandoc, or a custom renderer. No sync feature is implied for a local CLI.

## What would make this perfect

Fix all 31 findings and rerun the review from a fresh browser and clean clone.
In particular: keep every animated demo state above 4.5:1; make demo entry and
exit real, titled, focus-managed navigation; register or remove every public
claim; replace internal PDF/security jargon with tested outcomes; shorten the
three long README sentences; use stable terminology and result-naming controls;
and complete the route metadata/footer contract. A passing round must find
zero remaining issues, including during transitions rather than only after the
UI settles.
