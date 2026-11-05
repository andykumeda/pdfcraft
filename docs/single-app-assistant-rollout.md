# Single installed app with integrated assistant

Status: In progress. Updated 2026-11-05 (America/Los_Angeles).

Owner request: one normal PDFCraft, WordCraft, GridCraft and DeckCraft on both Macs; remove
active Assistant Preview copies. ArtCraft Launcher updates must retain the panel and track the
underlying official release. This supersedes the split official/Preview installation policy in
the November 4–5 rollout. Preserve documents, both sets of document profiles, shared AI settings,
credentials and conversations. Rollback bundles are retained outside active application folders.

| Workstream | Owner | Acceptance | State | Evidence |
| --- | --- | --- | --- | --- |
| Launcher | Launcher agent | Explicit trusted update command; official release freshness; compatible signed standard bundles only; no upstream-only fallback or self-update loss | Deployed | Source1917013 installed/verified both Macs; 437 tests, strict lint/WASM/packaging; nativea323d26b |
| PDF packaging | Primary | Standard identity/profile, signed source/protocol and underlying release metadata; preserve bundled OCR | Verified | Native5181ac82; fresh full10-gate CI/release passed; fixed standard package includes exact five OCR files; native d0dfb732 |
| Word/Grid packaging | Word/Grid agent | Same standard integration and profile preservation | Verified | Word372e2a46/22ecf9ed; Gridf5785668; respective full9/10 CI gates pass; Grid optimized artifact building |
| Deck packaging | Deck agent | Same, disable unsafe upstream self-update | Verified | Deckaef5d585/9a351298; full8 CI gates; actual-model candidate acceptance both Macs |
| Composer border | Shared helper agent | Clearly outlined bottom message box in light/dark idle/focus; no sidebar divider change; helper-only compatible update | Deployed | Helper0.1.7 sourcef2295dd installed both; 481 UI +30 helper tests; sourcepush verified; installed Word/Grid/Deck light/dark renders pass |
| Mini installation | Primary | All four normal paths and launcher, native synthetic assistant checks, Preview registrations removed | Not started | No installs yet |
| MacBook installation | Primary | Independent same checks and config, no source build required on MacBook | Not started | SSH host verified AKs-MBPro.local/arm64 |

Future official binaries cannot acquire a custom panel by renaming. The configured local builder
must merge the exact official tag with the assistant source, verify/build/package/sign, then the
launcher validates before replacing the installed app. Conflicts or failed checks preserve the
installed version and report the required reconciliation. No public release or new fork is needed.

Current normal candidates are Wordfccdfe38, Gridf5785668, Deckdfa02f39 and rebuilt PDF5181ac82,
based on the same current official releases. Shared helper0.1.7 is separate. Source, packaging
and each installed-path check are recorded independently below.

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

## Integration checkpoint

- Official release tags independently rechecked: PDFv0.18.2, Wordv0.15.2, Gridv0.16.1,
  Deckv0.15.1; launcher v0.14.1.
- Word and Deck standard candidates passed actual configured-model edit/no-early-save/Undo/Redo/
  explicit-save/reopen/render checks independently on both hosts. Normal profiles, shared AI
  settings and app histories remained unchanged; all owned processes/listeners exited normally.
  Reports and reviewed PNGs: `/private/tmp/craft-single-app-evidence/`.
- Both hosts have nine verified ZIP rollback archives (four normal apps, four Previews, launcher)
  under `~/Library/Application Support/Craft/retired-apps/2026-11-05-single-app.noindex/`.
  Preview removal and normal app/launcher installation remain pending.
- The local source adapter passed admission/cache tests and an uncached synthetic update with
  real Git/tag/merge/pinned-input operations (compilation/signature boundaries stubbed in that
  synthetic test). Cache invalidation and failed-build preservation passed. Cached-input guard
  removal was observed failing. Production PDF uncached build is running with real gates/build/sign.
- MacBook-to-Mini signed Word artifact transfer passed independently through the actual SSH adapter.
  Build configurations/scripts and launcher policies are staged owner-only on both hosts; the
  existing official launcher does not consume this policy until replaced.
- ShellCheck passed on the PDF packager/archive verifier and all three launcher source-adapter
  scripts; the positive standard archive passed, historical Preview and wrong underlying tag
  were rejected. The PDF archive verifier's negative profile check was made fail-fast explicitly.

Owner steering: the PDFCraft bottom message composer boundary is too faint. Review confirms
the fill-only dark composer lacks a clear outline. The authorized fix is only that message box,
implemented in the shared helper and therefore consistent across the four app panels; the host
sidebar divider remains unchanged. Source/visual/native checks and both-host helper activation
will be recorded separately from the app/launcher installation.

## Protected installations and three-app consolidation

