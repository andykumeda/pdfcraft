# Integrated assistant task ledger

Current delivery, 2026-11-05: PDF title-bar sparkle beside Display theme is **Deployed on both Macs**, native `dacec56f`/helper0.1.7. Scoped source gates and independent signed installed-path UI/data-preservation checks pass; [top-toolbar delivery ledger](assistant-top-toolbar.md). The consolidation baseline below is historical; All four toolbar deliveries are complete in the cross-app ledger.

Current delivery, 2026-11-05: **One normal assistant-enabled PdfCraft0.18.2/source5181ac82 on both Macs**, helper0.1.7 and managed ArtCraft Launcher0.14.1. Active Preview copies are removed. Full source gates, signed package/resource parity and independent final actual-model/Undo/Redo/save/reopen/render checks pass; see [the single-app ledger](single-app-assistant-rollout.md). All earlier split-installation/version statements below are historical.

Current update, 2026-11-04: [0.18.2 two-Mac preview rollout](assistant-v0182-rollout.md) is deployed and model-qualified on both Macs. Earlier installed/version statements below are historical; shared helper0.1.6 is deployed independently.

Current delivery, 2026-11-02: **0.18.0 (`8e86f96f`) standard and Assistant Preview Deployed on
Mini and MacBook**, using shared assistant 0.1.0. Packaging recipe `7881a923` adds the required
signed protocol/repository metadata; initial incompatible packages were refused and superseded.
Installed signatures/hashes and isolated native/MCP smoke are verified on both Macs. No public
release or live-provider/clipboard/native-IME/VoiceOver acceptance is claimed. See the final
installed-verification section below. Earlier sections preserve the 0.15 embedded-preview history.

Historical embedded-preview delivery: **0.15.0 (`b1f9d8f3`)**, separate Preview on both Macs.

Reference: WordCraft `4dd7cf7d`, craft-libs `bf932b91643c287502d539b88f08abd5796c9447`.
PDFCraft baseline `726ecf26`; referenced local `plan/` source documents were absent at baseline.

| Workstream | Owner | State | Acceptance and verification |
| --- | --- | --- | --- |
| Shared-session automation | Automation workstream | Verified | Validated existing tools operate on GUI Session; synthetic mutation, read-back, Undo and save/reopen tests |
| Native panel and Settings | Integration/native workstreams | Verified | Visible toolbar/View toggle; shared single-model settings, provider routing, native safety and WASM gates |
| Preview isolation/package | Packaging workstream | Verified | Distinct profile/bundle, disabled updater, ARM64 DMG/checksum; mounted and extracted verification |
| Acceptance and installation | Integration | Deployed | Synthetic UI/providers/read/edit/Undo/save/reopen; normal app unchanged; no previous preview required rollback |

Allowed states: Not started, In progress, Blocked, Verified, Deployed.

## Source verification history (before final native acceptance)

- Working trees were clean before work; changes isolated to `feat/integrated-assistant`.
- GitHub issue/PR search unavailable because API connectivity failed.
- Initial sandbox Cargo resolution failed due DNS and missing cached `md-5`.
- Network-enabled resolution identified a required Tokio patch update (1.53.1 → 1.53.2) for the pinned shared assistant.
- Scoped tests: **2,280 passed, 8 ignored, zero failed**, including doctest targets, for `pdfcraft`, `pdfcraft-automation`, `pdfcraft-ui-egui` and `xtask`. Log: `/private/tmp/pdfcraft-assistant-tests-final.log`. UI test code used opt-level 0 to bound build resources; dependencies retained project defaults.
- The first broad run exposed three Preferences layout regressions and a missing-glyph panic in malformed metadata. Existing tests reproduced both before repair. Assistant Settings now uses a Preferences subpage, and missing UI glyphs render tofu instead of panicking. The corrected full scoped run passed.
- Scoped all-target Clippy with `-D warnings`, full fmt check and packaging shell syntax passed. Log: `/private/tmp/pdfcraft-assistant-clippy.log`.
- Source lockfile preserves every prior registry package version except required Tokio 1.53.1 → 1.53.2; new shared/native dependencies are pinned. WordCraft and craft-libs working trees remain clean.
- Cache mutation proof: disabling cache preparation caused the cross-session paragraph-reference regression to fail; exact source restoration passed 1/1. Logs: `/private/tmp/pdfcraft-assistant-cache-{mutant,restored}.log`. An earlier build timeout was not counted as proof.
- Temporary `cargo-deny` 0.20.2 installed under `/private/tmp/pdfcraft-assistant-tools`; no global configuration or product dependency added. The dependency-source allowlist now includes only the explicitly pinned Craft fork in addition to existing allowed sources.
- Layers (37 crates), assets (289 repository assets / 6 bundled / 22 fetched), parity and WASM (31 crates) passed. Existing WASM warnings remain; new native-only helper warnings were removed with target gates.
- The initial license gate rejected MPL `option-ext` through `dirs-sys`. The permissive helper is now vendored at original 0.5.0 with its sole Option extension replaced by standard equality; no MPL source copied and no license exception added. Its nine parser tests pass. Shared commit and directory behavior remain unchanged. `cargo xtask deny` now passes advisories, bans, licenses and sources.
- Final post-license checks all passed: scoped tests (2,280 passed / 8 ignored), all-target Clippy `-D warnings`, minimal CLI without default features, layers, assets, parity, full dependency/advisory policy and WASM (31 crates). Existing WASM warnings remain outside the assistant changes. Detailed exit codes: `/private/tmp/pdfcraft-assistant-final-checks.json`.
- At this source checkpoint, native acceptance and packaging were pending; the completed delivery is recorded below.
- Read-only compatibility discovery: Codex CLI 0.165.0 is available; Ollama 0.40.13 reports installed tool-capable `qwen3:4b-instruct-2507-q4_K_M`. No Codex inference or model download was performed.
- At this source checkpoint, no preview had been installed and no live inference had run.

