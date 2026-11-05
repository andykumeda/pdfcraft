# PdfCraft 0.18.2 Assistant Preview rollout

Status: Deployed on Mini and MacBook. Updated: 2026-11-04.

Owner request: latest apps on Mini and MacBook, preserving custom assistant behavior and all writing.
Official tag `v0.18.2` resolves to `7faa4f35cefb0dea04137ec475bdc770945c22cd`.
Standard official apps are installed by the suite coordinator. This repository owns only the
separate `/Applications/PDFCraft Assistant Preview.app` update and its source reconciliation.

| Workstream | State | Evidence / acceptance |
| --- | --- | --- |
| Source reconciliation | Verified | Six conflicts resolved; preserve custom assistant and reduced-motion Preferences behavior; cargo check desktop/CLI passed |
| Gates | Verified | Full `cargo xtask ci`: all 10 steps passed; assets passed; separate exact cargo-deny policy check passed all categories |
| Candidate | Verified | Source32b9ca14; exact local signer, ZIP extraction, mounted/copied DMG, arm64 and source/version verified |
| Mini native qualification/install | Deployed | Actual configured Terra comment removal, Undo/Redo, no save until requested, save/reopen/render, preserved profiles and rollback |
| MacBook native qualification/install | Deployed | Same independent checks; never touch the owner's open WordCraft instance |
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

## Signed delivery checkpoint

Source `32b9ca14cbd5cb3609b29978f66c2d1dfe8ab40b` is committed and pushed to the existing
`fork/codex/independent-assistant`. Full CI reports6586 passing tests and16 ignored. Its optional
cargo-deny wrapper skipped the absent global executable; the separate exact policy gate passed
using the verified temporary cargo-deny executable. No dependency-policy check is claimed from
the skipped wrapper itself.

The packager rejected an unstamped binary and then missing repository metadata before delivery.
The final build uses documented `PDFCRAFT_BUILD_LABEL=32b9ca14`; packaging uses
`CRAFT_UPDATE_REPOSITORY=andykumeda/pdfcraft`. No rejected package was installed.

Artifacts: `dist/assistant-preview/32b9ca14/`. Native SHA-256:
`94d2b2f7fdf27919ff822d4c647ad117705773a446a03d85728a70e09013f1e4`.
DMG SHA-256: `31dac5f15399838e7584d3ab062eb2819980b7655b60a16dbd124e28294498ac`.
Exact signer leaf `D7B1B5524FFEB37AB01D3960017BDF6743E3EDFD`, identifier
`ai.storyteller.pdfcraft.assistant-preview`. This is local signing, without notarization or a public release.

Mini signed-candidate actual Codex/Terra/helper0.1.6 acceptance passed in23.49s. The requested
comment disappeared visibly, Undo restored it, Redo reapplied it, disk bytes stayed unchanged
until explicit save, independent latest-CLI readback and reopen/render passed. Both screenshots
were inspected. The owned process exited normally and its loopback listener released. Receipt:
`/var/folders/cg/796c5bys74db15ww0b_qvw8c0000gn/T/pdfcraft-v0182-native-hhma5blm/report.json`.
Installed `/Applications/PDFCraft Assistant Preview.app` signature/native hash match. Previous0.18.0
source71cc43cc retained intact at `/Users/andy/Library/Craft/app-update-backups/2026-11-04/pdfcraft-assistant-preview/previous-my0exwi8/PDFCraft Assistant Preview.app`.
Both hosts’ standard app, shared public settings and helper active pointer hashes are unchanged.
DMG/checksum/instructions are in both Downloads. Mini installed-path actual Terra confirmation passed in28.65s;
MacBook signed-candidate Terra passed in23.82s and installed-path Terra passed in20.21s. Preview launcher retains separate profile and
`--disable-updates`; native external actions remain outside document Undo. Physical VoiceOver,
IME and clipboard checks were not repeated.


## Final two-Mac acceptance

Mini installed receipt: `/var/folders/cg/796c5bys74db15ww0b_qvw8c0000gn/T/pdfcraft-v0182-native-md0bcadg/report.json`.
MacBook installed receipt: `/var/folders/z6/7ct4dgns2236tw0d27zdb9s40000gn/T/pdfcraft-v0182-native-utxdgrmh/report.json`.
MacBook first Preview installation receipt: `/var/folders/z6/7ct4dgns2236tw0d27zdb9s40000gn/T/pdfcraft-v0182-install-4rysqvxw/install-receipt.json`.
Both installed natives have the same source/version/SHA-256 above. Both actual model runs verify
exact comment removal, unchanged disk before requested save, Undo/Redo, independent saved count0,
reopen/render, private control UID/mode600, normal exit and released loopback listener. MacBook
also verifies the listener belongs to the owned QA PID. Candidate after/Undo PNGs on both hosts
and Mini installed reopen PNG were reviewed; Book installed render evidence is retained beside
its reports at `/private/tmp/pdfcraft-v0182-book-delivery/installed-evidence/` on Mini.
MacBook had no old Preview, so no existing profile/bundle was replaced. Mini prior Preview was
retained. No app/helper routing or public settings changed; no credential/history copy.

Final suite inventory independently matches PDF Preview source/native hashes on both Macs.
The only remaining suite update is unrelated standard WordCraft0.15.0 on MacBook, still open;
it is preserved pending the owner saving/quitting. Standard PDF0.18.2 is already installed both.
The task-owned temporary PDF release target was removed after packaging; signed packages,
checksums, receipts, pinned inputs and rollback bundles remain. No public release or notarization.
