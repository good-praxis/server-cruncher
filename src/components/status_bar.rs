use crate::{
    api,
    utils::{RemoteData, Secret},
};
use egui::{Context, CursorIcon, TopBottomPanel, Ui};
use std::sync::mpsc::Sender;

pub fn status_bar(
    application_list: &mut Option<RemoteData>,
    loading: &mut bool,
    secret: &Option<Secret>,
    tx: &Sender<RemoteData>,
    ctx: &Context,
) {
    TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
        let last_updated = match application_list {
            None => "Never".to_string(),
            Some(remote) => remote.updated_at.to_string(),
        };

        ui.horizontal(|ui| {
            button(ui, secret, loading, tx, ctx);
            ui.label(format!("Last updated: {}", last_updated));
        });
    });
}

fn button(
    ui: &mut Ui,
    secret: &Option<Secret>,
    loading: &mut bool,
    tx: &Sender<RemoteData>,
    ctx: &Context,
) {
    match (*loading, secret) {
        (true, _) => {
            ui.spinner().on_hover_cursor(CursorIcon::Wait);
        }
        (_, Some(Secret::Unencrypted(key))) => {
            if ui
                .button("⟳")
                .on_hover_text("Refresh Server List")
                .clicked()
            {
                *loading = true;
                api::req_application_list(key.clone(), tx.clone(), ctx.clone());
            }
        }
        _ => {
            ui.add_enabled_ui(false, |ui| {
                ui.button("🚫").on_disabled_hover_text("No API Configured");
            });
        }
    }
}
