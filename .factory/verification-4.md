# Independent verification 4 — FAIL

- Work order: `markdown-pdf-code-proof-verify-4`
- Candidate: `648c8eae0e768dffdc358925b109d28b50c37a3e`
- Repository/branch: `B-Divyesh/sf-markdown-pdf-code-proof` / `main`
- Live URL: <https://markdown-pdf-code-proof.sociobot.in>
- Verified: 2026-08-28 UTC
- Verdict: **FAIL** — the deployment is current and healthy, but the packaged CLI gives a clean release PASS to a one-line fenced command that visibly wraps onto two PDF lines. It also misses text outside the left and top page bounds while claiming to check the full media/crop box.

The candidate and `origin/main` both resolved to the exact SHA above. Verification
ran from a separate clean detached clone. No product source was changed.

## Release blockers

### Critical — a wrapped one-line fence receives a clean PASS

The researched job is to catch wrapping in code-heavy Markdown manuals. The
installed package false-passes that exact defect when a fenced block has one
non-empty source line.

I generated an independent one-page PDF with a 612×792pt media box, a valid
`#guide` named destination and matching link annotation, and a non-default text
color. The Markdown fence contained one source line:

```javascript
const endpoint = "https://example.test/api"; return endpoint;
```

The PDF painted that line as two separate text runs on distinct baselines:

```text
y=700: const endpoint = "https://example.test/api";
y=682: return endpoint;
```

Using the binary installed from the packaged `.crate`:

```sh
codeproof check wrapped-single.md \
  --pdf wrapped-single.pdf --out proof-wrapped --json
```

returned exit `0`, emitted no finding, and wrote a proof sheet titled `PASS —
Code Proof report` containing “No defects found.” The JSON was:

```json
{
  "summary": {
    "passed": true,
    "errors": 0,
    "warnings": 0,
    "info": 0,
    "pages": 1,
    "code_blocks": 1,
    "internal_links": 1,
    "pdf_link_annotations": 1
  },
  "findings": []
}
```

The fixture hashes were:

```text
bbe1fab675de79f951d01b48c76e6ead790a26f061147c1d90ce3765cb71a15f  wrapped-single.pdf
e17504ae65dd5102a36446ac8995e67c6824c5cd220d7f99f6268b15d8fe36a0  wrapped-single.md
```

The cause is visible at `cli/src/pdf.rs:85-114`: after normalized content is
found, `preserves_line_flow` is called only under `useful.len() > 1`. A
single-line fence therefore cannot produce `code.flow-changed`, even when the
PDF baseline evidence proves it wrapped. This defeats a primary acceptance
case and can approve a release-breaking artifact.

For control evidence, the repaired inverse case works: a two-line source fence
flattened into one `Tj` operation returned exit `1`, `code.flow-changed`, and a
`HOLD` proof; the same two lines on separate baselines passed.

### High — page geometry ignores the left, top, and bottom bounds

The README promises that “painted text stays within the page media/crop box.”
Independent 612×792pt fixtures put the first code line at x=-30pt (left of the
media box) and at y=820pt (above it). Both retained valid content, link, and
color evidence. Both commands returned exit `0`, zero findings, and `PASS`.

`cli/src/pdf.rs:517-573` tracks only `text_x + estimated_advance` and compares it
to page width. It does not check the left edge or any vertical coordinate. The
right-edge control at x=590pt correctly returned exit `1` with
`geometry.text-overflow`, confirming that the independent fixtures exercised
the intended geometry path rather than bypassing inspection.

## Clean checkout, build, and package evidence

Environment: Node 22.23.2, npm 10.9.8, default Rust 1.98.0; Rust 1.88.0 was
installed separately to verify the declared minimum.

| Gate | Result |
| --- | --- |
| `npm ci` | PASS — 22 packages installed, 23 audited, 0 vulnerabilities |
| `npm test` | PASS — MSRV metadata gate, 3 Rust unit tests, 13 CLI integration tests, 9 Playwright tests |
| `npm run typecheck` | PASS — `tsc --noEmit` |
| `npm run lint` | PASS — rustfmt plus Clippy for all targets with warnings denied |
| `npm run build` | PASS — exact release binary and `dist/site/` production site |
| `cargo package --manifest-path cli/Cargo.toml --locked` | PASS — 13 files, 109.9KiB unpacked / 29.3KiB compressed |
| packaged source `cargo +1.88.0 check --locked` | PASS |
| clean consumer install | PASS — `codeproof 0.1.0`, documented help and stable exit surface |
| clean Rust API consumer under 1.88 | PASS — `parse_markdown` parsed one fence and one internal link |

Release artifacts and budgets:

- `target/release/codeproof`: 2,008,632 bytes.
- initial JavaScript: 2,077 bytes / 0.95KiB gzip (budget ≤200KiB).
- CSS: 10,213 bytes / 3.17KiB gzip (budget ≤50KiB).
- fonts: 0 bytes (budget ≤120KiB).
- hero WebP: 210,844 bytes (budget ≤300KiB).

## Independent CLI matrix

All cases used the binary installed into an isolated consumer root from the
fresh `.crate`, not the repository target binary.

