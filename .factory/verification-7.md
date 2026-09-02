# Independent verification 7 — PASS

- Candidate commit: `63abecedf6c38bea914bbf2c4fb4485cf37a0923`
- Repository / branch: `B-Divyesh/sf-markdown-pdf-code-proof`, `main`
- Live URL: <https://markdown-pdf-code-proof.sociobot.in>
- Verified: 2026-09-02 UTC
- Verdict: **PASS** — the live documentation artifact matches the candidate production build, and the local CLI and its packaged consumer flow meet the researched brief.

## First read and demo

A cold desktop visit returned HTTPS 200 with the title “Code Proof — inspect
Markdown PDFs before release” and this first screen:

> Catch PDF bugs before release. For engineers and technical writers, Code
> Proof catches broken code, page overflow, and internal links in the final
> PDF. Try it with sample data.

This plainly states the job, audience, and first action. One click opened
`/?demo=1#demo`, showed the persistent “Demo — sample data, nothing is saved”
notice and Reset demo action, displayed the expected `code.flow-changed` HOLD,
and offered install instructions. The CLI demo uses bundled files and writes
only its isolated output directory.

## Claims gate — 20 / 20 passed

Every command in `.factory/claims.json` ran from the clean candidate checkout
after `npm ci`. All exact CLI claims passed: wrapped/missing code, page edges,
incorrect internal destinations, syntax-color policy, existing-PDF output,
local artifacts, shell-free renderer arguments, network denial, fail-closed
sandbox setup, timeout, JSON/HTML reports, exit codes, and bundled demo.
The duplicate exact commands for their respective claims were also run.

Both browser claim commands passed independently:

```sh
npm run test:site -- --grep @claim:private-site
npm run test:site -- --grep @claim:offline-reload
```

`npm run test:msrv` passed with Rust 1.88; `npm run test:install` installed
the public Git candidate (`#63abeced`) into an empty root and ran its version
command; and `npm run test:license` passed.

## Local quality and CLI consumer checks

The following passed:

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
cargo package --manifest-path cli/Cargo.toml --locked
```

`npm test` passed 3 Rust unit tests, 21 CLI integration tests, 13 browser
tests, the Rust 1.88 compatibility check, and license check. Formatting,
Clippy with warnings denied, type checking, the release build, and crate
packaging passed. The `.crate` archive is 34,073 bytes.

A clean consumer installed `target/package/codeproof-0.1.0` into a new Cargo
root. `codeproof --version` returned `0.1.0`. `codeproof demo --out <temp>`
returned its documented intentional exit `1` and created `sample-manual.md`,
`sample-manual.pdf`, and `proof/index.html`; the proof contains `HOLD` and
`code.flow-changed`. A missing source path returns exit `2` and `Markdown
source not found: …`.

## Production, privacy, accessibility, and performance

`PLAYWRIGHT_BASE_URL=https://markdown-pdf-code-proof.sociobot.in npx
playwright test` passed all 13 live tests. It covers request-origin logging,
empty cookies/Web Storage, console/page errors, direct demo/reset, offline
reload in a dedicated context, service-worker update, desktop and 390 px
layout, keyboard skip navigation/focus, reduced motion, routes, 404, and axe.
Axe reported no serious or critical findings.

The cold demo flow made requests only to the product origin and left cookies,
localStorage, and sessionStorage empty. Headers include a self-only CSP with
header-delivered `frame-ancestors 'none'`, `nosniff`, HSTS, strict-origin
referrer policy, and restrictive permissions policy. HTML caches for 30
seconds and returned 304 with its ETag; hashed JS/CSS are immutable for a year;
`sw.js` is no-cache.

Every checked local/live SHA-256 digest matched for root, Privacy, Terms, 404,
service worker, robots, sitemap, hashed assets, and shipped artwork. Root,
Privacy, Terms, robots, sitemap, and service worker returned 200; an unknown
route returned the designed 404. All discovered site and GitHub links returned
200.

Production sizes: JavaScript 2,999 bytes raw / 1,289 gzip; CSS 11,251 bytes
raw / 3,360 gzip; fonts 0 bytes; hero WebP 210,844 bytes. Live mobile
Lighthouse scored Performance 98, Accessibility 100, Best Practices 100, and
SEO 100: FCP 1.0 s, LCP 1.8 s, TBT 150 ms, CLS 0.

This is a local CLI with static documentation: it has no server-side product
endpoint, sign-in, payment, tracking, AI feature, or product API. Rate-limit
and Entra checks are not applicable.

## Defects by severity

| Severity | Count | Defects |
| --- | ---: | --- |
| Critical | 0 | — |
| High | 0 | — |
| Medium | 0 | — |
| Low | 0 | — |

No product code was changed during verification.
