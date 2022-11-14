use super::App;
use crate::utils::{generate_application_list, Data, Error, Key, RemoteData};
use egui::Context;
use serde::{Deserialize, Serialize};

mod hetzner;

#[derive(Serialize, Deserialize)]
pub enum Endpoints {
    Unconfigured,
    Hcloud,
}

const NO_API_ENDPOINT: &str = "No API endpoint configured";

impl App {
    pub fn req_application_list(&mut self, origin: &'static str, ctx: &Context) {
        match self.endpoint {
            Endpoints::Unconfigured => {
                self.error_log.push(Error::new(NO_API_ENDPOINT));
                self.unset_loading(origin);
            }
            Endpoints::Hcloud => {
                let api_key: Key = self.hcloud_api_secret.clone().unwrap().into();
                let tx = self.tx.clone();
                let ctx = ctx.clone();

                tokio::spawn(async move {
                    let servers = hetzner::get_server_list(&api_key).await;
                    let images = hetzner::get_image_list(&api_key).await;

                    let _ = match (servers, images) {
                        (Err(e), _) => tx.send(RemoteData::new(Data::Error(e.to_string()), origin)),
                        (_, Err(e)) => tx.send(RemoteData::new(Data::Error(e.to_string()), origin)),
                        (Ok(servers), Ok(images)) => tx.send(RemoteData::new(
                            Data::Application(generate_application_list(&servers, &images)),
                            origin,
                        )),
                    };
                    ctx.request_repaint();
                });
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::NO_API_ENDPOINT;
    use crate::app::App;
    use egui::Context;

    #[test]
    fn req_application_list() {
        // TODO: Use mock library to mock api modules
        let mut app = App::default();
        let ctx = Context::default();
        app.req_application_list("", &ctx);
        assert_eq!(app.error_log[0].error, NO_API_ENDPOINT)
    }
}
