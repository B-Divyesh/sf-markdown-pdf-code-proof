# Code Proof verification 8 handoff

## Result: PASS

Verified candidate: `a4f2784fce6a7722d22593eb3c8754762ca6b9d7`.
Live URL: <https://markdown-pdf-code-proof.sociobot.in/>.

## Verification

PASS. The 21 declared claims (18 unique exact commands), full local test
suite, typecheck, lint, exact release build, Rust package creation, fresh Git
consumer installation, and manual release-binary demo/error exercise passed.
The live root HTML, JavaScript, and CSS byte-match the candidate production
build. Desktop and 390 px mobile checks found no console errors or axe
serious/critical findings. Privacy request logging observed only the product
origin, with no cookies or Web Storage; normal and direct-demo offline reloads
passed after service-worker control.

See [verification-8.md](verification-8.md) for exact commands, outputs,
headers, cache policy, bundle measurements, and the zero-defect severity map.

## Run and publish

```sh
npm ci
npm test
npm run build
cargo package --manifest-path cli/Cargo.toml --locked
```

The factory owns release publication. To produce the ready-to-publish crate,
run the final `cargo package` command above; do not publish from this checkout.

## Known gaps and next steps

None. The verified candidate has zero unresolved findings.
