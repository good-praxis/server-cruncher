use chrono::Utc;

mod timestamp;
use serde::{Deserialize, Serialize};
pub use timestamp::Timestamp;

mod application;
pub use application::{generate_application_list, Application};

mod secret;
pub use secret::{Key, Secret};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RemoteData {
    pub data: Data,
    pub updated_at: Timestamp,
    pub origin: String,
}

impl RemoteData {
    pub fn new(data: Data, origin: &str) -> Self {
        Self {
            data,
            updated_at: Timestamp::new(Utc::now()),
            origin: origin.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Data {
    Application(Vec<Application>),
    Error(String),
}

#[derive(Debug, Clone)]
pub struct Error {
    pub error: String,
    pub ts: Timestamp,
}
impl Error {
    pub fn new(e: &str) -> Self {
        Self {
            error: e.to_string(),
            ts: Timestamp::now(),
        }
    }
}

#[cfg(test)]
mod testing;
#[cfg(test)]
pub use testing::{empty_server, empty_snapshot};
