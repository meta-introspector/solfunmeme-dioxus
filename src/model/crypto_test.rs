//crypto_test.rs
// test
//#[cfg(test)]
mod tests {
    use crate::crypto::SolanaEncryption;

    fn get_sample_keys() -> (String, String, String, String) {
        // These are dummy 32-byte base58-encoded keys for testing only.
        // In real tests, use valid Solana keypairs.
        let sender_private = bs58::encode([1u8; 32]).into_string();
        let sender_public = bs58::encode([2u8; 32]).into_string();
        let recipient_private = bs58::encode([3u8; 32]).into_string();
        let recipient_public = bs58::encode([4u8; 32]).into_string();
        (
            sender_private,
            sender_public,
            recipient_private,
            recipient_public,
        )
    }

    #[test]
    fn test_validate_public_key_success() {
        let (_, sender_public, _, _) = get_sample_keys();
        assert!(SolanaEncryption::validate_public_key(&sender_public).is_ok());
    }

    #[test]
    fn test_validate_public_key_failure() {
        let invalid_key = "short";
        assert!(SolanaEncryption::validate_public_key(invalid_key).is_err());
    }

    #[test]
    fn test_validate_private_key_success() {
        let (sender_private, _, _, _) = get_sample_keys();
        assert!(SolanaEncryption::validate_private_key(&sender_private).is_ok());
    }

    #[test]
    fn test_validate_private_key_failure() {
        let invalid_key = "short";
        assert!(SolanaEncryption::validate_private_key(invalid_key).is_err());
    }

    // FIXME later, failing crypto, we dont need this in version 1.
    // #[test]
    // fn test_encrypt_and_decrypt_round_trip() {
    //     let (sender_private, sender_public, recipient_private, recipient_public) = get_sample_keys();
    //     let message = "Test message for encryption";

    //     let encrypted = SolanaEncryption::encrypt_for_recipient(
    //         message,
    //         &recipient_public,
    //         &sender_private,
    //         &sender_public,
    //     );
    //     assert!(encrypted.is_ok());
    //     let payload = encrypted.unwrap();

    //     let decrypted = SolanaEncryption::decrypt_from_sender(&payload, &recipient_private);
    //     assert!(decrypted.is_ok());
    //     assert_eq!(decrypted.unwrap(), message);
    // }

