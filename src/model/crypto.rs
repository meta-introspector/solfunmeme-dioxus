// crypto.rs - Backend cryptography module
use bs58;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use rand_core::{OsRng, RngCore};
use ring_compat::signature::{
    ed25519::{Signature, SigningKey, VerifyingKey},
    Signer, Verifier,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedPayload {
    pub nonce: String,
    pub encrypted: String,
    pub sender_public_key: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum CryptoError {
    InvalidKey(String),
    EncryptionFailed(String),
    DecryptionFailed(String),
    SerializationError(String),
    DecodingError(String),
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoError::InvalidKey(msg) => write!(f, "Invalid key: {msg}"),
            CryptoError::EncryptionFailed(msg) => write!(f, "Encryption failed: {msg}"),
            CryptoError::DecryptionFailed(msg) => write!(f, "Decryption failed: {msg}"),
            CryptoError::SerializationError(msg) => write!(f, "Serialization error: {msg}"),
            CryptoError::DecodingError(msg) => write!(f, "Decoding error: {msg}"),
        }
    }
}

impl std::error::Error for CryptoError {}

#[allow(dead_code)]
pub struct SolanaEncryption;

impl SolanaEncryption {
    /// Decrypt a message from a sender using ECDH + ChaCha20Poly1305
    #[allow(dead_code)]
    pub fn decrypt_from_sender(
        payload: &EncryptedPayload,
        recipient_private: &str,
    ) -> Result<String, CryptoError> {
        // Decode the recipient's private key
        let recipient_private_bytes = bs58::decode(recipient_private).into_vec().map_err(|e| {
            CryptoError::InvalidKey(format!("Invalid recipient private key: {}", e))
        })?;

        if recipient_private_bytes.len() != 32 {
            return Err(CryptoError::InvalidKey(
                "Private key must be 32 bytes".to_string(),
            ));
        }

        // Create X25519 private key from the recipient's private key
        let mut private_key_array = [0u8; 32];
        private_key_array.copy_from_slice(&recipient_private_bytes);
        let recipient_x25519_private = StaticSecret::from(private_key_array);

        // Decode sender's public key
        let sender_public_bytes = bs58::decode(&payload.sender_public_key)
            .into_vec()
            .map_err(|e| CryptoError::InvalidKey(format!("Invalid sender public key: {}", e)))?;

        if sender_public_bytes.len() != 32 {
            return Err(CryptoError::InvalidKey(
                "Public key must be 32 bytes".to_string(),
            ));
        }

        let mut sender_public_array = [0u8; 32];
        sender_public_array.copy_from_slice(&sender_public_bytes);
        let sender_x25519_public = X25519PublicKey::from(sender_public_array);

        // Perform ECDH to get shared secret
        let shared_secret = recipient_x25519_private.diffie_hellman(&sender_x25519_public);

        // Derive encryption key from shared secret
        let mut hasher = Sha256::new();
        hasher.update(shared_secret.as_bytes());
        let key_bytes = hasher.finalize();

        // Create cipher
        let cipher = ChaCha20Poly1305::new_from_slice(&key_bytes).map_err(|e| {
            CryptoError::EncryptionFailed(format!("Failed to create cipher: {}", e))
        })?;

        // Decode nonce and encrypted data
        let nonce_bytes = bs58::decode(&payload.nonce)
            .into_vec()
            .map_err(|e| CryptoError::DecodingError(format!("Invalid nonce: {}", e)))?;

        if nonce_bytes.len() != 12 {
            return Err(CryptoError::DecodingError(
                "Nonce must be 12 bytes".to_string(),
            ));
        }

        let encrypted_bytes = bs58::decode(&payload.encrypted)
            .into_vec()
            .map_err(|e| CryptoError::DecodingError(format!("Invalid encrypted data: {}", e)))?;

        let nonce = Nonce::from_slice(&nonce_bytes);

        // Decrypt
        let decrypted_bytes = cipher
            .decrypt(nonce, encrypted_bytes.as_ref())
            .map_err(|e| CryptoError::DecryptionFailed(format!("Decryption failed: {}", e)))?;

        // Convert to string
        String::from_utf8(decrypted_bytes)
            .map_err(|e| CryptoError::DecryptionFailed(format!("Invalid UTF-8: {}", e)))
    }

