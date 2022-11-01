use crate::{api, utils::Data};
use chrono::prelude::*;
use hcloud::models::server::Server;
use std::sync::mpsc::{Receiver, Sender};
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ServerCruncherApp {
    #[serde(skip)]
    tx: Sender<Data>,
    #[serde(skip)]
    rx: Receiver<Data>,

    #[serde(skip)] // FIXME: During dev only
    server_list: Option<Vec<Server>>,
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

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            if ui.button("Request Server List").clicked() {
                api::req_server_list(self.tx.clone(), ctx.clone())
            }

            const THIRTY_SECONDS_MILLIS: i64 = 30000;
            let last_updated = match self.server_list_updated {
                None => "Never".to_string(),
                Some(time)
                    if Utc::now().timestamp_millis() - time.timestamp_millis()
                        > THIRTY_SECONDS_MILLIS =>
                {
                    // TODO: This could be extracted with the timestamp itself into a data struct; => Less cpu load
                    const HOUR: i64 = 3600000;
                    const MINUTE: i64 = 60000;
                    const SECOND: i64 = 1000;
                    let mut duration = Utc::now().timestamp_millis() - time.timestamp_millis();
                    let h = duration / HOUR;
                    duration = duration % HOUR;

                    let m = duration / MINUTE;
                    duration = duration % MINUTE;

                    let s = duration / SECOND;

                    format!("{}h {}m {}s ago", h, m, s) //FIXME: Request redraw?
                }
                Some(_time) => "Just now".to_string(),
            };

            ui.label(format!("Last updated: {}", last_updated));

            if let Ok(Data::Servers(servers)) = self.rx.try_recv() {
                self.server_list = Some(servers);
                self.server_list_updated = Some(Utc::now());
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            if let Some(servers) = &self.server_list {
                for server in servers {
                    egui::Window::new(server.name.as_str()).show(ctx, |ui| {
                        ui.label(format!(
                            "IP: {}",
                            server.public_net.ipv4.as_ref().unwrap().ip.as_str()
                        ));
                        ui.label(format!(
                            "Datacenter: {}",
                            server.datacenter.description.as_str()
                        ));
                        ui.label(format!("Status: {:?}", server.status));
                        ui.collapsing("details", |ui| ui.label(format!("{:?}", server)));
                    });
                }
            }

            egui::warn_if_debug_build(ui);
        });
    }
}
