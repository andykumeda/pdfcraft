//! Help ▸ Check for updates and the check at start, through `craft_update`: the flow itself is its
//! `Updater`; this module supplies the suite, the quit path and the status dialog.

#[cfg(not(target_arch = "wasm32"))]
use craft_update::updater::{Action, CheckSource, Status, Updater};
#[cfg(not(target_arch = "wasm32"))]
use craft_update::{CheckResultDialog, CheckResultResponse};
use egui::{Align, Layout};

use crate::{PdfCraftApp, theme, widgets};

/// Where every PdfCraft release is listed.
pub const RELEASES_PAGE: &str = "https://github.com/storytold/pdfcraft/releases";

/// Stands in for the network in tests.
#[cfg(not(target_arch = "wasm32"))]
pub type UpdateSource = CheckSource;

pub(crate) struct Updates {
    disabled: bool,
    #[cfg(not(target_arch = "wasm32"))]
    updater: Updater,
    /// The Updates dialog is showing.
    pub(crate) open: bool,
    /// Ask for a newer release at start, unless Preferences says not to; the desktop app turns
    /// it on, so tests never reach the network.
    #[cfg(not(target_arch = "wasm32"))]
    startup_check: bool,
    #[cfg(not(target_arch = "wasm32"))]
    check_requested: bool,
    /// How many times `on_exit` called [`Updates::exiting`].
    #[cfg(test)]
    exits: u32,
}

impl Default for Updates {
    fn default() -> Self {
        Self {
            disabled: false,
            #[cfg(not(target_arch = "wasm32"))]
            updater: Updater::new(&crate::UPDATE_APP, env!("CARGO_PKG_VERSION")),
            open: false,
            #[cfg(not(target_arch = "wasm32"))]
            startup_check: false,
            #[cfg(not(target_arch = "wasm32"))]
            check_requested: false,
            #[cfg(test)]
            exits: 0,
        }
    }
}

impl Updates {
    /// Starts the helper of an update that waits for the app to exit. Later calls do nothing.
    pub(crate) fn exiting(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        self.updater.start_pending_update();
        #[cfg(test)]
        {
            self.exits += 1;
        }
    }
}

