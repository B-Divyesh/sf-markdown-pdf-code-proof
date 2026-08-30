# Code Proof visual thesis

## Direction: the release-room risograph

Code Proof borrows from a two-ink risograph proof pulled beside a printing
press: tactile, slightly misregistered, and covered in practical crop marks.
That metaphor is specific to the job. The CLI is the last physical-feeling
inspection pass between fluid Markdown and a fixed PDF release. Decoration
always explains that transition: source lines become printed sheets, and
registration marks become pass/fail evidence.

The treatment is intentionally single-mode, like a sheet of warm stock under
neutral shop light. This avoids a cosmetic theme toggle and keeps every status
color predictable for a release-critical tool.

## Tokens

- `paper #F3E9D2` — warm uncoated stock; page background.
- `paper-light #FFF9EA` — fresh proof sheet; primary surface.
- `ink #171817` — near-black carbon ink; text (13.7:1 on paper).
- `ink-muted #55534D` — pencil note; secondary text (6.3:1 on paper).
- `cobalt #184E9E` — first riso drum; links, focus, informational marks.
- `cobalt-dark #10376F` — interactive hover and small text.
- `tomato #C73D2F` — second drum; defects and deliberate emphasis.
- `green #28613F` — passed checks, always paired with a word or icon.
- `amber #8A4B0F` — warnings, always paired with a word or icon.
- `rule #292A27` — outlines and crop marks.

No gradients. Ink fields may use a CSS halftone dot pattern at low opacity.
Misregistration is expressed with one hard 4px offset, never soft shadows.

## Type and spacing

The pairing is local system typography: `Georgia` for editorial display copy
and `ui-monospace, SFMono-Regular, Consolas, monospace` for commands, labels,
and measurements. It ships no font payload and evokes technical publishing
without a third-party request. Body copy uses the system sans stack for dense,
legible instructions.

The scale is 16 / 18 / 22 / 32 / 48 / 72px. Body leading is 1.55 and reading
measure is capped at 68ch. An 8px base rhythm expands to 12, 16, 24, 32, 48,
64, and 96px. Rules group evidence; boxes are reserved for genuinely separate
artifacts such as the proof report and install command.

## Layout and interaction grammar

The masthead is a compact press label. The hero reads like a cover proof:
message on the left, an original printed-page collage on the right. Below it,
the workflow is one continuous numbered galley rather than a grid of generic
cards. Buttons are rectangular ink stamps with a visible 3px focus outline and
at least 44px hit areas. Copy buttons confirm in text through a polite live
region. At 390px, the art follows the primary install action and wide command
lines scroll inside their own region; nothing essential is dropped.

## Motion

On arrival, proof layers settle by 8px over 240ms and status stamps press down
for 160ms. Only transform and opacity animate. Nothing loops. Under
`prefers-reduced-motion: reduce`, layers render in their final position and all
transitions are removed; hierarchy remains through overlap, hard offsets, and
rules.

## Original asset plan and provenance

- `site/public/code-proof-press.webp`: original AI-generated tactile collage
  showing a code-heavy manual passing through a two-ink inspection press.
  Generated for this repository with `/opt/fleet/lib/gen-image.sh` using the
  factory image deployment, then converted to WebP at <=300 KB. Prompt:
  “Editorial risograph collage for a developer CLI landing page; warm uncoated
  paper, cobalt blue and tomato red two-ink registration, a stack of technical
  manual pages with abstract code lines, crop marks, magnifying loupe finding
  one overflowing line and one linked anchor, tactile halftone and paper grain,
  landscape 4:3 composition, no readable words, no logos, no watermark.”
  License: project-owned generated asset under the repository MIT license.
- `site/public/code-proof-social.jpg` and `site/public/apple-touch-icon.png`:
  deterministic crops of `code-proof-press.webp` made in this repository for
  the required 1200×630 social card and 180×180 touch icon. They carry the
  same project-owned provenance and MIT license; no third-party artwork or
  remote asset is loaded.
- Registration marks, checker icons, and page fragments elsewhere are original
  CSS/SVG geometry drawn in-repository; they do not reproduce an icon library.
