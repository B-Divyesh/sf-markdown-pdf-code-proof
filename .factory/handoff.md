# Code Proof handoff — independent verification 6

## Result: PASS

- Candidate: `774749fdccdefcdc23607b4d7254061f9bf1a542`
- Live URL: <https://markdown-pdf-code-proof.sociobot.in>
- Verified: 2026-09-02 UTC
- Defects: 0 critical, 0 high, 0 medium, 0 low

Independent verification found no release-blocking defect. The cold first
screen explains the job and audience and offers a one-click sample. All 20
claim tests pass. The complete local test, type, lint, build, package, and
clean-consumer install gates pass. The live site matches the candidate build
byte for byte.

Live desktop, 390 px mobile, keyboard, focus, reduced motion, Axe, privacy,
headers, caching, offline reload, service-worker update, routes, metadata, and
links pass. Lighthouse mobile scored 99 performance and 100 for accessibility,
best practices, and SEO; LCP was 1.8 s, TBT 0 ms, and CLS 0.

Full commands, claim-by-claim evidence, hashes, measurements, and applicability
notes are in `.factory/verification-6.md`. No product code was changed.

## Reproduce

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
npm run test:install
cargo package --manifest-path cli/Cargo.toml --locked
```

Deploy `dist/site/` only through the factory deployment workflow. Publish the
crate only through the factory registry workflow; this verifier did neither.

## Known gaps and next steps

No release-blocking or follow-up product gaps were found. Normal post-release
maintenance is to keep dependencies, the Rust 1.88 contract, and the recorded
PDF fixtures current.
