use crate::{api, utils::RemoteData};
use egui::{Context, CursorIcon, InnerResponse, TopBottomPanel, Ui};
use std::sync::mpsc::Sender;

type Component = InnerResponse<()>;

pub struct StatusBar;
impl StatusBar {
    pub fn build(
        server_list: &mut Option<RemoteData>,
        loading: &mut bool,
        tx: &Sender<RemoteData>,
        ctx: &Context,
    ) -> Component {
        TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            let last_updated = match server_list {
                None => "Never",
                Some(remote) => {
                    remote.generate_update_label();
                    &remote.label
                }
            };

            ui.horizontal(|ui| {
                Self::button(ui, loading, tx, ctx);
                ui.label(format!("Last updated: {}", last_updated));
            });
        })
    }

    fn tooltip(ui: &mut Ui) {
        ui.label("Refresh Server List");
    }

    fn button(ui: &mut Ui, loading: &mut bool, tx: &Sender<RemoteData>, ctx: &Context) {
        match loading {
            true => {
                ui.spinner().on_hover_cursor(CursorIcon::Wait);
            }
            false => {
                if ui.button("⟲").on_hover_ui(Self::tooltip).clicked() {
                    *loading = true;
                    api::req_server_list(tx.clone(), ctx.clone());
                }
            }
        }
    }
}
