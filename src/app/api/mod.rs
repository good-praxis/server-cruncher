use super::App;
use crate::utils::Error;
use egui::Context;
use serde::{Deserialize, Serialize};

mod hetzner;
pub use hetzner::Hetzner;

#[derive(Serialize, Deserialize)]
pub enum Endpoints {
    Unconfigured,
    Hcloud,
}

const NO_API_ENDPOINT: &str = "No API endpoint configured";

pub trait Endpoint {
    fn req_application_list(&self, app: &mut App, origin: &'static str, ctx: &Context);
}

#[derive(Debug, Clone)]
pub struct Unconfigured;
impl Endpoint for Unconfigured {
    fn req_application_list(&self, app: &mut App, origin: &'static str, _ctx: &Context) {
        app.error_log.push(Error::new(NO_API_ENDPOINT));
        app.unset_loading(origin);
    }
}

impl App {
    pub fn req_application_list(&mut self, origin: &'static str, ctx: &Context) {
        let endpoint = self.endpoint.clone();
        endpoint.req_application_list(self, origin, ctx);
    }
}

#[cfg(test)]
mod test {
    use super::NO_API_ENDPOINT;
    use crate::app::App;
    use egui::Context;

    #[test]
    fn req_application_list_unconfigured() {
        let mut app = App::default();
        let ctx = Context::default();
        let endpoint = app.endpoint.clone();
        endpoint.req_application_list(&mut app, "", &ctx);
        assert_eq!(app.error_log[0].error, NO_API_ENDPOINT)
    }
}