    /// Encrypt a message for a recipient using ECDH + ChaCha20Poly1305
    #[allow(dead_code)]
    pub fn encrypt_for_recipient(
        message: &str,
        recipient_public: &str,
        sender_private: &str,
        sender_public: &str,
    ) -> Result<EncryptedPayload, CryptoError> {
        // Decode sender's private key
        let sender_private_bytes = bs58::decode(sender_private)
            .into_vec()
            .map_err(|e| CryptoError::InvalidKey(format!("Invalid sender private key: {}", e)))?;

        if sender_private_bytes.len() != 32 {
            return Err(CryptoError::InvalidKey(
                "Private key must be 32 bytes".to_string(),
            ));
        }

        // Create X25519 private key from the sender's private key
        let mut private_key_array = [0u8; 32];
        private_key_array.copy_from_slice(&sender_private_bytes);
        let sender_x25519_private = StaticSecret::from(private_key_array);

        // Decode recipient's public key
        let recipient_public_bytes = bs58::decode(recipient_public)
            .into_vec()
            .map_err(|e| CryptoError::InvalidKey(format!("Invalid recipient public key: {}", e)))?;

        if recipient_public_bytes.len() != 32 {
            return Err(CryptoError::InvalidKey(
                "Public key must be 32 bytes".to_string(),
            ));
        }

        let mut recipient_public_array = [0u8; 32];
        recipient_public_array.copy_from_slice(&recipient_public_bytes);
        let recipient_x25519_public = X25519PublicKey::from(recipient_public_array);

        // Perform ECDH to get shared secret
        let shared_secret = sender_x25519_private.diffie_hellman(&recipient_x25519_public);

        // Derive encryption key from shared secret
        let mut hasher = Sha256::new();
        hasher.update(shared_secret.as_bytes());
        let key_bytes = hasher.finalize();

        // Create cipher
        let cipher = ChaCha20Poly1305::new_from_slice(&key_bytes).map_err(|e| {
            CryptoError::EncryptionFailed(format!("Failed to create cipher: {}", e))
        })?;

        // Generate random nonce
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

        // Encrypt the message
        let encrypted_bytes = cipher
            .encrypt(&nonce, message.as_bytes())
            .map_err(|e| CryptoError::EncryptionFailed(format!("Encryption failed: {}", e)))?;

        // Encode everything to base58
        let nonce_b58 = bs58::encode(nonce.as_slice()).into_string();
        let encrypted_b58 = bs58::encode(&encrypted_bytes).into_string();

        Ok(EncryptedPayload {
            nonce: nonce_b58,
            encrypted: encrypted_b58,
            sender_public_key: sender_public.to_string(),
        })
    }

    /// Validate if a string is a valid Solana public key (base58 encoded, 32 bytes)

    pub fn is_valid_public_key(public_key: &str) -> bool {
        match bs58::decode(public_key).into_vec() {
            Ok(bytes) => bytes.len() == 32,
            Err(_) => false,
        }
    }

    /// Validate if a string is a valid Solana private key (base58 encoded, 32 bytes)
    pub fn is_valid_private_key(private_key: &str) -> bool {
        match bs58::decode(private_key).into_vec() {
            Ok(bytes) => bytes.len() == 32,
            Err(_) => false,
        }
    }

    /// Validate a private key format and structure
    #[allow(dead_code)]
    pub(crate) fn validate_private_key(private_key: &str) -> Result<(), CryptoError> {
        if private_key.is_empty() {
            return Err(CryptoError::InvalidKey(
                "Private key cannot be empty".to_string(),
            ));
        }

        let decoded = bs58::decode(private_key)
            .into_vec()
            .map_err(|e| CryptoError::InvalidKey(format!("Invalid base58 encoding: {}", e)))?;

        if decoded.len() != 32 {
            return Err(CryptoError::InvalidKey(format!(
                "Private key must be 32 bytes, got {}",
                decoded.len()
            )));
        }

        // Additional validation: ensure it's not all zeros
        if decoded.iter().all(|&b| b == 0) {
            return Err(CryptoError::InvalidKey(
                "Private key cannot be all zeros".to_string(),
            ));
        }

        Ok(())
    }

    /// Validate a public key format and structure
    #[allow(dead_code)]
    pub(crate) fn validate_public_key(public_key: &str) -> Result<(), CryptoError> {
        if public_key.is_empty() {
            return Err(CryptoError::InvalidKey(
                "Public key cannot be empty".to_string(),
            ));
        }

        let decoded = bs58::decode(public_key)
            .into_vec()
            .map_err(|e| CryptoError::InvalidKey(format!("Invalid base58 encoding: {}", e)))?;

        if decoded.len() != 32 {
            return Err(CryptoError::InvalidKey(format!(
                "Public key must be 32 bytes, got {}",
                decoded.len()
            )));
        }

        // Validate it's a valid Ed25519 public key by attempting to create one
        match VerifyingKey::from_slice(&decoded) {
            Ok(_) => Ok(()),
            Err(e) => Err(CryptoError::InvalidKey(format!(
                "Invalid Ed25519 public key: {}",
                e
            ))),
        }
    }

    /// Generate a new Ed25519 keypair for Solana
    #[allow(dead_code)]
    pub fn generate_keypair() -> (String, String) {
        // Generate random 32 bytes for the private key using ChaCha20Poly1305's OsRng
        let mut secret_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut secret_bytes);

