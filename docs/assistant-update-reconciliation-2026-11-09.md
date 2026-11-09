# Assistant-integrated updates — 2026-11-09

The official releases diverged from the older releases on which the installed assistant builds
were based. The protected launcher rejected the resulting automatic merges. Reconciliation must
retain the official release, assistant, safe update route and user documents. The existing
installations remain in place until a candidate is built, checked, signed and admitted by the
protected route.

| Workstream | State | Acceptance and current evidence |
| --- | --- | --- |
| Shared libraries | In progress | The isolation-preserving 83e-based candidate passed formatting, workspace all-target Clippy and 318 updater tests (four ignored). The first full test run failed three proof tests that also fail on unchanged upstream because of an installed French dictionary. Two later runs hit timing-sensitive assistant tests that passed individually; an isolated run hit a sandbox mode-bit limitation. A split-environment rerun and publication remain. |
| GridCraft 0.18.0 | In progress | The isolated UI compiles against the unpublished shared candidate; 37 assistant tests, three updater tests and a light/dark title-toolbar test passed. Full CI, release build and installation remain unverified. |
| WordCraft 0.17.0 | In progress | The focused assistant suite passed 36 tests (two ignored); screenshot binding and title-bar tests passed, and the document-switch negative control failed as expected. Ambiguous multi-window turns and queued input are conservatively restricted. Full gates and installation remain unverified. |
| PdfCraft 0.20.0 | In progress | Isolated UI check, 30 assistant tests, three updater tests, split-pane targeting and four shared-session automation tests passed. An OCR cache regression failed before its fix and passed afterward. Full gates, OCR packaging and installation remain unverified. |
| DeckCraft 0.17.0 | In progress | Isolated UI and desktop checks passed without optional craft-fonts. Assistant safety changes and test compilation are unverified; the build was paused to preserve disk space. Release build and installation remain unverified. |
| ArtCraft Launcher 0.17.0 | In progress | Source commit `4b51723` passed native and UI suites, scoped strict Clippy, formatting, WASM and synthetic adapter checks. Its locally signed package was verified and installed on Mini; installed certificate, marker, source, native hash and live assistant-preserving route passed. MacBook delivery remains. Existing auto-install settings triggered protected app-update attempts on startup; conflicts and a running PdfCraft prevented replacement. |
| VectorCraft assistant | In progress | An isolated prototype includes a guarded panel, helper bridge and packaging drafts. Two initial view-target tests passed with a negative control; later close/reopen, draft, helper, packaging and integration changes remain untested. Temporary local dependency paths remain. |

Isolated candidates are under `/private/tmp/` and are not release artifacts. The VectorCraft
prototype additionally needs a tested non-reused view identity and end-to-end assistant, draft,
chooser, update and packaging checks. The launcher installer draft has passed eight synthetic
tests with signature/archive operations stubbed; its final process check still has a launch race,
so the launcher must remain closed throughout a real replacement. No app or launcher
replacement is claimed by this ledger. The launcher source adapter must see both the reviewed
assistant source and the exact official tag in the resulting history; an unreviewed patch or
ordinary upstream binary is not an eligible replacement. Both Macs need separate installed-path
verification if updated.

After builds and verification are complete, remove the task-owned temporary build caches,
checkouts and staging files, including the dedicated external-drive build folder. Retain committed
source and recovery backups required for installed apps.

To restore system-disk headroom during the work, the redundant internal GridCraft and VectorCraft
build caches were cleaned after their copies were made on the external drive. Source checkouts and
external caches remain for continuing verification.
