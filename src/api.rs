/// Collection of structs and helpers for interaction with the HCLOUD API
use crate::utils::Data;
use hcloud::apis::configuration::Configuration;
use hcloud::apis::servers_api;
use lazy_static::lazy_static;
use std::env;
use std::sync::mpsc::Sender;

lazy_static! {
    static ref HCLOUD_API_TOKEN: String =
        env::var("HCLOUD_API_TOKEN").expect("No HCLOUD_API_TOKEN found");
    static ref CONFIGURATION: Configuration = {
        let mut configuration = Configuration::new();
        configuration.bearer_access_token = Some(HCLOUD_API_TOKEN.clone());
        configuration
    };
}

pub fn req_server_list(tx: Sender<Data>, ctx: egui::Context) {
    tokio::spawn(async move {
        let servers = servers_api::list_servers(&CONFIGURATION, Default::default())
            .await
            .expect("Unable to fetch Server list") // TODO: Propogade error to UI
            .servers;

        let _ = tx.send(Data::Servers(servers));
        ctx.request_repaint();
    });
}
