use super::*;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT: AtomicU64 = AtomicU64::new(1);
struct Directory(PathBuf);
impl Directory {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!("pdfcraft-mcp-parity-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Directory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn call(name: &str, arguments: Value) -> ToolCall {
    ToolCall { id: "synthetic".into(), name: name.into(), arguments }
}

fn success(app: &mut PdfCraftApp, guard: &mut Guard, name: &str, arguments: Value) -> Value {
    let result = execute(app, guard, &call(name, arguments)).unwrap();
    assert_eq!(result["isError"], false, "{result}");
    result.get("structuredContent").cloned().unwrap_or(result)
}

#[test]
fn full_catalog_matches_external_mcp_descriptors_including_metadata() {
    let (sender, _receiver) = mpsc::channel();
    let cancel = Cancellation::new();
    let mut host = McpHost::new(sender, cancel.clone(), egui::Context::default(), Duration::from_secs(1), 1024);
    let tools = host.tools(&Target { id: String::new(), revision: 0 }, &cancel).unwrap();
    let mut server = McpServer::new(Automation::new());
    let reply = server.handle(&json!({"jsonrpc":"2.0","id":1,"method":"tools/list"})).unwrap();
    let expected = reply["result"]["tools"].as_array().unwrap();
    assert_eq!(tools.len(), expected.len());
    assert!(tools.len() > 18);
    for (actual, expected) in tools.iter().zip(expected) {
        assert_eq!(actual.to_mcp(), *expected);
    }
    assert!(tools.iter().find(|tool| tool.name == "comment_add").unwrap().input_schema["properties"].get("path").is_some());
    assert!(tools.iter().find(|tool| tool.name == "doc_save").unwrap().input_schema["properties"].get("full").is_some());
}

#[test]
fn full_mcp_results_match_external_calls_including_png_and_engine_errors() {
    let mut app = PdfCraftApp::new();
    app.session = pdfcraft_engine::Session::new().with_clock(|| 1_700_000_000);
    let mut guard = Guard::new(&app, String::new(), Vec::new());
    let mut external = McpServer::new(Automation::new().with_clock(|| 1_700_000_000));
    for (name, arguments) in [
        ("doc_create", json!({"from":"blank","pages":2})),
        ("doc_set_info", json!({"doc":1,"key":"Title","value":"Same MCP title"})),
        ("doc_info", json!({"doc":1})),
        ("page_insert_blank", json!({"doc":1,"at":2,"width":200,"height":300})),
        ("page_move", json!({"doc":1,"pages":[2],"to":1})),
        ("page_render", json!({"doc":1,"page":1,"dpi":12})),
        ("doc_close", json!({"doc":1})),
    ] {
        let expected =
            external.handle(&json!({"jsonrpc":"2.0","id":"synthetic","method":"tools/call","params":{"name":name,"arguments":arguments}})).unwrap();
        let actual = execute(&mut app, &mut guard, &call(name, arguments)).unwrap();
        assert_eq!(actual, expected["result"], "{name} differs from external MCP");
        if name == "page_render" {
            assert_eq!(actual["content"][0]["type"], "image");
            assert_eq!(actual["content"][0]["mimeType"], "image/png");
            assert!(!actual["content"][0]["data"].as_str().unwrap().is_empty());
        }
    }
    assert_eq!(app.views.len(), 1);
    assert_eq!(app.session.get(app.views[0].id).unwrap().info.pages.len(), 3);
}

#[test]
fn explicit_document_ids_creation_background_edits_and_normal_undo_work() {
    let mut app = PdfCraftApp::new();
    let mut guard = Guard::new(&app, String::new(), Vec::new());
    let first = success(&mut app, &mut guard, "doc_create", json!({"from":"blank"}))["doc"].as_u64().unwrap();
    let second = success(&mut app, &mut guard, "doc_create", json!({"from":"blank"}))["doc"].as_u64().unwrap();
    assert_eq!(app.active_ids().unwrap().1, DocId(second));
    success(&mut app, &mut guard, "doc_set_info", json!({"doc":first,"key":"Title","value":"Background document"}));
    assert_eq!(app.session.get(DocId(first)).unwrap().info.title.as_deref(), Some("Background document"));
    assert_eq!(app.active_ids().unwrap().1, DocId(second));
    success(&mut app, &mut guard, "edit_undo", json!({"doc":first}));
    assert_ne!(app.session.get(DocId(first)).unwrap().info.title.as_deref(), Some("Background document"));
    success(&mut app, &mut guard, "doc_close", json!({"doc":first,"discard_changes":true}));
    assert_eq!(app.views.len(), 1);
    assert_eq!(app.active_ids().unwrap().1, DocId(second));
}

#[test]
fn assistant_targets_explicit_documents_across_split_panes() {
    let mut app = PdfCraftApp::new();
    let mut guard = Guard::new(&app, String::new(), Vec::new());
    let first = success(&mut app, &mut guard, "doc_create", json!({"from":"blank"}))["doc"].as_u64().unwrap();
    let second = success(&mut app, &mut guard, "doc_create", json!({"from":"blank"}))["doc"].as_u64().unwrap();
    app.split_tab(0, false);
    assert_ne!(app.place_of(0), app.place_of(1));
    let active = app.active_ids().unwrap().1;
    guard = Guard::new(&app, String::new(), Vec::new());
    let target = if active == DocId(first) { second } else { first };
    success(&mut app, &mut guard, "doc_set_info", json!({"doc":target,"key":"Title","value":"Split target"}));
    assert_eq!(app.session.get(DocId(target)).unwrap().info.title.as_deref(), Some("Split target"));
    assert_ne!(app.session.get(active).unwrap().info.title.as_deref(), Some("Split target"));
    assert_eq!(app.active_ids().unwrap().1, active);
    assert_ne!(app.place_of(0), app.place_of(1));
}

#[test]
fn normal_mcp_save_open_and_root_confinement_replace_side_panel_file_grants() {
    let directory = Directory::new();
    let root = directory.0.join("allowed");
    std::fs::create_dir(&root).unwrap();
    std::fs::write(directory.0.join("outside.pdf"), b"synthetic outside file").unwrap();
    let mut app = PdfCraftApp::new();
    let mut guard = Guard::new(&app, String::new(), Vec::new());
    app.set_assistant_automation(Automation::new().with_root(&root).unwrap());
    let id = success(&mut app, &mut guard, "doc_create", json!({"from":"blank"}))["doc"].clone();
    success(&mut app, &mut guard, "doc_save", json!({"doc":id,"path":"saved.pdf","full":true}));
    assert!(root.join("saved.pdf").is_file());
    let opened = success(&mut app, &mut guard, "doc_open", json!({"path":"saved.pdf"}));
    assert_ne!(opened["doc"], id);
    assert_eq!(app.views.len(), 2);
    guard = Guard::new(&app, String::new(), Vec::new());
    let outside = execute(&mut app, &mut guard, &call("doc_open", json!({"path":"../outside.pdf"}))).unwrap();
    assert_eq!(outside["isError"], true);
    assert_eq!(app.session.docs().len(), 2);
    let dirty = execute(&mut app, &mut guard, &call("doc_close", json!({"doc":opened["doc"]}))).unwrap();
    assert_eq!(dirty["isError"], false, "opening a saved file is clean and closable");
    success(&mut app, &mut guard, "doc_set_info", json!({"doc":id,"key":"Title","value":"Unsaved"}));
    let refused = execute(&mut app, &mut guard, &call("doc_close", json!({"doc":id}))).unwrap();
    assert_eq!(refused["isError"], true);
    assert!(app.session.get(DocId(id.as_u64().unwrap())).is_some());
}

#[test]
fn cancellation_prevents_queued_mcp_edits_without_replay() {
    let mut app = PdfCraftApp::new();
    let mut guard = Guard::new(&app, String::new(), Vec::new());
    let id = success(&mut app, &mut guard, "doc_create", json!({"from":"blank"}))["doc"].clone();
    let cancel = Cancellation::new();
    cancel.cancel();
    let (reply, receive) = mpsc::channel();
    dispatch(
        &mut app,
        &egui::Context::default(),
        &mut guard,
        Pending {
            call: call("doc_set_info", json!({"doc":id,"key":"Title","value":"Must not execute"})),
            reply,
            cancel,
            expires: Instant::now() + Duration::from_secs(1),
        },
    );
    assert!(receive.recv().unwrap().is_err());
    assert_ne!(app.session.get(DocId(id.as_u64().unwrap())).unwrap().info.title.as_deref(), Some("Must not execute"));
}

#[test]
fn paragraph_listing_identity_survives_new_assistant_turns() {
    let content = "BT /F1 12 Tf 14.4 TL 1 0 0 1 10 250 Tm (Alpha paragraph, first row.) Tj T* (Alpha paragraph, second row.) Tj ET \
                   BT /F1 12 Tf 14.4 TL 1 0 0 1 10 200 Tm (Bravo paragraph, a single row.) Tj ET \
                   BT /F1 12 Tf 14.4 TL 1 0 0 1 10 150 Tm (Charlie paragraph, first row.) Tj T* (Charlie paragraph, second row.) Tj ET";
    let objects = [
        "<< /Type /Catalog /Pages 2 0 R >>".to_owned(),
        "<< /Type /Pages /Kids [4 0 R] /Count 1 >>".to_owned(),
        "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_owned(),
        "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 300] /Resources << /Font << /F1 3 0 R >> >> /Contents 5 0 R >>".to_owned(),
        format!("<< /Length {} >>\nstream\n{content}\nendstream", content.len()),
    ];
    let mut bytes = b"%PDF-1.7\n".to_vec();
    let mut offsets = Vec::new();
    for (index, object) in objects.iter().enumerate() {
        offsets.push(bytes.len());
        bytes.extend_from_slice(format!("{} 0 obj\n{object}\nendobj\n", index + 1).as_bytes());
    }
    let xref = bytes.len();
    bytes.extend_from_slice(b"xref\n0 6\n0000000000 65535 f \n");
    for offset in offsets {
        bytes.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
    }
    bytes.extend_from_slice(format!("trailer\n<< /Size 6 /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n").as_bytes());
    let mut app = PdfCraftApp::new();
    app.open_bytes("Synthetic paragraphs.pdf", None, bytes).unwrap();
    let id = app.active_ids().unwrap().1;
    let mut guard = Guard::new(&app, String::new(), Vec::new());
    let listed = success(&mut app, &mut guard, "text_paragraphs", json!({"doc":id.0,"page":1}));
    assert_eq!(listed["count"], 3);
    // Upstream 0.18 keeps rewritten regions distinct; a pure move still regroups Alpha/Bravo.
    let edited = success(&mut app, &mut guard, "text_edit", json!({"doc":id.0,"page":1,"paragraph":1,"dy":-21.2}));
    assert_eq!(edited["stale_paragraphs"], json!([1, 2]));
    let mut next_turn = Guard::new(&app, String::new(), Vec::new());
    let refused =
        execute(&mut app, &mut next_turn, &call("text_edit", json!({"doc":id.0,"page":1,"paragraph":2,"text":"Must not overwrite Charlie"})))
            .unwrap();
    assert_eq!(refused["isError"], true, "a new turn must retain MCP's stale-paragraph protection");
    let text = success(&mut app, &mut next_turn, "text_extract", json!({"doc":id.0})).to_string();
    assert!(text.contains("Charlie paragraph, first row."));
    assert!(!text.contains("Must not overwrite Charlie"));
}
