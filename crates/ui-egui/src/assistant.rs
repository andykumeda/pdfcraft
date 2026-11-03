//! PdfCraft's small host adapter for the reusable Craft assistant.

use craft_ui::assistant::{Action, State};
use serde_json::{Value, json};

use crate::PdfCraftApp;

#[cfg(not(target_arch = "wasm32"))]
mod host;
#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
mod remote;

pub struct Assistant {
    pub open: bool,
    pub(crate) settings_open: bool,
    pub widget: State,
    pub enabled: bool,
    #[cfg(not(target_arch = "wasm32"))]
    native: native::Native,
    #[cfg(not(target_arch = "wasm32"))]
    remote: remote::Remote,
}

impl Drop for Assistant {
    fn drop(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        self.native.cancel();
        #[cfg(not(target_arch = "wasm32"))]
        self.remote.cancel();
    }
}

impl PdfCraftApp {
    /// Attaches the independently installed assistant and its explicit app-update source.
    /// Desktop entry points call this once before startup update checks.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn enable_shared_assistant(&mut self) {
        self.assistant.native.cancel();
        self.assistant.remote.enabled = true;
        self.assistant.enabled = true;
        self.assistant.widget.available = true;
        self.configure_assistant_updates(craft_assistant::config::compatibility().protocol);
    }
}

pub(crate) fn owns_keyboard(app: &PdfCraftApp) -> bool {
    #[cfg(not(target_arch = "wasm32"))]
    {
        app.assistant.remote.enabled && app.assistant.remote.owns_keyboard()
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = app;
        false
    }
}

pub fn commands() -> Vec<Value> {
    vec![
        json!({"id":"ui.assistant","label":"Assistant","location":"View › Assistant","params":"value?: bool","enabled":true}),
        json!({"id":"ui.assistant.send","label":"Send assistant prompt","location":"Agents","params":"text: string; files?: array of exact paths","enabled":cfg!(not(target_arch="wasm32"))}),
        json!({"id":"ui.assistant.newChat","label":"New assistant chat","location":"Agents","params":"{}","enabled":cfg!(not(target_arch="wasm32"))}),
        json!({"id":"ui.assistant.stop","label":"Stop assistant","location":"Agents","params":"{}","enabled":true}),
        json!({"id":"ui.assistant.status","label":"Assistant status","location":"Agents","params":"{}","enabled":true}),
        json!({"id":"ui.assistant.updateCheck","label":"Check shared assistant updates","location":"Agents","params":"{}","enabled":cfg!(not(target_arch="wasm32"))}),
        json!({"id":"ui.assistant.updateInstall","label":"Install offered shared assistant update","location":"Agents","params":"{}","enabled":cfg!(not(target_arch="wasm32"))}),
        json!({"id":"ui.assistant.updateStatus","label":"Shared assistant update status","location":"Agents","params":"{}","enabled":cfg!(not(target_arch="wasm32"))}),
        json!({"id":"ui.assistant.reloadComponent","label":"Reload shared assistant component","location":"Agents","params":"{}","enabled":cfg!(not(target_arch="wasm32"))}),
    ]
}

