use super::App;
use crate::utils::{Key, Secret};
use egui::{Context, TextEdit, Window};

#[derive(Default, Clone)]
pub struct ApiPerfsData {
    buf: String,
    open: bool,
}

pub trait ApiPerfsWindow {
    fn draw_api_perfs_window(&mut self, ctx: &Context);
    fn open_api_perfs_window(&mut self);
    fn _enable_submit(&self) -> bool;
}

impl ApiPerfsWindow for App {
    fn draw_api_perfs_window(&mut self, ctx: &Context) {
        let ApiPerfsData { mut open, mut buf } = self.api_perfs.clone();
        Window::new("API Preferences")
            .open(&mut open)
            .show(ctx, |ui| {
                ui.label("HCloud API Key"); // TODO: Add info about how tokens are stored
                ui.add(TextEdit::singleline(&mut buf).password(true));
                ui.separator();
                ui.add_enabled_ui(self._enable_submit(), |ui| {
                    if ui.button("Submit").clicked() {
                        self.hcloud_api_secret = Some(Secret::Unencrypted(Key(buf.to_owned())));
                        self.api_perfs.open = false;
                    }
                })
            });

        //FIXME: weird handling of window internal closing
        if !open {
            self.api_perfs.open = open;
        }

        self.api_perfs.buf = buf;
    }
    fn open_api_perfs_window(&mut self) {
        if !self.api_perfs.open {
            self.api_perfs.buf = match self.hcloud_api_secret.clone() {
                Some(Secret::Unencrypted(Key(token))) => token,
                _ => Default::default(),
            };
            self.api_perfs.open = true;
        }
    }

    fn _enable_submit(&self) -> bool {
        !self.api_perfs.buf.is_empty()
    }
}
