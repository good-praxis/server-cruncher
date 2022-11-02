use crate::{api, utils::RemoteData};
use egui::{Context, InnerResponse, TopBottomPanel};
use std::sync::mpsc::Sender;

type Component = InnerResponse<()>;

pub struct StatusBar {}
impl StatusBar {
    pub fn build(
        server_list: &mut Option<RemoteData>,
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
                if ui.button("Request Server List").clicked() {
                    api::req_server_list(tx.clone(), ctx.clone())
                }
                ui.label(format!("Last updated: {}", last_updated));
            });
        })
    }
}