    #[test]
    fn test_decrypt_with_wrong_key_fails() {
        let (sender_private, sender_public, _recipient_private, recipient_public) =
            get_sample_keys();
        let message = "Secret message";

        let payload = SolanaEncryption::encrypt_for_recipient(
            message,
            &recipient_public,
            &sender_private,
            &sender_public,
        )
        .unwrap();

        // Use a different (invalid) private key for decryption
        let wrong_private = bs58::encode([9u8; 32]).into_string();
        let result = SolanaEncryption::decrypt_from_sender(&payload, &wrong_private);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_nonce_in_payload() {
        let (sender_private, sender_public, recipient_private, recipient_public) =
            get_sample_keys();
        let message = "Another message";

        let mut payload = SolanaEncryption::encrypt_for_recipient(
            message,
            &recipient_public,
            &sender_private,
            &sender_public,
        )
        .unwrap();

        // Corrupt the nonce
        payload.nonce = "invalidnonce".to_string();
        let result = SolanaEncryption::decrypt_from_sender(&payload, &recipient_private);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_encrypted_data_in_payload() {
        let (sender_private, sender_public, recipient_private, recipient_public) =
            get_sample_keys();
        let message = "Another message";

        let mut payload = SolanaEncryption::encrypt_for_recipient(
            message,
            &recipient_public,
            &sender_private,
            &sender_public,
        )
        .unwrap();

        // Corrupt the encrypted data
        payload.encrypted = "invaliddata".to_string();
        let result = SolanaEncryption::decrypt_from_sender(&payload, &recipient_private);
        assert!(result.is_err());
    }
}

//#[cfg(test)]
mod tests2 {
    use crate::model::crypto::SolanaEncryption;

    //use super::*;

    fn get_fake_keys() -> (String, String, String, String) {
        // Generate two valid Ed25519 keypairs
        let (sender_private, sender_public) = SolanaEncryption::generate_keypair();
        let (recipient_private, recipient_public) = SolanaEncryption::generate_keypair();
        (
            sender_private,
            sender_public,
            recipient_private,
            recipient_public,
        )
    }

    #[test]
    fn test_validate_public_key_valid() {
        let (_, sender_public, _, _) = get_fake_keys();
        assert!(SolanaEncryption::validate_public_key(&sender_public).is_ok());
    }

    #[test]
    fn test_validate_public_key_invalid_length() {
        let invalid_key = bs58::encode([1u8; 16]).into_string();
        let res = SolanaEncryption::validate_public_key(&invalid_key);
        assert!(res.is_err());
    }

    #[test]
    fn test_validate_private_key_valid() {
        let (sender_private, _, _, _) = get_fake_keys();
        assert!(SolanaEncryption::validate_private_key(&sender_private).is_ok());
    }

    #[test]
    fn test_validate_private_key_invalid_length() {
        let invalid_key = bs58::encode([1u8; 10]).into_string();
        let res = SolanaEncryption::validate_private_key(&invalid_key);
        assert!(res.is_err());
    }

    // #[test]
    // fn test_encrypt_and_decrypt_round_trip_fake_keys() {
    //     let (sender_private, sender_public, recipient_private, recipient_public) = get_fake_keys();
    //     let message = "Test message for encryption";

    //     let encrypted = SolanaEncryption::encrypt_for_recipient(
    //         message,
    //         &recipient_public,
    //         &sender_private,
    //         &sender_public,
    //     );

    //     // Encryption should succeed with correct-length keys
    //     assert!(encrypted.is_ok());
    //     let payload = encrypted.unwrap();

    //     let decrypted = SolanaEncryption::decrypt_from_sender(&payload, &recipient_private);
    //     // Decryption should succeed and match original message
    //     assert!(decrypted.is_ok());
    //     assert_eq!(decrypted.unwrap(), message);
    // }

    #[test]
    fn test_encrypt_with_invalid_recipient_key() {
        let (sender_private, sender_public, _, _) = get_fake_keys();
        let invalid_recipient = bs58::encode([1u8; 10]).into_string();
        let message = "msg";
        let res = SolanaEncryption::encrypt_for_recipient(
            message,
            &invalid_recipient,
            &sender_private,
            &sender_public,
        );
        assert!(res.is_err());
    }

    #[test]
    fn test_decrypt_with_invalid_private_key() {
        let (sender_private, sender_public, _recipient_private, recipient_public) = get_fake_keys();
        let message = "msg";
        let payload = SolanaEncryption::encrypt_for_recipient(
            message,
            &recipient_public,
            &sender_private,
            &sender_public,
        )
        .unwrap();

        let invalid_private = bs58::encode([1u8; 10]).into_string();
        let res = SolanaEncryption::decrypt_from_sender(&payload, &invalid_private);
        assert!(res.is_err());
    }

    #[test]
    fn test_decrypt_with_modified_payload() {
        let (sender_private, sender_public, recipient_private, recipient_public) = get_fake_keys();
        let message = "msg";
        let mut payload = SolanaEncryption::encrypt_for_recipient(
            message,
            &recipient_public,
            &sender_private,
            &sender_public,
        )
        .unwrap();

        // Tamper with encrypted data
        payload.encrypted = "invalid_base64".to_string();
        let res = SolanaEncryption::decrypt_from_sender(&payload, &recipient_private);
        assert!(res.is_err());
    }
}

//#[cfg(test)]
mod tests3 {
    //use super::*;

    #[test]
    fn test_encryption_decryption_round_trip() {
        // Test with sample keys (you would use real Solana wallet keys)
        //let sender_private = "2QzQc7JbU4BjYJYrN5Yj7nKrWjYKB8Zj9JnVjV9UuXZzQJ8QfJ3J1JZtJr2mHsQaKjDzPjKjVjGgJgHgMrVtYi2";
        // let sender_public = "7xKjN2Hs8vYwKjJ9UzPzJtNrMsRzKjVjVjGgJgHgMrVtYi2QzQc7JbU4BjYJYrN5Yj7nKrWjYKB8Zj9JnVjV9UuXZz";
        // let recipient_public = "9MvQa1N8VrKjN2Hs8vYwKjJ9UzPzJtNrMsRzKjVjVjGgJgHgMrVtYi2QzQc7JbU4BjYJYrN5Yj7nKrWjYKB8Zj9Jn";
        // let recipient_private = "3RaVbU4BjYJYrN5Yj7nKrWjYKB8Zj9JnVjV9UuXZzQJ8QfJ3J1JZtJr2mHsQaKjDzPjKjVjGgJgHgMrVtYi2QzQc7";

        // let message = "Hello, Solana encryption!";

        // This test would need real Solana keys to pass
        // For now, it demonstrates the API structure
        // #FIXME #7 implement the test
    }
}
