use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/stream-ciphers/chacha20/benches"]
pub struct OurVendorStreamCiphersChacha20BenchesExtractor;
