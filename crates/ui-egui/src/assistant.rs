//! PdfCraft's small host adapter for the reusable Craft assistant.

use craft_ui::assistant::{Action, State};
use serde_json::{Value, json};

use crate::PdfCraftApp;

#[cfg(not(target_arch = "wasm32"))]
mod host;
#[cfg(not(target_arch = "wasm32"))]
mod native;

pub struct Assistant {
    pub open: bool,
    pub(crate) settings_open: bool,
    pub widget: State,
    pub enabled: bool,
    #[cfg(not(target_arch = "wasm32"))]
    native: native::Native,
}

impl Drop for Assistant {
    fn drop(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        self.native.cancel();
    }
}

pub fn commands() -> Vec<Value> {
    vec![
        json!({"id":"ui.assistant","label":"Assistant","location":"View › Assistant","params":"value?: bool","enabled":true}),
        json!({"id":"ui.assistant.send","label":"Send assistant prompt","location":"Agents","params":"text: string; files?: array of exact paths","enabled":cfg!(not(target_arch="wasm32"))}),
        json!({"id":"ui.assistant.newChat","label":"New assistant chat","location":"Agents","params":"{}","enabled":cfg!(not(target_arch="wasm32"))}),
        json!({"id":"ui.assistant.stop","label":"Stop assistant","location":"Agents","params":"{}","enabled":true}),
        json!({"id":"ui.assistant.status","label":"Assistant status","location":"Agents","params":"{}","enabled":true}),
    ]
}

pub(crate) fn command(app: &mut PdfCraftApp, id: &str, params: &Value) -> Option<Result<Value, String>> {
    if !id.starts_with("ui.assistant") {
        return None;
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
    native::ensure_loaded(app);
    #[cfg(target_arch = "wasm32")]
    let _ = app;
}

fn action(app: &mut PdfCraftApp, action: Action) -> Result<(), String> {
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
