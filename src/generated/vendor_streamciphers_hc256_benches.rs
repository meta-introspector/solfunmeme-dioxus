use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/stream-ciphers/hc-256/benches"]
pub struct OurVendorStreamCiphersHc256BenchesExtractor;
