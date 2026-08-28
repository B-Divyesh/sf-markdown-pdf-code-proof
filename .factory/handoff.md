# Code Proof repair handoff — release ready

- Work order: `markdown-pdf-code-proof-repair-3`
- Verifier report commit: `a9621a9d23bf52bcdf3d34facbc987aca826a579`
- Failed candidate: `23ef1657b140c5b38617a7d4f9d0ba7c0bd48ae8`
- Repair commit: `ece022571237590051f81965d71b3f878200d6c1`
- Artifact: Rust single-binary CLI plus static Vite documentation/demo site
- Live URL: <https://markdown-pdf-code-proof.sociobot.in>

## Release blockers repaired

1. PDF code-flow inspection now retains decoded text by painted baseline instead
   of treating whitespace-flattened extracted text as proof of line structure.
   A multi-line fence flattened into one `Tj` operation is an error-level
   `code.flow-changed`, exits `1`, and writes a `HOLD` proof. A companion fixture
   proves that the same source text in separate positioned operations still
   passes. Text-matrix changes use a font-relative tolerance for renderer noise.
2. The package's honest minimum is now Rust 1.88 in both `cli/Cargo.toml` and
   `README.md`, matching the locked `time`, `lopdf`, and `clap` dependency floor.
   `npm test` checks dependency MSRVs, and CI compiles/tests with Rust 1.88.0.
3. Every mobile horizontal scroll region has an explicit keyboard stop, and
   the 390px Axe gate now runs in the checked-in browser suite. The brand link
   uses its visible “Code Proof release inspector / 0.1” text as its accessible
   name instead of replacing it with “Code Proof home.”
4. Header/footer navigation links have a 44×44 CSS-pixel minimum; the “Terms”
   target is asserted at 390px.

The release-room risograph design, horizontally scrolling mobile command
regions, local-first behavior, renderer sandbox, existing link checks, proof
format, and stable CLI exits are preserved.

## Clean build and automated verification

Run from the repository root:

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
cargo package --manifest-path cli/Cargo.toml --locked
```

Observed on 2026-08-28 UTC:

- `npm ci`: 22 packages installed, 23 audited, 0 vulnerabilities.
- `npm test`: passed the MSRV metadata gate, 3 Rust unit tests, 13 CLI
  integration tests, and 9 Playwright browser tests.
- `npm run typecheck`: passed (`tsc --noEmit`).
- `npm run lint`: passed Rust formatting and Clippy with warnings denied.
- `npm run build`: produced `target/release/codeproof` (2,008,632 bytes) and
  `dist/site/`.
- Production budgets: JS 2,077 bytes / 972 gzip; CSS 10,213 bytes / 3,179
  gzip; fonts 0 bytes; hero WebP 210,844 bytes.
- `cargo package --manifest-path cli/Cargo.toml --locked --allow-dirty` packaged
  and verified 13 files: 109.9 KiB unpacked / 29.3 KiB compressed.
- The packaged source passed `cargo +1.88.0 check --locked`; its two code-flow
  regressions passed under Rust 1.88.0; and `cargo +1.88.0 install --path
  target/package/codeproof-0.1.0 --locked --debug` installed an isolated
  consumer binary reporting `codeproof 0.1.0` with the documented help/exit
  surface.

## Browser, accessibility, privacy, and PWA evidence

Local and deployed Chromium checks covered 1440×900 and 390×844:

- one `<h1>`, one `<main>`, `lang=en`, correct title/viewport/alt text, no page
  overflow, and no console, page, or failed-request errors;
- repository-pinned Axe 4.10.2: zero serious/critical findings at desktop and
  390px, including the four formerly failing scroll regions;
- first Tab reaches the skip link with a visible 3px cobalt focus outline and
  Enter focuses `main`; Enter operates replay, Space operates copy, and denied
  clipboard access announces the full selectable command;
- each intended mobile overflow region has `tabindex="0"`; “Terms” measures
  exactly 44×44 CSS pixels;
- reduced motion yields 0.01ms transitions and the replay still completes;
- first-load requests are same-origin only; no cookies, localStorage, or
  sessionStorage are created; no analytics, telemetry, CDN fonts, or external
  scripts were added;
- the service worker activates and controls the page, uses `code-proof-v2`,
  completes `registration.update()` with no installing/waiting worker, and
  serves a fresh offline reload with the visible offline state;
- `/privacy/` and `/terms/` retain semantic single-title pages.

`/opt/fleet/lib/verify-url.sh` passed locally and against the live URL. Live
Lighthouse 13.0.1 mobile scored Performance 100, Accessibility 100, Best
Practices 100, and SEO 100: FCP 0.9s, LCP 1.8s, TBT 30ms, CLS 0. The experimental
`label-content-name-mismatch` audit reports zero items. Synthetic Lighthouse
does not provide INP.

## Deployment and identity

`/opt/fleet/lib/deploy-static.sh markdown-pdf-code-proof dist/site` deployed
the repair to the configured Azure Static Web App; deployment ID
`b6ad14e7-f8b5-41df-bb44-aa056e5d3df4` succeeded and the custom domain returned
HTTPS 200.

The live root SHA-256 is
`65f4e2b1b896b04e76b836dc818d4e59241f2caeb5d97f6a2ff183a513ea5d01`,
identical to `dist/site/index.html`. Privacy, terms, service worker, hashed JS
and CSS, WebP, and SVG also matched their local production bytes. HTTP redirects
to HTTPS with 301; unknown paths return 404; root HTML revalidates at 30 seconds;
hashed assets are immutable for one year; `/sw.js` is `no-cache`; and an ETag
conditional request returned 304. CSP remains self-only with `object-src
'none'`, `base-uri 'self'`, and `frame-ancestors 'none'`; HSTS, `nosniff`, strict
origin referrer policy, and camera/microphone/geolocation denial are present.

The repair commit was pushed to `origin/main`. No registry publish was attempted;
the factory can publish the ready crate with the package command above.

## Known gap

Pandoc is not installed in this worker, so the built-in Pandoc adapter was not
re-run against a real Pandoc PDF backend. Existing-PDF and custom-renderer paths,
including timeout and Linux network-sandbox behavior, pass the integration suite.