- Managed ArtCraft Launcher0.14.1/source19170137f5968ffd2a7e911278bb075a1b3eb072 is installed
  on both Macs. Both native hashes matcha323d26b78fe6bdbe0f941a7e4f4a7706a253602469e6233898a5aebb4bdab6e.
  Exact local certificate, standard identity, source stamp and signed managed marker verify.
- Helper0.1.7 is active on both Macs;0.1.6 retained. Four app protocol1 handshakes each host,
  app binary/public settings preservation and installed light/dark message-box outlines pass.
- The installed launcher CLI performed the actual protected replacements for normal Word/Grid/Deck
  on both Macs. One initial concurrent Book request met Mini's per-app build lock and failed
  without upstream fallback; after Mini finished, the serialized Book retry passed all three.
- Final installed no-model panel/status/light/dark/update-guidance and cleanup checks pass for
  all three on both hosts, retaining earlier actual-model candidate evidence for exact same binaries.
  Reports: `/private/tmp/craft-single-app-evidence/{word,grid}-{mini,book}-installed/` and
  `/private/tmp/craft-single-app-evidence/deck-installed-{mini,book}/`.
- Their Preview bundles are retired on both Macs; Mini's three old `with MCP.app` shortcuts were
  also retired after checking known MCP registrations did not reference those app paths. The
  underlying launcher scripts, existing MCP registrations and out-of-scope LightCraft shortcut
  remain. Preview installers moved out of Downloads. Exact archives/receipts remain in the
  per-host non-indexed retirement folder; document profiles, recovery files and shared AI data
  are not moved or merged. PDF Preview remains pending final PDF installation.
- The input repositories' omitted partial-clone promisor configuration was restored and historical
  font revision37827 hydrated; a clean temporary worktree check passed without changing active
  b076 inputs. Adapter follow-up4ea27c7 avoids writing shared input FETCH_HEAD metadata; its
  regression failed before the one-flag fix and both synthetic suites/ShellCheck passed afterward.
  Runtime script replacement/cache refresh waits until the current PDF builder exits.
- Live launcher grid displayed the current six installed suite release versions and reported
  "Checked just now / using GitHub". Details-line light/dark rendered checks pass; an attempted
  live drawer interaction hit the computer-use server's no-windows error, so no live drawer
  interaction is claimed.
- Deck dark-theme redraw limitation: with assistant closed and no helper process, switching theme
  can temporarily hide synthetic slide title text. Readback/disk bytes stay intact and resizing
  restores drawing. This reproduces independently of the helper outline change; no out-of-scope
  renderer fix was made. Evidence: `/private/tmp/craft-single-app-evidence/deck-dark-ab-mini/`.

## PDF build and resource-parity checkpoint

The production adapter's fresh full10-gate CI passed with6586 tests/16ignored, then its optimized
native5181ac824a76fdb6cb211e45caf3f11a4482625e built and packaged successfully. The optional
cargo-deny step explicitly skipped absent cargo-deny; existing WASM warnings remain.
Before any PDF installation, resource comparison found the Preview-derived standard assembler
omitted the official bundle's OCR models. The first package was rejected. The standard assembler
now reuses existing `stage_models` from the official packager, fetching only manifest-pinned,
hash-verified assets plus licences/attribution; Preview assembly is unchanged. The archive
regression fails on the omitted-model package and passes on the corrected package, which also
passes full source/tag ancestry checks and strict extraction/mount/copy signatures. Native code
is unchanged5181; the corrected assembler is committed separately. All five OCR files match the
currently installed official bundle byte-for-byte. Other three apps' original functional resource
files are retained; omitted AppleDouble `._` records are Finder metadata only.

Corrected PDF artifact: `dist/integrated/5181ac82/`; native SHA
`d0dfb732cd4e6fb16b7345f1f104f5c12aeb7ce38ad0e4b096b14597aade6a7d`.
The first no-models cache/package is not eligible for installation and is replaced by this
qualified artifact before invoking the installed launcher's protected route.

The shared-helper message-box fix is now also verified inside PDF itself on both Macs: reviewed
light/dark idle, actually focused (unsent draft visible), and cleared PNGs. No inference occurred,
synthetic/normal PDF bytes, profiles/recovery/shared AI data stayed unchanged, and all owned
processes/listeners/keys cleaned up. Evidence: `/private/tmp/craft-single-app-evidence/pdf-composer-{mini,book}/`.
Initial broad-a11y focus selection incorrectly selected zoom; those frames are invalidated and
retained under `*-initial-zoom-focus`, superseded by actual composer pointer/draft evidence.

Older in-scope rollback .app bundles were also converted to exact verified ZIPs and unregistered,
so they cannot reappear as launchable old versions. The first pass stopped on LaunchServices
application-not-found -10814; that already-unregistered case is now handled, preserving verified
archives. No out-of-scope app or user document/recovery directory was removed.
