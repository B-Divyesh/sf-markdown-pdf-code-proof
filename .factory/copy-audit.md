# Landing-page copy audit

Audited 2 September 2026. Counts treat hyphenated terms, command tokens, and
versions as one word. Every sentence is 22 words or fewer. No sentence uses a
banned marketing word.

## Sentences

| Words | Sentence |
| ---: | --- |
| 5 | Catch PDF bugs before release. |
| 19 | For engineers and technical writers, Code Proof catches broken code, page overflow, and internal links in the final PDF. |
| 10 | See a sample PDF defect and failed check. |
| 6 | Check the source against the PDF. |
| 10 | Code Proof compares your Markdown with the finished PDF. |
| 9 | It writes an HTML proof sheet for review. |
| 14 | Check an existing PDF, or use a custom renderer command. |
| 9 | Renderer arguments never pass through a shell. |
| 10 | Check links, code colors, and text that runs outside the page. |
| 10 | Match each code fence with the PDF. |
| 18 | Open the HTML proof sheet, save a JSON report in CI, and use exit codes to stop a broken release. |
| 4 | Sample failed release check. |
| 4 | Find release-breaking PDF defects. |
| 4 | Missing or wrapped code. |
| 9 | Flags code fence lines that disappear, merge, or wrap in the PDF. |
| 9 | Flags text that runs outside a page edge. |
| 10 | Flags Markdown fragments with a missing or wrong PDF destination. |
| 11 | Warns when a language-tagged code fence has no detectable syntax color. |
| 14 | Install from the public repository with Rust 1.88 or newer. |
| 9 | Then check a Markdown and PDF pair. |
| 10 | Code Proof runs a renderer only after Linux sandbox setup succeeds. |
| 8 | Checking an existing PDF does not start a renderer. |
| 5 | Check Markdown PDFs before release. |
| 7 | The docs and recorded proof still work. |
| 3 | Proof run started. |
| 8 | Proof run complete: hold, with one expected error. |
| 2 | Demo opened. |
| 9 | Sample data is active and nothing is saved. |
| 3 | Install commands opened. |
| 2 | Copy unavailable. |

## Terminology

| Concept | Term used |
| --- | --- |
| Source document | Markdown |
| Finished document | PDF |
| Markdown code region | code fence |
| Browser-readable result | HTML proof sheet |
| Machine-readable result | JSON report |
| Recorded browser example | sample failed release check |
| Program that creates a PDF | renderer |
| Failed release decision | HOLD — do not release |
