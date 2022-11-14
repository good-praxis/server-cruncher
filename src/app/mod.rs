mod api;
mod components;

use crate::utils::{Data, Error, RemoteData, Secret};
use api::Endpoints;
use components::*;
use serde::{Deserialize, Serialize};
use serde_encrypt::{shared_key::SharedKey, AsSharedKey};
use std::{
    collections::HashSet,
    sync::mpsc::{Receiver, Sender},
    time::Duration,
};

pub(crate) type App = ServerCruncherApp;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ServerCruncherApp {
    #[serde(skip)]
    tx: Sender<RemoteData>,
    #[serde(skip)]
    rx: Receiver<RemoteData>,

    local_key: SharedKey,

    endpoint: Endpoints,
    hcloud_api_secret: Option<Secret>,
    application_list: Option<RemoteData>,

    #[serde(skip)] // Always skip UI Indicators
    remote_loading: HashSet<String>,

    #[serde(skip)] // Skip error log
    error_log: Vec<Error>,

    #[serde(skip)]
    show_error_log: bool,
    #[serde(skip)]
    api_perfs: ApiPerfsData,
}

impl Default for ServerCruncherApp {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        Self {
            tx,
            rx,
            local_key: AsSharedKey::generate(),
            endpoint: Endpoints::Unconfigured,
            hcloud_api_secret: None,
            application_list: None,
            remote_loading: HashSet::new(),
            error_log: Vec::new(),
            show_error_log: false,
            api_perfs: Default::default(),
        }
    }
}

impl ServerCruncherApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            let mut loaded_app: Self =
                eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();

            // Unencrypt api key
            if let Some(secret) = loaded_app.hcloud_api_secret {
                loaded_app.hcloud_api_secret = Some(secret.decrypt(&loaded_app.local_key));
                loaded_app.endpoint = Endpoints::Hcloud;
            }
            return loaded_app;
        }

        Default::default()
    }
}

impl eframe::App for ServerCruncherApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // Encrypt API keys before writing to disk
        if let Some(secret) = self.hcloud_api_secret.clone() {
            self.hcloud_api_secret = Some(secret.encrypt(&self.local_key));
        }
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
                ui.menu_button("🔧 Settings", |ui| {
                    if ui.button("Show Error Log").clicked() {
                        self.show_error_log = true;
                    }
                    if ui.button("API Preferences").clicked() {
                        self.open_api_perfs_window();
                    }
                });
            });
        });

        if let Ok(remote) = self.rx.try_recv() {
            self.handle_incoming_remote(remote);
        }

        self.draw_status_bar(ctx);
        self.draw_error_window(ctx);
        self.draw_api_perfs_window(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            self.draw_application_windows(ctx);
            ctx.request_repaint_after(Duration::new(1, 0));
            egui::warn_if_debug_build(ui);
        });
    }
}

impl App {
    pub fn set_loading(&mut self, origin: &str) {
        self.remote_loading.insert(origin.to_string());
    }
    pub fn unset_loading(&mut self, origin: &str) {
        self.remote_loading.remove(origin);
    }
    fn handle_incoming_remote(&mut self, remote: RemoteData) {
        match remote.data {
            Data::Application(_) => {
                self.unset_loading(&remote.origin);
                self.application_list = Some(remote);
            }
            Data::Error(e) => {
                self.error_log.push(Error {
                    error: e,
                    ts: remote.updated_at,
                });
                self.unset_loading(&remote.origin);
                self.show_error_log = true;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::App;
    use crate::utils::{Data, RemoteData};

    #[test]
    fn set_loading() {
        const ORIGIN: &str = "loading";
        let mut app = App::default();
        assert!(app.remote_loading.is_empty());
        app.set_loading(ORIGIN);
        assert!(app.remote_loading.contains(ORIGIN));
    }

    #[test]
    fn unset_loading() {
        const FIRST_ORIGIN: &str = "loading";
        const SECOND_ORIGIN: &str = "not loading";
        let mut app = App::default();
        assert!(app.remote_loading.is_empty());
        app.set_loading(FIRST_ORIGIN);
        app.set_loading(SECOND_ORIGIN);
        app.unset_loading(SECOND_ORIGIN);

        assert!(app.remote_loading.contains(FIRST_ORIGIN));
        assert!(!app.remote_loading.contains(SECOND_ORIGIN));
    }

    #[test]
    fn handle_incoming_remote() {
        const ORIGIN: &str = "loading";
        const ERROR: &str = "oopsie";
        let mut app = App::default();
        app.set_loading(ORIGIN);
        assert!(app.application_list.is_none());

        let remote_application = RemoteData::new(Data::Application(vec![]), ORIGIN);
        app.handle_incoming_remote(remote_application);
        assert!(app.application_list.is_some());
        assert!(!app.remote_loading.contains(ORIGIN));

        let remote_error = RemoteData::new(Data::Error(ERROR.to_string()), ORIGIN);
        assert!(app.error_log.is_empty());
        app.handle_incoming_remote(remote_error);
        assert!(!app.error_log.is_empty());
        assert_eq!(app.error_log[0].error, ERROR);
    }
}
