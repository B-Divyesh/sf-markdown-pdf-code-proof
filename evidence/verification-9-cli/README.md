# Verification 9 CLI reproductions

Run these with the candidate release binary from the repository root.

```sh
target/release/codeproof check \
  evidence/verification-9-cli/color-scope/manual.md \
  --pdf evidence/verification-9-cli/color-scope/manual.pdf \
  --out /tmp/codeproof-color-default --json

target/release/codeproof check \
  evidence/verification-9-cli/color-scope/manual.md \
  --pdf evidence/verification-9-cli/color-scope/manual.pdf \
  --out /tmp/codeproof-color-deny --json --deny-warnings

target/release/codeproof check \
  evidence/verification-9-cli/page-bounds/manual.md \
  --pdf evidence/verification-9-cli/page-bounds/manual.pdf \
  --out /tmp/codeproof-width --json --no-highlight-check

target/release/codeproof check \
  evidence/verification-9-cli/heading-forms/setext.md \
  --pdf evidence/verification-9-cli/heading-forms/manual.pdf \
  --out /tmp/codeproof-setext --json

target/release/codeproof check \
  evidence/verification-9-cli/heading-forms/explicit.md \
  --pdf evidence/verification-9-cli/heading-forms/manual.pdf \
  --out /tmp/codeproof-explicit --json
```

The two color commands and page-bounds command incorrectly exit 0. The two
heading commands exit 1 with `link.missing-source-target` before PDF
inspection. Captured JSON output from the original runs is stored beside each
fixture.