pub(crate) fn command(app: &mut PdfCraftApp, id: &str, params: &Value) -> Option<Result<Value, String>> {
    if !id.starts_with("ui.assistant") {
        return None;
    }
    if matches!(id, "ui.assistant.updateCheck" | "ui.assistant.updateInstall" | "ui.assistant.updateStatus" | "ui.assistant.reloadComponent") {
        if !params.as_object().is_some_and(serde_json::Map::is_empty) {
            return Some(Err("This assistant service command takes an empty parameter object.".into()));
        }
        #[cfg(not(target_arch = "wasm32"))]
        return Some(remote::service_command(app, id));
        #[cfg(target_arch = "wasm32")]
        return Some(Err("The shared assistant is available in the desktop app.".into()));
    }
    let result = match id {
        "ui.assistant" => {
            app.assistant.open = params.get("value").and_then(Value::as_bool).unwrap_or(!app.assistant.open);
            ensure_loaded(app);
            Ok(json!({"open":app.assistant.open}))
        }
        "ui.assistant.send" => {
            let Some(text) = params.get("text").and_then(Value::as_str).filter(|s| !s.trim().is_empty()) else {
                return Some(Err("give a nonempty text prompt".into()));
            };
            if let Some(files) = params.get("files") {
                let Some(files) = files.as_array().filter(|files| files.len() <= 32) else {
                    return Some(Err("files must contain at most 32 exact file paths".into()));
                };
                let Some(files) = files.iter().map(|v| v.as_str().map(str::to_owned)).collect::<Option<Vec<_>>>() else {
                    return Some(Err("file paths must be strings".into()));
                };
                app.assistant.widget.authorized_files = files;
            }
            app.assistant.open = true;
            action(app, Action::Send(text.into())).map(|()| json!({"started":true}))
        }
        "ui.assistant.newChat" => action(app, Action::NewChat).map(|()| json!({"started":true})),
        "ui.assistant.stop" => action(app, Action::Stop).map(|()| json!({"stopped":true})),
        "ui.assistant.status" => Ok(
            json!({"open":app.assistant.open,"busy":app.assistant.widget.busy,"available":app.assistant.enabled,"settingsBusy":app.assistant.widget.settings.busy,"chatId":app.assistant.widget.active_chat,"model":app.assistant.widget.chat_route,"chatModel":app.assistant.widget.chat_route,"controlModel":app.assistant.widget.chat_route,"status":app.assistant.widget.status,"error":app.assistant.widget.error}),
        ),
        _ => Err("unknown assistant command".into()),
    };
    Some(result)
}

pub fn pane(app: &mut PdfCraftApp, ui: &mut egui::Ui) {
    #[cfg(not(target_arch = "wasm32"))]
    if app.assistant.remote.enabled {
        remote::show(app, ui, false);
        return;
    }
    ensure_loaded(app);
    if !app.assistant.widget.busy {
        app.assistant.widget.target_label =
            app.active_ids().and_then(|(_, id)| app.session.get(id)).map(|doc| doc.name.clone()).unwrap_or_else(|| "No active PDF".into());
    }
    #[cfg(not(target_arch = "wasm32"))]
    if app.assistant.widget.settings_open {
        configuration(app, ui);
    }
    let id = egui::Id::unique("assistant");
    let actions = craft_ui::assistant::show(ui, id, &mut app.assistant.widget, |text| tl!(text).into());
    apply_actions(app, actions);
}

pub fn settings(app: &mut PdfCraftApp, ui: &mut egui::Ui) {
    #[cfg(not(target_arch = "wasm32"))]
    if app.assistant.remote.enabled {
        remote::show(app, ui, true);
        return;
    }
    ensure_loaded(app);
    if !app.assistant.enabled {
        ui.label(tl!("The assistant is available in the desktop app."));
        return;
    }
    let id = egui::Id::unique("assistant_settings");
    let actions = craft_ui::assistant::settings(ui, id, &mut app.assistant.widget.settings, |text| tl!(text).into());
    apply_actions(app, actions);
    #[cfg(not(target_arch = "wasm32"))]
    configuration(app, ui);
}

#[cfg(not(target_arch = "wasm32"))]
fn configuration(app: &PdfCraftApp, ui: &mut egui::Ui) {
    if let Some(id) = &app.assistant.widget.settings.app_id
        && let Ok(choice) = craft_ai_native::resolve_choice(&app.assistant.widget.settings.store, id)
    {
        let endpoint =
            choice.connection.endpoint.as_deref().map(|endpoint| endpoint.split(['?', '#']).next().unwrap_or("")).unwrap_or("No HTTP endpoint");
        ui.label(format!("Effective provider: {} · Model: {}", choice.connection.provider, choice.model));
        ui.label(format!("Endpoint: {endpoint}"));
    }
    #[cfg(not(target_arch = "wasm32"))]
    if let Some(config) = native::effective_config(app) {
        ui.label(format!(
            "Context budget: {} bytes · Response limit: {} bytes · Tool rounds: {}",
            config.max_context_bytes, config.max_response_bytes, config.max_rounds
        ));
        ui.label("Runtime limits: Craft/ai/runtime.json (CRAFT_AI_CONFIG_DIR overrides the root).");
    }
}

