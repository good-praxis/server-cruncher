use std::sync::mpsc::{SendError, Sender};

use super::Endpoint;
use crate::utils::{generate_application_list, Data, Key, RemoteData};
use hcloud::apis::{
    configuration::Configuration, images_api::ListImagesError, servers_api::ListServersError, Error,
};
#[cfg(not(all(test, mock)))]
use hcloud::apis::{images_api, servers_api};
use hcloud::models::{ListImagesResponse, ListServersResponse};

#[cfg(all(test, mock))]
use test::{ImagesApiMock as images_api, ServersApiMock as servers_api};

pub struct Hetzner;
impl Endpoint for Hetzner {
    fn req_application_list(
        &self,
        app: &mut crate::app::App,
        origin: &'static str,
        ctx: &egui::Context,
    ) {
        let api_key: Key = app.hcloud_api_secret.clone().unwrap().into();
        let tx = app.tx.clone();
        let ctx = ctx.clone();

        tokio::spawn(Self::application_list_future(api_key, origin, tx, ctx));
    }
}
impl Hetzner {
    async fn application_list_future(
        api_key: Key,
        origin: &'static str,
        tx: Sender<RemoteData>,
        ctx: egui::Context,
    ) -> Result<(), SendError<RemoteData>> {
        let servers = Self::get_server_list(&api_key).await;
        let images = Self::get_image_list(&api_key).await;

        let res = match (servers, images) {
            (Err(e), _) => tx.send(RemoteData::new(Data::Error(e.to_string()), origin)),
            (_, Err(e)) => tx.send(RemoteData::new(Data::Error(e.to_string()), origin)),
            (Ok(servers), Ok(images)) => tx.send(RemoteData::new(
                Data::Application(generate_application_list(&servers, &images)),
                origin,
            )),
        };
        ctx.request_repaint();
        res
    }

    async fn get_server_list(
        api_key: &Key,
    ) -> Result<ListServersResponse, Error<ListServersError>> {
        let config: Configuration = {
            let mut configuration = Configuration::new();
            configuration.bearer_access_token = Some(api_key.0.clone());
            configuration
        };

        servers_api::list_servers(&config, Default::default()).await
    }

    async fn get_image_list(api_key: &Key) -> Result<ListImagesResponse, Error<ListImagesError>> {
        let config: Configuration = {
            let mut configuration = Configuration::new();
            configuration.bearer_access_token = Some(api_key.0.clone());
            configuration
        };

        images_api::list_images(&config, Default::default()).await
    }
}

#[cfg(test)]
mod test {
    use super::Hetzner;
    use crate::utils::Key;

    #[cfg(mock)]
    use hcloud::{
        apis::{
            configuration::Configuration,
            images_api::{ListImagesError, ListImagesParams},
            servers_api::{ListServersError, ListServersParams},
            Error,
        },
        models::{ListImagesResponse, ListServersResponse},
    };
    #[cfg(mock)]
    use http::StatusCode;

    #[cfg(mock)]
    pub struct ServersApiMock;
    #[cfg(mock)]
    impl ServersApiMock {
        pub async fn list_servers(
            config: &Configuration,
            _params: ListServersParams,
        ) -> Result<ListServersResponse, Error<ListServersError>> {
            let key = config.bearer_access_token.clone();
            if key.is_none() || key.unwrap().is_empty() {
                return Err(Error::ResponseError(hcloud::apis::ResponseContent {
                    status: StatusCode::from_u16(500).unwrap(),
                    content: "".to_string(),
                    entity: None,
                }));
            }
            Ok(ListServersResponse::default())
        }
    }
    #[cfg(mock)]
    pub struct ImagesApiMock;
    #[cfg(mock)]
    impl ImagesApiMock {
        pub async fn list_images(
            config: &Configuration,
            _params: ListImagesParams,
        ) -> Result<ListImagesResponse, Error<ListImagesError>> {
            let key = config.bearer_access_token.clone();
            if key.is_none() || key.unwrap().is_empty() {
                return Err(Error::ResponseError(hcloud::apis::ResponseContent {
                    status: StatusCode::from_u16(500).unwrap(),
                    content: "".to_string(),
                    entity: None,
                }));
            }
            Ok(ListImagesResponse::default())
        }
    }

    #[cfg_attr(not(mock), ignore = "mocking is disabled")]
    #[tokio::test]
    async fn list_servers_mock() {
        let valid: Key = Key("you".to_string());
        assert!(Hetzner::get_server_list(&valid).await.is_ok());

        let invalid: Key = Key(String::new());
        assert!(Hetzner::get_server_list(&invalid).await.is_err());
    }

    #[cfg_attr(not(mock), ignore = "mocking is disabled")]
    #[tokio::test]
    async fn list_images_mock() {
        let valid: Key = Key("you".to_string());
        assert!(Hetzner::get_image_list(&valid).await.is_ok());

        let invalid: Key = Key(String::new());
        assert!(Hetzner::get_image_list(&invalid).await.is_err());
    }

    #[cfg_attr(not(mock), ignore = "mocking is disabled")]
    #[tokio::test]
    async fn application_list_future() {
        let (tx, rx) = std::sync::mpsc::channel();
        let origin = "";
        let api_key = Key("secret".to_string());
        let ctx = egui::Context::default();
        let future = Hetzner::application_list_future(api_key, origin, tx, ctx).await;
        assert!(future.is_ok());
        assert!(rx.recv().is_ok());
    }
}
