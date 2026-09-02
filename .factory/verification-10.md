# Independent verification 10 — PASS

- Candidate commit: `0fff412476781d63482d2d540adc8de2caea8c94`
- Repository / branch: `B-Divyesh/sf-markdown-pdf-code-proof`, `main`
- Live URL: <https://markdown-pdf-code-proof.sociobot.in>
- Verified: 2026-09-02 UTC
- Verdict: **PASS**

The candidate is releasable. The repaired PDF checks cover the earlier false
negative cases, the complete installed claims gate passes, the crate works in
a clean consumer, and production byte-matches the candidate build. No Critical,
High, or Medium defects were found.

## Mandatory first read and demo

The cold first screen passes on desktop and at 390 px. “Catch PDF bugs before
release” says what the product does. The next sentence names engineers and
technical writers and identifies broken code, page overflow, and internal
links. “Try it with sample data” is visible in the first viewport, with “See a
sample PDF defect and failed check” beside it.

One keyboard-activated click opened `/?demo=1#demo`, changed the title to
“Demo — Code Proof”, focused “Sample failed release check”, showed the
persistent “Demo — sample data, nothing is saved” banner, and reproduced the
real CLI result:

```text
DEMO HOLD — do not release — 1 expected defect found
Error [code.flow-changed] Code fence on line 7 is present but its line flow changed
```

Reset demo worked with Space and retained focus. View install commands left
demo mode. Cookies, localStorage, and sessionStorage remained empty.

## Claims gate — 25 / 25 passed

`.factory/claims.json` exists and contains 25 entries. After the clean-clone
dependency bootstrap (`npm ci`), every declared `test` command was run. Each
passed, including repeated commands where more than one claim maps to the same
fixture:

- PDF contract: `single-line-wrap`, `code-lines-merge`, `page-bounds`,
  `code-content`, `internal-links`, `syntax-color`, `font-metrics`, and
  `heading-fragments`.
- CLI behavior and safety: `existing-pdf`, `local-cli-files`,
  `input-unchanged`, `renderer-no-shell`, `renderer-network`,
  `renderer-fail-closed`, `renderer-timeout`, `html-proof`, `json-report`,
  `exit-codes`, and `sample-demo`.
- Site/package: `demo-transcript`, `private-site`, `offline-reload`,
  `rust-msrv`, `install-from-git`, and `mit-license`.

The Git-install claim resolved revision `0fff4124`. The Rust 1.88 check compiled
all locked dependencies. The exact browser claim commands were also rerun
individually after installation and each passed.

## Local gates and production build

```text
npm ci                                      PASS (23 packages, 0 vulnerabilities)
npm test                                    PASS
  Rust unit tests                           4 passed
  Rust CLI integration tests               26 passed
  Playwright site tests                     19 passed
npm run typecheck                           PASS
npm run lint                                PASS (fmt + clippy -D warnings)
npm run build                               PASS
cargo package --manifest-path cli/Cargo.toml --locked
                                            PASS (15 files, 38.5 KiB compressed)
```

The exact build created `target/release/codeproof` and `dist/site`. The site
build contains 4,320 bytes of JavaScript and 11,262 bytes of CSS uncompressed;
there are no web-font files. The 210,844-byte hero WebP is within the 300 KB
mobile budget.

## Clean-consumer CLI exercise

The generated `.crate` was unpacked into a new temporary consumer and installed
with `cargo install --path ... --locked`. The installed binary printed
`codeproof 0.1.0` and exposed useful command, option, example, and exit-code
help.

- Bundled demo: exit 1, one expected `code.flow-changed` defect, isolated
  Markdown/PDF files, and a non-empty HOLD HTML proof sheet.
- Normal existing-PDF case: a corrected two-line fence checked against the
  bundled PDF; exit 0, JSON `passed: true`, one page, one fence, one internal
  link, one PDF annotation, and a non-empty PASS proof sheet.
- Source integrity: SHA-256 remained
  `65e4e28669be425b951d1e6ebec5f61c3ffa68c162e32b3c38a5f1facb8abaa0`
  before and after checking.
