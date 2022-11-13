use super::App;
use crate::utils::{generate_application_list, Data, RemoteData, Secret};
use egui::Context;
use hcloud::apis::configuration::Configuration;
use hcloud::apis::{images_api, servers_api};

impl App {
    pub fn req_application_list(&self, ctx: Context) {
        let api_secret = self.hcloud_api_secret.clone();
        let tx = self.tx.clone();
        match api_secret {
            Some(Secret::Unencrypted(api_key)) => {
                tokio::spawn(async move {
                    let config: Configuration = {
                        let mut configuration = Configuration::new();
                        configuration.bearer_access_token = Some(api_key.0.clone());
                        configuration
                    };
                    let servers = servers_api::list_servers(&config, Default::default()).await;
                    let images = images_api::list_images(&config, Default::default()).await;

                    let _ = match (servers, images) {
                        (Err(e), _) => tx.send(RemoteData::new(Data::Error(e.to_string()))),
                        (Ok(_), Err(e)) => tx.send(RemoteData::new(Data::Error(e.to_string()))),
                        (Ok(servers), Ok(images)) => tx.send(RemoteData::new(Data::Application(
                            generate_application_list(&servers, &images),
                        ))),
                    };
                    ctx.request_repaint();
                });
            }
            _ => {
                let err = RemoteData::new(Data::Error("No valid Api configuration".to_string()));
                self.tx.send(err).unwrap(); //FIXME: Unhandeled panic?
            }
        };
    }
}
