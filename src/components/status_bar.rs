use crate::{api, utils::RemoteData};
use egui::{Context, CursorIcon, TopBottomPanel, Ui};
use std::sync::mpsc::Sender;

pub fn status_bar(
    application_list: &mut Option<RemoteData>,
    loading: &mut bool,
    tx: &Sender<RemoteData>,
    ctx: &Context,
) {
    TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
        let last_updated = match application_list {
            None => "Never".to_string(),
            Some(remote) => remote.updated_at.to_string(),
        };

        ui.horizontal(|ui| {
            button(ui, loading, tx, ctx);
            ui.label(format!("Last updated: {}", last_updated));
        });
    });
}

fn tooltip(ui: &mut Ui) {
    ui.label("Refresh Server List");
}

fn button(ui: &mut Ui, loading: &mut bool, tx: &Sender<RemoteData>, ctx: &Context) {
    match loading {
        true => {
            ui.spinner().on_hover_cursor(CursorIcon::Wait);
        }
        _ => {
            if ui.button("⟳").on_hover_ui(tooltip).clicked() {
                *loading = true;
                api::req_application_list(tx.clone(), ctx.clone());
            }
        }
    }
}
