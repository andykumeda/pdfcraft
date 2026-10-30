//! Bounded in-process tools over the active GUI session; no server or listener is started.
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use craft_ai_core::assistant::{Target, ToolCall, ToolDefinition};
use craft_ai_core::{AiError, ErrorKind};
use craft_ai_native::{Cancellation, ToolHost};
use pdfcraft_automation::{Automation, Content};
use pdfcraft_engine::DocId;
use serde_json::{Value, json};

use crate::PdfCraftApp;

const TOOLS: &[&str] = &[
    "doc_info",
    "doc_set_info",
    "text_extract",
    "text_find",
    "text_paragraphs",
    "text_edit",
    "page_rotate",
    "page_duplicate",
    "page_delete",
    "comment_list",
    "comment_add",
    "comment_edit",
    "form_fields",
    "form_fill",
    "edit_undo",
    "edit_redo",
    "doc_save",
];

pub(super) struct Pending {
    call: ToolCall,
    reply: mpsc::Sender<Result<Value, AiError>>,
    cancel: Cancellation,
    expires: Instant,
    limit: usize,
}

pub(super) struct Guard {
    doc: Option<DocId>,
    revision: u64,
    selection: String,
    read: bool,
    uncertain: bool,
    grants: Result<Vec<(PathBuf, PathBuf)>, String>,
    automation: Automation,
    limit: usize,
}

fn selection(app: &PdfCraftApp) -> String {
    // Identity includes selected glyphs, not just their text (identical words can occur twice).
    app.active_ids().and_then(|(i, _)| app.views.get(i)).map(|view| view.assistant_selection()).unwrap_or_default()
}

impl Guard {
    pub(super) fn new(app: &PdfCraftApp, prompt: String, mut files: Vec<String>) -> Self {
        files.extend(named_files(&prompt));
        files.sort();
        files.dedup();
        let grants = if files.len() > 32 {
            Err("allow at most 32 exact files".into())
        } else {
            files
                .into_iter()
                .map(|file| {
                    let path = PathBuf::from(file);
                    resolve_output(&path).map(|resolved| (path, resolved))
                })
                .collect()
        };
        let doc = app.active_ids().map(|(_, doc)| doc);
        Self {
            doc,
            revision: doc.and_then(|id| app.session.get(id)).map_or(0, |doc| doc.edit_generation()),
            selection: selection(app),
            read: false,
            uncertain: false,
            grants,
            automation: Automation::new(),
            limit: super::native::effective_config(app).cloned().unwrap_or_default().max_context_bytes,
        }
    }
    pub(super) fn target(&self) -> Target {
        Target { id: self.doc.map(|id| id.0.to_string()).unwrap_or_default(), revision: self.revision }
    }
}

fn resolve_output(path: &Path) -> Result<PathBuf, String> {
    if !path.is_absolute() {
        return Err("allow an exact absolute file path".into());
    }
    if path.is_dir() {
        return Err("grant individual files, not folders".into());
    }
    if path.exists() {
        return path.canonicalize().map_err(|_| "the allowed file cannot be resolved".into());
    }
    let parent = path
        .parent()
        .ok_or("the output needs an existing parent folder")?
        .canonicalize()
        .map_err(|_| "the output needs an existing parent folder")?;
    Ok(parent.join(path.file_name().ok_or("the output needs a file name")?))
}

fn named_files(prompt: &str) -> Vec<String> {
    let mut paths = Vec::new();
    let mut token = String::new();
    let mut quote = None;
    let flush = |token: &mut String, paths: &mut Vec<String>| {
        if Path::new(token.as_str()).is_absolute() && paths.len() < 33 {
            paths.push(std::mem::take(token));
        } else {
            token.clear();
        }
    };
    for ch in prompt.chars() {
        if quote == Some(ch) {
            flush(&mut token, &mut paths);
            quote = None;
        } else if quote.is_some() {
            token.push(ch);
        } else if matches!(ch, '\'' | '"' | '`') && token.is_empty() {
            quote = Some(ch);
        } else if ch.is_whitespace() {
            flush(&mut token, &mut paths);
        } else {
            token.push(ch);
        }
    }
    if quote.is_none() {
        flush(&mut token, &mut paths);
    }
    paths
}

