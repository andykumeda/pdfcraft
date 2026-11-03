use std::sync::atomic::{AtomicUsize, Ordering};

use craft_ai_core::assistant::{AssistantRole, Message, MessageRole, RuntimeConfig, tool_model};
use craft_ai_core::{AppId, Connection, ConnectionDraft, ModelId, ModelInfo};
use craft_ai_native::{ChatProvider, ModelChoice, ModelRequest, TurnEvent, TurnRequest, run_turn};

use super::*;

const ORIGINAL: &str = "Synthetic original title";
const EDITED: &str = "Synthetic assistant title";

struct SyntheticProvider {
    choice: ModelChoice,
    responses: AtomicUsize,
    doc: u64,
}

impl ChatProvider for SyntheticProvider {
    fn complete(&self, request: &ModelRequest<'_>, cancel: &Cancellation, _emit: &mut dyn FnMut(&str)) -> Result<Message, AiError> {
        cancel.check()?;
        assert_eq!(request.connection, &self.choice.connection, "every response must use the selected connection");
        assert_eq!(request.model, &self.choice.model, "every response must use the selected model");
        assert!(request.tools.iter().all(|tool| tool.name != "control_app"), "one model calls the PDF tools directly");
        assert!(request.messages.first().unwrap().content.contains("untrusted data"));
        let step = self.responses.fetch_add(1, Ordering::SeqCst);
        if step > 0 {
            let last = request.messages.last().unwrap();
            assert_eq!(last.role, MessageRole::Tool, "tool results stay distinct from user instructions");
            let result: Value = serde_json::from_str(&last.content).unwrap();
            assert_ne!(result.get("isError"), Some(&json!(true)));
            if step != 2 {
                assert_eq!(
                    result["structuredContent"]["title"].as_str(),
                    Some(if step == 1 { ORIGINAL } else { EDITED }),
                    "use actual MCP document read-back"
                );
            }
        }
        let (name, arguments) = match step {
            0 => ("doc_info", json!({"doc":self.doc})),
            1 => ("doc_set_info", json!({"doc":self.doc,"key":"Title","value":EDITED})),
            2 => ("doc_info", json!({"doc":self.doc})),
            3 => return Ok(Message::text(MessageRole::Assistant, "The synthetic title was edited and read back.")),
            _ => panic!("the synthetic provider was called beyond its expected sequence"),
        };
        let mut answer = Message::text(MessageRole::Assistant, "");
        answer.tool_calls.push(ToolCall { id: format!("synthetic-{step}"), name: name.into(), arguments });
        Ok(answer)
    }

    fn models(&self, _connection: &Connection, _config: &RuntimeConfig, _cancel: &Cancellation) -> Result<Vec<ModelInfo>, AiError> {
        panic!("a configured turn must not discover a replacement model")
    }
}

#[test]
fn one_selected_fake_provider_edits_the_live_document_through_the_full_tool_turn() {
    let mut app = PdfCraftApp::new();
    app.open_bytes("Synthetic.pdf", None, include_bytes!("../../../tests/data/form.pdf").to_vec()).unwrap();
    let id = app.active_ids().unwrap().1;
    app.session.apply(id, pdfcraft_engine::Edit::SetInfo { key: "Title".into(), value: ORIGINAL.into() }).unwrap();
    let mut guard = Guard::new(&app, "Change only the title and verify it.".into(), vec![]);
    let model = ModelId::new("synthetic-tool-model").unwrap();
    let mut draft = ConnectionDraft::default();
    draft.label = "Synthetic local provider".into();
    draft.provider = "ollama".into();
    draft.endpoint = "http://127.0.0.1:11434/v1".into();
    draft.uses_key = false;
    let mut connection = draft.prepare(&[]).unwrap().connection;
    connection.models = vec![tool_model(model.clone()).unwrap()];
    let choice = ModelChoice { connection, model };
    let request = TurnRequest {
        app: AppId::new("pdfcraft").unwrap(),
        messages: vec![Message::text(MessageRole::User, "Change only the title and verify it.")],
        target: guard.target(),
        chat: choice.clone(),
        control: choice.clone(),
        config: RuntimeConfig { max_tools: pdfcraft_automation::tools().len(), ..RuntimeConfig::default() },
    };
    let cancel = Cancellation::new();
    let worker_cancel = cancel.clone();
    let ctx = egui::Context::default();
    let wake = ctx.clone();
    let (sender, calls) = mpsc::channel();
    let worker = std::thread::spawn(move || {
        let provider = SyntheticProvider { choice, responses: AtomicUsize::new(0), doc: id.0 };
        let mut host = McpHost::new(sender, worker_cancel.clone(), wake, Duration::from_secs(2), RuntimeConfig::default().max_context_bytes);
        let mut routes = Vec::new();
        let result = run_turn(request, &provider, &mut host, &worker_cancel, |event| {
            if let TurnEvent::Routed { role, connection, model } = event {
                routes.push((role, connection, model));
            }
        });
        (result, routes, provider.responses.load(Ordering::SeqCst))
    });
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        match calls.recv_timeout(Duration::from_millis(25)) {
            Ok(pending) => dispatch(&mut app, &ctx, &mut guard, pending),
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
            Err(mpsc::RecvTimeoutError::Timeout) if Instant::now() >= deadline => {
                cancel.cancel();
                break;
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {}
        }
    }
    drop(calls);
    let (messages, routes, responses) = worker.join().unwrap();
    let messages = messages.unwrap();
    assert_eq!(responses, 4);
    assert_eq!(routes.len(), 4);
    assert!(routes.iter().all(|(role, connection, model)| {
        *role == AssistantRole::Chat && connection == "Synthetic local provider" && model == "synthetic-tool-model"
    }));
    assert_eq!(messages.iter().filter(|message| message.role == MessageRole::Tool).count(), 3);
    assert_eq!(app.session.get(id).unwrap().info.title.as_deref(), Some(EDITED));
    assert!(app.session.get(id).unwrap().dirty);
    app.session.undo(id).unwrap();
    assert_eq!(app.session.get(id).unwrap().info.title.as_deref(), Some(ORIGINAL), "normal Undo restores pre-existing work");
}
