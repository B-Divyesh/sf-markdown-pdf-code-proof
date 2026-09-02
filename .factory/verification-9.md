# Independent verification 9 — FAIL

- Candidate commit: `4c5346a5740217724683957255bee8cb9c31fd1e`
- Repository / branch: `B-Divyesh/sf-markdown-pdf-code-proof`, `main`
- Live URL: <https://markdown-pdf-code-proof.sociobot.in>
- Verified: 2026-09-02 UTC
- Verdict: **FAIL**

The candidate is not releasable. The declared tests and all web quality gates
pass, but fresh adversarial PDF checks found two false-negative paths in the
core release verifier. A third defect rejects valid Markdown heading forms.

## Release-blocking findings

### CP-V9-01 — High — Unrelated page color hides unhighlighted code

The syntax-color check treats any non-black PDF color operator as evidence for
every language-tagged code fence. The color does not have to paint text, be
near the code, or occur on the same code run. In `cli/src/pdf.rs:682-685`, any
non-zero `rg`, `RG`, `k`, `K`, `sc`, `SC`, `scn`, or `SCN` operand sets one
document-wide `has_color` flag.

Fresh reproduction:

1. Create a one-page PDF containing a blue rectangle:
   `0 0 1 rg; 36 740 24 24 re f`.
2. Reset to black and paint the only code as black:
   `0 0 0 rg; /F1 12 Tf; 1 0 0 1 72 700 Tm; (fn main\(\) {}) Tj`.
3. Check matching Markdown containing a `rust` code fence.

Observed twice, with default warning policy and with `--deny-warnings`:

```text
exit 0
summary.passed = true
summary.warnings = 0
findings = []
```

Expected: `code.highlight-not-detected`; with `--deny-warnings`, exit 1. This
violates the `syntax-color` claim and the brief's requirement to flag
unhighlighted fenced blocks. A colored heading, link, logo, or decoration in a
normal manual can mask black code.

### CP-V9-02 — High — Page-bounds check misses real glyph overflow

The geometry check estimates every string width as
`byte_count × font_size × 0.58` (`cli/src/pdf.rs:801`) rather than using the
PDF font's glyph advances. This produces false passes and false failures for
proportional fonts.

Fresh false-negative reproduction used a 612 pt-wide page, standard Helvetica
at 12 pt, and `(WWWWWW) Tj` positioned at x=550. Helvetica's standard `W`
advance is 944/1000 em, so the painted right edge is:

```text
550 + 6 × 0.944 × 12 = 617.968 pt
```

That is 5.968 pt outside the page and 3.968 pt past the default 2 pt
tolerance. Code Proof instead estimates 591.76 pt and returned:

```text
exit 0
summary.passed = true
findings = []
```

Expected: exit 1 with `geometry.text-overflow`. This violates the
`page-bounds` claim and can allow clipped text into a released PDF.

### CP-V9-03 — Medium — Valid heading targets are rejected

The Markdown parser recognizes only ATX headings (`# Heading`) at
`cli/src/markdown.rs:122-129`. It does not recognize CommonMark Setext
headings, and it treats Pandoc-style `{#id}` attributes as heading text rather
than explicit IDs.

Two fresh documents were checked against the bundled PDF, which contains a
valid `retry-policy` named destination and link annotation:

- `Retry policy` followed by `------------` (Setext)
- `## Retry behavior {#retry-policy}` (explicit Pandoc ID)

Both returned exit 1 with `link.missing-source-target`, `pages: 0`, and
`pdf_link_annotations: 0`; the valid PDF was never inspected. Engineers must
rewrite valid manuals to the parser's undocumented subset.

## Mandatory first read and demo

The cold live first screen passes. It says “Catch PDF bugs before release,”
names engineers and technical writers, states that it checks broken code, page
overflow, and internal links, and presents “Try it with sample data” in the
first viewport. One click opens `/?demo=1#demo`, focuses the sample result,
shows the persistent “Demo — sample data, nothing is saved” banner, and plays
the real bundled CLI transcript.

The CLI demo returned the documented exit 1, reported
`code.flow-changed`, and wrote its sample Markdown, PDF, and HTML proof sheet
to an isolated temporary directory.

## Claims gate — 23 / 23 declared commands passed

After the required clean dependency install (`npm ci`), every `test` entry in
`.factory/claims.json` was run separately. Repeated commands were run again for
each claim. All passed:

`single-line-wrap`, `code-lines-merge`, `page-bounds`, `code-content`,
`internal-links`, `syntax-color`, `existing-pdf`, `local-cli-files`,
`input-unchanged`, `renderer-no-shell`, `renderer-network`,
`renderer-fail-closed`, `renderer-timeout`, `html-proof`, `json-report`,
`exit-codes`, `sample-demo`, `demo-transcript`, `private-site`,
`offline-reload`, `rust-msrv`, `install-from-git`, and `mit-license`.

The Git-install claim resolved and installed revision `4c5346a5`. The two high
findings above show that the current claim fixtures do not cover common
counterexamples to `syntax-color` and `page-bounds`.

## Local build, package, and CLI exercise

All declared repository gates passed:

```text
npm ci                                      PASS
npm test                                    PASS
  Rust unit tests                           3 passed
  Rust CLI integration tests               22 passed
  Playwright site tests                     19 passed
npm run typecheck                           PASS
npm run lint                                PASS
npm run build                               PASS
cargo package --manifest-path cli/Cargo.toml --locked  PASS
```

The exact production build created `target/release/codeproof` and `dist/site`.
The crate contained 15 files and was 33.5 KiB compressed. Installing the
packaged crate into a fresh consumer root succeeded; `codeproof --version`
printed `0.1.0`, and its demo produced the expected HOLD proof.

Independent normal and recovery paths also behaved correctly:

- A one-page Markdown/PDF pair with one code fence and one internal link
  returned exit 0, JSON `passed: true`, and a PASS HTML proof sheet.
- The Markdown SHA-256 was unchanged after checking.
- Empty Markdown returned exit 1 with `source.empty` and a HOLD proof.
- Missing Markdown, missing PDF, missing renderer placeholders, and `NaN`
  overflow tolerance returned exit 2 with actionable errors.

## Live site, privacy, accessibility, and PWA

The repository's 19 browser tests also passed directly against production.
Fresh browser evidence is under `evidence/verification-9-live/`; the three CLI
reproductions and captured JSON are under `evidence/verification-9-cli/`.

- Fleet URL verification: HTTPS 200 in 641 ms, correct title and `lang`, one
  H1, a main landmark, no missing alt text, and no console errors.
- Desktop and 390 px mobile: no horizontal overflow; every visible control was
  at least 44 px high and 44 px wide; the first Tab stop was the skip link;
  Enter moved focus to main; focus used a visible 3 px cobalt outline.
- Axe: zero serious or critical findings on desktop demo and mobile demo.
- Reduced motion: animation and transition durations reduced to `0.00001s`,
  with automatic scrolling disabled.
- Privacy: the complete landing/demo/offline request log contained only
  `https://markdown-pdf-code-proof.sociobot.in`; cookies, localStorage, and
  sessionStorage remained empty; no console or page errors occurred.
- Service worker: active and controlling, cache `code-proof-v5`; update found
  no waiting/installing worker; direct demo reloaded offline with its title,
  H1, and visible offline status.
- Routes and links: `/`, `/privacy/`, and `/terms/` returned 200 with one H1;
  the designed unknown route returned 404 and offered Return home; every
  discovered production link returned 200 and every fragment target existed.
- There is no backend endpoint, product-unlock call, authentication, payment,
  analytics, or AI feature. Rate-limit, persistence, concurrency, and Entra
  checks are therefore not applicable.

Response headers include a self-only CSP with header-delivered
`frame-ancestors 'none'`, HSTS, `nosniff`, strict-origin referrer policy, and a
restrictive permissions policy. HTML revalidates after 30 seconds; hashed JS,
CSS, and images are one-year immutable; `sw.js` is `no-cache`.

Fresh mobile Lighthouse scores were Performance 98, Accessibility 100, Best
Practices 100, and SEO 100. LCP was 1,974 ms, TBT 128 ms, and CLS 0. JavaScript
is 4,320 bytes raw / 1,830 gzip; CSS is 11,262 bytes raw / 3,382 gzip; no web
fonts load; the hero WebP is 210,844 bytes.

## Deployment identity

The local candidate build byte-matched the deployed root, Privacy, Terms,
404, hashed JavaScript and CSS, images, icons, robots, sitemap, and service
worker. Root `index.html` SHA-256 is
`ab16dcc89f47fabae535fa07e91276b81ea6a3dbd6e320934ddf6f3225e705ff`
both locally and live.

## Defects by severity

| Severity | Count | Defects |
| --- | ---: | --- |
| Critical | 0 | — |
| High | 2 | CP-V9-01, CP-V9-02 |
| Medium | 1 | CP-V9-03 |
| Low | 0 | — |

No product code was changed during verification.