## First native candidate and visual correction

Source `7cb1bccf1b9a4ee3b18d34f14e645b35afdd13cc` built successfully as ARM64 release
in 15m35s. Version/commit-labelled DMG and ZIP plus checksums were produced. Assembled,
mounted and extracted apps passed strict signature/version checks; signing is ad-hoc,
not Developer ID signed or notarized. This candidate was **not installed**.

The real toolbar opened the assistant and showed the expected local route using an isolated
profile and synthetic PDF. Panel and Preferences Settings screenshots were inspected. Visual
inspection found a redundant header Settings button beside the shared widget's own control.
The wrapper button is removed; the shared editor remains, and both Settings surfaces show the
same effective configuration. A synthetic regression failed with the old second button
(2 controls instead of 1), then passed after restoring the fix. The candidate quit normally
with no save prompt or document changes. No inference was sent in the first candidate. The corrected build and acceptance are recorded below. Logs: `/private/tmp/pdfcraft-assistant-settings-{mutant,restored}.log`.

Corrected-source gates passed: scoped tests **2281 passed / 8 ignored / zero failed**,
Clippy, fmt, layers, assets, parity, dependency/advisory policy and WASM (31 crates).
Exit codes: `/private/tmp/pdfcraft-assistant-visual-fix-checks.json`. Corrected release
packaging and native/local-provider acceptance subsequently passed as recorded below.

## Verified delivery checkpoint

### Source and build

- PDFCraft build source: `dd68eaeda15c93d895111c35f705050744d75686` on `feat/integrated-assistant`.
- Incremental source commits: automation `41ceb7794add87fd8184d2dc54b1b298a5b395ba`,
  integration `31656475a49e370f92f5d8f7eb34e683a59ea8f8`, preview isolation
  `7cb1bccf1b9a4ee3b18d34f14e645b35afdd13cc`, final visual fix `dd68eaeda15c93d895111c35f705050744d75686`.
- Shared libraries: `bf932b91643c287502d539b88f08abd5796c9447`. WordCraft reference: `4dd7cf7d3f1bd6bdb7f330e164da4dc0f1ad469e`.
  Both reference working trees remain unchanged.
- Committed ARM64 release built with `PDFCRAFT_BUILD_LABEL=dd68eaed`, `cargo build --release --locked -p pdfcraft -p pdfcraft-cli -j 2`; final build passed in 8m46s.
- Final checks: **2,281 scoped tests passed, 8 ignored**, plus **9 directory-parser tests passed**.
  fmt, all-target Clippy `-D warnings`, minimal CLI without default features, layers, assets, parity,
  dependency licenses/advisories/bans/sources, and the 31-crate WASM check passed.
  WASM retains nine pre-existing UI warnings; assistant-native helper warnings were removed.
  Mutation proofs demonstrated failing-before-fix behavior for session cache identity and duplicate Settings.

### Native and provider verification

- The actual toolbar opens/closes the in-window panel, with a selected open state. The View menu's
  checked command is covered by the menu test. Native panel, single Settings control, shared model
  editor, collapsed advanced options, effective provider/endpoint/context and running tool activity
  were visually inspected using synthetic content and `PDFCRAFT_SYSTEM_FONTS=0`.
- One keyless local trial used **Ollama 0.40.13 / qwen3:4b-instruct-2507-q4_K_M**. Its actual
  resident-model digest was captured alongside the effective route. Tool sequence: `doc_info`,
  `page_rotate`, `doc_info`; structured reads showed **0° → 90°**, including independent mutation
  read-back. Final conversation was not interrupted. Observed completion: **59.68s**,
  one workload observation rather than a performance/reliability benchmark.
- Normal application Undo restored the original portrait page and clean state. Redo reapplied the
  edit; normal Save followed by independent CLI reopen/info/render confirmed 90° and 400×300 pt.
  Text, metadata, fields and attachments were unchanged, and saved bytes retained the entire original
  revision as a prefix. The fixture was restored to 0° through normal Undo/Save and quit cleanly.
- Safe HTTP error-body handling, cancellation, stale/foreign targets, exact path grants, context
  bounds and uncertain-outcome handling passed synthetic tests. No paid inference, private PDF,
  model download or credential copying occurred. Existing Ollama settings/service were unchanged.

