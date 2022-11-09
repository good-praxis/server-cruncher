use chrono::Utc;

mod timestamp;
pub use timestamp::Timestamp;

mod application;
pub use application::{generate_application_list, Application};

mod secret;
pub use secret::{Key, Secret};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct RemoteData {
    pub data: Data,
    pub updated_at: Timestamp,
}

impl RemoteData {
    pub fn new(data: Data) -> Self {
        Self {
            data,
            updated_at: Timestamp::new(Utc::now()),
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum Data {
    Application(Vec<Application>),
    Error(String),
}

#[derive(Debug, Clone)]
pub struct Error {
    pub error: String,
    pub ts: Timestamp,
}

#[cfg(test)]
mod testing;
#[cfg(test)]
pub use testing::{empty_server, empty_snapshot};