pub(super) struct McpHost {
    sender: mpsc::Sender<Pending>,
    cancel: Cancellation,
    wake: egui::Context,
    timeout: Duration,
    limit: usize,
}
impl McpHost {
    pub(super) fn new(sender: mpsc::Sender<Pending>, cancel: Cancellation, wake: egui::Context, timeout: Duration, limit: usize) -> Self {
        Self { sender, cancel, wake, timeout, limit }
    }
}
impl ToolHost for McpHost {
    fn tools(&mut self, _target: &Target, cancel: &Cancellation) -> Result<Vec<ToolDefinition>, AiError> {
        cancel.check()?;
        Ok(pdfcraft_automation::tools().into_iter().filter(|tool| TOOLS.contains(&tool.name)).map(|tool| {
            let mut parameters = tool.input_schema;
            if let Some(props) = parameters.get_mut("properties").and_then(Value::as_object_mut) {
                props.remove("doc");
                if tool.name == "comment_add" {
                    props.remove("path");
                    if let Some(types) = props.get_mut("type").and_then(|t| t.get_mut("enum")).and_then(Value::as_array_mut) {
                        types.retain(|kind| kind != "attachment");
                    }
                }
            }
            if let Some(required) = parameters.get_mut("required").and_then(Value::as_array_mut) { required.retain(|key| key != "doc"); }
            ToolDefinition {name:tool.name.into(), description:format!("{} Active PDF only. Read before editing; mutation results include independent read-back. Saving writes a file and cannot be undone on disk. Attachments are unavailable.",tool.description), input_schema: parameters}
        }).collect())
    }
    fn call(&mut self, _target: &Target, call: &ToolCall, cancel: &Cancellation) -> Result<Value, AiError> {
        cancel.check()?;
        self.cancel.check()?;
        if !TOOLS.contains(&call.name.as_str()) {
            return Err(AiError::invalid("this PDF tool is unavailable"));
        }
        let (reply, receiver) = mpsc::channel();
        let expires = Instant::now().checked_add(self.timeout).ok_or_else(|| AiError::invalid("invalid assistant timeout"))?;
        self.sender
            .send(Pending { call: call.clone(), reply, cancel: self.cancel.clone(), expires, limit: self.limit })
            .map_err(|_| AiError::cancelled())?;
        self.wake.request_repaint();
        loop {
            cancel.check()?;
            self.cancel.check()?;
            match receiver.recv_timeout(Duration::from_millis(50)) {
                Ok(result) => return result,
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    self.cancel.cancel();
                    return Err(AiError::new(ErrorKind::Provider, "The app stopped answering. Outcome unknown; the action will not be replayed."));
                }
                Err(mpsc::RecvTimeoutError::Timeout) if Instant::now() >= expires => {
                    self.cancel.cancel();
                    return Err(AiError::new(ErrorKind::Provider, "The app did not answer. Outcome unknown; the action will not be replayed."));
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {}
            }
        }
    }
}

pub(super) fn dispatch(app: &mut PdfCraftApp, ctx: &egui::Context, guard: &mut Guard, pending: Pending) {
    guard.uncertain = false;
    guard.limit = pending.limit;
    let result = if pending.cancel.is_cancelled() || Instant::now() >= pending.expires {
        Err("the assistant action expired or was stopped".into())
    } else {
        execute(app, guard, &pending.call)
    };
    if result.is_err()
        && guard.uncertain
        && let Some(document) = guard.doc.and_then(|id| app.session.get(id))
    {
        for view in &mut app.views {
            if Some(view.id) == guard.doc {
                view.document_changed(&document.info);
            }
        }
    }
    let result = result.map_err(|error| {
        if guard.uncertain {
            AiError::new(
                ErrorKind::Internal,
                format!("{error} The action may have completed; inspect the PDF before continuing. It will not be replayed."),
            )
        } else {
            AiError::invalid(error)
        }
    });
    let _ = pending.reply.send(result);
    ctx.request_repaint();
}

fn json_result(content: Vec<Content>) -> Result<Value, String> {
    content
        .into_iter()
        .find_map(|content| match content {
            Content::Json(value) => Some(value),
            Content::Png { .. } => None,
        })
        .ok_or_else(|| "expected a structured PDF result".into())
}