### Package and installation

- Installed: `/Applications/PDFCraft Assistant Preview.app`; installed launch reports `pdfcraft 0.15.0 (dd68eaed)`.
- DMG: `/Users/andy/Dev/pdfcraft/dist/assistant-preview/dd68eaed/pdfcraft-assistant-preview-0.15.0-dd68eaed-macos-aarch64.dmg`.
- Checksum: `/Users/andy/Dev/pdfcraft/dist/assistant-preview/dd68eaed/pdfcraft-assistant-preview-0.15.0-dd68eaed-macos-aarch64.dmg.sha256`.
- DMG SHA-256: `5734c85603c74d854779f115cca4fba472e23a4b2099b6124625b8a1981171d0`.
- Signed native binary SHA-256: `2d9e8311ec0d2e037d8128e90ab3c8cd53b0024b4dbc28879f65428a570d00db`.
- Version/commit-named ZIP/checksum and INSTALL notes are adjacent to the DMG. Assembled, mounted,
  ZIP-extracted and DMG-copied apps passed strict signature/version verification. Final DMG integrity
  and extracted native identity were independently checked before installation.
- **Ad-hoc signed; no Developer ID signature or notarization.** Local signature verification does
  not establish receiving-Mac Gatekeeper acceptance. Use Apple's per-app approval flow if needed.
- The installed bundle matches the verified extraction. Its normal launcher created the separate
  `~/Library/Application Support/PDFCraft Assistant Preview` profile; a fresh disposable assistant
  store showed no chosen model, proving test model choices were not baked into the app. Shared
  production settings, credentials, conversations, profiles and existing MCP wrappers were preserved.
- Startup/manual updater opt-out was unit-tested and the actual native disabled-updater message was
  verified in both extracted and installed apps. Source updates remain deliberate integrations and
  rebuilt preview DMGs; no service split or auto-registration was introduced.
- Main `/Applications/PdfCraft.app` **0.13.0** full bundle hash remains
  `1a4ddb3fd68f02ddde84857ba7796d9dea7090b07f8dceb715a3bf7ed6869890`.
  No prior PDFCraft preview existed, so no replacement rollback was needed. Future replacements
  must retain a rollback bundle and wait for normal exit without discarding unsaved work.
- Installed and extracted test apps quit normally; no preview/control listener remains. No startup
  job or proxy was added. Required local service inventory was updated outside Git.

### Evidence and remaining acceptance

Receipts: `/private/tmp/pdfcraft-assistant-install.json`, `/private/tmp/pdfcraft-assistant-acceptance-sa4x8tse/native-acceptance.json`.
Synthetic states, conversation, before/after CLI reports and screenshots are under `/private/tmp/pdfcraft-assistant-acceptance-sa4x8tse`.
Source/build/gate logs are `/private/tmp/pdfcraft-assistant-*`; the local-only handoff is
`plan/ASSISTANT-HANDOFF.md` (ignored, not committed or pushed).

On the other Apple-silicon Mac: verify the checksum, install beside the main app, complete any
per-app Gatekeeper approval, configure that Mac's Codex/Ollama connection and model, then repeat
read/edit/read-back/Undo/save/reopen. Live PDFCraft Codex and hosted API inference, broader
local-model reliability, native Windows/Linux/FreeBSD, and large/complex real-document acceptance
remain unperformed. The initial assistant intentionally exposes only the documented tool subset;
it does not establish full external-MCP or Acrobat AI parity.

## Authorized source distribution

After local delivery, the user explicitly authorized creating `andykumeda/pdfcraft` and
pushing `feat/integrated-assistant` there, without a public release. The upstream push to
`storytold/pdfcraft` was rejected with HTTP 403; no upstream branch was changed. The fork
was created successfully under the verified `andykumeda` account. `origin` remains upstream;
the `fork` remote targets `https://github.com/andykumeda/pdfcraft.git`.

This documentation-only source distribution update does not change the installed build or
DMG source `dd68eaeda15c93d895111c35f705050744d75686`. No GitHub release, tag, installer upload,
or upstream pull request is part of this authorization. Branch publication is verified by
comparing the remote branch commit to local HEAD after pushing.

## Annotation deletion follow-up — 2026-11-02

State: **Verified** (source). Owner: Codex. Scope: the native assistant host tool list and
mutation read-back; existing automation and engine deletion remain the source of truth.
Acceptance: list a synthetic StrikeOut, delete it through the assistant, verify its
absence through independent comment read-back, and restore it with Undo.
The installed preview predates this fix. Real Fabric Site 1 PDF acceptance remains pending.
GitHub issue/PR search failed because api.github.com was unreachable.

