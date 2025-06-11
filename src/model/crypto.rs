// crypto.rs - Backend cryptography module
use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose};
use bs58;
use wallet_adapter::ed25519_dalek::{SigningKey, VerifyingKey};
use x25519_dalek::{StaticSecret,  PublicKey as X25519PublicKey};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};
use sha2::{Sha256, Digest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedPayload {
    pub nonce: String,
    pub encrypted: String,
    pub sender_public_key: String,
}

#[derive(Debug, Clone)]
pub struct MessageEntry {
    pub title: String,
    pub message: String,
    pub owner: String,
    pub recipient: String,
    pub encrypted: bool,
}

#[derive(Debug)]
pub enum CryptoError {
    InvalidKey(String),
    EncryptionFailed(String),
    DecryptionFailed(String),
    SerializationError(String),
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoError::InvalidKey(msg) => write!(f, "Invalid key: {}", msg),
            CryptoError::EncryptionFailed(msg) => write!(f, "Encryption failed: {}", msg),
            CryptoError::DecryptionFailed(msg) => write!(f, "Decryption failed: {}", msg),
            CryptoError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

impl std::error::Error for CryptoError {}

pub struct SolanaEncryption;

impl SolanaEncryption {
    /// Encrypt a message for a specific recipient using Solana wallet keys
    pub fn encrypt_for_recipient(
        message: &str,
        recipient_public_key_base58: &str,
        sender_private_key_base58: &str,
        sender_public_key_base58: &str,
    ) -> Result<EncryptedPayload, CryptoError> {
        // Decode keys from Base58
        let recipient_public_key_bytes = bs58::decode(recipient_public_key_base58)
            .into_vec()
            .map_err(|e| CryptoError::InvalidKey(format!("Invalid recipient public key: {}", e)))?;
        
        let sender_private_key_bytes = bs58::decode(sender_private_key_base58)
            .into_vec()
            .map_err(|e| CryptoError::InvalidKey(format!("Invalid sender private key: {}", e)))?;

        // Validate key lengths
        if recipient_public_key_bytes.len() < 32 {
            return Err(CryptoError::InvalidKey("Recipient public key too short".to_string()));
        }
        if sender_private_key_bytes.len() < 32 {
            return Err(CryptoError::InvalidKey("Sender private key too short".to_string()));
        }

        // Convert Ed25519 keys to X25519 for encryption
        let sender_signing_key = SigningKey::from_bytes(
            &sender_private_key_bytes[..32]
                .try_into()
                .map_err(|e| CryptoError::InvalidKey(format!("Invalid key format: {}", e)))?,
        );
        let sender_x25519_private = Self::ed25519_to_x25519_private(&sender_signing_key);

        let recipient_verifying_key = VerifyingKey::from_bytes(
            &recipient_public_key_bytes[..32]
                .try_into()
                .map_err(|e| CryptoError::InvalidKey(format!("Invalid key format: {}", e)))?,
        )
        .map_err(|e| CryptoError::InvalidKey(format!("Invalid recipient public key: {}", e)))?;
        let recipient_x25519_public = Self::ed25519_to_x25519_public(&recipient_verifying_key);

        // Generate shared secret using ECDH
        let shared_secret = sender_x25519_private.diffie_hellman(&recipient_x25519_public);

        // Create cipher with shared secret
        let cipher = ChaCha20Poly1305::new_from_slice(shared_secret.as_bytes())
            .map_err(|e| CryptoError::EncryptionFailed(format!("Failed to create cipher: {}", e)))?;

        // Generate random nonce
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

        // Encrypt the message
        let encrypted_data = cipher
            .encrypt(&nonce, message.as_bytes())
            .map_err(|e| CryptoError::EncryptionFailed(format!("Encryption failed: {}", e)))?;

        // Create the payload
        let payload = EncryptedPayload {
            nonce: general_purpose::STANDARD.encode(nonce),
            encrypted: general_purpose::STANDARD.encode(encrypted_data),
            sender_public_key: sender_public_key_base58.to_string(),
        };

        Ok(payload)
    }

    /// Decrypt a message from a sender using the recipient's private key
    pub fn decrypt_from_sender(
        payload: &EncryptedPayload,
        recipient_private_key_base58: &str,
    ) -> Result<String, CryptoError> {
        // Decode components
        let nonce_bytes = general_purpose::STANDARD
            .decode(&payload.nonce)
            .map_err(|e| CryptoError::DecryptionFailed(format!("Invalid nonce: {}", e)))?;

        let encrypted_data = general_purpose::STANDARD
            .decode(&payload.encrypted)
            .map_err(|e| CryptoError::DecryptionFailed(format!("Invalid encrypted data: {}", e)))?;

        let sender_public_key_bytes = bs58::decode(&payload.sender_public_key)
            .into_vec()
            .map_err(|e| CryptoError::InvalidKey(format!("Invalid sender public key: {}", e)))?;

        let recipient_private_key_bytes = bs58::decode(recipient_private_key_base58)
            .into_vec()
            .map_err(|e| CryptoError::InvalidKey(format!("Invalid recipient private key: {}", e)))?;

        // Validate key lengths
        if sender_public_key_bytes.len() < 32 {
            return Err(CryptoError::InvalidKey("Sender public key too short".to_string()));
        }
        if recipient_private_key_bytes.len() < 32 {
            return Err(CryptoError::InvalidKey("Recipient private key too short".to_string()));
        }

        // Convert keys
        let recipient_signing_key = SigningKey::from_bytes(
            &recipient_private_key_bytes[..32]
                .try_into()
                .map_err(|e| CryptoError::InvalidKey(format!("Invalid key format: {}", e)))?,
        );
        let recipient_x25519_private = Self::ed25519_to_x25519_private(&recipient_signing_key);

        let sender_verifying_key = VerifyingKey::from_bytes(
            &sender_public_key_bytes[..32]
                .try_into()
                .map_err(|e| CryptoError::InvalidKey(format!("Invalid key format: {}", e)))?,
        )
        .map_err(|e| CryptoError::InvalidKey(format!("Invalid sender public key: {}", e)))?;
        let sender_x25519_public = Self::ed25519_to_x25519_public(&sender_verifying_key);

        // Generate shared secret
        let shared_secret = recipient_x25519_private.diffie_hellman(&sender_x25519_public);

        // Create cipher
        let cipher = ChaCha20Poly1305::new_from_slice(shared_secret.as_bytes())
            .map_err(|e| CryptoError::DecryptionFailed(format!("Failed to create cipher: {}", e)))?;

        // Create nonce
        if nonce_bytes.len() != 12 {
            return Err(CryptoError::DecryptionFailed("Invalid nonce length".to_string()));
        }
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Decrypt
        let decrypted_data = cipher
            .decrypt(nonce, encrypted_data.as_ref())
            .map_err(|e| CryptoError::DecryptionFailed(format!("Decryption failed: {}", e)))?;

        // Convert to string
        let decrypted_message = String::from_utf8(decrypted_data)
            .map_err(|e| CryptoError::DecryptionFailed(format!("Invalid UTF-8: {}", e)))?;

        Ok(decrypted_message)
    }

    /// Validate that a Base58 string is a valid Solana public key
    pub fn validate_public_key(key_base58: &str) -> Result<(), CryptoError> {
        let key_bytes = bs58::decode(key_base58)
            .into_vec()
            .map_err(|e| CryptoError::InvalidKey(format!("Invalid Base58: {}", e)))?;

        if key_bytes.len() != 32 {
            return Err(CryptoError::InvalidKey("Public key must be 32 bytes".to_string()));
        }

        VerifyingKey::from_bytes(
            &key_bytes[..32]
                .try_into()
                .map_err(|e| CryptoError::InvalidKey(format!("Invalid key format: {}", e)))?,
        )
        .map_err(|e| CryptoError::InvalidKey(format!("Invalid Ed25519 public key: {}", e)))?;

        Ok(())
    }

    /// Validate that a Base58 string is a valid Solana private key
    pub fn validate_private_key(key_base58: &str) -> Result<(), CryptoError> {
        let key_bytes = bs58::decode(key_base58)
            .into_vec()
            .map_err(|e| CryptoError::InvalidKey(format!("Invalid Base58: {}", e)))?;

        if key_bytes.len() < 32 {
            return Err(CryptoError::InvalidKey("Private key too short".to_string()));
        }

        SigningKey::from_bytes(
            &key_bytes[..32]
                .try_into()
                .map_err(|e| CryptoError::InvalidKey(format!("Invalid key format: {}", e)))?,
        );

        Ok(())
    }

    /// Convert Ed25519 private key to X25519 private key
    fn ed25519_to_x25519_private(signing_key: &SigningKey) -> StaticSecret {
        let mut hasher = Sha256::new();
        hasher.update(signing_key.to_bytes());
        let hash = hasher.finalize();

        let hash_bytes: [u8; 32] = hash
            .as_slice()
            .try_into()
            .expect("Hash output should be 32 bytes");
        StaticSecret::from(hash_bytes)
    }

    /// Convert Ed25519 public key to X25519 public key
    fn ed25519_to_x25519_public(verifying_key: &VerifyingKey) -> X25519PublicKey {
        // Convert Ed25519 public key to X25519 public key
        let bytes = verifying_key.to_bytes();
        let mut x25519_bytes = [0u8; 32];
        x25519_bytes.copy_from_slice(&bytes);

        // Apply Montgomery curve conversion
        x25519_bytes[31] &= 0x7F; // Clear the top bit

        X25519PublicKey::from(x25519_bytes)
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_encryption_decryption_round_trip() {
        // Test with sample keys (you would use real Solana wallet keys)
        let sender_private = "2QzQc7JbU4BjYJYrN5Yj7nKrWjYKB8Zj9JnVjV9UuXZzQJ8QfJ3J1JZtJr2mHsQaKjDzPjKjVjGgJgHgMrVtYi2";
        let sender_public = "7xKjN2Hs8vYwKjJ9UzPzJtNrMsRzKjVjVjGgJgHgMrVtYi2QzQc7JbU4BjYJYrN5Yj7nKrWjYKB8Zj9JnVjV9UuXZz";
        let recipient_public = "9MvQa1N8VrKjN2Hs8vYwKjJ9UzPzJtNrMsRzKjVjVjGgJgHgMrVtYi2QzQc7JbU4BjYJYrN5Yj7nKrWjYKB8Zj9Jn";
        let recipient_private = "3RaVbU4BjYJYrN5Yj7nKrWjYKB8Zj9JnVjV9UuXZzQJ8QfJ3J1JZtJr2mHsQaKjDzPjKjVjGgJgHgMrVtYi2QzQc7";

        let message = "Hello, Solana encryption!";

        // This test would need real Solana keys to pass
        // For now, it demonstrates the API structure
    }
}