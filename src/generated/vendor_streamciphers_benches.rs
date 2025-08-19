use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/stream-ciphers/benches"]
pub struct OurVendorStreamCiphersBenchesExtractor;