Verification: the new test failed before the fix with “this PDF tool is unavailable”
and passed after adding `comment_delete` and routing its read-back to `comment_list`.
All five assistant-host tests pass. The full UI library suite passed outside the
sandbox: **621 passed, 1 ignored**; its initial sandbox run failed on nine socket
permission checks and one unavailable GPU. Scoped all-target Clippy with warnings
denied, formatting, layering (37 crates), asset validation and WASM (31 crates) pass.
Logs: `/private/tmp/pdfcraft-delete-{red,green,tests-unrestricted,clippy,fmt,layers,assets,wasm}.log`.
No engine API, automation schema, dependency or asset changed. Existing locked-comment,
permission, active-document and draft guards remain in force. Release build and local
preview update are tracked separately from this verified source change.

Release verification: `timeout 300 cargo build --offline --release -p pdfcraft -j 2`
reached the app compile/link stage but exceeded the five-minute bound (exit 124).
No new package or installed preview was produced. Source fix `11c720e5` was pushed
to `fork/feat/integrated-assistant`; a completed preview rebuild/install and the
real-document smoke check remain pending. This is not a deployed fix.

## Two-Mac annotation deletion rollout — 2026-11-02

Target source: `b1f9d8f38ad450a48fb82f5c671664e480bef9c8`, version 0.15.0.
Acceptance per host: installed preview version and native hash match packaged build,
strict code-signature validation passes, launcher starts with preview profile and
updates disabled. Keep the standard app and user data; retain previous preview for rollback.

| Device / verified host | Component | Target | Installed path | State | Evidence / remaining action |
| --- | --- | --- | --- | --- | --- |
| Mini / AKsMini, arm64, macOS 27.2 | Assistant Preview | b1f9d8f3 | /Applications/PDFCraft Assistant Preview.app | Deployed | Version/hash/signature verified; synthetic GUI inspection passed |
| MacBook / AKs-MBPro.local, arm64, macOS 27.2 | Assistant Preview | b1f9d8f3 | /Applications/PDFCraft Assistant Preview.app | Deployed | SSH install; matching version/hash/signature; synthetic GUI inspection passed |

Rollout completed: release build passed in 7m54s with `PDFCRAFT_BUILD_LABEL=b1f9d8f3`.
The existing preview packager passed strict signing, ZIP extraction, mounted-DMG,
and copied-DMG validation. Artifacts: `dist/assistant-preview/b1f9d8f3/`.
Both installed native executables have SHA-256
`9b6a6c69a78600d8a6fb2bc948255883be2f606fd4f4344e33e03a45a5927698`.
Each reports `pdfcraft 0.15.0 (b1f9d8f3)` and passes strict code-signature validation.
The installer hashed the standard app before/after and confirmed it unchanged on both hosts.
Mini rollback: `/Applications/.PDFCraft-Preview-before-b1f9d8f3.app`.
MacBook had no previous preview. SSH independently verified its identity and installation.
Receipts: `/private/tmp/pdfcraft-preview-mini-install.json` on Mini and
`/private/tmp/pdfcraft-preview-macbook-install.json` on MacBook.

Installed-app startup checks on each host used a temporary profile, existing synthetic
`form.pdf`, and the opt-in authenticated control channel on an OS-assigned loopback port.
`pdfcraft-cli ui inspect` succeeded and showed the synthetic document on each Mac.
Inspect receipts are `/private/tmp/pdfcraft-preview-{mini,macbook}-inspect.json` on their
respective hosts. The temporary instances were quit normally, stopping those listeners;
normal preview launches were then requested on both Macs. Existing user profiles,
provider settings and credentials were not copied or edited. Local signing remains
ad-hoc, without notarization. The user's real Fabric Site 1 deletion and provider
conversation are the remaining manual acceptance checks.

## Full MCP catalog parity — 2026-11-02

| Workstream | Owner | State | Acceptance and evidence |
| --- | --- | --- | --- |
| Live-session MCP API | PDF integration | Verified | `McpServer::handle_on_session` delegates the existing protocol dispatcher, returns the live Session after dispatch and retains its Undo; `cargo test --offline -p pdfcraft-automation -j 2 live_session_tests`: 1 passed, including exact external/GUI results, PNG, engine errors and Undo |
| Complete assistant tool catalog | PDF integration | Verified | Removed the 18-tool allowlist, schema/document stripping, private read-back wrapper and separate file grants; the baseline source and CLI full catalog contained 181 matching names before 0.18 reconciliation. `cargo test --offline -p pdfcraft-ui-egui -j 2 --lib assistant::`: 25 passed on shared Git revision `c28f430ee4e932e7cbfa9e149b92f24e0f0bc202` (approved local-loopback test permission), covering full descriptor metadata equality, exact MCP results/PNG, multi-document edits, Undo, save/open/root/dirty close, cross-turn paragraph identity, cancellation/self-stop, native settings/panel and a complete synthetic provider turn |
| Persistent MCP client state and UI synchronization | PDF integration | Verified | `Assistant.mcp` keeps paragraph listings and automation caches across turns. `set_assistant_automation` accepts the same configured Automation path policies as an external client. New/closed documents synchronize tabs; changed generations/saved sources refresh views; pending ordinary typing commits through the existing UI path |
| Installed/native delivery | PDF integration | In progress | Source changes do not replace the existing preview. No newly installed app, live provider inference, native accessibility or signed release is claimed for this increment |

