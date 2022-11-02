use crate::{
    components::{ServerWindow, StatusBar},
    utils::{Data, RemoteData},
};
use chrono::prelude::*;
use std::{
    borrow::BorrowMut,
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

    #[serde(skip)] // FIXME: During dev only
    server_list: Option<RemoteData>,
    //#[serde(with = "ts_seconds_option")]
    #[serde(skip)]
    server_list_updated: Option<DateTime<Utc>>,
}

impl Default for ServerCruncherApp {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        Self {
            tx,
            rx,
            server_list: None,
            server_list_updated: None,
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
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for ServerCruncherApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
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
            });
        });

        if let Ok(remote) = self.rx.try_recv() {
            match remote.data {
                Data::Servers(_) => self.server_list = Some(remote),
            }
        }

        StatusBar::build(self.server_list.borrow_mut(), &self.tx, ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            if let Some(RemoteData {
                data: Data::Servers(servers),
                ..
            }) = &self.server_list
            {
                for server in servers {
                    ServerWindow::build(server, ctx);
                }
            }

            ctx.request_repaint_after(Duration::new(1, 0));
            egui::warn_if_debug_build(ui);
        });
    }
}
