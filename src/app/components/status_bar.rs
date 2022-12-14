use super::App;
use crate::utils::Secret;
use egui::{Context, CursorIcon, TopBottomPanel, Ui};

const API_ORIGIN: &str = "status_bar";

impl App {
    pub fn draw_status_bar(&mut self, ctx: &Context) {
        TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            let last_updated = match self.application_list.clone() {
                None => "Never".to_string(),
                Some(remote) => remote.updated_at.to_string(),
            };

            ui.horizontal(|ui| {
                self.button(ctx, ui);
                ui.label(format!("Last updated: {}", last_updated));
            });
        });
    }

    fn button(&mut self, ctx: &Context, ui: &mut Ui) {
        let Self {
            hcloud_api_secret, ..
        } = self;

        match (self.remote_loading.contains(API_ORIGIN), hcloud_api_secret) {
            (true, _) => {
                ui.spinner().on_hover_cursor(CursorIcon::Wait);
            }
            (_, Some(Secret::Unencrypted(_))) => {
                if ui
                    .button("⟳")
                    .on_hover_text("Refresh Server List")
                    .clicked()
                {
                    self.remote_loading.insert(API_ORIGIN.to_string());
                    self.req_application_list(API_ORIGIN, ctx);
                }
            }
            _ => {
                ui.add_enabled_ui(false, |ui| {
                    ui.button("🚫").on_disabled_hover_text("No API Configured");
                });
            }
        }
    }
}