The side panel now forwards ordinary MCP document IDs, paths and options. Normal engine
permissions, signatures, root confinement, atomic saves and explicit dirty-document close
validation remain authoritative. GUI-side transport cancellation still prevents queued work
from executing after Stop and never automatically replays uncertain operations. Tool output
retains the MCP `content`/image/`structuredContent`/`isError` contract. Provider catalog discovery,
context budgets and image handling are tracked with the shared assistant runtime workstream.

Scoped all-target Clippy for `pdfcraft-automation` and `pdfcraft-ui-egui` with `-D warnings`
passed against shared revision `17c5ea725866fa2e8f7f89a6cb3bec269ad8d571`. The subsequent published shared revision `c28f430ee4e932e7cbfa9e149b92f24e0f0bc202`
adds lossless MCP metadata forwarding and provider discovery/image support. The final consumer
suite above verifies that actual Git dependency; real provider and installed-app qualification
remain separate gates. Final reruns on that Git dependency passed: the live-session MCP parity test (1 passed),
all-target UI/automation Clippy with `-D warnings`, and changed-source formatting/diff checks.
The consumer assistant suite passed all 25 tests in one run with the synthetic loopback fixture
permitted; no external provider was contacted.

Native PDF qualification update (2026-11-02): rebuilt actual c28f430 consumers/helper passed coordinate typing, Enter/send, synthetic streaming, accessibility Focus with accented draft text, 2x Retina, window resize and dark-theme propagation with retained draft. A seven-step actual private-pipe MCP turn completed discovery, doc_list, doc_info, page_rotate, page_render PNG, edit_undo and final doc_info with no tool errors; native HTTP received an image_url block. Read-back confirmed page rotation0 and Undo returned dirty=false. Repeated the seven-step turn after explicit adoption of the freshly rebuilt helper. PDF SHA256 b2e84c6d69349d8ae5adf5a2cd13328ad7aabe6e5d827be772cfe4f1bc97cbc2; helper SHA25655ea11eedada3f071307092849d6c3c9460c60b79bb9d607cc7d3f4daf26792c. No real inference, credentials or installed bundles were involved. Earlier A2-preview timeouts remain recorded and their cause is unconfirmed; they did not recur during the rebuilt-app smoke. Native clipboard, real IME, VoiceOver, sustained scrolling/performance, Word/Grid native windows and signed update qualification remain pending.

Additional remote dispatch regressions passed3/3 (existing hidden-keyboard test plus new nested-begin/reload and deferred self-stop cases).


## PDFCraft 0.18 reconciliation — 2026-11-02

| Workstream | Owner | State | Acceptance and evidence |
| --- | --- | --- | --- |
| Upstream source reconciliation | PDF integration | Verified | Isolated branch `codex/independent-assistant-v018` merges upstream release `3936f93a` into assistant baseline `cd2eb01d`; source conflicts resolved without changing the original checkout or installed applications |
| Assistant compatibility | PDF integration | Verified | Preserve full live-session MCP descriptors/results, persistent tool state, shared helper integration and explicit fork update sources. Retain upstream two-page startup parsing, Preferences module split, updater app identity and engine changes |
| Regression verification | PDF integration | Verified | Final shared Git pin `3ff2754eeb1c079612247f4d288a22a20c169ac9`: assistant tests 27 passed, moved Preferences control 1 passed, GUI links/document-link tests 39 passed. The 0.18 stale-paragraph fixture uses upstream’s pure-move regrouping case because rewritten regions now remain distinct. Standalone live-session MCP regression 1 passed (318 unrelated integration cases filtered). Desktop startup tests 8 passed. Strict all-target Clippy for UI/automation/desktop, WASM (11 existing upstream warnings), formatting, layers (37 crates), assets (281 repository / 6 dependency / 22 fetched), and source license-check passed. Both new regressions failed when their fixes were temporarily removed and passed after exact restoration |
| Delivery | PDF integration | In progress | No installed-app replacement, signed component activation or public release is part of this source reconciliation. Native live accessibility/input and three-app independent-update acceptance remain pending |

The previous preview installation evidence above still describes the 0.15 preview. Source
version 0.18 does not imply that installed applications have been updated. The custom app
update result modal remains app-owned because the upstream generic result dialog uses its
static release repository; the assistant-enabled build must display only its configured fork
repository and must not silently fall back to ordinary upstream releases.


Before producing 0.18 local candidates, source review found pre-existing restricted ArtCraft
logos embedded in Home/About. This fork removes their `include_bytes!` helpers, eight logo
files and matching asset entries, replaces promotional branding with permitted plain-text
upstream provenance, and regenerates the asset index. Original copyright and dependency/font
licences remain. The existing About link regression now asserts that restricted mark alt text
is absent and upstream attribution remains; Home retains clearly labelled upstream links.

The shared packaging helper now includes `ATTRIBUTION.md`/`.toml` and source-relative
asset, font and vendored licence texts in signed Resources. Standard macOS bundles use
the same helper; the pinned optional craft-fonts OFL notices are retained. Candidate
receipts must verify these files after extraction, before claiming packaging complete.

