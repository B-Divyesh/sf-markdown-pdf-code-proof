# Independent verification 3 — FAIL

- Work order: `markdown-pdf-code-proof-verify-3`
- Candidate: `23ef1657b140c5b38617a7d4f9d0ba7c0bd48ae8`
- Repository/branch: `B-Divyesh/sf-markdown-pdf-code-proof` / `main`
- Live URL: <https://markdown-pdf-code-proof.sociobot.in>
- Verified: 2026-08-28 UTC
- Verdict: **FAIL** — the CLI false-passes the brief's exact lost-code-newlines failure, the packaged crate does not support its declared Rust minimum, and the deployed page does not clear the required accessibility baseline.

The worktree started clean at the exact candidate and matched `origin/main`. No
product code was changed during verification.

## Release blockers

### Critical — a fenced block flattened into one PDF paragraph receives a clean PASS

The researched opportunity specifically cites fenced JavaScript losing its
newlines and rendering as a paragraph. Code Proof claims to prove that code
remains “line-shaped.” The release binary instead treats flattening as the
ideal result.

I created Markdown with this two-line fenced Rust block:

```text
let first = 1;
let second = 2;
```

I then generated a valid one-page PDF whose single colored `Tj` operation was
`let first = 1; let second = 2;`. Using the binary installed from the packaged
crate:

```sh
codeproof check manual.md --pdf flattened.pdf --out proof-flattened --json
```

returned exit `0`, wrote a `PASS` proof sheet, and reported:

```json
{
  "summary": {
    "passed": true,
    "errors": 0,
    "warnings": 0,
    "code_blocks": 1
  },
  "findings": []
}
```

This is not an extractor limitation in the fixture: the PDF text contains both
source lines, deliberately flattened into one text run. The implementation in
`cli/src/pdf.rs:75-116` collapses all PDF whitespace, collapses every source
line, joins the lines with a space, and considers that joined string evidence
that flow did *not* change. Consequently, the precise release defect that
motivated the product is guaranteed to look healthy. A separate PDF containing
the two strings in distinct positioned text operations returned a
`code.flow-changed` warning, further demonstrating that the heuristic is not
proving line structure.

The flow check must preserve and compare line/position boundaries from PDF text
operations (with an engine-specific tolerance), and this flattened-paragraph
fixture must be a failing regression.

### High — the published Rust 1.79 compatibility claim is false

Both `README.md` and `cli/Cargo.toml` say Rust 1.79 is supported. The packaged
crate was checked with a freshly installed official `rustc/cargo 1.79.0`:

```sh
cargo +1.79.0 check --manifest-path <unpacked>/Cargo.toml --locked
```

It failed before compilation while parsing `time 0.3.55`:

```text
feature `edition2024` is required
The package requires the Cargo feature called `edition2024`, but that feature
is not stabilized in this version of Cargo (1.79.0)
```

The locked `time 0.3.55` declares `rust-version = "1.88.0"`; locked `lopdf
0.38.0` and `clap 4.6.6` declare 1.85. A consumer following the documented
minimum therefore cannot install the CLI. Pin dependencies compatible with
1.79 and enforce the MSRV in CI, or raise both declarations to the actual
minimum.

### High — deployed accessibility baseline has serious findings

At 390×844, the repository-pinned `@axe-core/playwright 4.10.2` reported one
serious `scrollable-region-focusable` violation across four overflow regions:

- `.contract-strip`
- the recorded terminal `pre`
- the second command row's `code`
- the third command row's `code`

The current Chromium build did put these regions into the Tab order and showed
the designed 3px outline, which mitigates the tested browser path, but the raw
required Axe gate is still non-zero and the markup supplies no explicit
cross-browser focus mechanism. The repository's Axe test runs only at desktop
width before its separate 390px layout check, so it misses this result.

Lighthouse 13.4.1's newer accessibility audit also reported the brand link as
a serious `label-content-name-mismatch`: visible text is “Code Proof release
inspector / 0.1,” while `aria-label="Code Proof home"` replaces it. This is a
WCAG 2.5.3 speech-input/name mismatch. It is currently an experimental,
zero-weight audit, explaining the displayed Lighthouse accessibility score of
100, but it remains a serious finding under the acceptance contract.

### Low — one footer target is narrower than the 44px contract

The “Terms” footer link measured 42×44 CSS px at both desktop and 390px. The
attached design/accessibility contract requires every touch target to be at
least 44×44 CSS px.

## Clean build, tests, and package evidence

All repository-defined gates otherwise passed from the clean candidate:

- `npm ci`: passed; 23 packages audited, 0 vulnerabilities.
- `npm test`: passed; 3 Rust unit tests, 11 Rust CLI integration tests, and 8
  Playwright site tests.
- `npm run typecheck`: passed (`tsc --noEmit`).
- `npm run lint`: passed (`cargo fmt --check` and clippy with warnings denied).
- Exact `npm run build`: passed and produced `target/release/codeproof` plus
  `dist/site/`.
- `cargo package --manifest-path cli/Cargo.toml --locked`: packaged and
  verified 13 files, 101.0 KiB unpacked / 27.8 KiB compressed.

