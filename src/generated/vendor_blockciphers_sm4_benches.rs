use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/block-ciphers/sm4/benches"]
pub struct OurVendorBlockCiphersSm4BenchesExtractor;
