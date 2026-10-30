# Integrated assistant task ledger

Updated 2026-10-29. Private preview integration; no external publication authorized.

Reference: WordCraft `4dd7cf7d`, craft-libs `bf932b91643c287502d539b88f08abd5796c9447`.
PDFCraft baseline `726ecf26`; local `plan/` source documents are absent.

| Workstream | Owner | State | Acceptance and verification |
| --- | --- | --- | --- |
| Shared-session automation | Automation workstream | Verified | Validated existing tools operate on GUI Session; synthetic mutation, read-back, Undo and save/reopen tests |
| Native panel and Settings | Integration/native workstreams | In progress | Visible toolbar/View toggle; shared single-model settings, provider routing, native safety and WASM gates |
| Preview isolation/package | Packaging workstream | In progress | Distinct profile/bundle, disabled updater, ARM64 DMG/checksum; mounted and extracted verification |
| Acceptance and installation | Integration | Not started | Synthetic UI/providers/read/edit/Undo/save/reopen; normal app unchanged; rollback retained |

Allowed states: Not started, In progress, Blocked, Verified, Deployed.

## Current verification

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
- Native visual/provider acceptance and packaging remain pending. Source commits are being prepared; no installation or public release has occurred.
- Read-only compatibility discovery: Codex CLI 0.165.0 is available; Ollama 0.40.13 reports installed tool-capable `qwen3:4b-instruct-2507-q4_K_M`. No Codex inference or model download was performed.
- No preview has been packaged or installed. No private document or live inference used.
