use crate::utils::Key;
use hcloud::apis::{
    configuration::Configuration, images_api, images_api::ListImagesError, servers_api,
    servers_api::ListServersError, Error,
};
use hcloud::models::{ListImagesResponse, ListServersResponse};

pub async fn get_server_list(
    api_key: &Key,
) -> Result<ListServersResponse, Error<ListServersError>> {
    let config: Configuration = {
        let mut configuration = Configuration::new();
        configuration.bearer_access_token = Some(api_key.0.clone());
        configuration
    };

    servers_api::list_servers(&config, Default::default()).await
}

pub async fn get_image_list(api_key: &Key) -> Result<ListImagesResponse, Error<ListImagesError>> {
    let config: Configuration = {
        let mut configuration = Configuration::new();
        configuration.bearer_access_token = Some(api_key.0.clone());
        configuration
    };

    images_api::list_images(&config, Default::default()).await
}
