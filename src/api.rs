/// Collection of structs and helpers for interaction with the HCLOUD API
use crate::utils::{Data, RemoteData};
use hcloud::apis::configuration::Configuration;
use hcloud::apis::servers_api;
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

pub fn req_server_list(tx: Sender<RemoteData>, ctx: egui::Context) {
    tokio::spawn(async move {
        let servers = servers_api::list_servers(&CONFIGURATION, Default::default()).await;

        let _ = match servers {
            Ok(servers) => tx.send(RemoteData::new(Data::Servers(servers.servers))),
            Err(e) => tx.send(RemoteData::new(Data::Error(e.to_string()))),
        };

        ctx.request_repaint();
    });
}
