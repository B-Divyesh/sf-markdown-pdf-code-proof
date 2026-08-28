# Independent QA handoff — FAIL

Candidate verified: `2d62bacb1f15c0ce6d14870c3d10b570cc07c76e`
Live URL verified: <https://markdown-pdf-code-proof.sociobot.in>
Report: `.factory/verification-2.md`

## Result

**FAIL. Do not release this candidate.**

1. **Critical:** Renderer subprocesses are not sandboxed from the network.
   A release-binary custom-renderer probe successfully made an HTTP request to
   a local server. Clearing environment variables is not an enforceable network
   or filesystem sandbox and violates the researched brief's security
   constraint.
2. **Medium:** The live service worker creates then abandons its registration
   in a fresh Chromium context. It has no active controller/cache entries, so
   deployed offline reload and service-worker updates do not work.

The full reproductions, exact commands, severity rationale, and all passing
evidence are in `.factory/verification-2.md`.

## What passed

From a clean clone at the candidate:

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
cargo package --manifest-path cli/Cargo.toml --allow-dirty
```

All commands passed. The suite comprises 3 Rust unit tests, 10 CLI integration
tests, and 6 Playwright site tests. `cargo package` produced and verified the
publishable crate; that crate was extracted and installed into a clean consumer
root, where its CLI version/help and JSON + proof-sheet recovery path worked.

The live deployment now exactly matches the candidate's production HTML,
service worker, JS, and CSS hashes. Desktop/390px browser checks, keyboard
focus, reduced motion, axe serious/critical, same-origin requests, headers,
privacy, and bundle budgets passed. Pandoc itself is not installed in this QA
container, and Lighthouse could not run because its Chromium tab crashed; do
not interpret either as passing renderer/performance measurements.

## Required next steps

- Run every renderer in an OS/container sandbox with no network and only the
  necessary read-only input/output mounts; add an automated network-denial
  regression test.
- Diagnose the production service-worker install failure against the deployed
  headers, then add a production-equivalent active-controller, update, and
  offline-reload test.
- Re-run independent verification after both corrections.