Packaging verification before the release build: shell syntax passed; all 11 source asset
licence paths and 13 pinned craft-fonts notices were staged successfully, and all five maintained
bundled font/CMap notice files matched their source bytes. The packaging guard rejected the
prior branded release binary and accepted the new 0.18 UI test binary. These are source/package
helper checks, not signed release-candidate delivery or native installed acceptance.


## App update restart safety — 2026-11-02

| Workstream | Owner | State | Acceptance and evidence |
| --- | --- | --- | --- |
| Underlying app restart guard | PDF integration | Verified | Document-clean state is combined with the assistant’s authoritative host turn/dispatch state and shared `Controller::restart_safe()`, including queued input and unsaved assistant drafts/settings. Legacy provider/settings jobs are also protected |
| Missing-helper recovery | PDF integration | Verified | A failed or missing helper with no live transport does not block app recovery updates through stale settings-busy UI state; current helper/host work still blocks restart |
| Shared contract | Shared/Word integration | Verified | Shared revision `986cb16234cfdac1ee1337fb4cdf00945cad4e5a`; no wire change. Shared owner reported 23 helper library tests plus strict lint/formatting passing |
| Candidate build | PDF integration | In progress | Committed baseline `3fc5750d` optimized ARM64 app/CLI build completed with required pinned fonts; final app candidate must be rebuilt after this guard is verified and committed |

If the helper has unacknowledged rendered/input frames, app restart is conservatively deferred.
Finish or stop chats, send or clear any draft, finish settings changes and close the assistant
panel to let the remaining frames drain. The app update status dialog explains this; the host
never silently closes the panel, discards a draft, cancels a turn or replays a document operation.

The provisional CLI from source `3fc5750d` is separately signed with the explicitly selected local
certificate (not notarized); its extracted archive signature and binary hash match. A real
stdio MCP session listed **183 tools** and passed seven synthetic create/metadata/read/render/
save/close/open calls under a temporary root. It changes no installed CLI path or client config.
Receipt: `/Users/andy/Dev/pdfcraft/dist/assistant-candidates/0.18.0-3fc5750d/cli-receipt.json`.
The final guarded build supersedes this provisional candidate before app rollout.

Verification for the restart guard on shared Git revision `986cb16234cfdac1ee1337fb4cdf00945cad4e5a`:
29 assistant tests and 2 app-updater tests passed. The app-update regression failed with the
previous document-only condition; after exact restoration all 3 focused safety regressions
passed. Strict scoped all-target Clippy, WASM (the same 11 upstream warnings), formatting and
source license-check passed. No installed app, CLI path, source configuration or client config
was changed. The final optimized app/CLI build and signed candidate verification follow this
verified source increment; provisional `3fc5750d` artefacts are not the final guarded app.


### Signed update compatibility metadata correction

The first 8e86f96f local bundles were correctly refused by the rollout installer: their signed
Info.plists lacked `CraftAssistantBridgeProtocol` and `CraftUpdateRepository`. No app was
installed from those bundles. Both macOS packagers now add integer protocol `1` and the explicit
`CRAFT_UPDATE_REPOSITORY` before signing. Missing or malformed repository input fails packaging;
the configured repository for these candidates is `andykumeda/pdfcraft`. The shared function is
used by the standard and preview packagers. Source/native binaries remain the verified
8e86f96f build; this is a packaging-resource correction, with replacement signatures/archives
and new receipts required. An unsigned receipt stating compatibility is not a substitute for
signed Info.plist fields. Initial missing-repository negative smoke and typed field checks passed.


## Installed 0.18 verification — 2026-11-02

| Workstream | State | Evidence |
| --- | --- | --- |
| Standard and Preview installation | Deployed | Parent rollout installed both variants on both Macs with verified backups; independent post-install strict signatures and native hashes match corrected R2 candidates |
| Installed panel/helper smoke | Verified | Four isolated instances (two variants on each Mac) opened a synthetic PDF, rendered a signed helper child and accessible composer, typed/cleared a synthetic draft, resized, opened empty connection Settings and exited normally |
| Theme and Retina rendering | Verified | Initial rapid-toggle screenshot captured the previous asynchronous helper frame. Separate two-second-settled dark/light captures on both Macs visually match host/helper theme; screenshots are 2200×1560 for the 1100×780 logical window |
| Installed document MCP | Verified | Each installed CLI lists 183 tools; eight temporary-root calls create/set metadata/inspect/render/save/close/list/reopen pass. Metadata mutation, PNG result and empty closed-session state checked |
| Isolation and cleanup | Verified | Only public component.json/sources.json copied to disposable AI roots; connections explicitly empty, no inference or personal credential/settings inspection. Existing process baselines preserved; all owned instances quit normally and released authenticated OS-assigned loopback listeners; helper uses private pipes |
| Remaining native acceptance | In progress | No new live-provider inference, clipboard, native composition or VoiceOver session was run. This smoke does not claim the three-app assistant-only update/recovery demonstration or a public release |

