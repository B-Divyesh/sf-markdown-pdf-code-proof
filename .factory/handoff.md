# Code Proof handoff — perfection loop round 2

## Result: PASS

- Repaired candidate: `f1474e5871a1c5c28d4e9967c8f9476a41f20a79`
- Repair commits: `0a18c6a83db473b2ca129a4b297b6882f807cc08` and
  `18510ca669597dfbb6a8fe2159d9a05998242db4`
- Live URL: <https://markdown-pdf-code-proof.sociobot.in>
- Demo URL: <https://markdown-pdf-code-proof.sociobot.in/?demo=1#demo>
- Deployed resource: `sf-markdown-pdf-code-proof` in `sociobot`
- Deployed: 2 September 2026 UTC through `/opt/fleet/lib/deploy-static.sh`

All 31 findings from review 1 and all eight findings in review 2 are closed.
The detailed finding-to-change-to-evidence map is in `.factory/polish-2.md`.

## What changed

- Replaced the last inconsistent “block,” bare “report,” and “Code flow” terms
  with “code fence,” “HTML proof sheet,” “JSON report,” and “Missing or wrapped
  code.” The site, terminal output, generated HTML, diagnostics, and JSON
  `code_fences` field now agree.
- Renamed the focused demo heading to “Sample failed release check,” while
  retaining “HOLD — do not release” inside the terminal result where it is
  explained.
- Removed the unregistered checkout installation promise. Recast the test and
  build coverage sentences as direct commands without promised implementation
  details.
- Replaced PDF and Linux implementation jargon with observable plain-language
  outcomes.
- Added a regression that renders the direct demo and rejects every retired
  public phrase across the site and README.
- Added a live-base option to the Playwright configuration so the same browser,
  accessibility, privacy, offline, route, focus, mobile, and 404 tests can run
  against production.
- Updated `.factory/claims.json`, `.factory/copy-audit.md`, `.factory/demo.md`,
  and the 97-character verb-first catalog description.

The release-room risograph identity, original assets, CLI artifact class, and
static landing/docs deployment remain unchanged.

## Exact verification

The public repository was cloned fresh at
`/tmp/codeproof-polish2-final.sMJ5XD/repo`, SHA
`18510ca669597dfbb6a8fe2159d9a05998242db4`.

Every command in `.factory/claims.json` ran separately: 20 of 20 passed. This
included the dedicated browser contexts for same-origin privacy and offline
reload. `npm run test:install` installed the public Git commit `18510ca6` into
an empty root and returned `codeproof 0.1.0`.

The following clean-clone gates passed:

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
cargo package --manifest-path cli/Cargo.toml --locked
```

Results: Rust 1.88 locked compile passed; 3 unit tests, 21 CLI integrations,
and 13 Playwright tests passed; rustfmt and Clippy passed with warnings denied;
the crate packaged 15 files at 33.3 KiB compressed and verified successfully.
The real release demo returned its intentional exit 1 and wrote isolated
`sample-manual.md`, `sample-manual.pdf`, and `proof/index.html` files.

The static build measured 2,999 bytes JavaScript (1,308 gzip), 11,251 bytes
CSS (3,371 gzip), no fonts, and a 210,844-byte hero WebP. Local Lighthouse was
99/100/100/100 with LCP 2.1 s and CLS 0.

## Production verification

`/opt/fleet/lib/verify-url.sh` passed with HTTPS 200, no console errors,
`lang=en`, one H1, one main landmark, complete alt text, and labeled buttons.
The live Playwright suite passed all 13 tests against the custom domain. It
covered:

- first-screen and 390×844 layout;
- direct `?demo=1` entry, reset, exit, Back, Forward, title, banner,
  announcements, and focus;
- Axe at demo start/completion and on every page;
- same-origin requests, empty cookies, and empty Web Storage;
- dedicated-context offline reload;
- reduced motion and keyboard skip navigation;
- `/privacy/`, `/terms/`, and the styled 404.

Live Lighthouse scored Performance 100, Accessibility 100, Best Practices
100, and SEO 100, with FCP 0.9 s, LCP 1.8 s, TBT 0 ms, CLS 0, and 215 KiB
total transfer. `/`, `/privacy/`, and `/terms/` returned 200; an unknown route
returned 404. The live root and hashed JS/CSS matched the local build byte for
byte. Evidence is under `.factory/evidence/polish-2-live/`.

## Run and verify

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
target/release/codeproof demo
```

To rerun the production browser audit:

```sh
PLAYWRIGHT_BASE_URL=https://markdown-pdf-code-proof.sociobot.in npx playwright test
```

## Known gaps and next steps

None. No publishing was performed, as registry publication remains factory
owned. The crate is ready for `cargo package --manifest-path cli/Cargo.toml
--locked`.
