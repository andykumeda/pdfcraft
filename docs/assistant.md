# Integrated desktop assistant preview

Status: private preview **0.15.0 (`dd68eaed`)** is installed at
`/Applications/PDFCraft Assistant Preview.app`. Native panel/Settings, local Ollama
inspection/rotation/read-back/Undo and save/reopen passed. It is ad-hoc signed and not
notarized. See [the delivery ledger](assistant-implementation.md) for exact source, checks,
DMG/checksum and remaining other-Mac/Codex acceptance. No public release was created. The user subsequently authorized pushing the source branch to the `andykumeda/pdfcraft` fork.

Open **Assistant** in the toolbar or **View → Assistant**. The selected toolbar button
and checked menu entry indicate that the native side panel is open. Settings opens the shared in-panel connection editor. The same editor is available through
**Preferences → Assistant settings**. One model handles both chat and requested PDF actions.

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

The initial tools inspect document metadata/pages, find/extract text, inspect/edit supported
paragraphs, rotate/duplicate/delete pages, list/add/edit comments, list/fill forms, Undo/Redo,
and explicitly save. Existing PDFCraft validation, permissions and signature protections apply.
This does not add PDF editing capabilities or expose the full external automation catalog.
Attachments, arbitrary scripts, security/signing changes, printing and bulk filesystem work
are unavailable to the assistant. Unsupported text edits fail visibly.

Reads target the active PDF. Document data is untrusted tool context, not user instructions.
General conversation does not automatically read the PDF. Request only needed pages or details;
oversized results require a smaller request. Document information omits unrelated comments,
field values and attachment names; dedicated tools read comments/forms when needed.
A turn stops editing if the document, selection, relevant editor target or revision changed.
Finish in-progress local editor drafts before assistant mutations.

Mutations run against the open window's engine session and normal Undo history. Results include
independent post-edit reads. Provider narration alone is not evidence. Uncertain outcomes stop
the turn and are never automatically replayed. Stop invalidates queued work; completed edits
remain. PDFCraft's existing save/history boundaries still apply. Save writes to disk and Undo
cannot reverse that disk write. Save requires an exact path granted in the prompt or file controls;
quote paths containing spaces. Granting a folder grants nothing. The assistant refuses overwriting
a different existing file, forced rewrites and flattening. Saved bytes are read back from disk.

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
