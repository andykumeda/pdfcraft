# Single installed app with integrated assistant

Current toolbar delivery, 2026-11-05: All four top-toolbar toggles are installed and independently verified on both Macs. [Current source, placement and delivery ledger](assistant-top-toolbar.md) supersedes the toolbar/native source positions in the historical consolidation evidence below. The one-normal-app policy is unchanged.

Status: **Deployed and verified on both Macs**. Updated 2026-11-05 (America/Los_Angeles).

Owner request: one normal PDFCraft, WordCraft, GridCraft and DeckCraft on both Macs; remove
active Assistant Preview copies. ArtCraft Launcher updates must retain the panel and track the
underlying official release. This supersedes the split official/Preview installation policy in
the November 4–5 rollout. Preserve documents, both sets of document profiles, shared AI settings,
credentials and conversations. Rollback bundles are retained outside active application folders.

| Workstream | Owner | Acceptance | State | Evidence |
| --- | --- | --- | --- | --- |
| Launcher | Launcher agent | Explicit trusted update command; official release freshness; compatible signed standard bundles only; no upstream-only fallback or self-update loss | Deployed | Source1917013 installed/verified both Macs; 437 tests, strict lint/WASM/packaging; nativea323d26b |
| PDF packaging | Primary | Standard identity/profile, signed source/protocol and underlying release metadata; preserve bundled OCR | Deployed | Native5181ac82; fresh full10-gate CI/release passed; fixed standard package includes exact five OCR files; native d0dfb732 |
| Word/Grid packaging | Word/Grid agent | Same standard integration and profile preservation | Deployed | Word372e2a46/22ecf9ed; Gridf5785668; respective full9/10 CI gates pass; Grid optimized artifact building |
| Deck packaging | Deck agent | Same, disable unsafe upstream self-update | Deployed | Deckaef5d585/9a351298; full8 CI gates; actual-model candidate acceptance both Macs |
| Composer border | Shared helper agent | Clearly outlined bottom message box in light/dark idle/focus; no sidebar divider change; helper-only compatible update | Deployed | Helper0.1.7 sourcef2295dd installed both; 481 UI +30 helper tests; sourcepush verified; installed Word/Grid/Deck light/dark renders pass |
| Mini installation | Primary | All four normal paths and launcher, native synthetic assistant checks, Preview registrations removed | Deployed | Final signed inventory, actual model edits + no-model UI/resource/protected-data checks pass |
| MacBook installation | Primary | Independent same checks and config, no source build required on MacBook | Deployed | Same artifact signatures/native hashes; independent actual-model/UI checks and final inventory pass |

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

## PdfCraft 0.20.0 reconciliation — 2026-11-09

| Workstream | State | Acceptance and evidence |
| --- | --- | --- |
| Official source and assistant | In progress | Resolve the 0.20.0 release and retain assistant, OCR review, split panes, automation, and managed updates. Textual conflicts resolved; formatting, a locked UI check and 30 locked assistant unit tests against locally resolved shared commit b347 passed. Earlier local-path checks passed three updater unit tests and four automation shared-session tests. The split-pane test edits only the explicitly targeted document; an OCR review identity regression failed before its cleanup and passed afterward. These checks used an uncommitted shared-library candidate and the exact upstream 7ee print source. Generated attribution was refreshed; `cargo --offline --locked xtask layers` passed (40 crates), and `cargo --offline --locked xtask assets` passed (250 repository assets, seven bundled, 22 fetched at build time). The lock advances Tokio from 1.53.1 to the minimum compatible 1.53.2 and adds assistant dependencies. The local CI bundle passed with cargo-deny skipped; a locked release build, signing and live acceptance remain pending. |
| Signed package and installation | Deployed | Signed 0.20.0 package and matching CLI installed and verified on both Macs; see delivery evidence below. The protected launcher adapter was not exercised. |

The corrected local `cargo --offline --locked xtask ci` completed its ten listed steps against
shared b347 and the pinned font and translation inputs. The local `deny` step explicitly skipped
because cargo-deny was unavailable; advisories and dependency bans remain unverified. The first
run lacked OCR models and failed the OCR tests; the rerun used installed ONNX models whose hashes
match `ATTRIBUTION.toml`. Official 0.20.0 also reproduced the APFS non-UTF-8 filename probe and
minimum-width menu test failures. The updated tests verify the volume's rejection fails closed,
and open the collapsed menu to reach File and Open while checking the Assistant button at minimum
width. Both focused tests and the corrected full test suite passed. This is not package or live
installation acceptance.

