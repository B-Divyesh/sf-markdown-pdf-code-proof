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
<https://markdown-pdf-code-proof.sociobot.in/?demo=1#demo>.

## Sample and isolation

The source is `cli/examples/sample-manual.md`. The command writes a generated
one-page PDF beside a self-contained `proof/index.html`. Its one-line JavaScript
fence is deliberately split across two PDF baselines, so the demo shows an
expected `code.flow-changed` HOLD.

Without `--out`, each run creates a new persistent directory under the operating
system temporary directory. The demo never reads user files, uses browser
storage, or calls a renderer. Delete that printed directory to reset it. On the
site, “Reset demo” replays the recording and “Start for real” returns to the
install instructions.
