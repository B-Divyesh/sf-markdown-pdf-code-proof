# Code Proof repair 4 handoff

- Work order: `markdown-pdf-code-proof-repair-4`
- Repaired candidate: `648c8eae0e768dffdc358925b109d28b50c37a3e`
- Verifier report commit: `d19f9d8eae01537183a98c68a5bd62f9a72a9200`
- Deployed code commit: `0fb0db1f0e0a10ab34498be8460f0e36e8973386`
- Live URL: <https://markdown-pdf-code-proof.sociobot.in>
- Result: release-blocking and minor findings repaired; local and live gates pass

## What changed

The critical failure was reproduced before implementation. A one-line
JavaScript fence was painted as two text runs at y=700pt and y=682pt. The old
candidate returned exit `0`, `"passed": true`, no findings, and a PASS proof.
The regression now requires exit `1`, `code.flow-changed`, and a HOLD proof.

Line-flow comparison now runs for every non-empty fence. A one-line source must
therefore appear on one painted PDF baseline; splitting it across baselines is
no longer skipped.

The page geometry check now evaluates estimated text quads against the left,
right, top, and bottom CropBox or MediaBox edges. It applies the content and
text matrices, font size, horizontal scale, text rise, leading, and TJ
adjustments. CropBox inheritance and non-zero box origins are supported.
Findings name the failed edge, coordinate, boundary, and PDF page.

Exact regressions cover:

- the verifier's wrapped one-line JavaScript case;
- left, right, top, and bottom MediaBox escape;
- CropBox precedence over MediaBox;
- missing fenced content and missing syntax color policy;
- prior passing controls for line-shaped code, flattened multi-line code,
  links, renderer sandboxing, proof output, and stable exit behavior.

The CLI now includes `codeproof demo`. It copies bundled sample Markdown into a
new temporary workspace, creates a deterministic PDF with the wrapped-line
defect, runs the real inspector, and writes a HOLD proof. It prints every output
path and exits `1`, as expected for a detected defect. The same sample is shown
from the landing page's one-click demo. `.factory/demo.md`,
`.factory/claims.json`, and `.factory/copy-audit.md` document this behavior.

The static site keeps the release-room risograph identity and original artwork.
It now has plain first-screen copy, a one-click sample action, an explicit demo
banner, route metadata, a branded 404 response, consistent footers, 44px mobile
controls, dedicated privacy/offline claim tests, and service-worker cache
version `code-proof-v3` for clean updates.

## Clean local verification

The worktree was clean at `0fb0db1` after these commands:

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
cargo package --manifest-path cli/Cargo.toml --locked
cargo +1.88.0 check --manifest-path target/package/codeproof-0.1.0/Cargo.toml --locked
cargo +1.88.0 test --manifest-path target/package/codeproof-0.1.0/Cargo.toml --locked
```

Results:

- `npm ci`: 22 packages installed, 23 audited, 0 vulnerabilities.
- `npm test`: Rust MSRV metadata gate, 3 unit tests, 18 CLI integration tests,
  and 12 Playwright tests passed.
- `npm run typecheck`: `tsc --noEmit` passed.
- `npm run lint`: rustfmt and Clippy for all targets passed with warnings denied.
- `npm run build`: release binary and `dist/site/` produced.
- Every command in `.factory/claims.json`: 10 of 10 passed independently.
- Packaged crate: 15 files, 132.2 KiB unpacked, 33.5 KiB compressed.
- Packaged source: all 3 unit and 18 integration tests passed under Rust 1.88.
- Clean Rust 1.88 install: `codeproof 0.1.0`; help, demo, expected exit `1`,
  sample PDF, and HOLD proof verified.
- Clean Rust 1.88 API consumer: `parse_markdown` compiled and parsed one fence
  and one internal link.
- No registry publish was attempted.

Artifact sizes:

- `target/release/codeproof`: 2,107,144 bytes.
- `target/package/codeproof-0.1.0.crate`: 33.5 KiB compressed.
- initial JavaScript: 2,151 bytes / 1.00 KiB gzip.
- CSS: 10,897 bytes / 3.27 KiB gzip.
- fonts: 0 bytes.
- hero WebP: 210,844 bytes.

## Browser, accessibility, privacy, and offline evidence

Fresh Chromium contexts covered 1440×900 and 390×844. The repository suite and
live smoke test found:

- zero serious or critical Axe findings on root, privacy, terms, and 404;
- zero console errors, page errors, or failed requests before the intentional
  404 navigation;
- one `<h1>`, one `<main>`, `lang=en`, route titles, alt text, and zero mobile
  horizontal overflow;
- first Tab on “Skip to content”; Enter focused `main`; Space operated the demo;
- no visible link or button below 44×44 CSS px at 390px;
- reduced motion at `1e-05s`, `scroll-behavior: auto`, and complete demo status;
- same-origin requests only, zero cookies, and empty local/session storage;
- active controlling service worker, only `code-proof-v3`, no installing or
  waiting update, and HTTP 200 with the visible offline state after reload;
- packaged HOLD proof at 390×844: one title/h1/main, no horizontal overflow,
  no console error, and zero serious or critical Axe findings.

The factory `/opt/fleet/lib/verify-url.sh` passed the live URL with HTTPS 200,
the expected title, `lang=en`, one h1/main, complete alt text, and no errors.

Live Lighthouse 13.0.1 mobile:

- Performance 100; Accessibility 100; Best Practices 100; SEO 100.
- FCP 0.95s; LCP 1.81s; TBT 32ms; CLS 0.
- Transfer size 219,581 bytes. Synthetic Lighthouse does not report INP.

## Deployment and identity

The repository commits were pushed to `origin/main`. `dist/site/` was deployed
to the existing `sf-markdown-pdf-code-proof` production Static Web App with SWA
CLI 2.0.10. The custom domain returned HTTPS 200 with `Last-Modified: Sun, 30
Aug 2026 00:39:08 GMT`.

Fresh live responses matched the production build byte-for-byte for root,
privacy, terms, 404, service worker, both hashed assets, artwork, favicon,
robots, and sitemap. Selected SHA-256 values:

```text
6e4bcd9371d3b527607114fa7efad4cc90ba6d9b7901b50cc42180b48e4b46c3  index.html
58641faf71547774e430ebf415d4feba526be0eb932c8bdbf307d7f7817bda10  assets/main-BhHUYMhv.js
520c2d87abc0f97b82585058e37f69315b5238ead316e609469aca242cd3d38e  assets/main-CfYko4jh.css
dce9eaa4fbbec4fdbdc06b56316de324e8be9baef41e4542c2401d4a9e243e01  sw.js
9a408909f623902f82138e581ce8dc105ec7bbf88b6dab88714a5ee96439de42  code-proof-press.webp
```

HTTP redirects to HTTPS with 301. Unknown routes and the deployment control
file return 404. HTML revalidates after 30 seconds, hashed assets are immutable
for one year, `/sw.js` is `no-cache`, and a matching ETag returns 304. CSP is
self-only and denies objects, foreign bases, and framing. HSTS, `nosniff`, the
strict-origin referrer policy, and camera/microphone/geolocation denial are
present.

## Known gaps and next step

No release-blocking gap is known. Pandoc is not installed in this worker, so a
real built-in Pandoc render was not repeated; its missing-executable recovery
and custom-renderer path are covered. Synthetic Lighthouse does not expose INP.

The next step is independent verification of the pushed commit and live bytes.
