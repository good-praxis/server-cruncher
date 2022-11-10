/// Collection of structs and helpers for interaction with the HCLOUD API
use crate::utils::{generate_application_list, Data, Key, RemoteData};
use hcloud::apis::configuration::Configuration;
use hcloud::apis::{images_api, servers_api};
use std::sync::mpsc::Sender;

pub fn req_application_list(api_key: Key, tx: Sender<RemoteData>, ctx: egui::Context) {
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
