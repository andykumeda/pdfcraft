# Assistant-integrated updates — 2026-11-09

**Resumed at the owner's request.** The owner subsequently authorized parallel builds and full
available CPU. Preserve source candidates and caches until verification completes, then perform
the requested cleanup.

The official releases diverged from the older releases on which the installed assistant builds
were based. The protected launcher rejected the resulting automatic merges. Reconciliation must
retain the official release, assistant, safe update route and user documents. The existing
installations remain in place until a candidate is built, checked, signed and admitted by the
protected route.

| Workstream | State | Acceptance and current evidence |
| --- | --- | --- |
| Shared libraries | Verified | Merge `b3474c019d28054396931e8e295cec1b44e60009` passed locked workspace Clippy and split verification: 2,884 tests passed, 34 ignored, and one mode-bit test passed separately. Earlier failures and test-environment isolation are recorded in its reconciliation document. The owner explicitly approved publication, and the commit was pushed to `andykumeda/craft-libs` branch `codex/assistant-upstream-20261109`. |
| GridCraft 0.18.0 | Deployed | Source merge `b28dd31f` passed all ten CI gates with 3,655 tests passed and four ignored. Signed app and matching CLI are installed on both Macs; both configured MCP routes passed synthetic workbook edit/readback, and both assistant panels passed toolbar/helper handshake checks. No live inference was run. The manual one-job build did not exercise the two-job launcher adapter. Delivery record `7a0adae3` is pushed. Automatic review rejected removal of redundant staged old bundles; verified rollback ZIPs remain. |
| WordCraft 0.17.0 | Deployed | Full local CI passed all nine stages with exact shared, font and translation inputs. Signed source `b291ac20` and its matching CLI are installed on both Macs; both configured MCP routes passed synthetic edit/undo/redo/save/reopen/render, and both assistant panels passed the helper handshake. No live inference was run. Ambiguous multi-window turns and queued visual input are conservatively restricted. Delivery record `cd218e51` is pushed; rollback ZIPs and prior staged bundles are retained. |
| PdfCraft 0.20.0 | Deployed | Corrected source `b09ee0ec` passed local CI; optional cargo-deny was unavailable and skipped. Signed app and matching CLI are installed on both Macs; configured MCP routes passed synthetic PDF edits/save/reopen, assistant/helper smoke passed, and OCR model hashes matched. No live inference was run. Existing old CLI processes on Mini must restart to use 0.20.0. Delivery record `73ef1ca4f` is pushed. |
| DeckCraft 0.17.0 | Deployed | Reviewed two-parent merge `e035264a` passed all eight CI stages with pinned fonts. Signed app and matching CLI are installed on both Macs; configured MCP routes and assistant/helper checks passed with synthetic presentations. No live inference was run. Assistant-origin self-control and ambiguous queued visual input are blocked. Delivery record `83c2985d` is committed locally; no upstream push or new remote. |
| ArtCraft Launcher 0.17.0 | Deployed | Source commit `4b51723` passed native and UI suites, scoped strict Clippy, formatting, WASM and synthetic adapter checks. Its locally signed package was verified and installed on both Macs; installed certificate, marker, source and native hash passed independently. The live assistant-preserving route was checked on Mini; MacBook live UI remains untested. Existing auto-install settings triggered protected app-update attempts on Mini startup; conflicts and a running PdfCraft prevented replacement. |
| VectorCraft 0.21.0 | Deployed | Signed assistant-enabled source `5f6cb582` and matching CLI installed on both Macs with guarded managed update policies. Parent `6cefb540` passed all 12 CI gates; the final panel-width fix passed 17 assistant tests and feature-enabled Clippy. Both installed apps passed synthetic SVG edit/undo, visible panel, helper and safe-quit checks; the signed CLI initialized isolated MCP with 34 tools. No live inference or persistent MCP registration was performed. Source is committed locally on `codex/vectorcraft-assistant`; no personal fork is configured. Policy backups and guarded reversal stages remain on both Macs. |
| CLI and MCP delivery | Deployed | Matching Grid 0.18.0, Word 0.17.0, PDF 0.20.0 and Deck 0.17.0 CLIs and their existing MCP routes were installed and tested on both Macs; existing wrappers and connection options were retained. Mini's preexisting PDF 0.18.0 MCP processes remain running and must restart to use the new executable. VectorCraft 0.21.0 CLI is installed on both Macs and passed isolated MCP initialization; no MCP registration was created. |

## Final delivery and cleanup status

The launcher and installed apps use assistant-preserving source routes; exact release and
assistant ancestry was verified before installation. VectorCraft's official release is the
freshness source and its local assistant branch is the managed build source. Its full CI passed
on the parent of the final small panel-width fix; the final fix received focused tests, Clippy,
and native signed-app acceptance on both Macs. The launcher Vector adapter has not yet performed
a live update. Live model inference was not tested. Existing Mini PdfCraft 0.18.0 MCP processes
must restart to use the installed CLI.

Cleanup remains incomplete. Automatic command review rejected deleting the task-owned PdfCraft
CI cache (`/Volumes/X10 Pro/codex-craft-build-20261109/pdfcraft-ci-target`) because `rm -f` style
commands were not permitted; no alternate deletion was attempted. The PdfCraft release cache
and local temporary cache remain too. Earlier automatic review also rejected removing the two
Grid staged old-app and rollback-check paths on each Mac. Preserve all independently verified
rollback ZIPs, policy backups, Vector reversal stages, and user data. Other task-owned caches,
checkouts, and stages remain pending a permitted cleanup.

The following checkpoint is historical and does not describe current deployment status.

## Pause checkpoint

- GridCraft post-fix CI passed the translation report, formatting and workspace Clippy; tests
  were still compiling when stopped. The corrected socket test passed ten consecutive runs.
- WordCraft's two full CI attempts passed formatting and Clippy but failed on startup-argument
  timeouts; 20 direct reruns passed and the cause remains unresolved. The targeted investigation
  was compiling when stopped. The focused assistant suite and GUI/CLI checks passed.
- PdfCraft CI passed formatting, all-target Clippy and the CLI no-default-features check; its
  workspace tests were still compiling when stopped. Resume details are in the temporary
  `pdfcraft-ci-paused-20261109.txt` until cleanup.
- VectorCraft's prototype passed focused tests and scoped Clippy but still needs full gates,
  pinned published shared inputs, matching CLI, packaging and live acceptance.
- Shared commit `b3474c019d28054396931e8e295cec1b44e60009` has since been published to the
  existing `andykumeda/craft-libs` fork with explicit owner approval. The user asked for cleanup
  after builds complete, so the external task build folder, staged source and verified rollback
  archives remain in place.
- After resumption, the Grid CI rerun passed i18n reporting, formatting and workspace Clippy; it
  was stopped during workspace UI tests. Word's restarted build was stopped during Clippy.
  A later Deck source review found that helper-origin generic commands can reach assistant
  controls; that unverified recursion route needs a fix and test before release.