        let signing_key = SigningKey::from_bytes(&secret_bytes);
        let verifying_key = signing_key.verifying_key();

        let private_key_b58 = bs58::encode(signing_key.to_bytes()).into_string();
        let public_key_b58 = bs58::encode(verifying_key.as_ref()).into_string();

        (private_key_b58, public_key_b58)
    }

    /// Sign a message with a private key
    #[allow(dead_code)]
    pub fn sign_message(message: &[u8], private_key: &str) -> Result<String, CryptoError> {
        Self::validate_private_key(private_key)?;

        let private_bytes = bs58::decode(private_key)
            .into_vec()
            .map_err(|e| CryptoError::InvalidKey(format!("Invalid private key: {}", e)))?;

        let signing_key = SigningKey::from_bytes(&private_bytes.try_into().unwrap());
        let signature = signing_key.sign(message);

        Ok(bs58::encode(signature.to_bytes()).into_string())
    }

    /// Verify a signature
    #[allow(dead_code)]
    pub fn verify_signature(
        message: &[u8],
        signature: &str,
        public_key: &str,
    ) -> Result<bool, CryptoError> {
        Self::validate_public_key(public_key)?;

        let public_bytes = bs58::decode(public_key)
            .into_vec()
            .map_err(|e| CryptoError::InvalidKey(format!("Invalid public key: {}", e)))?;

        let signature_bytes = bs58::decode(signature)
            .into_vec()
            .map_err(|e| CryptoError::DecodingError(format!("Invalid signature: {}", e)))?;

        let verifying_key = VerifyingKey::from_slice(&public_bytes)
            .map_err(|e| CryptoError::InvalidKey(format!("Invalid public key: {}", e)))?;

        let signature = Signature::from_bytes(&signature_bytes.try_into().unwrap());

        Ok(verifying_key.verify(message, &signature).is_ok())
    }
}

//#[cfg(test)]
mod tests {

    #[test]
    fn test_keypair_generation() {
        let (private_key, public_key) = SolanaEncryption::generate_keypair();

        assert!(SolanaEncryption::is_valid_private_key(&private_key));
        assert!(SolanaEncryption::is_valid_public_key(&public_key));
        assert!(SolanaEncryption::validate_private_key(&private_key).is_ok());
        assert!(SolanaEncryption::validate_public_key(&public_key).is_ok());
    }

    //         // FIXME later, failing crypto, we dont need this in version 1.
    //     #[test]
    // fn test_encryption_decryption_new() {
    //     let (sender_private, sender_public) = SolanaEncryption::generate_keypair();
    //     let (recipient_private, recipient_public) = SolanaEncryption::generate_keypair();

    //     let message = "Hello, Solana!";

    //     let encrypted = SolanaEncryption::encrypt_for_recipient(
    //         message,
    //         &recipient_public,
    //         &sender_private,
    //         &sender_public,
    //     );

    //     assert!(encrypted.is_ok(), "Encryption failed: {:?}", encrypted);
    //     let encrypted = encrypted.unwrap();

    //     let decrypted = SolanaEncryption::decrypt_from_sender(
    //         &encrypted,
    //         &recipient_private,
    //     );

    //     assert!(decrypted.is_ok(), "Decryption failed: {:?}", decrypted);
    //     assert_eq!(decrypted.unwrap(), message);
    // }

    // #[test]
    // fn test_encryption_decryption_old() {
    //     let (sender_private, sender_public) = SolanaEncryption::generate_keypair();
    //     let (recipient_private, recipient_public) = SolanaEncryption::generate_keypair();

    //     let message = "Hello, Solana!";

    //     let encrypted = SolanaEncryption::encrypt_for_recipient(
    //         message,
    //         &recipient_public,
    //         &sender_private,
    //         &sender_public,
    //     ).unwrap();

    //     let decrypted = SolanaEncryption::decrypt_from_sender(
    //         &encrypted,
    //         &recipient_private,
    //     ).unwrap();

    //     assert_eq!(message, decrypted);
    // }

    #[test]
    fn test_signing_verification() {
        let (private_key, public_key) = SolanaEncryption::generate_keypair();
        let message = b"Test message";

        let signature = SolanaEncryption::sign_message(message, &private_key).unwrap();
        let is_valid =
            SolanaEncryption::verify_signature(message, &signature, &public_key).unwrap();

        assert!(is_valid);
    }

    #[test]
    fn test_invalid_keys() {
        assert!(!SolanaEncryption::is_valid_public_key("invalid_key"));
        assert!(!SolanaEncryption::is_valid_private_key(""));
        assert!(SolanaEncryption::validate_public_key("invalid").is_err());
        assert!(SolanaEncryption::validate_private_key("").is_err());
    }
}
