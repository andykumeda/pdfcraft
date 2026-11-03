# Integrated desktop assistant preview

Status: **PDFCraft 0.18.0 (`8e86f96f`)** standard and Assistant Preview are installed on
Mini and MacBook with the independently installed shared assistant **0.1.0**. Both variants
passed isolated native helper/composer/Settings and synthetic MCP smoke on both Macs. The
installed CLI lists **183 tools**. Local certificate signing is verified; these are not
notarized or public releases. Clipboard, native IME/VoiceOver and live-provider acceptance
remain separate checks. See [the delivery ledger](assistant-implementation.md) for exact
source, package hashes, verification and historical preview evidence.

Open **Assistant** in the toolbar or **View → Assistant**. The selected toolbar button
and checked menu entry indicate that the native side panel is open. The settings gear opens the shared in-panel connection editor. The same editor is available through
**Preferences → Assistant settings**. One model handles both chat and requested PDF actions.

In the current source, the Chats menu holds new, saved and delete actions. The rounded
composer contains a `+` file menu, a model dropdown and Send/Stop. Quick model choices use the
existing streaming/tool validation and set a PDFCraft override; settings retains suite-wide
choices and custom model IDs. Assistant version, update checks and installation appear in
settings. Missing-helper recovery controls remain available. These layout changes are source
only; the installed versions described above retain their previous layout until rebuilt and
installed with the updated shared helper.

Use the shared connection/model controls for Codex or OpenAI-compatible providers, including
Ollama. No model is chosen automatically beyond an explicitly configured suite default.
Advanced options start collapsed. The effective endpoint/model and validated context limits
are visible; credentials stay in the platform credential store. Codex uses the separate
Craft-managed sign-in, never the coding assistant's credentials/history/settings.

Shared configuration uses `CRAFT_AI_CONFIG_DIR`, otherwise the platform's `Craft/ai` folder
(`~/Library/Application Support/Craft/ai` on macOS). Connections and suite defaults are shared;
PDFCraft overrides and conversations use app identity `pdfcraft`. Runtime limits are validated
from `runtime.json`; provider failure never changes the chosen endpoint, provider or model.
Connection tests request model catalogs; capability probes and chat can incur provider usage.

The source now exposes the complete external MCP tool catalog through the assistant. The same
table supplies unchanged names, descriptions, argument schemas, titles and annotations; `pdfcraft-cli tools` lists
the current set. Tools retain their ordinary document IDs, file paths and options. This includes
opening/creating multiple PDFs, rendering pages, organizing, attachments, forms, OCR, security,
signatures, export and the other tools supported by the engine. Availability still depends on
the same engine permissions, platform support and configured resources as external MCP clients.

Calls use the existing MCP dispatcher against the window's live engine session. There is no
assistant tool allowlist, argument stripping, active-document-only rule or separate file-grant
policy. Configured automation roots retain the ordinary MCP path checks. Results keep the MCP
`content`, PNG image blocks, `structuredContent` and `isError` fields without an assistant-specific
read-back wrapper. Document data remains untrusted tool context, not user instructions.

The UI commits ordinary pending typing before mutations, refreshes changed page views and opens
or closes tabs to match the resulting engine session. Mutations retain normal Undo history.
Stop invalidates queued work; uncertain outcomes are never automatically replayed. Saving and
closing use normal MCP behavior: disk writes cannot be undone; closing unsaved engine changes
requires saving first or explicitly passing `discard_changes: true`. Read-back is available
through the regular tools; provider narration alone is not evidence that a change succeeded.

This source change is separate from the installed preview identified above. Its verification
and delivery state are recorded in the ledger; no new installed or published build is implied.

Chats persist outside PDFs and may contain document text and tool results. New/Open/Delete chat
manage them independently. Reopening a chat does not resume interrupted edits. Changing providers
does not remove earlier context from a conversation.

## Preview delivery and updates

The intended installation is `/Applications/PDFCraft Assistant Preview.app`, separate from
`/Applications/PdfCraft.app`. The portable launcher uses a destination-relative home profile at
`~/Library/Application Support/PDFCraft Assistant Preview`, overrideable with the validated
absolute `PDFCRAFT_PROFILE_DIR`. Existing standard preferences/recovery and shared credentials
are not copied. The preview disables standard startup/manual updates with `--disable-updates`.
Update it by deliberately integrating upstream source, verifying, and rebuilding a new preview
DMG. An assistant service split is deferred.

See [packaging instructions](../packaging/macos/assistant-preview.md) for commit-labelled ARM64
DMGs, checksums, mounted/extracted verification and signing limitations. Local signature verification
does not establish notarization or acceptance on another Mac.

2026-11-02 source update: annotation deletion is available through the built-in assistant
with independent comment read-back and Undo. The updated preview is installed on both Macs; real-document acceptance remains manual.
