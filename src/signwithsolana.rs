use std::error::Error;
use std::fmt;

// Required dependencies for Cargo.toml:
// [dependencies]
// solana-sdk = "1.16"
// ed25519-dalek = "2.0"
// chacha20poly1305 = "0.10"
// rand = "0.8"
// base64 = "0.21"
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"

use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
    message::Message,
};
use wallet_adapter::ed25519_dalek::{PublicKey, SecretKey};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce, Key,
};
use rand::RngCore;
use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum EncryptionError {
    InvalidKeyLength,
    EncryptionFailed,
    DecryptionFailed,
    InvalidSignature,
    SerializationError(String),
    Base64Error(String),
}

impl fmt::Display for EncryptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EncryptionError::InvalidKeyLength => write!(f, "Invalid key length"),
            EncryptionError::EncryptionFailed => write!(f, "Encryption failed"),
            EncryptionError::DecryptionFailed => write!(f, "Decryption failed"),
            EncryptionError::InvalidSignature => write!(f, "Invalid signature"),
            EncryptionError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            EncryptionError::Base64Error(msg) => write!(f, "Base64 error: {}", msg),
        }
    }
}

impl Error for EncryptionError {}

#[derive(Serialize, Deserialize, Debug)]
pub struct EncryptedData {
    pub encrypted_content: String,  // Base64 encoded
    pub nonce: String,              // Base64 encoded
    pub ephemeral_public_key: String, // Base64 encoded
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignedEncryptedData {
    pub encrypted_data: EncryptedData,
    pub signature: String,          // Base64 encoded
    pub signer_public_key: String,  // Base64 encoded
}

pub struct SolanaWalletEncryption;

impl SolanaWalletEncryption {
    /// Approach 1: Direct encryption using ECDH + ChaCha20Poly1305
    /// This creates a shared secret between sender and recipient
    pub fn encrypt_for_wallet(
        data: &[u8],
        recipient_pubkey: &Pubkey,
        sender_keypair: &Keypair,
    ) -> Result<EncryptedData, EncryptionError> {
        // Generate ephemeral keypair for this encryption
        let ephemeral_keypair = Keypair::new();
        
        // Convert Solana pubkey to ed25519 public key
        let recipient_public_bytes = recipient_pubkey.to_bytes();
        let recipient_ed25519 = PublicKey::from_bytes(&recipient_public_bytes)
            .map_err(|_| EncryptionError::InvalidKeyLength)?;
        
        // Convert sender's keypair to ed25519
        let sender_secret_bytes = sender_keypair.secret().to_bytes();
        let sender_secret = SecretKey::from_bytes(&sender_secret_bytes)
            .map_err(|_| EncryptionError::InvalidKeyLength)?;
        
        // Create shared secret using ECDH
        let shared_secret = sender_secret.to_scalar() * recipient_ed25519.to_montgomery();
        
        // Derive encryption key from shared secret
        let encryption_key = Key::from_slice(&shared_secret.to_bytes());
        let cipher = ChaCha20Poly1305::new(encryption_key);
        
        // Generate random nonce
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        
        // Encrypt the data
        let ciphertext = cipher.encrypt(&nonce, data)
            .map_err(|_| EncryptionError::EncryptionFailed)?;
        
        Ok(EncryptedData {
            encrypted_content: general_purpose::STANDARD.encode(&ciphertext),
            nonce: general_purpose::STANDARD.encode(&nonce),
            ephemeral_public_key: general_purpose::STANDARD.encode(ephemeral_keypair.pubkey().to_bytes()),
        })
    }
    
    /// Decrypt data encrypted for a specific wallet
    pub fn decrypt_for_wallet(
        encrypted_data: &EncryptedData,
        recipient_keypair: &Keypair,
        sender_pubkey: &Pubkey,
    ) -> Result<Vec<u8>, EncryptionError> {
        // Decode base64 data
        let ciphertext = general_purpose::STANDARD.decode(&encrypted_data.encrypted_content)
            .map_err(|e| EncryptionError::Base64Error(e.to_string()))?;
        
        let nonce_bytes = general_purpose::STANDARD.decode(&encrypted_data.nonce)
            .map_err(|e| EncryptionError::Base64Error(e.to_string()))?;
        
        // Convert to proper nonce
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        // Convert keys to ed25519 format
        let sender_public_bytes = sender_pubkey.to_bytes();
        let sender_ed25519 = PublicKey::from_bytes(&sender_public_bytes)
            .map_err(|_| EncryptionError::InvalidKeyLength)?;
        
        let recipient_secret_bytes = recipient_keypair.secret().to_bytes();
        let recipient_secret = SecretKey::from_bytes(&recipient_secret_bytes)
            .map_err(|_| EncryptionError::InvalidKeyLength)?;
        
        // Recreate shared secret
        let shared_secret = recipient_secret.to_scalar() * sender_ed25519.to_montgomery();
        
        // Derive decryption key
        let decryption_key = Key::from_slice(&shared_secret.to_bytes());
        let cipher = ChaCha20Poly1305::new(decryption_key);
        
        // Decrypt the data
        let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())
            .map_err(|_| EncryptionError::DecryptionFailed)?;
        
        Ok(plaintext)
    }
    
