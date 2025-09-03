use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/stream-ciphers/salsa20/benches"]
pub struct OurVendorStreamCiphersSalsa20BenchesExtractor;
