# Code Proof demo

## Run it

From a built checkout:

```sh
target/release/codeproof demo
```

For a predictable output path:

```sh
target/release/codeproof demo --out /tmp/codeproof-sample
```

The browser recording is available at
<https://markdown-pdf-code-proof.sociobot.in/?demo=1>. This route sets the
title to “Demo — Code Proof”, shows the persistent sample-data banner, and
focuses the “Sample failed release check” heading. “Reset demo” replays the
recording without losing keyboard focus. “View install commands” leaves demo
mode at `/#install`.

## Sample and isolation

The source is `cli/examples/sample-manual.md`. The command writes a generated
one-page PDF beside a self-contained `proof/index.html`. Its one-line JavaScript
fence is deliberately split across two PDF baselines, so the demo shows an
expected `code.flow-changed` result: `HOLD — do not release`.
The command exits `1`, matching the normal release-defect contract.

Without `--out`, each run creates a new persistent directory under the operating
system temporary directory. The demo never reads user files, uses browser
storage, or calls a renderer. Delete that printed directory to reset it. The
web demo has no stored state, so its isolation namespace is empty. Resetting
only restarts the bundled recording. The site transcript is generated from the
real command during each build. Its browser test runs the command again and
compares every line after normalizing only the temporary path.