The existing deployment described below remains the last verified installation. The focused UI
check is not a release build. The assistant tests initially had one sandbox-denied loopback bind;
the complete 30-test suite passed when the synthetic temporary listener was permitted. No
0.20.0 package, installation or live behavior is claimed at this checkpoint.

### PdfCraft matching CLI and MCP delivery

| Workstream | State | Acceptance / remaining verification |
| --- | --- | --- |
| PdfCraft 0.20.0 CLI | Deployed | Signed CLI built from b09, installed in its own versioned directory on both Macs, and exercised through the configured MCP routes. Existing old processes were not replaced. |
| Codex MCP on both Macs | Deployed | Mini points to the new versioned CLI with arguments preserved; MacBook retains its SHA-256-checking wrapper and arguments, with the approved version advanced. Each exact configured route passed a disposable synthetic edit/save/reopen. Existing MCP processes need a restart to use the new CLI. |

The integrated standard app package contains the GUI executable, not the separately distributed
CLI. The configured headless MCP server is independent of the app's embedded assistant bridge.

### PdfCraft 0.20.0 delivery — 2026-11-09

The normal `/Applications/PdfCraft.app` on Mini and MacBook now carries the signed integrated
0.20.0 build from `b09ee0eccfe0b9b51abee41fcbd5e64e453f1cc2`, with official underlying
`v0.20.0` / `2ad7717f87ad6f15b14bdf22cddc1019d6fd0aed` and assistant bridge 1. The
same-source, signed CLI is installed separately at `~/.local/share/pdfcraft/0.20.0/pdfcraft-cli`.
Both apps were closed immediately before an atomic directory exchange; existing 0.18.0 CLI
processes were not stopped and must be restarted by their owners to use the new executable.

| Workstream | State | Evidence / remaining checks |
| --- | --- | --- |
| Source and local CI | Verified | Merge `b09ee0ecc` has parents `b6b0ee057` and the official release and retains the existing task ledger. The ten-step local `cargo xtask ci` completed with pinned fonts, translations and hash-verified OCR models; cargo-deny was unavailable and explicitly skipped, so dependency advisory/ban checks are unverified. Earlier no-model, APFS filename and minimum-width failures and their targeted corrections are recorded above. |
| Signed package | Verified | Strict signature and source, release, bridge and repository stamps passed on the extracted ZIP; both OCR model hashes match `ATTRIBUTION.toml`. The signed package and CLI are not notarized. The managed launcher adapter was not exercised for this cutover. |
| Mini | Deployed | The existing Codex PdfCraft command now points to the 0.20.0 CLI with `mcp` arguments unchanged. Its exact route passed synthetic MCP initialize, edit, save and reopen. An isolated installed-app smoke opened the signed helper composer, typed and cleared synthetic text, and exited normally with its control listener released. Report: `/var/folders/cg/796c5bys74db15ww0b_qvw8c0000gn/T/pdfcraft-top-toolbar-d3_7wrqb/report.json`. |
| MacBook | Deployed | Existing Codex wrapper and `mcp` arguments remain; a verified 0.20.0 executable and approved SHA-256 were added and the wrapper's approved-current pointer updated. Its exact route passed synthetic MCP initialize, edit, save and reopen. The isolated installed-app/helper smoke passed and exited normally with its listener released. Report: `/var/folders/z6/7ct4dgns2236tw0d27zdb9s40000gn/T/pdfcraft-top-toolbar-xv82chll/report.json`. |

No model inference, real-document fidelity, VoiceOver, IME or clipboard acceptance was performed.
Each Mac retains its independently verified previous 0.18.2 ZIP at
`~/Library/Application Support/Craft/retired-apps/2026-11-09-pdfcraft-b09.noindex/PdfCraft-0.18.2-rollback.zip`.
The staged new archives and previous app copies remain at
`/private/tmp/pdfcraft-b09-stage-mini.JevugM` on Mini and
`/private/tmp/pdfcraft-b09-stage-book.ITIVMX` on MacBook. The Mini's prior Codex configuration
and MacBook's prior approved-version file are retained in their respective retired-apps folders.

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

## Final two-Mac acceptance — 2026-11-05

Exactly one active normal app per product remains on each Mac:

| App | Underlying official release | Native source | Exact signed native SHA-256 |
| --- | --- | --- | --- |
| PdfCraft | 0.18.2 /7faa4f35 | 5181ac824a76fdb6cb211e45caf3f11a4482625e | d0dfb732cd4e6fb16b7345f1f104f5c12aeb7ce38ad0e4b096b14597aade6a7d |
| WordCraft | 0.15.2 /edfea681 | fccdfe385a07e70f51ab71f9544e0be08f7a0484 | 3b6d253ef92373dae725ae500e24fe4cc0415be9bfbbad2c28e4b98fc56a8328 |
| GridCraft | 0.16.1 /e581bc95 | f5785668041bc35c22a0fe9265d5e4759b35074a | bf0c5910f56ceb41f839d089675c309d0192aa99c864d60215bea9efe89cf08d |
| DeckCraft | 0.15.1 /9bac60ff | dfa02f392524c9b23c1d8cc04616efdb24d7adbf | 5bfdd37235d9da677b43895aa198703334281be9f2688fa184ffb85f2fde7f03 |

Both independent inventories pass exact standard identity, version, underlying-tag/commit,
protocol/source metadata, certificate/native hash, wrapper profile/update behavior, helper0.1.7,
managed launcher1917013 and OCR-model hashes. They report **zero active Preview apps**:
`/private/tmp/craft-single-app-final-{mini,book}.json`. All current official tags, including
launcher0.14.1, were rechecked through official GitHub release metadata at final acceptance.

Normal PDF actual configured Codex/Terra/helper0.1.7 acceptance passed in40.0s Mini and21.2s Book:
exact synthetic note removal, no save before requested, one Undo/Redo, explicit save/reopen/render,
normal exit and released listener. PNGs were reviewed. Reports/PNGs:
`/private/tmp/craft-single-app-evidence/pdf-normal-{mini,book}/`.
One initial Mini provider conversation returned an interrupted error. The disposable fixture was
inspected independently and still had its original one note (no edit); the same configured
provider/model passed on a fresh fixture afterward. No fallback provider or production document
was used. Both normal/Preview profile/recovery/shared AI hash snapshots stayed identical across
all final PDF runs. All final private control files were removed after verified process exit.

The installed launcher performed PDF replacement on each host, sequentially against the shared
Mini cache. The corrected five-file OCR payload is exact on both. Its normal wrapper now shows
launcher-managed update guidance. All four active Preview apps, in-scope legacy MCP shortcuts
and Preview installer downloads are retired. Older in-scope rollback application bundles were
converted to verified ZIPs, with unrelated backup apps retained. Archives and per-host install/
retirement/legacy receipts remain under
`~/Library/Application Support/Craft/retired-apps/2026-11-05-single-app.noindex/`.
No document profile, recovery file, credential, connection, existing chat or existing MCP
registration was copied between hosts, merged, overwritten or removed.

### Updating from now on

1. Save/quit the application. In **ArtCraft Launcher**, check for updates and inspect its details:
   the displayed official base version and **Updates preserve Assistant** source identify this route.
2. Update Mini first. The configured adapter reconciles the exact official tag with the custom
   source in an isolated checkout, runs gates, builds/packages/signs, then the launcher verifies
   the staged copy before replacing the normal app. The first build can take substantial time;
   subsequent builds reuse the cache.
3. Update MacBook while Mini is reachable; it obtains the signed compatible artifact over the
   existing SSH connection and validates it independently. Both apps otherwise use their own
   local document/profile/helper installations; no Preview app is needed.
4. Conflicts, unavailable signing/build host or validation failures preserve the installed version
   and report an actionable failure. They never install an assistant-free upstream package.
   Ordinary launcher self-replacement is protected too; a new launcher version needs this
   integration retained when rebuilt. Do not bypass that guard with the historical stock installer.

Owner-controlled policy: `~/Library/Application Support/ArtCraft Launcher/assistant-updates.json`.
Adapter/configuration: `~/Library/Application Support/Craft/integrated-updates/`.
Build source/input/cache locations are explicit in those configurations; the remote connection
budget is configured, not hardcoded. Final adapter source includes4ea27c7/d2317c6 and the standard
PDF assembler includesdbdd3600. Signed native source remains1917013/5181ac82 respectively.
Qualified cache fingerprints are refreshed after documentation-only closeout without changing
signed native code or app data.

No public release, new fork, notarization or upstream-main push occurred. Existing writable
fork branches contain the source/docs commits; launcher and Deck commits remain local because
those existing upstream remotes do not grant push permission. Physical keyboard/IME, VoiceOver,
clipboard and private-document fidelity were not retested. The unrelated Deck theme-switch
redraw issue remains documented above; it reproduces without an assistant process and does not
change saved data. No required installation or verification work remains for this request.

Final shortcut cleanup: the one stale Book recent-Dock Preview tile was removed through the
per-user preferences API after an owner-only rollback plist was saved in the retirement folder.
Independent re-export reports no obsolete Craft recent tile; all other Dock preference keys
compared equal. No persistent Dock pin was changed on either host.