impl PdfCraftApp {
    /// Uses only explicitly configured assistant-enabled releases; never falls back upstream.
    /// Call when attaching the independently installed assistant bridge. An explicit update
    /// opt-out remains in force.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn configure_assistant_updates(&mut self, bridge_protocol: u32) {
        let root = craft_ai_native::storage::SharedStore::default_root().map_err(|error| error.to_string());
        self.updates.updater = craft_update::custom::updater(&crate::UPDATE_APP, env!("CARGO_PKG_VERSION"), root, bridge_protocol);
    }

    /// Custom previews must never install a standard release over themselves.
    pub fn disable_updates(&mut self) {
        self.updates.disabled = true;
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.updates.startup_check = false;
            self.updates.check_requested = false;
        }
    }

    /// Whether the app asks for a newer release on its first frame (off until the desktop app turns it on).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_startup_update_check(&mut self, on: bool) {
        self.updates.startup_check = on && !self.updates.disabled;
    }

    /// Replaces the network with `source` (tests).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_update_source(&mut self, source: UpdateSource) {
        self.updates.updater = Updater::new(&crate::UPDATE_APP, env!("CARGO_PKG_VERSION")).with_source(source);
    }

    /// Help ▸ Check for updates: ask for the latest release and show the outcome.
    pub fn check_for_updates(&mut self) {
        if self.updates.disabled {
            self.updates.open = true;
            return;
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.updates.open = true;
            self.updates.check_requested = true;
        }
        #[cfg(target_arch = "wasm32")]
        {
            self.open_url(RELEASES_PAGE);
        }
    }

    /// Starts the checks and picks up their answers (each frame).
    pub(crate) fn poll_updates(&mut self) {
        if self.updates.disabled {
            return;
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let Some(ctx) = self.ctx.clone() else { return };
            // Settled on the first frame: a preference turned on later waits for the next start.
            let at_start = std::mem::take(&mut self.updates.startup_check) && self.update_check_at_start;
            let updates = &mut self.updates;
            if std::mem::take(&mut updates.check_requested) {
                updates.updater.check_now(&ctx);
            } else if at_start {
                updates.updater.check_at_start(&ctx);
            }
            updates.updater.tick(&ctx);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn can_restart_for_update(app: &PdfCraftApp) -> bool {
    app.first_dirty().is_none() && app.assistant.can_restart_for_update()
}

/// The startup prompt, and the Updates dialog that answers Help ▸ Check for updates.
pub(crate) fn dialog(app: &mut PdfCraftApp, ctx: &egui::Context) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        // The helper is scheduled before close: protect documents, active chats and pending input.
        let can_restart = can_restart_for_update(app);
        if !app.updates.disabled {
            match app.updates.updater.show_if(ctx, can_restart) {
                Action::None => {}
                Action::OpenUrl(url) => app.open_url(&url),
                Action::Close => {
                    app.request_quit(ctx);
                }
            }
        }
        // A requested check that found a release is answered by the prompt, not by this dialog.
        if !app.updates.disabled && app.updates.open && !app.updates.check_requested && matches!(app.updates.updater.status(), Status::Idle) {
            app.updates.open = false;
        }
        // The shared dialog links the suite repository; custom sources keep their own routed UI.
        if !app.updates.disabled && app.updates.updater.repository() == Some(crate::UPDATE_APP.repo) {
            if app.updates.open {
                let dialog = CheckResultDialog::new(&crate::UPDATE_APP, env!("CARGO_PKG_VERSION"));
                match dialog.show(ctx, app.updates.updater.status()) {
                    CheckResultResponse::None => {}
                    CheckResultResponse::Close => {
                        app.updates.open = false;
                        app.updates.updater.dismiss_status();
                    }
                    CheckResultResponse::CheckAgain => app.updates.check_requested = true,
                    CheckResultResponse::RestartToUpdate => app.updates.updater.restart_to_update(),
                }
            }
            return;
        }
    }
    if !app.updates.open {
        return;
    }
    let t = theme::Tokens::get(ctx);
    let current = env!("CARGO_PKG_VERSION");
    let mut close = false;
    let modal = crate::widgets::show_modal(ctx, "updates", |ui| {
        ui.set_width(420.0);
        ui.horizontal(|ui| {
            ui.add(crate::icons::image("cloud", 22.0, t.accent));
            ui.label(egui::RichText::new(tl!("Check for updates")).font(theme::semibold(16.0)));
        });
        ui.add_space(8.0);
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(repo) = app.updates.updater.repository() {
            ui.label(crate::i18n::fmt(tl!("Update source: {repo}"), &[("repo", repo)]));
        }
        if app.updates.disabled {
            ui.label(tl!("App updates are managed by ArtCraft Launcher."));
        }
        #[cfg(not(target_arch = "wasm32"))]
        if !app.updates.disabled && !app.assistant.can_restart_for_update() {
            ui.label(tl!("Finish or stop the assistant and close its panel before restarting PdfCraft. Send or clear any draft and finish settings changes first."));
        }
        #[cfg(not(target_arch = "wasm32"))]
        if !app.updates.disabled {
            match app.updates.updater.status() {
                Status::Idle => {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label(tl!("Checking for a newer version…"));
                    });
                }
                Status::Checking => {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label(tl!("Checking for a newer version…"));
                    });
                }
                Status::UpToDate => {
                    ui.label(crate::i18n::fmt(tl!("PdfCraft {c} is up to date."), &[("c", current)]));
                }
                Status::Failed(e) => {
                    ui.label(crate::i18n::fmt(tl!("Couldn't check for updates: {e}"), &[("e", e)]));
                    let releases_page = app.updates.updater.repository().map(|repo| format!("https://github.com/{repo}/releases"));
                    ui.label(
                        egui::RichText::new(crate::i18n::fmt(
                            tl!("You have version {c}. All releases are listed at {page}."),
                            &[("c", current), ("page", releases_page.as_deref().unwrap_or(tl!("No compatible update source configured")))],
                        ))
                        .color(t.text_muted),
                    );
                }
                Status::Installing => {
                    ui.label(tl!("Preparing the update…"));
                }
                Status::Ready => {
                    if ui.add_enabled(can_restart_for_update(app), egui::Button::new(tl!("Restart to update"))).clicked() {
                        app.updates.updater.restart_to_update();
                    }
                }
                Status::Managed => {
                    ui.label(tl!("App updates are managed by ArtCraft Launcher."));
                }
                Status::Updated(version) => {
                    ui.label(format!("Updated to {version}."));
                }
                Status::InstallFailed(error) => {
                    ui.label(format!("Update failed: {error}"));
                }
            }
        }
        #[cfg(target_arch = "wasm32")]
        let _ = (&t, current);
        ui.add_space(10.0);
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if widgets::pill_button(ui, tl!("Close"), true).clicked() {
                close = true;
            }
        });
    });
    if modal.should_close() {
        close = true;
    }
    if close {
        app.updates.open = false;
        #[cfg(not(target_arch = "wasm32"))]
        app.updates.updater.dismiss_status();
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod preview_tests {
    use super::*;

    #[test]
    fn disabled_preview_updates_never_queue_startup_or_manual_checks() {
        let mut app = PdfCraftApp::new();
        app.set_startup_update_check(true);
        app.check_for_updates();
        assert!(app.updates.startup_check && app.updates.check_requested);
        app.disable_updates();
        app.configure_assistant_updates(1);
        app.set_startup_update_check(true);
        app.check_for_updates();
        assert!(app.updates.disabled && app.updates.open);
        assert!(!app.updates.startup_check && !app.updates.check_requested);
        app.poll_updates();
        assert!(matches!(app.updates.updater.status(), Status::Idle));
    }
    #[test]
    fn app_update_restart_waits_for_chat_and_settings_work() {
        let mut app = PdfCraftApp::new();
        assert!(can_restart_for_update(&app));
        app.assistant.widget.busy = true;
        assert!(!can_restart_for_update(&app));
        app.assistant.widget.busy = false;
        app.assistant.widget.settings.busy = true;
        assert!(!can_restart_for_update(&app));
        app.assistant.widget.settings.busy = false;
        assert!(can_restart_for_update(&app));
        app.enable_shared_assistant();
        assert!(can_restart_for_update(&app), "an unstarted helper does not block app updates");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eframe::App as _;

    #[test]
    fn on_exit_calls_the_update_exit_hook_once() {
        let mut app = PdfCraftApp::new();
        app.on_exit(None);
        assert_eq!(app.updates.exits, 1);
    }
}
