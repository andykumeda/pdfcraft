# Assistant-integrated updates — 2026-11-09

The official releases diverged from the older releases on which the installed assistant builds
were based. The protected launcher rejected the resulting automatic merges. Reconciliation must
retain the official release, assistant, safe update route and user documents. The existing
installations remain in place until a candidate is built, checked, signed and admitted by the
protected route.

| Workstream | State | Acceptance and current evidence |
| --- | --- | --- |
| Shared libraries | In progress | The isolation-preserving 83e-based candidate passed scoped check, formatting, strict Clippy and 318 updater tests (four ignored). Full workspace gates and publication remain. |
| GridCraft 0.18.0 | In progress | The isolated UI compiles against the unpublished shared candidate; 37 assistant tests, three updater tests and a light/dark title-toolbar test passed. Full CI, release build and installation remain unverified. |
| WordCraft 0.17.0 | In progress | The focused host suite passed 12 tests (one ignored), including document/window guards; a negative control failed as expected. Further changes to refuse ambiguous multi-window turns need testing. Build and installation remain unverified. |
| PdfCraft 0.20.0 | In progress | Isolated source conflicts were resolved; an OCR review-cache regression is being tested. Build, OCR packaging and installation remain unverified. |
| DeckCraft 0.17.0 | In progress | Isolated UI and desktop checks passed without optional craft-fonts. Assistant tests, release build, review and installation remain unverified. |
| ArtCraft Launcher 0.17.0 | In progress | Isolated source commit `4b51723` passed native and UI suites, scoped strict Clippy, formatting, WASM and synthetic adapter checks. Signed packaging and installed checks remain. |
| VectorCraft assistant | In progress | Integrate the shared panel with VectorCraft's command and multi-window model, protect updates, and verify document targeting before installation. Initial source inspection identified the need to guard document and view identity at dispatch. |

Isolated candidates are under `/private/tmp/` and are not release artifacts. No app or launcher
replacement is claimed by this ledger. The launcher source adapter must see both the reviewed
assistant source and the exact official tag in the resulting history; an unreviewed patch or
ordinary upstream binary is not an eligible replacement. Both Macs need separate installed-path
verification if updated.
