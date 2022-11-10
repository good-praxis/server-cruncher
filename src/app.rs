use crate::{
    components::{self, ApiPerfsComponent},
    utils::{Data, Error, RemoteData, Secret},
};
use serde_encrypt::{shared_key::SharedKey, AsSharedKey};
use std::{
    sync::mpsc::{Receiver, Sender},
    time::Duration,
};
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ServerCruncherApp {
    #[serde(skip)]
    tx: Sender<RemoteData>,
    #[serde(skip)]
    rx: Receiver<RemoteData>,

    local_key: SharedKey,

    hcloud_api_secret: Option<Secret>,
    application_list: Option<RemoteData>,

    #[serde(skip)] // Always skip UI Indicators
    remote_loading: bool,

    #[serde(skip)] // Skip error log
    error_log: Vec<Error>,

    #[serde(skip)]
    show_error_log: bool,
    #[serde(skip)]
    api_perfs: ApiPerfsComponent,
}

impl Default for ServerCruncherApp {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        Self {
            tx,
            rx,
            local_key: AsSharedKey::generate(),
            hcloud_api_secret: None,
            application_list: None,
            remote_loading: false,
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
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

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
                        self.api_perfs.open(&self.hcloud_api_secret);
                    }
                });
            });
        });

        if let Ok(remote) = self.rx.try_recv() {
            match remote.data {
                Data::Application(_) => {
                    self.application_list = Some(remote);
                    self.remote_loading = false
                }
                Data::Error(e) => {
                    self.error_log.push(Error {
                        error: e,
                        ts: remote.updated_at,
                    });
                    self.remote_loading = false;
                    self.show_error_log = true;
                }
            }
        }

        components::status_bar(
            &mut self.application_list,
            &mut self.remote_loading,
            &self.hcloud_api_secret,
            &self.tx,
            ctx,
        );

        components::error_window(&mut self.error_log, &mut self.show_error_log, ctx);
        self.api_perfs
            .api_prefs_window(&mut self.hcloud_api_secret, ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            if let Some(RemoteData {
                data: Data::Application(applications),
                ..
            }) = &self.application_list
            {
                for application in applications {
                    components::application_window(application, ctx);
                }
            }
            ctx.request_repaint_after(Duration::new(1, 0));
            egui::warn_if_debug_build(ui);
        });
    }
}
