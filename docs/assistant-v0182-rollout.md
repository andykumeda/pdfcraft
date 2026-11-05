# PdfCraft 0.18.2 Assistant Preview rollout

Status: In progress. Updated: 2026-11-04.

Owner request: latest apps on Mini and MacBook, preserving custom assistant behavior and all writing.
Official tag `v0.18.2` resolves to `7faa4f35cefb0dea04137ec475bdc770945c22cd`.
Standard official apps are installed by the suite coordinator. This repository owns only the
separate `/Applications/PDFCraft Assistant Preview.app` update and its source reconciliation.

| Workstream | State | Evidence / acceptance |
| --- | --- | --- |
| Source reconciliation | Verified | Six conflicts resolved; preserve custom assistant and reduced-motion Preferences behavior; cargo check desktop/CLI passed |
| Gates | Verified | Full `cargo xtask ci`: all 10 steps passed; assets passed; separate exact cargo-deny policy check passed all categories |
| Candidate | Not started | Stamped source build, signed ZIP/DMG, exact hashes and extracted/mounted/copied bundle verification |
| Mini native qualification/install | Not started | Actual configured Terra comment removal, Undo/Redo, no save until requested, save/reopen/render, preserved profiles and rollback |
| MacBook native qualification/install | Not started | Same independent checks; never touch the owner's open WordCraft instance |
| Public release | Not started | No upstream-main push, new fork or public release |

The merge retains the fork's assistant/native/UI libraries at compatible protocol1 revision
`83e61084`; menu/keymap/new translation support follows upstream `787a7a0f`. The existing
assistant requirement needs Tokio1.53.2, so only that locked runtime version is raised from
upstream1.53.1. Generated attribution is regenerated from the merged manifest, not hand-edited.
Preferences retains the upstream generic scroll/reduced-motion helper while the caller keeps
Assistant settings and its Back control. Both roadmap histories are preserved.

Inputs: fonts `b076417b5880bd4154bd22f00695b1d28c6bea83`, translations
`575cfdef63227b14daf173b8cc87928bfdfa325e`. No font files are committed. Restricted fork branding
stays removed and the packager checks the binary. Helpers/configuration/conversations stay
independent; helper0.1.6 is already deployed on both Macs. Settings/help now describe the normal
Codex profile, not the retained legacy isolated Craft sign-in.

Logs: `/private/tmp/pdfcraft-v0182-{check,assets,ci,deny,release-warm}.log`.
Native fixture `/private/tmp/pdfcraft-v0182-native-smoke.py` uses public config copies only,
normal Codex HOME/CODEX_HOME and a private workspace. Independent latest-CLI fixture creation
and one-comment read-back passed before GUI acceptance. Working-source GUI acceptance passed with actual Codex/Terra/helper0.1.6 in 26.52s: comment removed, Undo/Redo, unchanged disk until explicit save, independent saved/reopened read-back and rendered screenshots. A first fixture hit a read-only state admission deadline; only explicitly unapplied reads received a bounded retry on a fresh fixture. Final signed-candidate qualification remains pending.

Initial full CI found two upstream integration mismatches. Preferences coverage now follows native macOS Settings placement (other platforms retain Edit/Preferences). Standard repository update results reuse the shared dialog; custom-source routing and disabled-preview policy remain intact. Both focused regressions and the full rerun pass. Logs: `/private/tmp/pdfcraft-v0182-ci-final.log`, `/private/tmp/pdfcraft-v0182-regressions-restored.log`, `/private/tmp/pdfcraft-v0182-deny-direct.log`.
