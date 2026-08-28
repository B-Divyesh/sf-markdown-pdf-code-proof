# Independent verification — FAIL

- Work order: `markdown-pdf-code-proof-verify-2`
- Candidate: `2d62bacb1f15c0ce6d14870c3d10b570cc07c76e`
- Repository/branch: `B-Divyesh/sf-markdown-pdf-code-proof` / `main`
- Live URL: <https://markdown-pdf-code-proof.sociobot.in>
- Verified: 2026-08-28 UTC
- Verdict: **FAIL** — the explicit renderer-sandboxing security constraint is not met. The deployed PWA service worker also fails to install in a fresh production browser context.

## Release blockers

### Critical — renderer subprocesses retain unrestricted network access

The researched brief explicitly requires renderer subprocesses to be sandboxed
and prohibits execution with network access. `cli/src/render.rs` clears selected
environment variables and uses an isolated working directory, but launches the
chosen program directly with the host `PATH`; it does not apply a network or OS
sandbox.

Fresh reproduction used the release binary and a local HTTP server bound to
`127.0.0.1:18888`:

```sh
target/release/codeproof check README.md \
  --engine-command '/bin/sh -c "curl -fsS http://127.0.0.1:18888/network-probe >/dev/null; exit 0" {input} {output}' \
  --timeout 5
```

The command returned its expected operational error because the probe renderer
did not make a PDF, but the server independently logged:

```text
"GET /network-probe HTTP/1.1" 404
```

Thus the renderer made a network request despite Code Proof's environment
cleanup. A malicious or compromised renderer (and a renderer that processes
untrusted document content) can reach the network and the host filesystem.
This is not the required sandbox and conflicts with the documented safety
claim. Use an enforceable OS/container sandbox with a denied network namespace
and a read-only, scoped input mount; add a regression probe that proves network
access is denied.

### Medium — production service worker installs then disappears

The product ships and registers `/sw.js`, so its offline behavior is a claimed
PWA feature. In a fresh Chromium context against the live URL, after waiting
five seconds following normal page load, `navigator.serviceWorker
.getRegistrations()` returned `[]`. A direct `navigator.serviceWorker.register
('/sw.js')` created a worker and a `code-proof-v1` cache, but after three
seconds all registration fields were gone, the cache had no entries, and
`getRegistration()` was false. The worker therefore never activates and cannot
serve an offline reload or update in production.

The checked-in local Playwright test passes because Vite preview does not
reproduce the production hosting/response-policy interaction. Diagnose the
live install failure, test it against production-equivalent headers, and add a
deployed-browser smoke check that verifies an active controller before testing
offline reload and updates.

## Passing evidence

### Clean checkout and quality gates

Verification used a fresh clone at exactly the candidate SHA; `git status` was
clean before QA. All available repository gates passed:

- `npm ci` completed with 0 npm audit vulnerabilities.
- `npm test` exited 0: 3 Rust unit tests, 10 Rust CLI integration tests, and 6
  Playwright site tests passed.
- `npm run typecheck` passed (`tsc --noEmit`).
- `npm run lint` passed (`cargo fmt --all -- --check` and `cargo clippy
  --workspace --all-targets -- -D warnings`).
- Exact production `npm run build` passed, producing
  `target/release/codeproof` and `dist/site/`.
- `cargo package --manifest-path cli/Cargo.toml --allow-dirty` packaged and
  verified `codeproof 0.1.0` (25.0 KiB compressed).

The packaged `.crate` was extracted and installed with `cargo install --path
<unpacked-crate> --root <clean-consumer-root> --locked --debug`. Its separate
consumer binary reported `codeproof 0.1.0`, showed the documented help, and
returned JSON, exit 1, and an HTML proof sheet for empty Markdown.

### CLI behavior

- The documented existing-PDF path, valid multiple named fragment destinations,
  duplicate/wrong destination failures, and unresolved named destination
  failure all passed their isolated integration regressions.
- The release binary showed useful `check --help` text and stable exits:
  empty source -> 1 plus `source.empty` and a proof sheet; unsupported engine
  -> 2; missing custom-command placeholders -> 2; malformed negative tolerance
  -> Clap input error 2.
- JSON includes a schema version, summary, structured findings, and stable
  error/warning counts.
- Pandoc is not installed in this environment, so the built-in adapter was not
  exercised against a real Pandoc renderer.

### Live deployment, privacy, accessibility, and budget checks

The live deployment **does match this candidate**. SHA-256 matched the fresh
production build for `/`, `/sw.js`, and the hashed JS/CSS; live root response
was HTTP 200, ETag `"12809725"`, last modified `2026-08-28 01:49:03 UTC`.

- Response policy: self-only CSP for default/img/style/script/connect,
  `object-src 'none'`, `base-uri 'self'`, and `frame-ancestors 'none'`; HSTS,
  `nosniff`, strict-origin referrer policy, and camera/microphone/geolocation
  Permissions-Policy were present. Hashed JS/CSS and WebP are immutable; HTML
  revalidates at 30 seconds; `/sw.js` is `no-cache`.
- Fresh desktop browser audit: title, `lang`, one h1, main landmark, skip link,
  meaningful alt text, zero console/page errors, and zero axe serious/critical
  findings. First Tab focused “Skip to content” with a `rgb(24, 78, 158) solid
  3px` outline; Enter moved focus to main. Clipboard-denied recovery announced
  an actionable select-the-command message. Requests were same-origin only.
- At 390x844, scroll width equalled client width (390); copy-install and
  proof-run paths remained available. With reduced motion, animation duration
  was `1e-05s` and replay completed immediately with the announced status.
- Built assets: JS 2,077 bytes (0.95 KiB gzip), CSS 10,175 bytes (3.16 KiB
  gzip), no font payload, and hero WebP 210,844 bytes. These are within the
  200 KiB JS, 50 KiB CSS, and 300 KiB image budgets.
- No analytics, third-party scripts, third-party fonts, or telemetry were
  observed in source or browser requests. The static privacy and terms pages
  are present.

Lighthouse 13.4.1 could not complete in this container because the supplied
Playwright Chromium crashes under Lighthouse (`Browser tab has unexpectedly
crashed`). No Lighthouse score is claimed; the browser and bundle evidence
above was collected successfully.

## Severity summary

| Severity | Count | Defect |
| --- | ---: | --- |
| Critical | 1 | Renderer subprocess can make network requests; no enforceable sandbox. |
| High | 0 | — |
| Medium | 1 | Live service-worker registration fails, so production offline/update is unavailable. |
| Low | 0 | — |

No product code was modified during verification.
