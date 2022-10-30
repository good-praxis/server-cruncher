use crate::utils::Data;
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

    // Example stuff:
    label: String,

    #[serde(skip)] // FIXME: During dev only
    server_list: Option<Vec<Server>>,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,
}

impl Default for ServerCruncherApp {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        Self {
            tx,
            rx,
            server_list: None,
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
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
        let Self { label, value, .. } = self;

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

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });

            ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            if ui.button("Request Server List").clicked() {
                req_server_list(self.tx.clone(), ctx.clone())
            }

            if let Ok(Data::Servers(servers)) = self.rx.try_recv() {
                self.server_list = Some(servers);
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            if let Some(servers) = &self.server_list {
                for server in servers {
                    egui::Window::new(server.name.as_str()).show(ctx, |ui| {
                        ui.heading(server.name.as_str());
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

fn req_server_list(tx: Sender<Data>, ctx: egui::Context) {
    use hcloud::apis::configuration::Configuration;
    use hcloud::apis::servers_api;
    use std::env;

    tokio::spawn(async move {
        let mut configuration = Configuration::new();
        configuration.bearer_access_token =
            Some(env::var("HCLOUD_API_TOKEN").expect("No HCLOUD_API_TOKEN found"));

        let servers = servers_api::list_servers(&configuration, Default::default())
            .await
            .expect("Unable to fetch Server list")
            .servers;

        let _ = tx.send(Data::Servers(servers));
        ctx.request_repaint();
    });
}