fn apply_actions(app: &mut PdfCraftApp, actions: Vec<Action>) {
    for requested in actions {
        if let Err(error) = action(app, requested) {
            app.assistant.widget.error = Some(error.clone());
            app.assistant.widget.settings.error = Some(error);
        }
    }
}

fn ensure_loaded(app: &mut PdfCraftApp) {
    #[cfg(not(target_arch = "wasm32"))]
    if app.assistant.remote.enabled {
        remote::ensure_loaded(app);
        return;
    }
    #[cfg(not(target_arch = "wasm32"))]
    native::ensure_loaded(app);
    #[cfg(target_arch = "wasm32")]
    let _ = app;
}

fn action(app: &mut PdfCraftApp, action: Action) -> Result<(), String> {
    #[cfg(not(target_arch = "wasm32"))]
    if app.assistant.remote.enabled {
        return remote::action(app, action);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        native::action(app, action)
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = (app, action);
        Err("The assistant is available in the desktop app.".into())
    }
}

pub(crate) fn poll(app: &mut PdfCraftApp, ctx: &egui::Context) {
    #[cfg(not(target_arch = "wasm32"))]
    if app.assistant.remote.enabled {
        remote::poll(app, ctx);
        return;
    }
    #[cfg(not(target_arch = "wasm32"))]
    native::poll(app, ctx);
    #[cfg(target_arch = "wasm32")]
    let _ = (app, ctx);
}

impl Default for Assistant {
    fn default() -> Self {
        Self {
            open: false,
            settings_open: false,
            widget: State::default(),
            enabled: cfg!(not(target_arch = "wasm32")),
            #[cfg(not(target_arch = "wasm32"))]
            native: native::Native::default(),
            #[cfg(not(target_arch = "wasm32"))]
            remote: remote::Remote::default(),
        }
    }
}

pub(crate) fn panel(app: &mut PdfCraftApp, ui: &mut egui::Ui) {
    if app.assistant.open {
        egui::Panel::right("assistant_panel").default_size(360.0).size_range(280.0..=600.0).resizable(true).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading(tl!("Assistant"));
                if ui.button(tl!("Close")).clicked() {
                    app.assistant.open = false;
                }
            });
            ui.small(tl!("Edits use normal Undo. Undo cannot reverse a saved file on disk."));
            pane(app, ui);
        });
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use egui_kittest::{Harness, kittest::Queryable as _};
    #[test]
    fn panel_has_one_shared_settings_control_and_opens_its_editor() {
        let mut app = PdfCraftApp::new();
        native::enable_without_profile(&mut app);
        app.assistant.open = true;
        let mut fonts = false;
        let mut harness = Harness::builder().with_size(egui::vec2(1000.0, 700.0)).build_ui_state(
            move |ui, app: &mut PdfCraftApp| {
                if !fonts {
                    crate::theme::install_fonts(ui.ctx());
                    fonts = true;
                    return;
                }
                panel(app, ui);
            },
            app,
        );
        harness.run();
        assert_eq!(harness.query_all_by_label("Settings").count(), 1);
        harness.get_by_label("Settings").click();
        harness.run();
        assert!(harness.state().assistant.widget.settings_open);
        assert_eq!(harness.query_all_by_label("Settings").count(), 1);
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod shared_component_commands {
    use super::*;
    #[test]
    fn update_controls_require_activation_and_report_status_without_starting_a_helper() {
        let mut app = PdfCraftApp::new();
        for id in ["ui.assistant.updateCheck", "ui.assistant.updateInstall", "ui.assistant.reloadComponent"] {
            assert!(command(&mut app, id, &json!({})).unwrap().is_err());
        }
        app.enable_shared_assistant();
        assert!(app.assistant.remote.enabled);
        assert!(command(&mut app, "ui.assistant.updateStatus", &json!({})).unwrap().is_ok());
        assert!(command(&mut app, "ui.assistant.updateStatus", &json!({"source":"untrusted"})).unwrap().is_err());
        assert!(command(&mut app, "ui.assistant.updateCheck", &json!({})).unwrap().unwrap_err().contains("active app window"));
    }
}
