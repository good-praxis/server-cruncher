use super::App;
use crate::utils::{Key, Secret};
use egui::{Context, TextEdit, Window};

#[derive(Default, Clone)]
pub struct ApiPerfsData {
    buf: String,
    open: bool,
}

impl App {
    pub fn draw_api_perfs_window(&mut self, ctx: &Context) {
        let ApiPerfsData { mut open, mut buf } = self.api_perfs.clone();
        Window::new("API Preferences")
            .open(&mut open)
            .show(ctx, |ui| {
                ui.label("HCloud API Key"); // TODO: Add info about how tokens are stored
                ui.add(TextEdit::singleline(&mut buf).password(true));
                ui.separator();
                ui.add_enabled_ui(self.enable_submit(), |ui| {
                    if ui.button("Submit").clicked() {
                        self.submit();
                    }
                })
            });

        //FIXME: weird handling of window internal closing
        if !open {
            self.api_perfs.open = open;
        }

        self.api_perfs.buf = buf;
    }
    pub fn open_api_perfs_window(&mut self) {
        if !self.api_perfs.open {
            self.api_perfs.buf = match self.hcloud_api_secret.clone() {
                Some(Secret::Unencrypted(Key(token))) => token,
                _ => Default::default(),
            };
            self.api_perfs.open = true;
        }
    }

    fn enable_submit(&self) -> bool {
        !self.api_perfs.buf.is_empty()
    }

    fn submit(&mut self) {
        self.hcloud_api_secret = Some(Secret::Unencrypted(Key(self.api_perfs.buf.to_owned())));
        self.api_perfs.open = false;
    }
}
#[cfg(test)]
mod test {
    use super::ApiPerfsData;
    use crate::{
        app::App,
        utils::{Key, Secret},
    };

    #[test]
    fn enable_submit() {
        let mut app = App::default();
        assert!(!app.enable_submit());

        app.api_perfs.buf = String::from("filled");
        assert!(app.enable_submit());
    }

    #[test]
    fn open_api_perfs_window() {
        let mut app = App::default();
        let secret = String::from("trustnoone");
        app.hcloud_api_secret = Some(Secret::Unencrypted(Key(secret.clone())));

        assert!(app.api_perfs.buf.is_empty());
        assert!(!app.api_perfs.open);
        app.open_api_perfs_window();
        assert!(app.api_perfs.open);
        assert_eq!(app.api_perfs.buf, secret);

        app.hcloud_api_secret = None;
        app.api_perfs.open = false;
        app.open_api_perfs_window();
        assert!(app.api_perfs.buf.is_empty());
    }

    #[test]
    fn submit() {
        let mut app = App::default();
        let secret = String::from("Some data");
        app.api_perfs = ApiPerfsData {
            buf: secret.clone(),
            open: true,
        };
        app.submit();

        assert!(!app.api_perfs.open);
        assert!(matches!(
            app.hcloud_api_secret,
            Some(Secret::Unencrypted(Key(inner))) if inner == secret
        ));
    }
}
