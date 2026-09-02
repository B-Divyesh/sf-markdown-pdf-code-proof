# Code Proof

Code Proof checks code-heavy Markdown manuals before PDF release. It compares
your Markdown with a finished PDF and writes an HTML proof sheet. Code Proof
does not edit the supplied Markdown source.

Live docs: <https://markdown-pdf-code-proof.sociobot.in>

## Install

Install from the public repository with Rust 1.88 or newer:

```sh
cargo install --git https://github.com/B-Divyesh/sf-markdown-pdf-code-proof.git --locked codeproof
```

## Try the bundled sample

Run a complete check without your own files:

```sh
codeproof demo
```

The command creates an isolated temporary workspace and prints its HTML proof
sheet path. Its bundled sample contains a wrapped code fence line. It returns
exit `1` and prints `DEMO HOLD — do not release — 1 expected defect found`.

Keep artifacts in a chosen directory:

```sh
codeproof demo --out demo-proof
```

## Usage

Check an existing PDF without starting a renderer:

```sh
codeproof check manual.md --pdf dist/manual.pdf --out proof
```

Use a compatible custom renderer command. `{input}` and `{output}` become
individual arguments, never shell input:

```sh
codeproof check manual.md \
  --engine-command 'my-renderer --offline {input} --output {output}' \
  --out proof
```

Write a JSON report for CI:

```sh
codeproof check manual.md --pdf manual.pdf --json > proof.json
```

Exit `0` means pass. Exit `1` means defects. Exit `2` means the check could
not finish. Warnings do not fail by default. Add `--deny-warnings` to fail
warnings.

## Checks

- Each Markdown fragment must match one PDF link destination. That destination
  must open a page in the finished PDF.
- Code fence text must remain present and keep its line breaks. One source line
  fails if it wraps in the PDF.
- Text is checked against every visible PDF page edge.
- A language-tagged code fence warns when Code Proof cannot detect syntax
  color.

Run `codeproof check --help` for command options.

## Renderer safety

Code Proof applies its Linux sandbox before a renderer starts. Renderer
commands cannot create network sockets. Existing-PDF checks do not start a
renderer. A renderer has a deadline set by `--timeout`.

## Develop and verify

```sh
npm ci
npm test
npm run build
```

Verify the project: `npm test`. Build the CLI and site: `npm run build`.

Create the publishable Rust package without publishing:

```sh
cargo package --manifest-path cli/Cargo.toml --locked
```

The site uses Vite and vanilla TypeScript. Run it locally with `npm run dev`.
Build the static deployment with `npm run build:site`.

## Project status

Version 0.1.0. See [CHANGELOG.md](CHANGELOG.md). See the [MIT License](LICENSE).
