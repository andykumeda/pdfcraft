# Single installed app with integrated assistant

Status: In progress. Updated 2026-11-05 (America/Los_Angeles).

Owner request: one normal PDFCraft, WordCraft, GridCraft and DeckCraft on both Macs; remove
active Assistant Preview copies. ArtCraft Launcher updates must retain the panel and track the
underlying official release. This supersedes the split official/Preview installation policy in
the November 4–5 rollout. Preserve documents, both sets of document profiles, shared AI settings,
credentials and conversations. Rollback bundles are retained outside active application folders.

| Workstream | Owner | Acceptance | State | Evidence |
| --- | --- | --- | --- | --- |
| Launcher | Launcher agent | Explicit trusted update command; official release freshness; compatible signed standard bundles only; no upstream-only fallback or self-update loss | In progress | `../craft-launcher` |
| PDF packaging | Primary | Standard identity/profile, signed source/protocol and underlying release metadata, existing assistant native retained | In progress | `packaging/macos/assistant-preview.sh --standard` |
| Word/Grid packaging | Word/Grid agent | Same standard integration and profile preservation | In progress | Sibling packagers |
| Deck packaging | Deck agent | Same, disable unsafe upstream self-update | In progress | Sibling packager |
| Mini installation | Primary | All four normal paths and launcher, native synthetic assistant checks, Preview registrations removed | Not started | No installs yet |
| MacBook installation | Primary | Independent same checks and config, no source build required on MacBook | Not started | SSH host verified AKs-MBPro.local/arm64 |

Future official binaries cannot acquire a custom panel by renaming. The configured local builder
must merge the exact official tag with the assistant source, verify/build/package/sign, then the
launcher validates before replacing the installed app. Conflicts or failed checks preserve the
installed version and report the required reconciliation. No public release or new fork is needed.

Current packaging reuses the previously qualified assistant natives (PDF32b9ca14, Wordfccdfe38,
Grid2b995d6d, Deckdfa02f39), based on the currently installed official releases. Shared helper0.1.6
remains separate. New installation and future-update claims require new evidence below.

Storage correction: MacBook APFS reports about12.2GB immediately unallocated, while Foundation
`volumeAvailableCapacityForImportantUsage` reports287.0GB available, including reclaimable
capacity. The earlier11GiB `df` observation was not total available storage; it is not a rollout
blocker. Builds remain on Mini for shared artifact reuse, not because MacBook lacks capacity.

## PDF source verification checkpoint

Full `cargo xtask ci` passed all10 gates (log `/private/tmp/pdfcraft-single-app-ci.log`),
including formatting, strict Clippy, tests, assets/layers and WASM. Existing WASM warnings
remain. The optional cargo-deny wrapper reports its actual availability in that log.
Standard archive checks passed and the historical Preview ZIP was rejected by the same check.
No installation is claimed at this source checkpoint. `plan/STATUS.md` is absent locally.