| Case | Result |
| --- | --- |
| valid destination, annotation, two-line code flow, color | exit 0, PASS, proof HTML and JSON |
| two source lines flattened into one PDF line | exit 1, `code.flow-changed` |
| **one source line wrapped onto two PDF baselines** | **exit 0, false PASS — critical defect above** |
| missing fenced-code line | exit 1, `code.content-missing` |
| right-edge overflow | exit 1, `geometry.text-overflow` |
| left-edge or top-edge overflow | exit 0, false PASS — high defect above |
| wrong link destination | exit 1, `link.destination-missing` |
| no color operators | warning/exit 0 by default; exit 1 with `--deny-warnings` |
| empty source or unclosed fence | exit 1 with actionable finding and HTML proof |
| missing source/PDF or corrupt PDF | actionable exit 2 |
| negative/NaN tolerance, zero timeout | actionable exit 2 |
| missing renderer placeholders, unsupported engine | actionable exit 2 |
| proof output path already a file | actionable exit 2 |
| custom renderer copying a valid fixture | exit 0 with `engine: custom` |
| custom renderer exceeding one second | terminated with actionable exit 2 |
| absent Pandoc executable | actionable exit 2 |

The repository's real loopback sandbox integration passed: a renderer could not
reach the listening socket. Custom commands use fixed arguments rather than a
shell, Landlock/seccomp setup is fail-closed, and the timeout path stopped the
renderer. Pandoc was unavailable, so a real Pandoc backend run was not possible.

Generated PASS and HOLD proof sheets were loaded at 390×844 in Chromium. Each
had one title/h1/main, no console errors, and zero Axe violations.

## Live deployment and candidate identity

The previously reported deployment-only failure is not present. Fresh
production output was compared byte-for-byte with the live site. Root,
privacy, terms, service worker, hashed JS/CSS, WebP, SVG, robots, and sitemap
all matched. Selected candidate SHA-256 values:

```text
65f4e2b1b896b04e76b836dc818d4e59241f2caeb5d97f6a2ff183a513ea5d01  index.html
8533f48417647b7b1d5e55cf27ac4e0e86800ec7cf5f79fe4dea3225362eff1f  assets/main-CswFYPEl.js
2a0451c6026ca3354dd86701b10bb37fd68b0910cf1cdc47beb06bb3eee4d0f7  assets/main-DsqZkCpc.css
bac203c6a12cf510dac820db814439b6f4744d1919752226a3b00ec6a90b5aac  sw.js
```

The candidate commit time is 04:18:41 UTC; live `Last-Modified` was 04:19:23
UTC. The factory `verify-url.sh` returned HTTPS 200 with the expected title,
`lang=en`, one h1/main, complete alt text, and no browser errors.

## Browser, accessibility, privacy, and PWA evidence

Fresh Chromium 145 contexts exercised 1440×900 and 390×844:

- zero Axe 4.10.2 violations at either viewport, hence zero serious/critical;
- zero console errors, page errors, or failed requests;
- one `<h1>`, one `<main>`, `lang=en`, viewport metadata, meaningful image alt,
  and no page-level horizontal overflow;
- first Tab focused “Skip to content” with a visible 3px cobalt outline; Enter
  focused `main`; all subsequent keyboard stops retained the designed outline;
- Space activated replay and its live region announced the completed result;
  denied clipboard access announced the full selectable command;
- all visible primary/navigation/button targets met 44px; “Terms” measured
  exactly 44×44px at 390px;
- reduced motion produced 0.01ms animations/transitions and `scroll-behavior:
  auto`, while replay still completed;
- visual inspection at both sizes found intentional stacking, readable content,
  and no clipped primary action;
- privacy and terms returned 200, each with one h1/main and zero serious/critical
  Axe findings.

First-load requests were same-origin only (HTML, hashed JS/CSS, and the hero
image). No cookies, localStorage, or sessionStorage were created. Source/dependency
inspection found no analytics, telemetry, third-party scripts, CDN fonts, or
runtime CLI network API. The only site fetch logic is the same-origin service
worker cache path.

The service worker activated and controlled the page, retained only
`code-proof-v2`, completed `registration.update()` with no waiting/installing
worker, and returned the page with HTTP 200 from a fresh offline reload while
showing the offline status.

## HTTP policy, caching, and performance

- HTTP redirects to HTTPS with 301; unknown paths and the deployment control
  file return 404.
- CSP restricts default, image, style, script, and connect sources to self and
  denies objects, foreign bases, and framing.
- HSTS, `nosniff`, strict-origin referrer policy, and camera/microphone/geolocation
  denial are present.
- HTML uses `public, must-revalidate, max-age=30`; hashed assets and the hero use
  one-year immutable caching; `/sw.js` is `no-cache`; conditional ETag requests
  returned 304.
- Lighthouse 13.0.1 mobile: Performance 100, Accessibility 100, Best Practices
  100, SEO 100; FCP 1.1s, LCP 1.8s, TBT 40ms, CLS 0, total transfer 214KiB.
  Synthetic Lighthouse does not expose INP.

## Severity summary

| Severity | Count | Defect |
| --- | ---: | --- |
| Critical | 1 | A one-line fenced command wrapped onto two PDF baselines receives exit 0 and a clean PASS proof. |
| High | 1 | Text outside the tested left/top page bounds is not inspected and receives PASS. |
| Medium | 0 | — |
| Low | 0 | — |

The candidate must not be released as verified. Extend flow comparison to every
non-empty fence, including the one-source-line/multiple-PDF-baseline case, and
check transformed text bounds against all four CropBox/MediaBox edges. Add both
independent cases as regressions, then repeat package and deployment verification.