Installed native code/source: `8e86f96f564346bf893422dc0b3359d89bf64287`; shared source dependency
`986cb16234cfdac1ee1337fb4cdf00945cad4e5a`; pinned font input
`b076417b5880bd4154bd22f00695b1d28c6bea83`. Corrected packaging source
`7881a92305d468588cb5c271baa0b0689e542a72`. Signed Info.plists contain integer
`CraftAssistantBridgeProtocol = 1` and `CraftUpdateRepository = andykumeda/pdfcraft`.
Settings/history schema requirements are 1. Local certificate identity is
`D7B1B5524FFEB37AB01D3960017BDF6743E3EDFD`; no Apple Team ID/notarization is claimed.
Its private key remained on Mini. Neither packager changes app-update source settings or client config.

| Installed binary (identical on both Macs) | SHA-256 |
| --- | --- |
| `/Applications/PdfCraft.app/Contents/MacOS/PdfCraft` | `8e2097656384eb987ce382c7cf869f6536abe02bbaf536b994762ecb815cab3e` |
| `/Applications/PDFCraft Assistant Preview.app/Contents/MacOS/PDFCraft-native` | `88147558af8743988035e2b7c8e0ae0bf79371190fcfb291812706b1de8445fd` |
| `~/.local/share/pdfcraft/0.18.0/pdfcraft-cli` | `fff6d7d3bdfb797070defb8730151323c6632c98d0cb74b6421df718279fc13d` |

The Preview retains bundle ID `ai.storyteller.pdfcraft.assistant-preview`, its per-user document
profile and `--disable-updates`; both variants retain shared Craft AI settings/history architecture.
The smoke used disposable overrides to avoid reading personal connections or histories.
All 11 asset licence paths, five bundled licence documents and 13 pinned-font notices were
verified in signed extracted bundles. Standard includes two SHA-256-verified ONNX OCR models.
Restricted upstream marks are absent. No new font file or screenshot was committed.

Corrected local artifacts (initial non-R2 app bundles are superseded):

- Root: `/Users/andy/Dev/pdfcraft/dist/assistant-candidates/0.18.0-8e86f96f-r2/`.
- Standard DMG: `pdfcraft-0.18.0-macos-aarch64.dmg`, SHA-256 `3e247ae1f5718625041f93e4d4ea5dab793fef6410965f288535eda3c691d702`.
- Preview DMG: `pdfcraft-assistant-preview-0.18.0-8e86f96f-macos-aarch64.dmg`, SHA-256 `97c31410b9bd17c2dd3052e2346dc39cb8df99c95bc4d696527571e5d37f572b`.
- Full bundle/archive/public-certificate evidence: `app-receipt.json` in that directory.
- Matching CLI receipt/archive: `/Users/andy/Dev/pdfcraft/dist/assistant-candidates/0.18.0-8e86f96f/cli-receipt.json`; archive SHA-256 `201dea44a847210065387108ae24b45df6f87ef3b8559302a3a4d0d61a77b3d3`.

Native smoke receipts:

- Mini: `/var/folders/cg/796c5bys74db15ww0b_qvw8c0000gn/T/pdfcraft-installed-smoke-mcj99u91/report.json`.
- MacBook: `/var/folders/z6/7ct4dgns2236tw0d27zdb9s40000gn/T/pdfcraft-installed-smoke-74y2v9qq/report.json`.
- Settled-theme follow-ups: Mini `/var/folders/cg/796c5bys74db15ww0b_qvw8c0000gn/T/pdfcraft-installed-theme-38ryyoo2/`; MacBook `/var/folders/z6/7ct4dgns2236tw0d27zdb9s40000gn/T/pdfcraft-installed-theme-misa19qd/`.
- MacBook synthetic screenshots/report copied for review only to `/private/tmp/pdfcraft-installed-smoke-macbook-evidence/`.

Service inventory entries on each Mac record owned PIDs/ports and normal cleanup without tokens.
`~/.codex/LOCAL_SERVICES.md` is outside this Git repository and is not committed here.

### Compact chat layout — 2026-11-03

| Workstream | Owner | State | Acceptance / verification | Release |
| --- | --- | --- | --- | --- |
| Shared UI and settings/update placement | Primary | Verified | craft-libs e511754 + d771dc9: 486 UI checks and 30 helper checks passed; strict scoped lint and synthetic dark/light/narrow screenshots passed; negative control confirmed update visibility test | Source pushed; not installed |
| PDFCraft shared revision integration | Primary | Verified | d771dc9 pin and lockfile; 29 assistant tests, strict native UI lint, WASM, layers, assets, format and diff checks passed | Source only; not deployed |

No engine/tool contract, provider routing, credential or personal history change. Existing
installed builds remain unchanged; host rebuild and a compatible helper installation are
required for the complete new layout. Real installed-app and signed-update smoke remain pending.

Final PDFCraft validation used the locked d771dc9 Git dependency. WASM compilation passed
with unused/dead-code warnings in unchanged platform-specific render/UI code; native UI
Clippy passed with warnings denied. Layering checked 37 crates. Asset validation passed
(281 repository, 6 dependency-bundled and 22 build-time assets). No new assets or dependencies
were added. The installed app/helper and signed-update flow were not exercised for this revision.

