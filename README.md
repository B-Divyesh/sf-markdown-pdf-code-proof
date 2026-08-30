# Code Proof

Code Proof is a local, single-binary release check for engineers and technical
writers. It checks code-heavy Markdown manuals before PDF release. It uses your
existing engine and inspects PDF links, page geometry, and fenced code. It
writes a self-contained HTML proof sheet. It is a verifier, not another editor
or renderer.

Live docs: <https://markdown-pdf-code-proof.sociobot.in>

## Install

Download the binary for your platform from a release, or build it with Rust
1.88 or newer:

```sh
cargo install --path cli
```

Code Proof has no runtime service and sends no telemetry. A renderer is only
needed when Code Proof is asked to create the PDF. Existing PDFs can be checked
directly.

## Try the bundled sample

Run a complete check without installing a renderer or using your own files:

```sh
codeproof demo
```

The command creates an isolated temporary workspace, checks the bundled sample,
and prints the path to its HTML proof sheet. The sample contains one wrapped
code line, so its proof shows an expected HOLD result. Keep the artifacts in a
chosen directory with `codeproof demo --out demo-proof`. The demo exits `1`
because its sample intentionally contains a release defect.

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
were found, and `2` means the command or renderer could not complete. Warnings
do not fail the build by default. A fence with no detectable color is one
example. Add `--deny-warnings` to make warnings fail.

Checks in v0.1:

- every Markdown fragment link resolves to a heading and maps one-for-one to a
  PDF link annotation with the same named destination; that destination must
  resolve to a page in the final PDF;
- code fence text remains present and keeps its source line flow in the PDF,
  including a one-line fence that wraps onto multiple painted baselines;
- painted text stays within all four transformed media/crop box edges, with a
  configurable tolerance;
- fenced blocks contain non-default color operators when highlighting is
  expected;
- empty source, malformed fences, encrypted/unreadable PDF files, renderer
  errors, and timeouts produce actionable diagnostics.

Run `codeproof check --help` for all engine-specific controls.

## Renderer safety

On Linux, every renderer is contained with kernel-enforced Landlock and seccomp
rules. It can read the Markdown directory and required runtime files. It can
write only to its private proof workspace. It cannot create or use network
sockets. Code Proof refuses to launch a renderer without those kernel controls.
Checking an existing PDF remains available everywhere. The Pandoc adapter also
disables raw HTML and uses fixed arguments. Code Proof enforces a timeout and
never executes Markdown scripts.

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

Build the static deployment with `npm run build:site`. Deploy the generated
`dist/site/` directory to the configured Static Web App; no server is required.

## Project status

Version 0.1.0. See [CHANGELOG.md](CHANGELOG.md). Code Proof is free software
under the [MIT License](LICENSE).
