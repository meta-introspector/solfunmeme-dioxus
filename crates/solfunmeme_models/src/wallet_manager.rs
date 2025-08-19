use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit, Nonce};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Digest, Sha256};
use rand::RngCore;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletCredentials {
    pub wallet_address: String,
    pub encrypted_private_key: String,
    pub nonce: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretStore {
    pub aws_credentials: Option<EncryptedSecret>,
    pub github_token: Option<EncryptedSecret>,
    pub huggingface_token: Option<EncryptedSecret>,
    pub openai_key: Option<EncryptedSecret>,
    pub grok_key: Option<EncryptedSecret>,
    pub groq_key: Option<EncryptedSecret>,
    pub google_key: Option<EncryptedSecret>,
    pub solana_keys: Vec<EncryptedSecret>,
    pub ssh_keys: Vec<EncryptedSecret>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedSecret {
    pub name: String,
    pub encrypted_value: String,
    pub nonce: String,
    pub created_at: u64,
}

pub struct WalletManager {
    master_key: Option<[u8; 32]>,
    pub secrets: SecretStore,
}

impl WalletManager {
    pub fn new() -> Self {
        Self {
            master_key: None,
            secrets: SecretStore {
                aws_credentials: None,
                github_token: None,
                huggingface_token: None,
                openai_key: None,
                grok_key: None,
                groq_key: None,
                google_key: None,
                solana_keys: Vec::new(),
                ssh_keys: Vec::new(),
            },
        }
    }

    pub fn initialize_with_password(
        &mut self,
        password: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let key = self.derive_key_from_password(password)?;
        self.master_key = Some(key);
        Ok(())
    }

    pub fn encrypt_secret(
        &self,
        name: &str,
        value: &str,
    ) -> Result<EncryptedSecret, Box<dyn std::error::Error>> {
        let key = self.master_key.ok_or("Master key not initialized")?;
        let cipher = Aes256Gcm::new(&key.into());

        let nonce_bytes = self.generate_nonce();
        let nonce = Nonce::from_slice(&nonce_bytes);

        let encrypted_value = cipher
            .encrypt(nonce, value.as_bytes())
            .map_err(|e| format!("Encryption failed: {}", e))?;

        Ok(EncryptedSecret {
            name: name.to_string(),
            encrypted_value: general_purpose::STANDARD.encode(encrypted_value),
            nonce: general_purpose::STANDARD.encode(nonce_bytes),
            created_at: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
        })
    }

    pub fn decrypt_secret(
        &self,
        secret: &EncryptedSecret,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let key = self.master_key.ok_or("Master key not initialized")?;
        let cipher = Aes256Gcm::new(&key.into());

        let nonce_bytes = general_purpose::STANDARD.decode(&secret.nonce)?;
        let nonce = Nonce::from_slice(&nonce_bytes);

        let encrypted_value = general_purpose::STANDARD.decode(&secret.encrypted_value)?;

        let decrypted = cipher
            .decrypt(nonce, encrypted_value.as_slice())
            .map_err(|e| format!("Decryption failed: {}", e))?;

        Ok(String::from_utf8(decrypted)?)
    }

    pub fn store_aws_credentials(
        &mut self,
        access_key: &str,
        secret_key: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let combined = format!("{}:{}", access_key, secret_key);
        let encrypted = self.encrypt_secret("aws_credentials", &combined)?;
        self.secrets.aws_credentials = Some(encrypted);
        Ok(())
    }

    pub fn store_github_token(&mut self, token: &str) -> Result<(), Box<dyn std::error::Error>> {
        let encrypted = self.encrypt_secret("github_token", token)?;
        self.secrets.github_token = Some(encrypted);
        Ok(())
    }

    pub fn store_ai_key(
        &mut self,
        provider: &str,
        key: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let encrypted = self.encrypt_secret(&format!("{}_key", provider), key)?;

        match provider {
            "openai" => self.secrets.openai_key = Some(encrypted),
            "grok" => self.secrets.grok_key = Some(encrypted),
            "groq" => self.secrets.groq_key = Some(encrypted),
            "google" => self.secrets.google_key = Some(encrypted),
            _ => return Err(format!("Unknown AI provider: {}", provider).into()),
        }

        Ok(())
    }

    pub fn store_solana_key(
        &mut self,
        name: &str,
        private_key: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let encrypted = self.encrypt_secret(name, private_key)?;
        self.secrets.solana_keys.push(encrypted);
        Ok(())
    }

    pub fn store_ssh_key(
        &mut self,
        name: &str,
        private_key: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let encrypted = self.encrypt_secret(name, private_key)?;
        self.secrets.ssh_keys.push(encrypted);
        Ok(())
    }

    pub fn get_aws_credentials(
        &self,
    ) -> Result<Option<(String, String)>, Box<dyn std::error::Error>> {
        if let Some(ref encrypted) = self.secrets.aws_credentials {
            let decrypted = self.decrypt_secret(encrypted)?;
            let parts: Vec<&str> = decrypted.split(':').collect();
            if parts.len() == 2 {
                Ok(Some((parts[0].to_string(), parts[1].to_string())))
            } else {
                Err("Invalid AWS credentials format".into())
            }
        } else {
            Ok(None)
        }
    }

    pub fn get_ai_key(&self, provider: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let encrypted = match provider {
            "openai" => &self.secrets.openai_key,
            "grok" => &self.secrets.grok_key,
            "groq" => &self.secrets.groq_key,
            "google" => &self.secrets.google_key,
            _ => return Err(format!("Unknown AI provider: {}", provider).into()),
        };

        if let Some(ref encrypted) = encrypted {
            Ok(Some(self.decrypt_secret(encrypted)?))
        } else {
            Ok(None)
        }
    }

    pub fn export_secrets(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_json::to_string_pretty(&self.secrets)?)
    }

    pub fn import_secrets(&mut self, json_data: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.secrets = serde_json::from_str(json_data)?;
        Ok(())
    }

    fn derive_key_from_password(
        &self,
        password: &str,
    ) -> Result<[u8; 32], Box<dyn std::error::Error>> {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        hasher.update(b"solfunmeme_salt"); // Add salt
        let result = hasher.finalize();
        let mut key = [0u8; 32];
        key.copy_from_slice(&result);
        Ok(key)
    }

    fn generate_nonce(&self) -> [u8; 12] {
        let mut nonce = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce);
        nonce
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_manager_initialization() {
        let mut manager = WalletManager::new();
        assert!(manager.initialize_with_password("test_password").is_ok());
    }

    #[test]
    fn test_secret_encryption_decryption() {
        let mut manager = WalletManager::new();
        manager.initialize_with_password("test_password").unwrap();

        let secret_value = "my_secret_key";
        let encrypted = manager.encrypt_secret("test_secret", secret_value).unwrap();
        let decrypted = manager.decrypt_secret(&encrypted).unwrap();

        assert_eq!(decrypted, secret_value);
    }

    #[test]
    fn test_aws_credentials_storage() {
        let mut manager = WalletManager::new();
        manager.initialize_with_password("test_password").unwrap();

        let access_key = "AKIAIOSFODNN7EXAMPLE";
        let secret_key = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY";

        manager
            .store_aws_credentials(access_key, secret_key)
            .unwrap();
        let retrieved = manager.get_aws_credentials().unwrap();

        assert!(retrieved.is_some());
        let (retrieved_access, retrieved_secret) = retrieved.unwrap();
        assert_eq!(retrieved_access, access_key);
        assert_eq!(retrieved_secret, secret_key);
    }

    #[test]
    fn test_ai_key_storage() {
        let mut manager = WalletManager::new();
        manager.initialize_with_password("test_password").unwrap();

        let openai_key = "sk-1234567890abcdef";
        manager.store_ai_key("openai", openai_key).unwrap();

        let retrieved = manager.get_ai_key("openai").unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap(), openai_key);
    }

    #[test]
    fn test_solana_key_storage() {
        let mut manager = WalletManager::new();
        manager.initialize_with_password("test_password").unwrap();

        let private_key = "5J1F7GHaDxuOmfdiPnwxs1hVrDRVKvJ2ox4B5VkT8tNQ";
        manager
            .store_solana_key("main_wallet", private_key)
            .unwrap();

        assert_eq!(manager.secrets.solana_keys.len(), 1);
        assert_eq!(manager.secrets.solana_keys[0].name, "main_wallet");
    }

    #[test]
    fn test_export_import_secrets() {
        let mut manager1 = WalletManager::new();
        manager1.initialize_with_password("test_password").unwrap();

        manager1.store_github_token("ghp_1234567890").unwrap();
        let exported = manager1.export_secrets().unwrap();

        let mut manager2 = WalletManager::new();
        manager2.initialize_with_password("test_password").unwrap();
        manager2.import_secrets(&exported).unwrap();

        assert!(manager2.secrets.github_token.is_some());
    }

    #[test]
    fn test_invalid_password_fails_decryption() {
        let mut manager1 = WalletManager::new();
        manager1
            .initialize_with_password("correct_password")
            .unwrap();
        let encrypted = manager1.encrypt_secret("test", "secret_value").unwrap();

        let mut manager2 = WalletManager::new();
        manager2.initialize_with_password("wrong_password").unwrap();

        assert!(manager2.decrypt_secret(&encrypted).is_err());
    }
}
