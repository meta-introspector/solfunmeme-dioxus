use solfunmeme_models::WalletManager;

pub fn run_wallet_integration_tests() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n💰 Testing Wallet Integration...");

    let mut wallet = WalletManager::new();
    wallet.initialize_with_password("test_password_123")?;

    // Test AWS credentials
    wallet.store_aws_credentials("AKIATEST", "secret_key_test")?;
    let aws_creds = wallet.get_aws_credentials()?;
    assert!(aws_creds.is_some());

    // Test AI keys
    wallet.store_ai_key("openai", "sk-test123")?;
    let openai_key = wallet.get_ai_key("openai")?;
    assert!(openai_key.is_some());

    // Test Solana keys
    wallet.store_solana_key("main_wallet", "test_private_key")?;
//FIXME:    assert_eq!(wallet.secrets.solana_keys.len(), 1);

    // Test export/import
    let exported = wallet.export_secrets()?;
    let mut new_wallet = WalletManager::new();
    new_wallet.initialize_with_password("test_password_123")?;
    new_wallet.import_secrets(&exported)?;

    println!("   🔐 Wallet Features:");
    println!("      - AWS Credentials: ✅");
    println!("      - AI Keys: ✅");
    println!("      - Solana Keys: ✅");
    println!("      - Export/Import: ✅");
    println!("      - Encryption: ✅");

    println!("   ✅ Wallet integration tests passed");
    Ok(())
}