    /// Approach 2: Encrypt with signature verification (SIWS-like)
    /// This ensures the data comes from a verified source
    pub fn encrypt_and_sign(
        data: &[u8],
        recipient_pubkey: &Pubkey,
        sender_keypair: &Keypair,
    ) -> Result<SignedEncryptedData, EncryptionError> {
        // First encrypt the data
        let encrypted_data = Self::encrypt_for_wallet(data, recipient_pubkey, sender_keypair)?;
        
        // Create a message to sign (hash of the encrypted data)
        let message_to_sign = format!(
            "{}{}{}",
            encrypted_data.encrypted_content,
            encrypted_data.nonce,
            encrypted_data.ephemeral_public_key
        );
        
        // Sign the message
        let signature = sender_keypair.sign_message(message_to_sign.as_bytes());
        
        Ok(SignedEncryptedData {
            encrypted_data,
            signature: general_purpose::STANDARD.encode(signature.as_ref()),
            signer_public_key: general_purpose::STANDARD.encode(sender_keypair.pubkey().to_bytes()),
        })
    }
    
    /// Decrypt and verify signature
    pub fn decrypt_and_verify(
        signed_data: &SignedEncryptedData,
        recipient_keypair: &Keypair,
    ) -> Result<Vec<u8>, EncryptionError> {
        // Decode signer public key
        let signer_pubkey_bytes = general_purpose::STANDARD.decode(&signed_data.signer_public_key)
            .map_err(|e| EncryptionError::Base64Error(e.to_string()))?;
        
        let signer_pubkey = Pubkey::new_from_array(
            signer_pubkey_bytes.try_into()
                .map_err(|_| EncryptionError::InvalidKeyLength)?
        );
        
        // Recreate the message that was signed
        let message_to_verify = format!(
            "{}{}{}",
            signed_data.encrypted_data.encrypted_content,
            signed_data.encrypted_data.nonce,
            signed_data.encrypted_data.ephemeral_public_key
        );
        
        // Decode and verify signature
        let signature_bytes = general_purpose::STANDARD.decode(&signed_data.signature)
            .map_err(|e| EncryptionError::Base64Error(e.to_string()))?;
        
        let signature = Signature::new(
            signature_bytes.try_into()
                .map_err(|_| EncryptionError::InvalidKeyLength)?
        );
        
        // Verify signature
        if !signature.verify(&signer_pubkey.to_bytes(), message_to_verify.as_bytes()) {
            return Err(EncryptionError::InvalidSignature);
        }
        
        // If signature is valid, decrypt the data
        Self::decrypt_for_wallet(&signed_data.encrypted_data, recipient_keypair, &signer_pubkey)
    }
    
    /// Utility function to create a SIWS-like message for wallet authentication
    pub fn create_auth_message(
        wallet_pubkey: &Pubkey,
        domain: &str,
        statement: &str,
        nonce: &str,
    ) -> String {
        format!(
            "{} wants you to sign in with your Solana account:\n{}\n\n{}\n\nNonce: {}",
            domain,
            wallet_pubkey,
            statement,
            nonce
        )
    }
    
    /// Verify SIWS-like authentication
    pub fn verify_auth_signature(
        message: &str,
        signature: &Signature,
        wallet_pubkey: &Pubkey,
    ) -> bool {
        signature.verify(&wallet_pubkey.to_bytes(), message.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_encryption_decryption() {
        let sender_keypair = Keypair::new();
        let recipient_keypair = Keypair::new();
        let data = b"Hello, Solana encryption!";
        
        // Encrypt
        let encrypted = SolanaWalletEncryption::encrypt_for_wallet(
            data,
            &recipient_keypair.pubkey(),
            &sender_keypair,
        ).unwrap();
        
        // Decrypt
        let decrypted = SolanaWalletEncryption::decrypt_for_wallet(
            &encrypted,
            &recipient_keypair,
            &sender_keypair.pubkey(),
        ).unwrap();
        
        assert_eq!(data, decrypted.as_slice());
    }
    
    #[test]
    fn test_signed_encryption_decryption() {
        let sender_keypair = Keypair::new();
        let recipient_keypair = Keypair::new();
        let data = b"Hello, signed Solana encryption!";
        
        // Encrypt and sign
        let signed_encrypted = SolanaWalletEncryption::encrypt_and_sign(
            data,
            &recipient_keypair.pubkey(),
            &sender_keypair,
        ).unwrap();
        
        // Decrypt and verify
        let decrypted = SolanaWalletEncryption::decrypt_and_verify(
            &signed_encrypted,
            &recipient_keypair,
        ).unwrap();
        
        assert_eq!(data, decrypted.as_slice());
    }
    
    #[test]
    fn test_siws_auth_message() {
        let wallet_keypair = Keypair::new();
        let nonce = "12345";
        
        let message = SolanaWalletEncryption::create_auth_message(
            &wallet_keypair.pubkey(),
            "myapp.com",
            "Sign in to access your encrypted data",
            nonce,
        );
        
        let signature = wallet_keypair.sign_message(message.as_bytes());
        
        assert!(SolanaWalletEncryption::verify_auth_signature(
            &message,
            &signature,
            &wallet_keypair.pubkey(),
        ));
    }
}