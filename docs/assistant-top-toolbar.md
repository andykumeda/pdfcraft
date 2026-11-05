# Assistant top-toolbar placement — 2026-11-05

Status: **In progress**. Owner requests the Assistant icon alongside the title-bar theme control in PDFCraft/GridCraft, beside Discord in WordCraft, and investigation of the unavailable DeckCraft assistant on MacBook. One normal integrated application per product remains required; no Preview installations.

| Workstream | Owner | Acceptance and verification | State | Delivery |
| --- | --- | --- | --- | --- |
| PDF toolbar | Primary | One accessible sparkle toggle beside theme; click/keyboard; narrow/wide light/dark; scoped Rust gates and real UI | In progress | Pending |
| Grid toolbar | Grid agent | One icon beside theme; toggle/accessibility/layout regressions and scoped gates | In progress | Pending |
| Word toolbar | Word agent | One icon beside Discord; toggle/accessibility/layout regressions and scoped gates | In progress | Pending |
| Deck MacBook | Deck agent | Verify installed/running app and helper; restore observable assistant without losing work | In progress | MacBook installed-path panel smoke passed; View ribbon absent on startup Home, so toggle is unreachable there. Moving Deck toggle to title bar |

Each product owns its source/tests and detailed evidence. Builds and installation are separate checkpoints. Preserve profiles, recovery, shared AI configuration, credentials and documents; never terminate a user-owned session to install.

PDF local `plan/STATUS.md`, `plan/architecture.md`, `plan/execution-plan.md` and crate README are absent. Current installation baseline is [single-app-assistant-rollout.md](single-app-assistant-rollout.md); it is historical evidence, not proof of current runtime behavior.

## PDF source checkpoint

- Reused the existing Lucide sparkle control in the title bar beside Display theme; removed its old mode-bar instance. Reserved space for the additional icon and gap outside scrolling document tabs.
- Existing Assistant command, selected state, accessible name, tooltip and panel behavior are retained. No new asset, dependency, provider configuration or document path.
- Regression observed failing against old placement (`32 != 28`), then passed at 600/1440 points in light/dark with mouse and keyboard toggling. Logs: `/private/tmp/pdf-toolbar-{red,green}.log`.
- Scoped final checks passed: `cargo fmt --check`; `cargo clippy -p pdfcraft-ui-egui -j 2 --all-targets -- -D warnings`; `cargo test -p pdfcraft-ui-egui -j 2` (2,236 passed, five ignored); `cargo xtask layers`; `cargo xtask assets`; `cargo check -p pdfcraft-ui-egui --target wasm32-unknown-unknown -j 2` (existing WASM warnings). Logs: `/private/tmp/pdf-toolbar-checks.log` and `/private/tmp/pdf-toolbar-check-{0..5}.log`.
- Actual shell light 600-point and dark 1440-point renders reviewed: `/private/tmp/pdf-toolbar-{light,dark}.png`; sparkle is adjacent to theme and visible. Native installed checks, packaging and delivery remain pending.
