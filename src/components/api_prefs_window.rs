use crate::utils::{Key, Secret};
use egui::{Context, TextEdit, Window};

#[derive(Default, Clone)]
pub struct ApiPerfsComponent {
    buf: String,
    open: bool,
}

impl ApiPerfsComponent {
    pub fn api_prefs_window(&mut self, api_data: &mut Option<Secret>, ctx: &Context) {
        let ApiPerfsComponent { mut open, mut buf } = self.clone();
        Window::new("API Preferences")
            .open(&mut open)
            .show(ctx, |ui| {
                ui.label("HCloud API Key"); // TODO: Add info about how tokens are stored
                ui.add(TextEdit::singleline(&mut buf).password(true));
                ui.separator();
                ui.add_enabled_ui(self.enable_submit(), |ui| {
                    if ui.button("Submit").clicked() {
                        *api_data = Some(Secret::Unencrypted(Key(buf.to_owned())));
                        self.open = false;
                    }
                })
            });

        //FIXME: weird handling of window internal closing
        if !open {
            self.open = open;
        }

        self.buf = buf;
    }
    pub fn open(&mut self, api_key: &Option<Secret>) {
        if !self.open {
            self.buf = match api_key {
                Some(Secret::Unencrypted(Key(token))) => token.to_owned(),
                _ => Default::default(),
            };
            self.open = true;
        }
    }

    fn enable_submit(&self) -> bool {
        !self.buf.is_empty()
    }
}
