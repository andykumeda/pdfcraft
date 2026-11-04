# Deferred assistant toolbar changes — 2026-11-04

Status: **Verified** (source). Implementation authorized on 2026-11-04. All three toolbar changes passed focused checks; installation is separate.

- **GridCraft:** Move the Assistant button shown at the left of the View ribbon to the far right of that ribbon, and add its missing icon. The screenshot also shows an existing Assistant control in the title bar; preserve its behavior unless the user directs otherwise.
- **WordCraft:** Move the Assistant button shown at the left of the View ribbon to the far right of that ribbon. Retain its existing icon.
- **PdfCraft:** Replace the text-only Assistant toolbar control with an AI sparkle-star icon. Preserve an accessible Assistant name and a clear tooltip.

Before implementing, inspect each app's current layout and nearby controls; confirm placement and icon behavior at narrow widths, in light and dark themes, and with keyboard/accessibility navigation. Preserve the existing assistant toggle behavior. Use assets permitted by each repository's asset policy. Test the actual interface and relevant automated checks. Record implementation, verification and any deployment separately; none has been performed for this request.

## Task ledger

| Workstream | Owner | Acceptance | State | Release |
| --- | --- | --- | --- | --- |
| GridCraft View ribbon | GridCraft agent | Assistant at far right with icon; title-bar toggle retained; narrow/light/dark and accessible toggle checks | Verified | Source only; not installed |
| WordCraft View ribbon | WordCraft agent | Assistant at far right retaining icon and command; narrow/light/dark and accessible toggle checks | Verified | Source only; not installed |
| PdfCraft mode bar | Primary | Existing Lucide sparkle icon, accessible Assistant name/tooltip, click/keyboard toggle in both themes | Verified | Source only; not installed |

The workstreams share no mutable product files or API changes. Per-app verification and delivery
are recorded in each app's assistant implementation ledger. Installed apps are unchanged.

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

No app installation, public release, live inference or native VoiceOver acceptance is claimed.

### Related reports added during implementation

| Workstream | Owner | State | Acceptance / release |
| --- | --- | --- | --- |
| Shared model selector flicker | Shared helper agent | In progress | Idle settings polling keeps model control enabled; actual shared settings changes still refresh |
| MacBook Codex executable discovery | Shared helper agent + primary delivery | In progress | GUI-launched helper finds installed Codex, explicit overrides remain exact; signed helper installation verified separately |

These two fixes are tracked in the shared `craft-libs` assistant and rollout ledgers.
