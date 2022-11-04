/// Collection of structs and helpers for interaction with the HCLOUD API
use crate::utils::{generate_application_list, Data, RemoteData};
use hcloud::apis::configuration::Configuration;
use hcloud::apis::{images_api, servers_api};
use lazy_static::lazy_static;
use std::env::{self, VarError};
use std::sync::mpsc::Sender;

lazy_static! {
    static ref HCLOUD_API_TOKEN: Result<String, VarError> = env::var("HCLOUD_API_TOKEN"); // TODO: opt for configurable through UI token
    static ref CONFIGURATION: Configuration = {
        let mut configuration = Configuration::new();
        if let Ok(token) = HCLOUD_API_TOKEN.clone() {
            configuration.bearer_access_token = Some(token);
        }
        configuration
    };
}

pub fn req_application_list(tx: Sender<RemoteData>, ctx: egui::Context) {
    tokio::spawn(async move {
        let servers = servers_api::list_servers(&CONFIGURATION, Default::default()).await;
        let images = images_api::list_images(&CONFIGURATION, Default::default()).await;

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
