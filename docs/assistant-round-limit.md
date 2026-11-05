# Shared assistant round limit — 2026-11-05

Status: **Deployed and verified on both Macs**.

Owner requested longer assistant tasks in PDFCraft, WordCraft, GridCraft and DeckCraft. On each Mac, the existing shared `~/Library/Application Support/Craft/ai/runtime.json` now contains:

```json
{
  "max_rounds": 50
}
```

Previously that file was absent, so the shared Rust default of 12 applied. This is an owner configuration override, not a compiled-default change. Unspecified limits continue to use the single shared defaults. Both writes were serialized with the existing `store.lock`, written atomically as owner-only 0600 files, and verified after replacement. Other runtime keys are preserved if present. Connections, component/source settings and chat hashes stayed unchanged; no credentials were copied or edited. App binaries and update routing are unchanged.

The helper reloads `SharedStore::config()` when preparing each new user turn (`craft-libs/crates/assistant/src/runtime/native.rs`), and the host tool proxy loads it when creating that turn (`craft-libs/crates/assistant/src/host_proxy.rs`). Existing in-progress requests retain their captured budget. A new request picks up 50 without reinstalling or restarting the app. The interrupted request does not resume automatically: inspect existing edits before asking it to continue.

| Device / verified host | App | Target rounds | State | Evidence / activation |
| --- | --- | --- | --- | --- |
| Mini / AKsMini | pdfcraft | 50 | Deployed | Installed helper0.1.7 loaded/validated settings under this app identity, normal shutdown; applies next request |
| Mini / AKsMini | wordcraft | 50 | Deployed | Installed helper0.1.7 loaded/validated settings under this app identity, normal shutdown; applies next request |
| Mini / AKsMini | gridcraft | 50 | Deployed | Installed helper0.1.7 loaded/validated settings under this app identity, normal shutdown; applies next request |
| Mini / AKsMini | deckcraft | 50 | Deployed | Installed helper0.1.7 loaded/validated settings under this app identity, normal shutdown; applies next request |
| MacBook / AKs-MBPro.local | pdfcraft | 50 | Deployed | Installed helper0.1.7 loaded/validated settings under this app identity, normal shutdown; applies next request |
| MacBook / AKs-MBPro.local | wordcraft | 50 | Deployed | Installed helper0.1.7 loaded/validated settings under this app identity, normal shutdown; applies next request |
| MacBook / AKs-MBPro.local | gridcraft | 50 | Deployed | Installed helper0.1.7 loaded/validated settings under this app identity, normal shutdown; applies next request |
| MacBook / AKs-MBPro.local | deckcraft | 50 | Deployed | Installed helper0.1.7 loaded/validated settings under this app identity, normal shutdown; applies next request |

## Verification and rollback

Independent host receipts: `~/Library/Application Support/Craft/config-backups/2026-11-05-round-limit/{receipt,verification}.json` (0600). They record the original absence of runtime.json on each host. Verification logs on Mini: `/private/tmp/craft-round-limit-{mini,book}-verification.log`. The installed helper was exercised through its actual private-stdio protocol for all four app identities on each Mac. It loaded and validated normal shared settings successfully; no model turn, document tool call or network listener was requested. A first verification harness decoder omitted flattened status events; corrected decoder passed all eight checks with normal shutdowns. No product change was required.

Readback confirms 50, with all other settings preserved. This verifies config loading/validation, not completion of a 50-round real model task. No paid inference or document edits were performed. Other timeout/context limits still apply; a higher round budget can permit longer, more costly requests.

To restore the previous default, remove only the `max_rounds` override while preserving any subsequently added keys; remove runtime.json only if then empty. The config files are machine-local application data outside this Git repository and are not committed. This document is the committed operational record; no new repository was initialized for application settings.
