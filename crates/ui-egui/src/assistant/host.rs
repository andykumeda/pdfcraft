//! The full MCP tool catalog over the live GUI session; no listener or separate document copy.
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, mpsc};
use std::time::{Duration, Instant};

use craft_ai_core::assistant::{Target, ToolCall, ToolDefinition};
use craft_ai_core::{AiError, ErrorKind};
use craft_ai_native::{Cancellation, ToolHost};
use pdfcraft_automation::{Automation, mcp::McpServer};
use pdfcraft_engine::DocId;
use serde_json::{Value, json};

use crate::PdfCraftApp;

pub(super) struct Pending {
    call: ToolCall,
    reply: mpsc::Sender<Result<Value, AiError>>,
    cancel: Cancellation,
    expires: Instant,
}

pub(super) struct Guard {
    target: Target,
}

impl Guard {
    pub(super) fn new(app: &PdfCraftApp, _prompt: String, _files: Vec<String>) -> Self {
        let doc = app.active_ids().map(|(_, id)| id);
        Self {
            target: Target {
                id: doc.map(|id| id.0.to_string()).unwrap_or_default(),
                revision: doc.and_then(|id| app.session.get(id)).map_or(0, |doc| doc.edit_generation()),
            },
        }
    }

    pub(super) fn target(&self) -> Target {
        // This identifies the transport turn, not which documents its MCP arguments may address.
        self.target.clone()
    }
}

pub(super) struct McpHost {
    sender: mpsc::Sender<Pending>,
    cancel: Cancellation,
    wake: egui::Context,
    timeout: Duration,
}

impl McpHost {
    pub(super) fn new(sender: mpsc::Sender<Pending>, cancel: Cancellation, wake: egui::Context, timeout: Duration, _limit: usize) -> Self {
        Self { sender, cancel, wake, timeout }
    }
}

impl ToolHost for McpHost {
    fn tools(&mut self, _target: &Target, cancel: &Cancellation) -> Result<Vec<ToolDefinition>, AiError> {
        cancel.check()?;
        let mut server = McpServer::new(Automation::new());
        let reply = server
            .handle(&json!({"jsonrpc":"2.0","id":1,"method":"tools/list"}))
            .ok_or_else(|| AiError::invalid("MCP did not return its PDF tool catalog"))?;
        reply
            .get("result")
            .and_then(|result| result.get("tools"))
            .and_then(Value::as_array)
            .ok_or_else(|| AiError::invalid("MCP returned an invalid PDF tool catalog"))?
            .iter()
            .map(ToolDefinition::from_mcp)
            .collect()
    }

    fn call(&mut self, _target: &Target, call: &ToolCall, cancel: &Cancellation) -> Result<Value, AiError> {
        cancel.check()?;
        self.cancel.check()?;
        let (reply, receiver) = mpsc::channel();
        let expires = Instant::now().checked_add(self.timeout).ok_or_else(|| AiError::invalid("invalid assistant timeout"))?;
        self.sender.send(Pending { call: call.clone(), reply, cancel: self.cancel.clone(), expires }).map_err(|_| AiError::cancelled())?;
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
    let result = (|| {
        if pending.cancel.is_cancelled() || Instant::now() >= pending.expires {
            return Err("the assistant action expired or was stopped".into());
        }
        let mutates = pdfcraft_automation::tools().into_iter().find(|tool| tool.name == pending.call.name).is_some_and(|tool| !tool.read_only);
        if mutates {
            let ids: Vec<_> = app.session.docs().iter().map(|document| document.id).collect();
            for id in ids {
                if !app.commit_typing_in(id) {
                    return Err("Finish the pending PDF editor changes before running this operation; the typed content was retained.".into());
                }
            }
        }
        // Committing form typing can run bounded scripts; cancellation may arrive during that work.
        if pending.cancel.is_cancelled() || Instant::now() >= pending.expires {
            return Err("the assistant action expired or was stopped".into());
        }
        execute(app, guard, &pending.call)
    })();
    let _ = pending.reply.send(result.map_err(AiError::invalid));
    ctx.request_repaint();
}

fn execute(app: &mut PdfCraftApp, _guard: &mut Guard, call: &ToolCall) -> Result<Value, String> {
    let before: HashMap<_, _> = app.session.docs().iter().map(|doc| (doc.id, (doc.edit_generation(), doc.saved_bytes()))).collect();
    let request = json!({"jsonrpc":"2.0","id":call.id,"method":"tools/call","params":{"name":call.name,"arguments":call.arguments}});
    let reply = app.assistant.mcp.handle_on_session(&mut app.session, &request).ok_or("MCP did not answer the PDF request")?;
    sync_views(app, &before);
    if let Some(error) = reply.get("error") {
        return Err(error.to_string());
    }
    reply.get("result").cloned().ok_or_else(|| "MCP returned an invalid PDF tool response".into())
}

fn sync_views(app: &mut PdfCraftApp, before: &HashMap<DocId, (u64, Arc<Vec<u8>>)>) {
    for index in (0..app.views.len()).rev() {
        if app.views.get(index).is_some_and(|view| app.session.get(view.id).is_none()) {
            app.close_tab(index);
        }
    }
    for view in &mut app.views {
        if let Some(doc) = app.session.get(view.id)
            && before.get(&view.id).is_none_or(|(generation, bytes)| *generation != doc.edit_generation() || !Arc::ptr_eq(bytes, &doc.saved_bytes()))
        {
            view.document_changed(&doc.info);
        }
    }
    let mut viewed: HashSet<_> = app.views.iter().map(|view| view.id).collect();
    for doc in app.session.docs() {
        if viewed.insert(doc.id) {
            app.views.push(crate::DocView::new(doc.id, &doc.info, app.view_defaults));
            app.active = app.views.len().checked_sub(1);
        }
    }
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod turn_tests;
