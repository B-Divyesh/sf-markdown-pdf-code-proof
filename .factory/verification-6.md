# Independent verification 6 — PASS

- Candidate commit: `774749fdccdefcdc23607b4d7254061f9bf1a542`
- Repository / branch: `B-Divyesh/sf-markdown-pdf-code-proof`, `main`
- Live URL: <https://markdown-pdf-code-proof.sociobot.in>
- Verified: 2026-09-02 UTC
- Verdict: **PASS** — no release-blocking defects found. The live static
  artifact matches the candidate build byte for byte.

## First read and one-click demo

A cold Chromium visit at desktop and 390 × 844 px showed this in the first
viewport:

> Catch PDF bugs before release. For engineers and technical writers, Code
> Proof catches broken code, page overflow, and internal links in the final
> PDF. Try it with sample data.

This says what the product does, names engineers and technical writers, and
makes the first action explicit. Activating the primary action once opened
`/?demo=1#demo`, set the title to `Demo — Code Proof`, focused “Sample HOLD
report”, and displayed the persistent sample-data notice, Reset demo, install
path, expected `code.flow-changed` defect, workspace, and proof path. Reset
completed again. The page stored no cookies, localStorage, or sessionStorage.

## Claims gate: 20 / 20 passed

Every command in `.factory/claims.json` ran to assertion completion from the
clean candidate checkout. The two browser commands were rerun after the
lockfile install because a clean clone does not contain Vite or Playwright.

| Claim | Result | Evidence |
| --- | --- | --- |
| `single-line-wrap` | PASS | Exact CLI test rejected one source line painted on two PDF baselines. |
| `page-bounds` | PASS | Exact CLI test rejected all four MediaBox edges and a non-default CropBox edge. |
| `code-content` | PASS | Exact CLI test reported `code.content-missing`. |
| `internal-links` | PASS | Exact CLI test rejected a wrong PDF destination. |
| `syntax-color` | PASS | Exact CLI test warned normally and failed under `--deny-warnings`. |
| `existing-pdf` | PASS | Exact CLI test checked a supplied PDF and produced PASS evidence. |
| `local-cli-files` | PASS | The supplied paths were read and local `proof/index.html` was written. |
| `renderer-no-shell` | PASS | Exact CLI test passed placeholders as arguments without a shell. |
| `renderer-network` | PASS | Sandboxed renderer could not reach the loopback listener. |
| `renderer-fail-closed` | PASS | Forced sandbox setup failure returned 2 and did not start the renderer. |
| `renderer-timeout` | PASS | Two-second renderer stopped at the one-second deadline with exit 2. |
| `html-proof` | PASS | Successful existing-PDF check wrote a PASS proof sheet. |
| `json-report` | PASS | Valid fixture emitted JSON with `passed: true`. |
| `exit-codes` | PASS | Valid, defect, and operational fixtures returned 0, 1, and 2. |
| `sample-demo` | PASS | Bundled sample wrote an isolated HOLD proof with `code.flow-changed`. |
| `private-site` | PASS | Browser demo used only its origin and left all tracking stores empty. |
| `offline-reload` | PASS | Dedicated context reloaded the installed shell offline. |
| `rust-msrv` | PASS | Locked workspace compiled with Rust 1.88. |
| `install-from-git` | PASS | Empty install root built Git commit `774749fd` and ran `--version`. |
| `mit-license` | PASS | LICENSE, crate metadata, landing page, and Terms consistently identify MIT. |

## Clean checkout, package, and end-to-end CLI

The following passed:

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
cargo package --manifest-path cli/Cargo.toml --locked
```

`npm test` passed the Rust 1.88 gate, 3 unit tests, 21 CLI integration tests,
12 Playwright tests, and the license gate. Formatting and Clippy passed with
warnings denied. The production build created `target/release/codeproof` and
`dist/site/`. The crate packaged 15 files as 33.3 KiB compressed.

The packaged crate was installed with `cargo install --path
target/package/codeproof-0.1.0 --root <empty-root> --locked`. The installed
binary reported `codeproof 0.1.0`; its help documented commands, examples, and
exit codes. Its bundled demo returned the intentional exit 1 and wrote
`sample-manual.md`, `sample-manual.pdf`, and `proof/index.html`; the proof
contained HOLD and `code.flow-changed`.

The integration suite exercised normal existing-PDF inspection, malformed and
missing source recovery, missing/flattened/wrapped code, syntax warning policy,
wrong/duplicate/unresolved destinations, every page edge, custom renderer
argument handling, sandbox refusal, network denial, renderer timeout, JSON,
HTML, and exit codes.

## Live deployment, privacy, accessibility, and PWA behavior

Fresh SHA-256 comparisons matched for `/`, `/privacy/`, `/terms/`,
`/404.html`, both hashed assets, all three raster assets, the SVG favicon,
`robots.txt`, `sitemap.xml`, and `sw.js`. Root `index.html`, for example, was
`69a8c60a70acdd48509a1960a1b12f48a81747ccc6c0f76eda07d759210845ca`
both locally and live.

- A cold load and the complete demo flow requested only
  `https://markdown-pdf-code-proof.sociobot.in`. There were no failed requests,
  console errors, page errors, cookies, localStorage entries, or sessionStorage
  entries.
- Live headers include a self-only CSP with header-delivered
  `frame-ancestors 'none'`, `nosniff`, HSTS, strict-origin referrer policy, and
  a restrictive permissions policy. HTML revalidates after 30 seconds; hashed
  assets and art are immutable for one year; `sw.js` is `no-cache`; a matching
  root ETag returned 304.
- Desktop and 390 px mobile had no horizontal overflow or controls smaller
  than 44 × 44 CSS px. First Tab focused “Skip to content” with a visible
  3 px cobalt outline. Demo entry moved focus to its heading. Axe found no
  violations at all, including no serious or critical findings.
- Reduced motion produced `1e-05s` animation and transition durations with
  `scroll-behavior: auto`; the proof still completed.
- `/opt/fleet/lib/verify-url.sh` passed: HTTPS 200, title, `lang=en`, one H1,
  one main landmark, complete alt text, labeled buttons, and no console errors.
- The service worker was active as `code-proof-v4`; its update had no waiting
  worker. Offline reload retained the headline and showed the offline status.
- Every discovered same-origin and GitHub link resolved. Privacy, Terms, and
  the branded 404 returned the correct status/title and complete metadata.

## Performance and applicability

- Initial JavaScript: 3,003 bytes raw / 1,312 bytes gzip.
- CSS: 11,251 bytes raw / 3,371 bytes gzip.
- Fonts: 0 bytes. Hero WebP: 210,844 bytes.
- Lighthouse 13 mobile: Performance 99, Accessibility 100, Best Practices 100,
  SEO 100; FCP 1.0 s, LCP 1.8 s, TBT 0 ms, CLS 0, total transfer 214 KiB.

This is a static documentation site for a local CLI. It has no server-side
product or unlock endpoint, authentication, payment, analytics, or AI feature.
The API rate-limit and Entra checks are therefore not applicable. The brief
does not imply a useful AI step; deterministic local verification is the safer
and more relevant implementation.

## Defects by severity

| Severity | Count | Defects |
| --- | ---: | --- |
| Critical | 0 | — |
| High | 0 | — |
| Medium | 0 | — |
| Low | 0 | — |

No product code was changed during verification.