fn run(app: &mut PdfCraftApp, guard: &mut Guard, name: &str, args: &Value) -> Result<Value, String> {
    // Errors can echo malformed PDF strings or arguments; diagnostic text stays fixed.
    let mut value = guard
        .automation
        .call_on_session(&mut app.session, name, args)
        .map_err(|error| match error {
            pdfcraft_automation::ToolError::InvalidArgs(_) => "Invalid PDF tool arguments; check the tool schema.".to_string(),
            _ => "PDF operation refused or failed. Check document permissions, signatures, page ranges and supported content.".into(),
        })
        .and_then(json_result)?;
    if name == "doc_info"
        && let Some(properties) = value.as_object_mut()
    {
        properties.retain(|key, _| {
            matches!(
                key.as_str(),
                "document"
                    | "pdf_version"
                    | "file_size"
                    | "title"
                    | "author"
                    | "subject"
                    | "keywords"
                    | "custom"
                    | "security"
                    | "pages"
                    | "has_javascript"
                    | "xfa"
            )
        });
    }
    Ok(value)
}

fn execute(app: &mut PdfCraftApp, guard: &mut Guard, call: &ToolCall) -> Result<Value, String> {
    if !TOOLS.contains(&call.name.as_str()) {
        return Err("this PDF tool is unavailable".into());
    }
    let doc = guard.doc.ok_or("open a PDF before requesting document work")?;
    if app.active_ids().map(|(_, id)| id) != Some(doc) || selection(app) != guard.selection || app.dialog.is_some() || app.password_prompt.is_some() {
        return Err("the active document, selection or dialog changed; start a new request".into());
    }
    let generation = app.session.get(doc).ok_or("the PDF was closed")?.edit_generation();
    let def = pdfcraft_automation::tools().into_iter().find(|tool| tool.name == call.name).ok_or("unknown PDF tool")?;
    let mut args = call.arguments.as_object().cloned().ok_or("arguments must be an object")?;
    if args.contains_key("doc") {
        return Err("document IDs are not accepted; tools target the active PDF".into());
    }
    if call.name == "comment_add" && (args.contains_key("path") || args.get("type").and_then(Value::as_str) == Some("attachment")) {
        return Err("assistant attachment operations are unavailable".into());
    }
    if !def.read_only && (!guard.read || generation != guard.revision) {
        return Err("read the current PDF state again before editing".into());
    }
    if call.name == "doc_save" {
        if args.get("full").and_then(Value::as_bool) == Some(true) || args.get("flatten_fill_sign").and_then(Value::as_bool) == Some(true) {
            return Err("assistant saving preserves editable content; full rewrite and flattening are unavailable".into());
        }
        let path = args
            .get("path")
            .and_then(Value::as_str)
            .map(PathBuf::from)
            .or_else(|| app.session.get(doc).and_then(|d| d.path.as_ref()).map(PathBuf::from))
            .ok_or("allow an exact save path")?;
        let resolved = resolve_output(&path)?;
        if !guard
            .grants
            .as_ref()
            .map_err(Clone::clone)?
            .iter()
            .any(|(original, allowed)| original == &path && allowed == &resolved && resolve_output(original).ok().as_ref() == Some(allowed))
        {
            return Err("save requires an exact file grant from the user; folders do not grant access".into());
        }
        if path.exists() && app.session.get(doc).and_then(|d| d.path.as_ref()).map(PathBuf::from).as_ref() != Some(&path) {
            return Err("choose a new output path; the assistant cannot overwrite a different existing file".into());
        }
        if path.exists() {
            verify_saved_file(app, doc, &json!({"path":path}))
                .map_err(|_| "The PDF changed on disk. Use normal Save to review the conflict before continuing.".to_string())?;
        }
        args.insert("full".into(), json!(false));
        args.insert("path".into(), json!(path));
    }
    args.insert("doc".into(), json!(doc.0));
    let args = Value::Object(args);
    if !def.read_only {
        if app
            .active_ids()
            .and_then(|(index, _)| app.views.get(index))
            .is_some_and(|view| app.session.get(doc).is_some_and(|document| view.assistant_has_draft(document)))
        {
            return Err("finish the current PDF editor or draft before assistant edits".into());
        }
        guard.uncertain = true;
    }
    let outcome = run(app, guard, &call.name, &args)?;
    let current = app.session.get(doc).ok_or("the PDF was closed")?.edit_generation();
    if !def.read_only {
        if let Some(document) = app.session.get(doc) {
            for view in &mut app.views {
                if view.id == doc {
                    view.document_changed(&document.info);
                }
            }
        }
        guard.selection = selection(app);
        // Read from the actual post-command document; never echo intended arguments as evidence.
        let read_name = match call.name.as_str() {
            "text_edit" => "text_paragraphs",
            "comment_add" | "comment_edit" => "comment_list",
            "form_fill" => "form_fields",
            _ => "doc_info",
        };
        let mut verify = json!({"doc":doc.0});
        if read_name == "text_paragraphs" {
            verify["page"] = args.get("page").cloned().ok_or("text verification needs its page")?;
        }
        let readback = run(app, guard, read_name, &verify)
            .map_err(|_| "The action may have completed, but read-back failed. Inspect the PDF; do not retry automatically.".to_string())?;
        if call.name == "doc_save" {
            verify_saved_file(app, doc, &args)?;
        }
        guard.revision = current;
        guard.read = true;
        let mut verified = json!({"outcome":outcome,"read_back":readback,"document_generation":current});
        if call.name == "doc_save" {
            verified["notice"] = json!("The file was written and read back. Undo cannot reverse the disk write.");
        }
        let result = bounded(verified, guard.limit)?;
        guard.uncertain = false;
        return Ok(result);
    }
    // A partial/oversized read must not authorize a subsequent mutation.
    guard.read = false;
    let result = bounded(outcome, guard.limit)?;
    guard.revision = current;
    guard.read = result.get("complete").and_then(Value::as_bool) != Some(false);
    Ok(result)
}

