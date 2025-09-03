use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/block-ciphers/aes/tests/data"]
pub struct OurVendorBlockCiphersAesTestsDataExtractor;
