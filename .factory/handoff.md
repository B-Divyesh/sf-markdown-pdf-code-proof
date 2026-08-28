# Repair handoff — Code Proof

Base candidate: `2d62bacb1f15c0ce6d14870c3d10b570cc07c76e`
Verifier report repaired: `.factory/verification-2.md`
Work order: `markdown-pdf-code-proof-repair-2`

## Result

Both release blockers were repaired without changing the researched CLI job or
the static-site deployment class.

1. Renderer processes are now contained on Linux with kernel-enforced
   Landlock filesystem rules and a seccomp filter. They can read only the
   Markdown input directory plus runtime directories, write only their private
   workspace, and cannot create or use sockets. If these controls are not
   available, Code Proof refuses to start a renderer rather than falling back
   to an unsafe execution. Existing-PDF checks remain available.
2. The service-worker generator now omits `staticwebapp.config.json` from the
   precache. Azure Static Web Apps consumes that deployment-control file and
   serves it as a 404; including it made `cache.addAll` reject the entire
   worker install. Cache `code-proof-v2` also calls `skipWaiting` and
   `clients.claim` so new versions activate promptly.

## Regression coverage

- `renderer_sandbox_denies_network_connections` opens a real loopback listener
  and runs a custom renderer that attempts a real `curl` request. The command
  is denied and the listener receives no connection.
- The browser worker test server intentionally returns production-equivalent
  404 for `/staticwebapp.config.json` and production CSP/cache headers. The
  Playwright suite asserts an activated controller, `code-proof-v2` cache,
  exclusion of the control file from the generated shell, and an offline
  reload after install.

## Verification run locally

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
cargo package --manifest-path cli/Cargo.toml --allow-dirty
```

All passed: 3 Rust unit tests, 11 CLI integration tests, and 8 Playwright
site tests. The browser suite includes axe serious/critical checks, desktop
and 390px paths, reduced-motion behavior, keyboard/skip-link behavior,
offline reload, and the production-shaped worker install. The production build
produced `target/release/codeproof` and `dist/site/`; generated assets remain
2.08 kB JavaScript (0.95 kB gzip), 10.18 kB CSS (3.16 kB gzip), and a 210,844
byte hero image.

`cargo package --manifest-path cli/Cargo.toml --allow-dirty` packaged and
verified `codeproof 0.1.0` (27.8 KiB compressed). The factory owns registry
credentials; do not publish from this checkout. The packaged crate was also
extracted and installed into a separate consumer root with `cargo install
--locked --debug`; its installed binary reported `codeproof 0.1.0` and showed
the documented `check --help` surface.

## Deployment and follow-up

This repository has no in-repository deploy workflow; static deployment is
triggered by the factory from the pushed `main` commit. After it completes,
verify the live URL in a fresh browser context: `navigator.serviceWorker.ready`
must have an activated registration and `navigator.serviceWorker.controller`
must be non-null before testing an offline reload.

Pandoc is not installed in this worker, so the built-in adapter was not run
against a real Pandoc binary. Lighthouse could not be collected in this
container because its supplied Chromium crashes; no Lighthouse score is
claimed. The checked browser/a11y/bundle evidence above passed.
