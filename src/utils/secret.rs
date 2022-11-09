use serde_encrypt::{
    serialize::impls::BincodeSerializer, shared_key::SharedKey, traits::SerdeEncryptSharedKey,
    EncryptedMessage, Error,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Key(pub String);
impl SerdeEncryptSharedKey for Key {
    type S = BincodeSerializer<Self>;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Secret {
    Unencrypted(Key),
    Encrypted(Vec<u8>),
}

impl Secret {
    pub fn encrypt(self, key: &SharedKey) -> Self {
        match self {
            Self::Unencrypted(cleartext) => {
                //FIXME: Unhandeled panic
                Self::Encrypted(encrypt_cleartext(cleartext, key).unwrap())
            }
            _ => self,
        }
    }

    pub fn decrypt(self, key: &SharedKey) -> Self {
        match self {
            //FIXME: Unhandeled panic
            Self::Encrypted(secret) => Self::Unencrypted(decrypt_secret(secret, key).unwrap()),
            _ => self,
        }
    }
}

fn encrypt_cleartext(cleartext: Key, key: &SharedKey) -> Result<Vec<u8>, Error> {
    let encrypted = cleartext.encrypt(&key);
    match encrypted {
        Err(e) => Err(e),
        Ok(encrypted) => Ok(encrypted.serialize()),
    }
}

fn decrypt_secret(secret: Vec<u8>, key: &SharedKey) -> Result<Key, Error> {
    let encrypted_key = EncryptedMessage::deserialize(secret)?;
    Key::decrypt_owned(&encrypted_key, &key)
}

#[cfg(test)]
mod test {
    use super::{Key, Secret};
    use serde_encrypt::shared_key::SharedKey;

    const KEY: SharedKey = SharedKey::new_const([0u8; 32]);

    #[test]
    fn encrypt() {
        let cleartext = Secret::Unencrypted(Key("Hello World".to_string()));
        let encrypted = cleartext.encrypt(&KEY);

        assert!(matches!(encrypted, Secret::Encrypted(_)));
    }

    #[test]
    fn decrypt() {
        let string = "Hello World".to_string();
        let seed = Secret::Unencrypted(Key(string.clone()));
        let encrypted = seed.encrypt(&KEY);
        let cleartext = encrypted.decrypt(&KEY);

        assert!(matches!(cleartext, Secret::Unencrypted(_)));

        if let Secret::Unencrypted(key) = cleartext {
            assert!(key.0 == string);
        }
    }
}
