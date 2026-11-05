# Assistant top-toolbar placement — 2026-11-05

Status: **In progress**. Owner requests the Assistant icon alongside the title-bar theme control in PDFCraft/GridCraft, beside Discord in WordCraft, and investigation of the unavailable DeckCraft assistant on MacBook. One normal integrated application per product remains required; no Preview installations.

| Workstream | Owner | Acceptance and verification | State | Delivery |
| --- | --- | --- | --- | --- |
| PDF toolbar | Primary | One accessible sparkle toggle beside theme; click/keyboard; narrow/wide light/dark; scoped Rust gates and real UI | Deployed | Nativedacec56f; standard archive and independent installed smoke pass both Macs |
| Grid toolbar | Grid agent | One icon beside theme; toggle/accessibility/layout regressions and scoped gates | Deployed | Nativef43086bc; docs51939848; independent normal-path checks both Macs pass |
| Word toolbar | Word agent | One icon beside Discord; toggle/accessibility/layout regressions and scoped gates | In progress | Pending |
| Deck MacBook | Deck agent | Verify installed/running app and helper; restore observable assistant without losing work | Deployed | Source86a35ba2; signed standard app, actual top-toolbar/Home toggle and helper0.1.7 checked independently on both Macs |

Each product owns its source/tests and detailed evidence. Builds and installation are separate checkpoints. Preserve profiles, recovery, shared AI configuration, credentials and documents; never terminate a user-owned session to install.

PDF local `plan/STATUS.md`, `plan/architecture.md`, `plan/execution-plan.md` and crate README are absent. Current installation baseline is [single-app-assistant-rollout.md](single-app-assistant-rollout.md); it is historical evidence, not proof of current runtime behavior.

## PDF source checkpoint

- Reused the existing Lucide sparkle control in the title bar beside Display theme; removed its old mode-bar instance. Reserved space for the additional icon and gap outside scrolling document tabs.
- Existing Assistant command, selected state, accessible name, tooltip and panel behavior are retained. No new asset, dependency, provider configuration or document path.
- Regression observed failing against old placement (`32 != 28`), then passed at 600/1440 points in light/dark with mouse and keyboard toggling. Logs: `/private/tmp/pdf-toolbar-{red,green}.log`.
- Scoped final checks passed: `cargo fmt --check`; `cargo clippy -p pdfcraft-ui-egui -j 2 --all-targets -- -D warnings`; `cargo test -p pdfcraft-ui-egui -j 2` (2,236 passed, five ignored); `cargo xtask layers`; `cargo xtask assets`; `cargo check -p pdfcraft-ui-egui --target wasm32-unknown-unknown -j 2` (existing WASM warnings). Logs: `/private/tmp/pdf-toolbar-checks.log` and `/private/tmp/pdf-toolbar-check-{0..5}.log`.
- Actual shell light 600-point and dark 1440-point renders reviewed: `/private/tmp/pdf-toolbar-{light,dark}.png`; sparkle is adjacent to theme and visible. Native installed checks, packaging and delivery remain pending.

## Cross-app installation checkpoint

- Grid source `f43086bc7fce24d273549653956382106b7a3922` passed all ten CI gates and four source renders. Corrected signed package includes all 13 font licences; an initial package missing licence payloads was rejected and removed. Candidate native checks passed. Installed through the normal managed launcher on both Macs, exact native SHA `fe4f83914ed46259d99cd242381d8dc94487c19fa251d8036f378c724909ec48`; installed smoke passed independently on both Macs (pointer toggle, helper0.1.7, light/dark, synthetic/protected-data preservation, exit/listener cleanup); Grid docs commit51939848. Per-host verified rollback ZIPs/receipts: `~/Library/Application Support/Craft/retired-apps/2026-11-05-top-toolbar.noindex/`.
- Deck source `86a35ba2` passed all eight CI gates. Actual installed-path checks independently pass on both Macs: top toggle open/close, Home with no presentation, helper0.1.7, 600/1440 light/dark screenshots/render, normal exit and released listeners. Exact native SHA `f63f1d94ba3acf141d2a7eca79a4a6b30182f4a0ddc2b614d24eab9ba958f92e`; all 35 functional resources match previous bundles. Protected profiles/shared AI data unchanged; no inference. Rollback ZIPs per host: `~/Library/Application Support/Craft/retired-apps/2026-11-05-deck-toolbar.noindex/`.
- Word source `847526d8838553ad2c15e8d3ab7db9795d3e7f48` passed all nine CI gates (5,202 native tests; nine ignored), four 600/1440 light/dark renders, and the red/green placement/keyboard regression. Optimized build and signed native delivery pending.
- PDF first optimized build exposed a stale cached build-script path in contributor-credit generation. Its package is not eligible. Touching the unchanged build script forced a correct build-script compilation; generated contributor tables are populated (26,226 bytes). Corrected optimized build/package is in progress. Source/test code is unchanged.

## PDF two-Mac delivery

Corrected standard ZIP/DMG at `dist/integrated/toolbar-corrected-dacec56f/` passes exact identity/source/protocol/base tag, signatures and the five-file OCR payload regression. Contributor tables are populated; the earlier ineligible package was removed (receipt `/private/tmp/pdf-toolbar-package-rejected.txt`). Native source `dacec56f353721181de2716142b952dff16536dd`, signed SHA `fd5853ad6be1057665b777cc572c762ec4a1e959cce92f4ab2fbdb3d60792820` is identical on both Macs. The installed managed launcher performed both replacements; per-host verified rollback ZIPs/receipts are in `~/Library/Application Support/Craft/retired-apps/2026-11-05-top-toolbar.noindex/`.

Independent installed-path smoke passed on Mini and MacBook: exactly one named title-bar sparkle at 4 points from theme, actual click open/close, signed helper composer typing/clearing without sending, light/dark captures, 600-point narrow placement, empty connection settings and synthetic PDF readback. Both normal exits/listener releases passed and private control keys were removed. Normal/retired-Preview profiles and shared AI hash snapshots are byte-identical before/after. Candidate registration/extraction is removed. Reports/screenshots:

- Mini: `/var/folders/cg/796c5bys74db15ww0b_qvw8c0000gn/T/pdfcraft-top-toolbar-6086ea3s/`.
- MacBook: `/var/folders/z6/7ct4dgns2236tw0d27zdb9s40000gn/T/pdfcraft-top-toolbar-nxz0xg8s/`; copied evidence `/private/tmp/pdf-toolbar-book-installed/` on Mini.

No fresh model inference, physical VoiceOver/IME/clipboard or private-document fidelity check was performed. No public release/notarization. Deck source is committed locally because its existing upstream remote is unwritable; PDF/Grid source and documentation use existing writable fork branches.