### Two-Mac compact-layout delivery — 2026-11-03

User authorized installing the verified layout on both Macs. The shared helper is versioned
0.1.1 (protocol/settings/history1), with immutable 0.1.0 retained. Source pin83e61084 includes
the layout, backwards-compatible update visibility and this release identity. App packaging
will preserve local certificate trust, fork metadata, required open fonts and existing CLI
selection. Global rollout state is in `../craft-libs/docs/two-mac-rollout.md`; app candidates
and native acceptance are in progress, with no installed replacement claimed yet.

PDFCraft host preparation passed29 assistant tests, native UI Clippy with warnings denied,
WASM UI compilation (warnings in unchanged platform code), layers37, assets281+6+22,
format and diff checks. Optimized signed desktop candidates are next; installed host
replacement and native settings-only update checks remain pending. Existing CLI is retained
because its tools/control protocol are unchanged and installed CLI smoke passed on both Macs.

### Installed compact-layout acceptance — 2026-11-03

This checkpoint supersedes the earlier source-only and candidate-preparation status above.

| Workstream | Owner | State | Evidence / release |
| --- | --- | --- | --- |
| Compatible host and helper integration | Primary | Deployed | PDFCraft71cc43cc pins shared83e61084; locally signed standard and Preview atomically installed on both Macs, verified metadata/signatures/hashes; helper0.1.2 installed separately |
| Native acceptance, both variants on both Macs | Primary | Verified | New chat, Chat history, Add files, Choose model and gear accessible; menus, draft typing/clear, resize/theme, actual update-check result and hiding all update copy after closing Settings passed |
| Synthetic document automation | Primary | Verified | Existing installed CLI retained;183 tools listed and8 create/metadata/render/save/close/reopen calls passed independently on each Mac |

Native receipts: Mini `/var/folders/cg/796c5bys74db15ww0b_qvw8c0000gn/T/pdfcraft-installed-smoke-suucytsw/report.json`; MacBook `/var/folders/z6/7ct4dgns2236tw0d27zdb9s40000gn/T/pdfcraft-installed-smoke-2fpep4p7/report.json`. Every test-owned instance quit normally and released its loopback listener. Public component/source settings were copied into empty disposable profiles; no provider request or personal history was used.

Signed native SHA-256: standard `8fde954589c52890d77d9d8491f551a5a9b58688fd140dd4ace6c82297883730`; Preview native `d8fa1f809cab3debdf2895a99aa3d873860bb906702040d97ef0f75abc63de25`. Exact package and installation receipts are under `/private/tmp/craft-compact-rollout-20261103/`. Previous apps are retained under `~/Applications/Craft Rollout Backups/2026-11-03-compact/`. Settings/chats/credentials and existing CLI selection were preserved.

Helper0.1.2 source3dda16ac changes navigation only and requires no additional host rebuild. The separately installed helper update was verified on both Macs. Local certificate trust remains explicit; no notarization or public release is claimed. Live provider authentication/inference, native IME, clipboard and VoiceOver remain manual checks.

### Assistant toolbar refinement — 2026-11-04

Owner: Primary. State: **Verified** (source). Scope: replace the mode-bar text control with the
existing attributed Lucide `sparkles` icon through the shared square icon-button helper.
Acceptance: an accessible Assistant name and tooltip, selected appearance, and unchanged
pointer/keyboard toggling at 800 and 1440 points in light and dark themes. No engine,
automation API, provider, settings, or document changes are required. Installation/release
remain separate from this source update.

Verification: UI library tests **731 passed, 1 ignored**; the focused toolbar regression
passed all four width/theme cases, including pointer and keyboard activation. Before the
change the same regression failed on the old 76.25-point text control versus 32 points.
Scoped all-target Clippy with `-D warnings`, formatting, layers (37 crates), assets
(281 repository + 6 dependency + 22 build-time), and scoped WASM compilation passed.
WASM reports 11 warnings in unchanged platform code. Headless real-shell screenshots at
800×600 light and 1440×900 dark were rendered and inspected. No native VoiceOver check,
provider inference, app installation, or public release was performed for this layout change.
Local check logs/screenshots: `/tmp/pdfcraft-toolbar-*`. UI tests used a command-line-only
package opt-level 0 override; build defaults and dependencies were unchanged.

### MacBook helper-only repair — 2026-11-04

Shared helper 0.1.3/source `aeb7d47` was signed and installed on the MacBook to stop idle model
picker flicker and discover Codex bundled with ChatGPT in GUI sessions. Installed signature,
three host handshakes and synthetic standard PdfCraft native acceptance passed, including 32
idle samples with the model control enabled. Existing apps/configuration were unchanged;
helper 0.1.2 remains for rollback. Close/reopen Assistant to adopt it in an existing window.
Live authentication/inference remains untested. See the [shared rollout ledger](https://github.com/andykumeda/craft-libs/blob/codex/independent-assistant/docs/two-mac-rollout.md)
for per-device receipts. The toolbar change above remains source-only, not installed.
