# Code Proof

Code Proof is a local, single-binary release check for engineers and technical
writers shipping code-heavy Markdown manuals as PDF. It renders through the
engine you already use, inspects PDF links and page geometry, matches fenced
code back to the final artifact, and writes an evidence-rich HTML proof sheet.
It is a verifier, not another Markdown editor or renderer.

Live docs: <https://markdown-pdf-code-proof.sociobot.in>

## Install

Download the binary for your platform from a release, or build it with Rust
1.79 or newer:

```sh
cargo install --path cli
```

Code Proof has no runtime service and sends no telemetry. A renderer is only
needed when Code Proof is asked to create the PDF. Existing PDFs can be checked
directly.

## Usage

Render with Pandoc, audit the result, and create `proof/index.html`:

```sh
codeproof check manual.md --engine pandoc --out proof
```

Audit an existing PDF without running a renderer:

```sh
codeproof check manual.md --pdf dist/manual.pdf --out proof
```

Use a compatible custom renderer. `{input}` and `{output}` are substituted as
individual arguments, never through a shell:

```sh
codeproof check manual.md \
  --engine-command 'my-renderer --offline {input} --output {output}' \
  --out proof
```

Emit the same report as JSON for CI:

```sh
codeproof check manual.md --pdf manual.pdf --json > proof.json
```

Exit codes are stable: `0` means the PDF contract passed, `1` means defects
were found, and `2` means the command or renderer could not complete. By
default warnings (such as a code fence with no detectable colored text) do not
fail the build; add `--deny-warnings` to promote them.

Checks in v0.1:

- every Markdown fragment link resolves to a heading and maps one-for-one to a
  PDF link annotation with the same named destination; that destination must
  resolve to a page in the final PDF;
- code fence text remains present and line-shaped in the PDF;
- painted text stays within the page media/crop box, with a configurable
  tolerance;
- fenced blocks contain non-default color operators when highlighting is
  expected;
- empty source, malformed fences, encrypted/unreadable PDF files, renderer
  errors, and timeouts produce actionable diagnostics.

Run `codeproof check --help` for all engine-specific controls.

## Renderer safety

Renderer processes are launched directly, with a clean environment, inside an
isolated temporary working directory. Network proxy variables and common
credential variables are removed. The built-in Pandoc adapter disables raw
HTML and uses a fixed argument list. Code Proof enforces a timeout and never
executes Markdown scripts itself. This is process isolation, not an OS security
boundary; use your CI sandbox for untrusted documents.

## Develop and verify

```sh
npm ci
npm test
npm run build
```

`npm test` runs Rust unit/integration tests and site checks. `npm run build`
creates the release binary in `target/release/codeproof` and the deployable
site in `dist/site/`. To create the publishable Rust package without publishing:

```sh
cargo package --manifest-path cli/Cargo.toml
```

The site is Vite + vanilla TypeScript. Run it locally with `npm run dev`.

## Project status

Version 0.1.0. See [CHANGELOG.md](CHANGELOG.md). Code Proof is free software
under the [MIT License](LICENSE).
