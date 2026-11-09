# Assistant-integrated updates — 2026-11-09

**Resumed at the owner's request.** Run builds serially with one job and low priority to limit
impact on the owner's work. Preserve source candidates and caches until verification completes,
then perform the requested cleanup.

The official releases diverged from the older releases on which the installed assistant builds
were based. The protected launcher rejected the resulting automatic merges. Reconciliation must
retain the official release, assistant, safe update route and user documents. The existing
installations remain in place until a candidate is built, checked, signed and admitted by the
protected route.

| Workstream | State | Acceptance and current evidence |
| --- | --- | --- |
| Shared libraries | Verified | Merge `b3474c019d28054396931e8e295cec1b44e60009` passed locked workspace Clippy and split verification: 2,884 tests passed, 34 ignored, and one mode-bit test passed separately. Earlier failures and test-environment isolation are recorded in its reconciliation document. The owner explicitly approved publication, and the commit was pushed to `andykumeda/craft-libs` branch `codex/assistant-upstream-20261109`. |
| GridCraft 0.18.0 | Deployed | Source merge `b28dd31f` passed all ten CI gates with 3,655 tests passed and four ignored. Signed app and matching CLI are installed on both Macs; both configured MCP routes passed synthetic workbook edit/readback, and both assistant panels passed toolbar/helper handshake checks. No live inference was run. The manual one-job build did not exercise the two-job launcher adapter. Delivery record `7a0adae3` is pushed. Automatic review rejected removal of redundant staged old bundles; verified rollback ZIPs remain. |
| WordCraft 0.17.0 | In progress | The focused assistant suite passed 36 tests (two ignored); screenshot binding and title-bar tests passed, and the document-switch negative control failed as expected. Ambiguous multi-window turns and queued input are conservatively restricted. Full gates and installation remain unverified. |
| PdfCraft 0.20.0 | In progress | Isolated UI check, 30 assistant tests, three updater tests, split-pane targeting and four shared-session automation tests passed. An OCR cache regression failed before its fix and passed afterward. Full gates, OCR packaging and installation remain unverified. |
| DeckCraft 0.17.0 | In progress | Isolated UI and desktop checks passed without optional craft-fonts. Assistant safety changes and test compilation are unverified; the build was paused to preserve disk space. Release build and installation remain unverified. |
| ArtCraft Launcher 0.17.0 | Deployed | Source commit `4b51723` passed native and UI suites, scoped strict Clippy, formatting, WASM and synthetic adapter checks. Its locally signed package was verified and installed on both Macs; installed certificate, marker, source and native hash passed independently. The live assistant-preserving route was checked on Mini; MacBook live UI remains untested. Existing auto-install settings triggered protected app-update attempts on Mini startup; conflicts and a running PdfCraft prevented replacement. |
| VectorCraft assistant | In progress | An isolated prototype includes a guarded panel, helper bridge and packaging drafts. Fifteen focused assistant tests, an MCP routing test, desktop check and scoped strict Clippy passed. Full gates, published newer shared input, packaging and live acceptance remain. |
| CLI and MCP delivery | In progress | Integrated GUI packagers do not update separate CLI executables. Grid 0.18.0's matching CLI and existing MCP routes were updated and tested on both Macs. Before further delivery, Mini's other registrations referenced PDF 0.18.0, Word 0.15.0 and Deck 0.9.0; the Deck binary failed signature verification. On MacBook the Word and Deck wrappers refuse startup because approval records are missing, while PDF's 0.18.0 wrapper starts. Build and verify the remaining matching CLIs, preserve each host's wrapper and connection options, update approved paths/records and exercise real MCP startup. |

Isolated candidates are under `/private/tmp/` and are not release artifacts. The VectorCraft
prototype additionally needs a tested non-reused view identity and end-to-end assistant, draft,
chooser, update and packaging checks. The launcher installer draft has passed eight synthetic
tests with signature/archive operations stubbed; its final process check still has a launch race,
so the launcher must remain closed throughout a real replacement. Only the launcher replacement
recorded above is claimed by this ledger. The launcher source adapter must see both the reviewed
assistant source and the exact official tag in the resulting history; an unreviewed patch or
ordinary upstream binary is not an eligible replacement. Both Macs need separate installed-path
verification if updated.

After builds and verification are complete, remove the task-owned temporary build caches,
checkouts and staging files, including the dedicated external-drive build folder. Retain committed
source and recovery backups required for installed apps.

To restore system-disk headroom during the work, the redundant internal GridCraft and VectorCraft
build caches were cleaned after their copies were made on the external drive. Source checkouts and
external caches remain for continuing verification.

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
