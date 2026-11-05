# Deferred assistant toolbar changes — 2026-11-04

Current toolbar delivery, 2026-11-05: PDF/Grid/Deck top-toolbar toggles are installed and independently verified on both Macs; Word toolbar build remains in progress. [Current source, placement and delivery ledger](assistant-top-toolbar.md) supersedes the toolbar/native source positions in the historical consolidation evidence below. The one-normal-app policy is unchanged.

Status: **Deployed as single normal assistant-integrated apps on both Macs** (2026-11-05). The owner requires one normal assistant-integrated app per product on both Macs and no active Assistant Preview copies. ArtCraft Launcher updates must preserve the assistant and report the underlying official release. See [the current consolidation ledger](single-app-assistant-rollout.md). Do not reinstall the historical split normal/Preview arrangement. The toolbar implementation and prior Preview delivery below are historical evidence.

- **GridCraft:** Move the Assistant button shown at the left of the View ribbon to the far right of that ribbon, and add its missing icon. The screenshot also shows an existing Assistant control in the title bar; preserve its behavior unless the user directs otherwise.
- **WordCraft:** Move the Assistant button shown at the left of the View ribbon to the far right of that ribbon. Retain its existing icon.
- **PdfCraft:** Replace the text-only Assistant toolbar control with an AI sparkle-star icon. Preserve an accessible Assistant name and a clear tooltip.

Before implementing, inspect each app's current layout and nearby controls; confirm placement and icon behavior at narrow widths, in light and dark themes, and with keyboard/accessibility navigation. Preserve the existing assistant toggle behavior. Use assets permitted by each repository's asset policy. Test the actual interface and relevant automated checks. Record implementation, verification and deployment separately; the original request and source checkpoints below are retained as history.

## Task ledger

| Workstream | Owner | Acceptance | State | Release |
| --- | --- | --- | --- | --- |
| GridCraft View ribbon | GridCraft agent | Assistant at far right with icon; title-bar toggle retained; narrow/light/dark and accessible toggle checks | Deployed | Latest separate Assistant Preview installed on both Macs |
| WordCraft View ribbon | WordCraft agent | Assistant at far right retaining icon and command; narrow/light/dark and accessible toggle checks | Deployed | Latest separate Assistant Preview installed on both Macs |
| PdfCraft mode bar | Primary | Existing Lucide sparkle icon, accessible Assistant name/tooltip, click/keyboard toggle in both themes | Deployed | Latest separate Assistant Preview installed on both Macs |

The workstreams share no mutable product files or API changes. Per-app verification and delivery
are recorded in each app's assistant implementation ledger. Standard official updates are coordinated separately.

### Verification checkpoint — 2026-11-04

- GridCraft: commit `4d30d1f3`; focused theme/layout/widgets and every-tab checks passed,
  four narrow/wide light/dark renders inspected; scoped lint/WASM and assets/layers passed.
  A broader locale sweep is blocked by missing Vietnamese glyphs in the available optional
  font input; that is separate from the toolbar change.
- WordCraft: commit `7a4736e7`; 39 ribbon tests, strict scoped lint, formatting, assets/layers and all 15 WASM
  crates passed; four 600/1440-point light/dark renders inspected. The new position regression
  failed on the old layout and passed on the change.
- PdfCraft: commit `ca25e628`; 731 UI tests passed, one ignored; strict scoped lint,
  formatting, assets/layers and scoped WASM passed. Existing WASM warnings remain. Light/dark
  real-shell renders inspected; the icon regression failed on the old text control.

At that source-only checkpoint, no installation or live inference was claimed. The later delivery below supersedes installation state; native VoiceOver remains untested.

### Related reports added during implementation

| Workstream | Owner | State | Acceptance / release |
| --- | --- | --- | --- |
| Shared model selector flicker | Shared helper agent | Deployed | MacBook helper 0.1.3; 32 installed idle-control samples remained enabled; shared settings and history refresh regressions passed |
| MacBook Codex executable discovery | Shared helper agent + primary delivery | Deployed | Helper0.1.3 signed installation and installed bundled-Codex discovery/version smoke passed; live authentication/inference untested |

These two fixes are tracked in the shared `craft-libs` assistant and rollout ledgers.


### Two-Mac preview delivery — 2026-11-04

All three authorized toolbar changes are now installed in their separate `/Applications/* Assistant Preview.app`
bundles on Mini and MacBook: Word0.15.2 sourcefccdfe38, Grid0.16.1 source2b995d6d,
PDF0.18.2 source32b9ca14. Full respective CI gates pass; signed DMG/mounted/copied bundles and
exact installed sources/native hashes verify. Actual Codex/Terra/helper0.1.6 edited synthetic
content, followed instructions not to save, and passed Undo/Redo, explicit save/reopen/render on
both hosts. Screenshots verify toolbar controls and content. Per-app current records:
Word `../wordcraft/docs/upstream-assistant-v015.md`, Grid `../gridcraft/docs/assistant-v0161-rollout.md`,
PDF `docs/assistant-v0182-rollout.md`. Shared helper1.6 supersedes the historical1.3 rows above.

Rollback bundles, independent profiles and conversations are retained; launchers disable normal
updaters. Locally signed previews are not notarized or public releases. Physical VoiceOver/IME/
clipboard were not repeated. Standard BookWord remains open on0.15.0 pending owner save/quit;
its separate Assistant Preview is current. Other standard suite apps are already current.
