use crate::crypto::SolanaEncryption;

#[allow(dead_code)]
pub fn validate_encryption_inputs(
    title: &str,
    message: &str,
    recipient_public_key: &str,
    my_private_key: &str,
    my_public_key: &str,
) -> Result<(), String> {
    if title.trim().is_empty() {
        return Err("Title is required".to_string());
    }
    if message.trim().is_empty() {
        return Err("Message is required".to_string());
    }
    if recipient_public_key.trim().is_empty() {
        return Err("Recipient's public key is required".to_string());
    }
    if my_private_key.trim().is_empty() {
        return Err("Your private key is required".to_string());
    }
    if my_public_key.trim().is_empty() {
        return Err("Your public key is required".to_string());
    }

    // Validate Solana key formats
    if !SolanaEncryption::is_valid_public_key(recipient_public_key) {
        return Err("Invalid recipient public key format".to_string());
    }
    if !SolanaEncryption::is_valid_public_key(my_public_key) {
        return Err("Invalid public key format".to_string());
    }
    if !SolanaEncryption::is_valid_private_key(my_private_key) {
        return Err("Invalid private key format".to_string());
    }

    Ok(())
}
#[allow(dead_code)]
pub fn validate_decryption_inputs(
    encrypted_payload: &str,
    my_private_key: &str,
) -> Result<(), String> {
    if encrypted_payload.trim().is_empty() {
        return Err("Encrypted payload is required".to_string());
    }
    if my_private_key.trim().is_empty() {
        return Err("Private key is required".to_string());
    }

    // Validate Solana key format
    if !SolanaEncryption::is_valid_private_key(my_private_key) {
        return Err("Invalid private key format".to_string());
    }

    Ok(())
}