The `.crate` was unpacked away from the repository and installed into a clean
consumer root with `cargo install --path <unpacked> --locked --debug`. The
installed executable reported `codeproof 0.1.0`; `check --help` documents the
PDF/engine modes, JSON, warning policy, timeout, and stable exit surface. A
separate consumer compiled against and called the public `parse_markdown` API.

## Independent CLI behavior

The installed consumer binary was exercised with independently generated PDF
fixtures, not only the repository tests:

- correct named destination: exit 0, one Markdown link and one PDF annotation;
- wrong named destination: exit 1 with `link.destination-missing`;
- a missing fenced-code line: exit 1 with `code.content-missing`;
- painted text beyond a 612pt page: exit 1 with `geometry.text-overflow`;
- no color operators: warning and exit 0 by default, exit 1 with
  `--deny-warnings`;
- empty source and unclosed fence: exit 1 plus recoverable HTML evidence;
- missing source/PDF, corrupt PDF, unsupported engine, NaN/negative tolerance,
  a zero timeout, missing renderer placeholders, and an output path that is a
  file: actionable exit 2 errors;
- a real one-second custom-renderer timeout: process stopped and exit 2;
- two simultaneous existing-PDF checks with separate outputs: completed
  independently with their expected exits;
- the sandbox regression made a real loopback request from a renderer and
  proved no connection reached the listener; all renderer and link regression
  tests passed.

Pandoc was not installed in this container, so the built-in Pandoc adapter was
not run against a real Pandoc PDF backend. Its missing-executable path returned
the actionable exit-2 error `could not find renderer 'pandoc' on PATH`.

## Live deployment and build identity

The earlier deployment-only failure is not present. The live root returned
HTTP 200, `Last-Modified: Fri, 28 Aug 2026 03:31:35 GMT` (after the candidate's
03:29:18 commit), and was byte-identical to this candidate's fresh production
build. SHA-256 matched for all material deployed resources checked:

- `/`, `/privacy/`, `/terms/`, and `/sw.js`;
- hashed JS and CSS;
- `code-proof-press.webp` and `proof-mark.svg`.

HTTP redirects to HTTPS with 301, unknown paths return 404, HTML revalidates at
30 seconds, hashed assets and the WebP are immutable for one year, `/sw.js` is
`no-cache`, and matching ETags produced 304 responses.

## Browser, privacy, policy, and PWA evidence

Fresh Chromium contexts at 1440×900 and 390×844 confirmed:

- correct title, `lang=en`, one `h1`, one `main`, viewport metadata, complete
  image alt text, and no horizontal page overflow;
- no console errors, page errors, or failed first-load requests;
- all first-load requests were same-origin; no cookies, localStorage, or
  sessionStorage were created;
- source and dependency inspection found no analytics, telemetry, third-party
  scripts, or third-party font loads;
- first Tab reaches “Skip to content” with a visible cobalt 3px outline and
  Enter focuses `main`; all ordinary links/buttons were reachable with the same
  focus treatment; keyboard Enter/Space operated replay and copy controls;
- denied clipboard access announces the actionable command in a polite status
  region;
- reduced motion yields 0.01ms animation/transition durations, disables smooth
  scrolling, and the proof replay still completes and announces status;
- privacy and terms each have one `h1`, one `main`, and no serious/critical Axe
  findings;
- visual inspection confirmed the documented release-room risograph system,
  readable hierarchy, intentional mobile stacking, and no clipped primary
  action.

The root, legal pages, scripts, images, and service worker receive a self-only
CSP (`default/img/style/script/connect-src 'self'`, `object-src 'none'`,
`base-uri 'self'`, `frame-ancestors 'none'`), HSTS, `nosniff`, strict-origin
referrer policy, and camera/microphone/geolocation denial.

The live service worker reached `activated`, controlled the page, populated
`code-proof-v2` without the deployment-only config file, survived an explicit
`registration.update()` with no stuck installing/waiting worker, and served a
fresh offline reload with the visible offline status. This confirms the prior
service-worker deployment defect is repaired.

## Performance and budgets

- JS: 2,077 bytes (972 bytes gzip), under 200 KiB.
- CSS: 10,175 bytes (3,177 bytes gzip), under 50 KiB.
- Font payload: 0 bytes, under 120 KiB.
- Hero WebP: 210,844 bytes, under 300 KiB.
- Lighthouse 13.4.1 mobile: Performance 96, Accessibility 100, Best Practices
  100, SEO 100; FCP 1.0s, LCP 1.8s, TBT 210ms, CLS 0, total transfer 214 KiB.
  INP is not available from a synthetic no-interaction run. The separate
  serious name-mismatch audit described above is experimental and not included
  in Lighthouse's numeric accessibility score.

## Severity summary

| Severity | Count | Defect |
| --- | ---: | --- |
| Critical | 1 | Flattened fenced-code lines receive a clean PASS. |
| High | 2 | Declared Rust 1.79 package fails; deployed accessibility baseline has serious findings. |
| Medium | 0 | — |
| Low | 1 | “Terms” touch target is 42px wide instead of 44px. |

The candidate must not ship as a passing verification until the critical flow
false-negative, MSRV contract, and serious accessibility findings are repaired
and covered by regressions at the relevant PDF/mobile boundaries.