fn bounded(value: Value, limit: usize) -> Result<Value, String> {
    if serde_json::to_vec(&value).map_err(|_| "could not encode PDF result")?.len() > limit {
        Err("PDF result exceeds the configured context limit. Request fewer pages or a smaller result. Completed edits remain undoable.".into())
    } else {
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn app() -> PdfCraftApp {
        let mut app = PdfCraftApp::new();
        app.open_bytes("Synthetic.pdf", None, include_bytes!("../../tests/data/form.pdf").to_vec()).unwrap();
        app
    }
    fn call(name: &str, args: Value) -> ToolCall {
        ToolCall { id: "synthetic".into(), name: name.into(), arguments: args }
    }
    #[test]
    fn actual_gui_document_read_edit_readback_and_undo() {
        let mut app = app();
        let id = app.active_ids().unwrap().1;
        let mut guard = Guard::new(&app, String::new(), vec![]);
        assert!(execute(&mut app, &mut guard, &call("doc_set_info", json!({"key":"Title","value":"Synthetic changed"}))).is_err());
        let information = execute(&mut app, &mut guard, &call("doc_info", json!({}))).unwrap();
        assert!(information.get("annotations").is_none() && information.get("fields").is_none() && information.get("attachments").is_none());
        let result = execute(&mut app, &mut guard, &call("doc_set_info", json!({"key":"Title","value":"Synthetic changed"}))).unwrap();
        assert!(result["read_back"].to_string().contains("Synthetic changed"));
        assert!(app.session.get(id).unwrap().dirty);
        let undone = execute(&mut app, &mut guard, &call("edit_undo", json!({}))).unwrap();
        assert!(!undone["read_back"].to_string().contains("Synthetic changed"));
        let redone = execute(&mut app, &mut guard, &call("edit_redo", json!({}))).unwrap();
        assert!(redone["read_back"].to_string().contains("Synthetic changed"));
    }
    #[test]
    fn stale_foreign_cancelled_and_unsupported_actions_are_rejected() {
        let mut app = app();
        let mut guard = Guard::new(&app, String::new(), vec![]);
        assert!(execute(&mut app, &mut guard, &call("doc_info", json!({"doc":1}))).is_err());
        assert!(execute(&mut app, &mut guard, &call("js_run", json!({}))).is_err());
        assert!(execute(&mut app, &mut guard, &call("comment_add", json!({"type":"attachment","path":"/tmp/private"}))).is_err());
        execute(&mut app, &mut guard, &call("doc_info", json!({}))).unwrap();
        let id = app.active_ids().unwrap().1;
        app.session.apply(id, pdfcraft_engine::Edit::SetInfo { key: "Title".into(), value: "Outside edit".into() }).unwrap();
        assert!(execute(&mut app, &mut guard, &call("doc_set_info", json!({"key":"Title","value":"Wrong"}))).is_err());
        let cancel = Cancellation::new();
        cancel.cancel();
        let (reply, rx) = mpsc::channel();
        dispatch(
            &mut app,
            &egui::Context::default(),
            &mut guard,
            Pending {
                call: call("doc_set_info", json!({"key":"Title","value":"Wrong"})),
                reply,
                cancel,
                expires: Instant::now() + Duration::from_secs(1),
                limit: craft_ai_core::assistant::RuntimeConfig::default().max_context_bytes,
            },
        );
        assert!(rx.recv().unwrap().is_err());
        app.active = None;
        assert!(execute(&mut app, &mut guard, &call("doc_info", json!({}))).is_err());
    }
    #[test]
    fn context_bounds_and_schemas_do_not_expose_other_documents_or_attachment_files() {
        assert!(bounded(json!({"data":"long synthetic text"}), 2).is_err());
        assert_eq!(named_files("Save to \"/tmp/Synthetic file.pdf\""), vec!["/tmp/Synthetic file.pdf"]);
        assert!(resolve_output(Path::new("relative.pdf")).is_err());
        let (tx, _rx) = mpsc::channel();
        let cancel = Cancellation::new();
        let mut host = McpHost::new(
            tx,
            cancel.clone(),
            egui::Context::default(),
            Duration::from_secs(1),
            craft_ai_core::assistant::RuntimeConfig::default().max_context_bytes,
        );
        let tools = host.tools(&Target { id: String::new(), revision: 0 }, &cancel).unwrap();
        assert_eq!(tools.len(), TOOLS.len());
        assert!(tools.iter().all(|tool| tool.input_schema["properties"].get("doc").is_none()));
        assert!(tools.iter().find(|t| t.name == "comment_add").unwrap().input_schema["properties"].get("path").is_none());
    }
    #[test]
    fn saving_needs_exact_grant_and_preserves_live_undo_session() {
        let mut app = app();
        let path = std::env::temp_dir().join(format!("pdfcraft-assistant-save-{}.pdf", std::process::id()));
        assert!(!path.exists());
        let mut guard = Guard::new(&app, String::new(), vec![]);
        execute(&mut app, &mut guard, &call("doc_info", json!({}))).unwrap();
        assert!(execute(&mut app, &mut guard, &call("doc_save", json!({"path":path}))).is_err());
        let mut guard = Guard::new(&app, String::new(), vec![path.to_string_lossy().into_owned()]);
        execute(&mut app, &mut guard, &call("doc_info", json!({}))).unwrap();
        execute(&mut app, &mut guard, &call("doc_set_info", json!({"key":"Title","value":"Save/reopen"}))).unwrap();
        execute(&mut app, &mut guard, &call("doc_save", json!({"path":path}))).unwrap();
        let mut reopened = Automation::new();
        let opened = json_result(reopened.call("doc_open", &json!({"path":path})).unwrap()).unwrap();
        let saved_id = opened["doc"].clone();
        let info = json_result(reopened.call("doc_info", &json!({"doc":saved_id})).unwrap()).unwrap();
        assert!(info.to_string().contains("Save/reopen"));
        std::fs::write(&path, b"Synthetic external change").unwrap();
        assert!(execute(&mut app, &mut guard, &call("doc_save", json!({"path":path}))).is_err());
        assert_eq!(std::fs::read(&path).unwrap(), b"Synthetic external change");
        std::fs::remove_file(path).unwrap();
    }
}

fn verify_saved_file(app: &PdfCraftApp, doc: DocId, args: &Value) -> Result<(), String> {
    use std::io::Read;
    let document = app.session.get(doc).ok_or("the PDF was closed")?;
    let expected = document.saved_bytes();
    let path = args.get("path").and_then(Value::as_str).ok_or("save verification needs its exact path")?;
    let mut file = std::fs::File::open(path).map_err(|_| "saved file could not be read back")?;
    if file.metadata().map_err(|_| "saved file could not be inspected")?.len()
        != u64::try_from(expected.len()).map_err(|_| "saved file is too large")?
    {
        return Err("saved file size differs from the document".into());
    }
    let mut offset = 0usize;
    let mut chunk = [0u8; 8192];
    loop {
        let n = file.read(&mut chunk).map_err(|_| "saved file read-back failed")?;
        if n == 0 {
            break;
        }
        let end = offset.checked_add(n).ok_or("saved file is too large")?;
        if expected.get(offset..end) != chunk.get(..n) {
            return Err("saved file differs from the document".into());
        }
        offset = end;
    }
    if offset != expected.len() {
        return Err("saved file was truncated".into());
    }
    Ok(())
}

#[cfg(test)]
mod turn_tests;
