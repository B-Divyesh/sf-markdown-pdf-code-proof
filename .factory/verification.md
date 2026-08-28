# Independent verification — FAIL

- Candidate: `572a95823cd4f2c659da207a9aed378aa2a2094d`
- Repository/branch: `B-Divyesh/sf-markdown-pdf-code-proof`, `main`
- Live URL: https://markdown-pdf-code-proof.sociobot.in
- Verified: 2026-08-28 UTC
- Verdict: **FAIL** — one critical false-pass in the CLI's primary internal-link proof contract.

## Release-blocking defect

### Critical — internal PDF link destinations are not validated

The brief requires that every Markdown fragment link resolve to its heading and
be represented in the final PDF. The candidate only counts PDF `Link`/`GoTo`
annotations; it does not compare each annotation destination with the Markdown
fragment or resolve that PDF destination.

Fresh reproduction used a Markdown source with headings `# Guide` and
`# Second`, links `[first](#guide)` and `[second](#second)`, and one Rust
fence. I generated a syntactically valid one-page PDF with two internal `Link`
annotations, both with `/Dest /guide` (therefore the `#second` contract is
broken; neither named destination was resolved). The release binary reported:

```json
{
  "summary": {
    "passed": true,
    "errors": 0,
    "internal_links": 2,
    "pdf_link_annotations": 2
  },
  "findings": []
}
```

and exited `0`, writing a `PASS` proof sheet. This is a release-blocking false
negative for the exact failure mode Code Proof is meant to catch. The check
must map each Markdown fragment to a corresponding, resolvable PDF named
destination (not merely compare annotation totals), with regression fixtures
for duplicate/wrong/missing destinations.

## Passing evidence

### Clean checkout, quality gates, and package

The worktree began clean at the candidate SHA.

- `npm ci` completed with 0 npm audit vulnerabilities.
- `npm test` passed: 3 Rust unit tests, 6 Rust CLI integration tests, and 6
  Playwright site tests.
- Exact production build `npm run build` succeeded, producing
  `target/release/codeproof` and `dist/site/`.
- `cargo fmt --all -- --check` and `cargo clippy --workspace --all-targets --
  -D warnings` passed.
- The repository has no configured TypeScript typecheck/lint script or
  `tsconfig.json`; `npx tsc --noEmit` consequently printed its help rather
  than checking sources. Vite build succeeded.
- `cargo package --manifest-path cli/Cargo.toml --allow-dirty` packaged and
  verified `codeproof 0.1.0` (23.4 KiB compressed).
- A clean temporary consumer installed that packaged crate using `cargo
  install --path target/package/codeproof-0.1.0 --root <temp>`. Its installed
  `codeproof 0.1.0` binary produced JSON and an HTML proof sheet, and returned
  exit `2` with `Markdown source not found` for a missing source.

### CLI end-to-end and recovery paths

- Normal existing-PDF flow: generated proof sheet and JSON from the installed
  package. (It is the false-pass fixture above, so it demonstrates the
  executable path but is not acceptance evidence for link correctness.)
- Empty Markdown: exit `1`, `source.empty`, and a recoverable HTML proof
  sheet.
- Unclosed JavaScript fence: exit `1`, `source.unclosed-fence`, and a proof
  sheet.
- Unsupported engine: exit `2` with actionable error.
- Direct custom-renderer timeout (`--timeout 1`): exit `2`, `renderer exceeded
  the 1 second timeout`.
- `--help`, JSON output, stable exit modes, direct (non-shell) custom command
  execution, and an existing-PDF success path are also covered by the passing
  CLI integration tests.
- Pandoc was not installed in this environment, so the built-in Pandoc adapter
  could not be exercised against a real renderer.

### Live deployment, privacy, browser, and PWA checks

The live deployment is not the earlier reported deployment failure. It served
HTTP 200 and was byte-identical to this candidate's fresh build for `/`,
`/privacy/`, `/terms/`, `/sw.js`, the hashed JS/CSS, and the WebP artwork.

- Browser audit at desktop and 390×844: one `h1`, `main`, `lang=en`, title,
  meaningful image alt text, no horizontal mobile overflow, and primary copy/
  proof paths available.
- Keyboard audit: first Tab reaches “Skip to content”; it has a visible
  `rgb(24, 78, 158) solid 3px` focus outline and Enter moves to `#main`.
- Clipboard-denied recovery announces: “Copy unavailable. Select this command:
  cargo install --path cli”.
- Reduced-motion audit: animation duration is `0.01s`; the proof replay still
  reaches its complete status. Axe found 0 serious/critical findings. Browser
  console and page errors were both empty.
- Browser network requests stayed same-origin only. Source inspection and the
  live page show no analytics, telemetry, third-party fonts, or third-party
  scripts. The CLI only runs the selected renderer; its built-in Pandoc command
  disables raw HTML.
- The passing site suite installed the service worker and verified offline
  reload. Live `/sw.js` is byte-identical and served `Cache-Control: no-cache`.
- Live headers: CSP restricts default/img/style/script/connect to `'self'` and
  sets `object-src 'none'`, `base-uri 'self'`, `frame-ancestors 'none'`;
  HSTS, `nosniff`, strict-origin referrer policy, and camera/microphone/
  geolocation permissions policy are present. Hashed assets and the 206 KiB
  WebP are immutable; HTML is revalidated every 30 seconds.

### Performance and visual QA

- Production assets: 2.08 KiB JS (0.95 KiB gzip), 10.18 KiB CSS (3.16 KiB
  gzip), no font payload, and a 210,844-byte hero image. All are within the
  200 KiB JS / 50 KiB CSS / 300 KiB hero budgets.
- Fresh mobile Lighthouse against the live URL: Performance **96**,
  Accessibility **100**, LCP **1.8 s**, CLS **0**, total transfer **214 KiB**.
- Visual inspection at desktop and 390px confirms the documented risograph
  system, legible hierarchy, responsive stacking, and no generic framework
  appearance.

## Severity summary

| Severity | Count | Defect |
| --- | ---: | --- |
| Critical | 1 | PDF link annotations are count-only; wrong/unresolvable destinations pass. |
| High | 0 | — |
| Medium | 0 | — |
| Low | 0 | — |

No product code was modified during this verification. The documentation
changes in this commit record the independent result.
