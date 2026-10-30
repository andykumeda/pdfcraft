# Integrated assistant task ledger

Updated 2026-10-29. **Implemented, tested, packaged and Deployed locally.**
Current delivery: **0.15.0 (`dd68eaed`)**, `/Applications/PDFCraft Assistant Preview.app`.
No public release was created. The user subsequently authorized a GitHub fork and branch push; see the source distribution record below. Live Codex/hosted-provider and other-Mac acceptance remain open.

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
