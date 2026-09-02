# Independent verification 8 — PASS

- Candidate commit: `a4f2784fce6a7722d22593eb3c8754762ca6b9d7`
- Repository / branch: `B-Divyesh/sf-markdown-pdf-code-proof`, `main`
- Live URL: <https://markdown-pdf-code-proof.sociobot.in>
- Verified: 2026-09-02 UTC
- Verdict: **PASS**

## Cold read and demo

A cold live visit returned HTTPS 200. The first screen says: “Catch PDF bugs
before release.” It then says it is for engineers and technical writers and
catches broken code, page overflow, and internal links in a final PDF. The
visible primary action is “Try it with sample data.” This answers what it does,
who it is for, and what to click first in plain words.

One click reached `/?demo=1#demo`, set the title to `Demo — Code Proof`, moved
focus to the sample result, showed the persistent “Demo — sample data, nothing
is saved” banner and Reset demo action, and displayed the bundled intentional
`code.flow-changed` HOLD result. The CLI `codeproof demo --out <temp>` likewise
returned the documented release-defect status 1 and wrote a self-contained HTML
proof sheet in its isolated output directory.

## Claims gate — 21 / 21 passed

`.factory/claims.json` is present and defines 21 claims over 18 unique exact
test commands. From the clean candidate checkout, after `npm ci`, every listed
command passed (shared commands were run once and satisfy each claim that names
them): all CLI release-contract checks; local file/output and input-integrity
checks; shell-free, network-denied, fail-closed and timed renderer checks;
JSON/HTML/exit-code checks; bundled demo; dedicated privacy and offline browser
claims; Rust 1.88; public Git install; and MIT licensing.

`npm run test:install` installed `codeproof` into a fresh temporary root from
the public repository at revision `a4f2784f` and ran `--version`.

## Local quality and CLI exercise

The following all passed:

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
cargo package --manifest-path cli/Cargo.toml --locked --no-verify
```

`npm test` passed the copy audit, Rust 1.88 check, 3 Rust unit tests, 22 CLI
integration tests, 13 Playwright tests, and MIT-license check. The exact
release build produced `target/release/codeproof` and `dist/site/`; formatting,
Clippy with warnings denied, and TypeScript type checking passed. Packaging
produced a 33.4 KiB compressed `codeproof` crate.

Manual public CLI checks against the release binary passed: `--help` documents
commands and the 0/1/2 exit contract; `demo --out <temp>` returned 1 and wrote
the expected HOLD proof with `code.flow-changed`; checking a missing Markdown
source returned 2 with a clear recovery error.

## Live production, privacy, accessibility, and performance

A fresh local production-site build byte-matched the live root HTML plus its
hashed JavaScript and CSS assets (SHA-256). Live desktop and 390 px mobile
Playwright checks found no console/page errors and no axe serious or critical
findings. Keyboard tabbing reaches the skip link and visible 3 px focus state;
the demo action is operable and reduced-motion mode is respected.

The cold demo request log contained only
`https://markdown-pdf-code-proof.sociobot.in`; cookies, localStorage, and
sessionStorage were empty. After service-worker control, the normal page and
direct demo path both reloaded offline and showed the offline state. There is
no product server endpoint, sign-in, payment, tracking, AI feature, or product
API, so rate-limit and Entra checks do not apply.

Response headers provide a self-only CSP (including header-delivered
`frame-ancestors 'none'`), HSTS, `nosniff`, strict-origin referrer policy, and a
restrictive permissions policy. HTML uses a 30-second revalidating cache;
hashed JS/CSS are one-year immutable and `sw.js` is no-cache. Root, Privacy,
Terms, 404, robots, sitemap, and all discovered internal/external links were
checked; internal routes are 200 except the intended unknown-route 404, and
GitHub links return 200.

Bundle budgets pass: JavaScript is 2,999 bytes raw / 1,291 gzip; CSS is 11,251
bytes raw / 3,349 gzip; no web fonts load; the 210,844-byte hero WebP remains
below the 300 KB mobile-image budget.

## Defects by severity

| Severity | Count | Defects |
| --- | ---: | --- |
| Critical | 0 | — |
| High | 0 | — |
| Medium | 0 | — |
| Low | 0 | — |

No product code was changed during this verification.