- Empty Markdown: exit 1 with `source.empty` and a HOLD proof sheet.
- Missing Markdown: exit 2 with the missing path and corrective context.
- Non-UTF-8 Markdown: exit 2 with a UTF-8 read error.
- Malformed PDF: exit 2 with `invalid file header`.
- Negative overflow tolerance: exit 2 with a finite, non-negative requirement.
- Renderer command without both placeholders: exit 2 with the exact contract.
- Renderer exceeding `--timeout 1`: exit 2 after the one-second deadline.

The prior verification 9 regressions are now exact passing tests: unrelated
blue graphics cannot mask black code, Helvetica glyph widths detect real
overflow, embedded widths/text matrices drive geometry, and Setext/Pandoc
explicit heading IDs resolve correctly.

## Live deployment, privacy, and accessibility

`/opt/fleet/lib/verify-url.sh` passed production: HTTPS 200 in 891 ms, correct
title and `lang=en`, one H1, a main landmark, no missing alt text, no unlabeled
buttons, and no console errors.

- Desktop, demo, 390 px, Privacy, Terms, and branded 404 axe scans: zero
  serious or critical findings.
- Keyboard: the skip link is the first stop, has a 3 px cobalt outline with
  4 px offset, and Enter focuses `main`. The primary demo path is reachable
  and operable by keyboard.
- Mobile: no horizontal overflow at 390 px; every visible link and button is
  at least 44×44 CSS px. A 200% root text-size check retained the full layout,
  primary action, and footer without horizontal overflow.
- Reduced motion: the media query matches, scrolling becomes `auto`, and no
  element retains an animation or transition longer than 1 ms.
- Normal routes produce no console errors, page errors, or failed requests.
  The deliberate 404 navigation produces only the browser's expected failed
  document-resource console message.
- Every discovered internal and GitHub link returned 200. `/`, `/privacy/`,
  and `/terms/` return 200; an unknown route returns the designed 404 page.
- A browser-context request log covering first load, service-worker install,
  demo entry, and reset contained only
  `https://markdown-pdf-code-proof.sociobot.in`. No third-party font, script,
  image, analytics, or tracking request occurred.

Response headers include HSTS, `nosniff`, strict-origin referrer policy, a
restrictive permissions policy, and a self-only CSP with header-delivered
`frame-ancestors 'none'`. HTML revalidates after 30 seconds; hashed JS/CSS use
one-year immutable caching; conditional requests return 304; `sw.js` uses
`no-cache`.

The service worker activated and controlled the page with cache
`code-proof-v5`. `registration.update()` completed with no waiting or
installing worker. After the browser went offline, reload returned the cached
page with the correct title/H1 and visible offline status.

There is no backend endpoint, product-unlock call, authentication, payment,
analytics, AI feature, or server-side state. Rate-limit, concurrency,
persistence, and Entra checks are not applicable.

## Performance

Fresh Lighthouse mobile results against production:

| Category / metric | Result |
| --- | ---: |
| Performance | 98 |
| Accessibility | 100 |
| Best Practices | 100 |
| SEO | 100 |
| FCP | 1.1 s |
| LCP | 2.0 s |
| TBT | 130 ms |
| CLS | 0 |
| Initial transfer | 215 KiB |

## Deployment identity

The deployed root, Privacy, Terms, 404, service worker, hashed JavaScript and
CSS, hero/social images, touch icon, SVG mark, robots file, and sitemap all
byte-match `dist/site` built from the candidate. Root `index.html` SHA-256 is
`ab16dcc89f47fabae535fa07e91276b81ea6a3dbd6e320934ddf6f3225e705ff`
both locally and live.

## Defects by severity

| Severity | Count | Defects |
| --- | ---: | --- |
| Critical | 0 | — |
| High | 0 | — |
| Medium | 0 | — |
| Low | 1 | CP-V10-01 |

### CP-V10-01 — Low — Stable-name images are cached as immutable

`code-proof-press.webp`, `code-proof-social.jpg`, and
`apple-touch-icon.png` have stable URLs but receive
`Cache-Control: public, max-age=31536000, immutable`. A future deployment that
changes one of those files without changing its URL can leave returning users
with stale art for up to one year, including during service-worker cache
replacement. Content-hash these filenames or give stable-name assets a
revalidating cache policy.

No product code was changed during verification.
