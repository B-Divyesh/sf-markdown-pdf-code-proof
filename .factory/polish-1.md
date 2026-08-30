# Polish 1 — review finding closure

- Repair commit: `465a09672a0be0cfe2bac2ea495575b7c1a08baa`
- Candidate repaired: `568d4cae10d24c7f3a08e1673e67bade51e46fe8`
- Local screenshots: `evidence/landing-mobile.png`, `evidence/demo-mobile.png`
- Clean clone: `/tmp/codeproof-clean.TVfVTh/repo` at the repair commit

## Finding map

| Finding | Change | Evidence |
| --- | --- | --- |
| F-1-1 | Demo lines use `visibility`, never low-opacity text. | `@claim:private-site` runs Axe immediately after start; `demo-mobile.png`. |
| F-1-2 | `?demo=1` sets title, canonical, live announcement, banner, and heading focus. Exit uses `/#install` and focuses Install; history is tested. | Playwright `sample demo is one click away and reports completion`. |
| F-1-3 | Replaced invalid path install with a public `cargo install --git … --locked codeproof` command. | `npm run test:install` in clean clone installed commit `465a0967` and ran `codeproof 0.1.0`. |
| F-1-4 | Removed the overbroad upload/device promise. Privacy now states only observable local path behavior. | `local-cli-files` claim test. |
| F-1-5 | Removed account, daemon, and CLI telemetry promises. Website privacy stays narrow and tested. | `private-site` claim request/storage test. |
| F-1-6 | Registered the no-shell guarantee. | `renderer-no-shell` claim test. |
| F-1-7 | Removed untested filesystem-limit copy; retained and tested socket isolation. | `renderer-network` claim test. |
| F-1-8 | Added forced sandbox-setup refusal before spawn. | `renderer-fail-closed` claim test. |
| F-1-9 | Removed untested Pandoc sanitizing copy from public docs. | Copy audit and README review. |
| F-1-10 | Removed the untested script-execution promise and added a direct deadline regression. | `renderer-timeout` claim test. |
| F-1-11 | MSRV test now compiles the locked workspace with Rust 1.88. | `npm run test:msrv`. |
| F-1-12 | Registered existing-PDF mode and says it does not start a renderer. | `existing-pdf` claim test. |
| F-1-13 | Removed unverified Pandoc marketing; documented custom renderer mode with its executable no-shell test. | `renderer-no-shell` claim test. |
| F-1-14 | Replaced PDF-internals inventory with user outcomes. | Landing copy audit. |
| F-1-15 | Registered HTML proof, JSON report, and exit-code contracts. | `html-proof`, `json-report`, and `exit-codes` claim tests. |
| F-1-16 | Removed subjective diagnostic-matrix promise. | README review. |
| F-1-17 | Registered the MIT license claim and exposes it in Terms. | `npm run test:license`. |
| F-1-18 | Split exit-code copy into three short sentences. | `.factory/copy-audit.md`. |
| F-1-19 | Rewrote the fragment-link description in two plain sentences. | README Checks. |
| F-1-20 | Rewrote code-flow description with clear wrap behavior. | README Checks. |
| F-1-21 | Replaced unexplained HOLD call-to-action copy and defined it in the result. | `landing-mobile.png`, `demo-mobile.png`. |
| F-1-22 | Replaced internal and decorative terms with Markdown, PDF, and HTML proof sheet. | Landing copy audit. |
| F-1-23 | Replaced PDF implementation inventory with visible defect outcomes. | Landing Checks section. |
| F-1-24 | Replaced vague lock-down wording with specific Linux socket behavior. | `renderer-network` claim test. |
| F-1-25 | Uses renderer consistently. | README and landing audit. |
| F-1-26 | Standardized on code fence, HTML proof sheet, and JSON report. | README and landing audit. |
| F-1-27 | Renamed workflow and sample headings for standalone meaning. | Landing headings. |
| F-1-28 | Visible copy controls now name their result; exit action says View install commands. | Playwright route and mobile tests. |
| F-1-29 | Added 1200×630 social image, 180px touch icon, full social metadata, and demo sitemap entry. | `npm run build:site`; built assets. |
| F-1-30 | Standardized Home, Privacy, Terms, and external GitHub footer on all routes. | Playwright route crawl and semantic checks. |
| F-1-31 | Compressed mobile hero and facts into the first 390×844 viewport. | Playwright `390px layout keeps primary paths available`; `landing-mobile.png`. |

## Verification

From the clean clone, every command in `claims.json` passed. The run included
21 CLI integration tests, 12 Playwright tests with transition-time Axe,
dedicated offline and privacy contexts, Rust 1.88 compilation, the public Git
install smoke test, lint, typecheck, production build, and `cargo package`.

The configured static deployment is triggered by the pushed `main` branch. At
the time of this document, the live host still served the previous artifact;
the final handoff records the live probe separately so stale deployment cannot
be mistaken for verification of this repair.
