use chrono::{DateTime, Utc};
use hcloud::models::{Image, Server};

mod remote_data;
pub use remote_data::RemoteData;

mod application;
pub use application::{generate_application_list, Application};

#[derive(Debug, Clone)]
pub enum Data {
    Application(Vec<Application>),
    Error(String),
}

#[derive(Debug, Clone)]
pub struct Error {
    pub error: String,
    pub ts: DateTime<Utc>,
}
