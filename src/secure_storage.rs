use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    convert::TryFrom,
    fs,
    io::{self},
    path::PathBuf,
};
use winreg::enums::*;
use winreg::RegKey;
use crate::constants::*;

#[derive(Serialize, Deserialize)]
struct EncryptedData {
    salt: String,
    nonce: String,
    payload: String,
}

pub struct SecureStorage {
    key_file: PathBuf,
}

impl SecureStorage {
    pub fn new() -> io::Result<Self> {
        let app_dir = dirs::data_local_dir()
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::NotFound, CANNOT_FIND_APP_DATA_DIR)
            })?
            .join(APP_NAME);

        fs::create_dir_all(&app_dir)?;
        let key_file = app_dir.join(STORAGE_FILE_NAME);
        Ok(SecureStorage { key_file })
    }

    pub fn store_api_key(&self, api_key: &str) -> io::Result<()> {
        let salt = rand::random::<[u8; SALT_SIZE_CONSTANT]>();
        let nonce_bytes = rand::random::<[u8; NONCE_SIZE_CONSTANT]>();

        let encryption_key = self.derive_key(&salt)?;
        let cipher = Aes256Gcm::new(&encryption_key);
        let nonce = Nonce::try_from(&nonce_bytes[..])
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        let encrypted_data = cipher
            .encrypt(&nonce, api_key.as_bytes())
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        let data = EncryptedData {
            salt: BASE64.encode(salt),
            nonce: BASE64.encode(nonce_bytes),
            payload: BASE64.encode(encrypted_data),
        };

        let json = serde_json::to_string(&data)?;
        fs::write(&self.key_file, json)?;

        Ok(())
    }

    pub fn get_api_key(&self) -> io::Result<String> {
        let content = fs::read_to_string(&self.key_file)?;
        let data: EncryptedData = serde_json::from_str(&content)?;

        let salt = BASE64
            .decode(data.salt)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
        let nonce_bytes = BASE64
            .decode(data.nonce)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
        let encrypted_data = BASE64
            .decode(data.payload)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

        let encryption_key = self.derive_key(&salt)?;
        let cipher = Aes256Gcm::new(&encryption_key);
        let nonce = Nonce::try_from(&nonce_bytes[..])
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        let decrypted_data = cipher
            .decrypt(&nonce, encrypted_data.as_ref())
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        String::from_utf8(decrypted_data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
    }

    pub fn api_key_exists(&self) -> bool {
        self.key_file.exists()
    }

    fn get_machine_guid() -> io::Result<String> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let crypto_key = hklm.open_subkey(REGISTRY_PATH)?;
        crypto_key.get_value(REGISTRY_KEY)
    }

    fn derive_key(&self, salt: &[u8]) -> io::Result<Key<Aes256Gcm>> {
        let machine_id = Self::get_machine_guid()?;

        let mut hasher = Sha256::new();
        hasher.update(machine_id.as_bytes());
        hasher.update(salt);

        Key::<Aes256Gcm>::try_from(&hasher.finalize()[..])
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() -> io::Result<()> {
        let storage = SecureStorage::new()?;
        let api_key = "test-api-key-123";

        storage.store_api_key(api_key)?;
        let retrieved_key = storage.get_api_key()?;

        assert_eq!(retrieved_key, api_key);
        Ok(())
    }
}
