# Independent verification 5 — PASS

- Candidate commit: `568d4cae10d24c7f3a08e1673e67bade51e46fe8`
- Repository / branch: `B-Divyesh/sf-markdown-pdf-code-proof`, `main`
- Live URL: <https://markdown-pdf-code-proof.sociobot.in>
- Verified: 2026-08-30 UTC
- Verdict: **PASS** — no release-blocking defects found. The current deployment
  is the candidate build, not the earlier deployment-only failure.

## First-read and demo acceptance

A cold Chromium visit showed, in the first viewport:

> Catch PDF bugs before release. For engineers and technical writers, Code
> Proof catches broken code, page overflow, and internal links in the final
> PDF. Try it with sample data.

This answers what it does, who it is for, and what to click first in plain
words. The primary action is one click away and opened `/?demo=1#demo`; it
showed the persistent “Demo — sample data, nothing is saved” banner and the
recorded `DEMO HOLD — 1 expected defect found` result. The reset control,
Start-for-real link, and reduced-motion completion state worked.

## Mandatory claims: 10 / 10 passed

All `.factory/claims.json` commands were run from this clean checkout before
the wider QA:

| Claim | Command | Result |
| --- | --- | --- |
| single-line-wrap | `cargo test --test cli wrapped_single_code_line_fails_the_release_contract -- --exact` | pass |
| page-bounds | `cargo test --test cli page_bounds_cover_every_media_and_crop_edge -- --exact` | pass |
| code-content | `cargo test --test cli missing_code_content_fails_the_release_contract -- --exact` | pass |
| internal-links | `cargo test --test cli wrong_pdf_destination_cannot_satisfy_a_fragment -- --exact` | pass |
| syntax-color | `cargo test --test cli missing_syntax_color_warns_and_respects_warning_policy -- --exact` | pass |
| renderer-network | `cargo test --test cli renderer_sandbox_denies_network_connections -- --exact` | pass |
| sample-demo | `cargo test --test cli demo_uses_bundled_sample_data_and_writes_an_isolated_proof -- --exact` | pass |
| private-site | `npm run test:site -- --grep @claim:private-site` | pass |
| offline-reload | `npm run test:site -- --grep @claim:offline-reload` | pass |
| rust-msrv | `npm run test:msrv` | pass |

## Local product and package QA

The following all passed:

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
cargo package --manifest-path cli/Cargo.toml --locked
```

`npm test` passed the Rust 1.88 metadata gate, 3 library unit tests, 18 CLI
integration tests, and 12 Playwright tests. Formatting and Clippy passed with
warnings denied. The exact build produced `target/release/codeproof` and
`dist/site/`.

I installed the packaged crate into a clean temporary consumer with:

```sh
cargo install --path target/package/codeproof-0.1.0 --root <clean-root> --locked
```

The installed binary reported `codeproof 0.1.0`; `--help` exposed the documented
CI contract. `codeproof demo --out <clean-root>/demo` returned the intentional
exit `1`, created `sample-manual.md`, `sample-manual.pdf`, and
`proof/index.html`, and the proof contained `HOLD` and `code.flow-changed`.
This exercises the published CLI's shipped, isolated demo end to end.

The integration suite additionally passed normal existing-PDF inspection,
wrong/duplicate/unresolved PDF destinations, missing code, flattened and
wrapped code flow, all MediaBox/CropBox edges, syntax-color warning policy,
invalid Markdown recovery, missing-source recovery, renderer command safety,
timeout behavior, and Linux renderer network denial.

## Live deployment, privacy, accessibility, and performance

Fresh live byte comparisons were identical for `/`, `/privacy/`, `/terms/`,
`/404.html`, `/sw.js`, both hashed assets, artwork, favicon, `robots.txt`, and
`sitemap.xml`. For example, root `index.html` was
`6e4bcd9371d3b527607114fa7efad4cc90ba6d9b7901b50cc42180b48e4b46c3`
locally and live. The primary document had HTTPS 200 and the candidate asset
references; no stale deployment was observed.

- The cold desktop request log contained only
  `https://markdown-pdf-code-proof.sociobot.in`; cookies, localStorage, and
  sessionStorage were empty. There were no console errors, page errors, or
  failed requests.
- Live headers include a self-only CSP, `X-Content-Type-Options: nosniff`,
  HSTS, strict-origin referrer policy, and a restrictive permissions policy.
  HTML revalidates after 30 seconds; hashed JS/CSS and WebP are immutable for a
  year; `sw.js` is `no-cache`; a matching ETag returned 304.
- Desktop and 390 px mobile had one `h1`, one `main`, `lang=en`, an informative
  image alt, no horizontal overflow, and no Axe serious or critical issues.
  First Tab landed on Skip to content with a visible `rgb(24, 78, 158)` 3 px
  outline. All visible mobile controls measured at least 44 by 44 CSS px.
  Reduced motion yielded 0.01 ms-style (`1e-05s`) animation/transition durations
  and `scroll-behavior: auto` while the demo still completed.
- `/opt/fleet/lib/verify-url.sh` passed against the live URL: HTTPS 200,
  title, `lang=en`, one h1/main, no missing alt, no unlabeled button, and no
  browser errors.
- The live service worker was active as `code-proof-v3`; an update created no
  waiting worker. Offline reload retained the headline and displayed “Offline.
  The docs and recorded proof still work.”
- Bundle measurements: JS 2,151 bytes (1.00 KiB gzip), CSS 10,897 bytes
  (3.27 KiB gzip), no font payload, and hero WebP 210,844 bytes. These meet the
  stated budgets. Live Lighthouse 13 mobile scored Performance 97,
  Accessibility 100, Best Practices 100, and SEO 100; FCP 1.0 s, LCP 1.8 s,
  TBT 180 ms, CLS 0, total transfer 214 KiB.

This is a static CLI documentation site plus a local binary. It has no
server-side product API, sign-in, payment, or product-unlock endpoint; rate
limit and Entra tenant checks are therefore not applicable.

## Defects by severity

| Severity | Count | Defects |
| --- | ---: | --- |
| Critical | 0 | — |
| High | 0 | — |
| Medium | 0 | — |
| Low | 0 | — |

No product code was changed during this verification.
