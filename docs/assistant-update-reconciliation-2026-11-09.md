# Assistant-integrated updates — 2026-11-09

The official releases diverged from the older releases on which the installed assistant builds
were based. The protected launcher rejected the resulting automatic merges. Reconciliation must
retain the official release, assistant, safe update route and user documents. The existing
installations remain in place until a candidate is built, checked, signed and admitted by the
protected route.

| Workstream | State | Acceptance and current evidence |
| --- | --- | --- |
| Shared libraries | In progress | Combine the new updater and assistant APIs. An isolated candidate passed `cargo check`, scoped strict Clippy and formatting; 318 updater tests passed, four ignored, and 19 font-build tests passed. Full workspace gates and publication remain. |
| GridCraft 0.18.0 | In progress | Integrate the assistant with the official release; isolated candidate compilation is underway. Build, assistant behavior and installation remain unverified. |
| WordCraft 0.17.0 | In progress | Integrate the assistant with the official release, including safe document and window targeting. Isolated candidate tests are compiling; installation remains unverified. |
| PdfCraft 0.20.0 | In progress | Reconcile the assistant and official release, including automation, update protection and OCR packaging. An isolated patch has unresolved conflicts; build and installation remain unverified. |
| DeckCraft 0.17.0 | Not started | Reconcile and verify the assistant-enabled release before protected installation. |
| ArtCraft Launcher 0.17.0 | In progress | Preserve protected app updates and the launcher self-update guard. Isolated native and UI suites and scoped strict Clippy passed; WASM, signed packaging and installed checks remain. |
| VectorCraft assistant | Not started | Integrate the shared panel with VectorCraft's command and multi-window model, protect updates, and verify document targeting before installation. Initial source inspection only. |

Isolated candidates are under `/private/tmp/` and are not release artifacts. No app or launcher
replacement is claimed by this ledger. The launcher source adapter must see both the reviewed
assistant source and the exact official tag in the resulting history; an unreviewed patch or
ordinary upstream binary is not an eligible replacement. Both Macs need separate installed-path
verification if updated.
