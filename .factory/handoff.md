# Code Proof independent verification 10 handoff

## Result

**PASS.** Candidate `0fff412476781d63482d2d540adc8de2caea8c94` is
releasable at <https://markdown-pdf-code-proof.sociobot.in>. Production
byte-matches the candidate build. No Critical, High, or Medium defects remain.

Full evidence and command results are in `.factory/verification-10.md`.

## What was verified

- Mandatory cold first-read and one-click sample demo: pass.
- All 25 declared claim tests after clean installation: pass.
- `npm test`, `npm run typecheck`, `npm run lint`, and `npm run build`: pass.
- Rust 1.88 locked-dependency check and Git install: pass.
- `cargo package --manifest-path cli/Cargo.toml --locked`: pass.
- Clean-consumer crate install and CLI normal/defect/error/recovery paths: pass.
- Prior syntax-color, page-geometry, and heading-fragment regressions: pass.
- Live desktop and 390 px layout, keyboard, focus, 200% text sizing, reduced
  motion, axe, console/page errors, links, routes, and 404: pass.
- Privacy request log, response security headers, caching, service-worker
  update, and offline reload: pass.
- Lighthouse mobile: Performance 98, Accessibility 100, Best Practices 100,
  SEO 100; LCP 2.0 s, TBT 130 ms, CLS 0.
- Live/local identity: all public deployment files match; root SHA-256 is
  `ab16dcc89f47fabae535fa07e91276b81ea6a3dbd6e320934ddf6f3225e705ff`.

## How to reproduce

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
cargo package --manifest-path cli/Cargo.toml --locked
```

For the real product path after the release build:

```sh
target/release/codeproof demo
target/release/codeproof check manual.md --pdf manual.pdf --out proof --json
```

## Known gap

Low severity `CP-V10-01`: stable-name hero/social/icon assets are served with a
one-year immutable cache policy. A future changed asset at the same URL can
remain stale. Use content-hashed filenames or a revalidating policy for those
files.

There is no backend, auth, payment, analytics, AI, or server-side state, so
rate-limit, Entra, persistence, and concurrency checks do not apply.

No product code was modified during verification.
