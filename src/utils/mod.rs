use chrono::Utc;

mod timestamp;
use serde_encrypt::{serialize::impls::BincodeSerializer, traits::SerdeEncryptSharedKey};
pub use timestamp::Timestamp;

mod application;
pub use application::{generate_application_list, Application};

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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Secret {
    Unencrypted(Key),
    Encrypted(Vec<u8>),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Key(pub String);
impl SerdeEncryptSharedKey for Key {
    type S = BincodeSerializer<Self>;
}

#[cfg(test)]
mod testing;
#[cfg(test)]
pub use testing::{empty_server, empty_snapshot};
