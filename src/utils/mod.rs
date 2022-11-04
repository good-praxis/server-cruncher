use chrono::{DateTime, Utc};
use hcloud::models::{server::Server, Image};

mod remote_data;
pub use remote_data::RemoteData;

#[derive(Debug, Clone)]
pub enum Data {
    Servers(Vec<Server>),
    Images(Vec<Image>),
    Error(String),
}

#[derive(Debug, Clone)]
pub struct Error {
    pub error: String,
    pub ts: DateTime<Utc>,
}
