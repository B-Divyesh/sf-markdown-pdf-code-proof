# Code Proof handoff — polish 1

- Work order: `markdown-pdf-code-proof-polish-1`
- Repair commit: `465a09672a0be0cfe2bac2ea495575b7c1a08baa`
- Base reviewed: `ffa56a93f19ba5ded11254a2de448d1c6ad101fa`
- Deployment target: <https://markdown-pdf-code-proof.sociobot.in>

## Done

Closed all 31 findings in `review-1.md`. The repair fixes the inaccessible
demo reveal; the demo lifecycle, title, focus, announcement, reset, exit, and
history behavior; the cold install command; every listed or remaining public
claim; mobile first-screen facts; first-read copy; metadata; social/touch
assets; sitemap; 404; and consistent legal/footer links.

The CLI now has direct regressions for fail-closed sandbox setup and renderer
timeouts. The claims contract grew to cover all remaining public promises,
including Git installation, Rust 1.88 compilation, reports, exits, licenses,
privacy, and sandbox behavior. `polish-1.md` maps each review ID to its change
and evidence.

## Verified

Fresh clone: `/tmp/codeproof-clean.TVfVTh/repo` at `465a096`.

```sh
npm ci
# every command in .factory/claims.json, individually
npm test
npm run typecheck
npm run lint
npm run build
cargo package --manifest-path cli/Cargo.toml --locked
```

Results: all 20 claim entries passed; `npm test` passed 3 unit tests, 21 CLI
integration tests, and 12 Playwright tests. The production build is 3.00 KB JS
(1.29 KB gzip), 11.25 KB CSS (3.36 KB gzip), no font payload, 206 KB hero
WebP, 230 KB social JPEG, and 57 KB touch icon. The public clean install test
installed `codeproof 0.1.0` from Git commit `465a0967` into an empty temporary
root and ran `codeproof --version`.

Mobile evidence: `evidence/landing-mobile.png` and `evidence/demo-mobile.png`.
The browser suite includes immediate-transition Axe checks, 390×844 layout,
keyboard focus, Back/Forward route behavior, privacy request/storage capture,
and a dedicated offline browser context.

## Deployment status

The repair commit was pushed to `origin/main`. No deploy command or credential
is present in this work order or repository; per repository rules, deployment
infrastructure is factory-owned. A cold live probe after the push still served
the previous artifact (`Last-Modified: 2026-08-30 00:47:00 UTC`, ETag
`"73833608"`) and did not contain the new Git install command or social asset.
Do not treat the current live URL as verification of `465a096` until the static
factory deployment consumes the pushed commit. Local build and clean-clone
evidence are complete.

## Run and publish

Run `npm ci && npm test && npm run build`. Deploy `dist/site/` with the
factory’s configured static deployment. Publish the CLI only through the
factory registry workflow after `cargo package --manifest-path cli/Cargo.toml
--locked`; do not publish from this repository.
