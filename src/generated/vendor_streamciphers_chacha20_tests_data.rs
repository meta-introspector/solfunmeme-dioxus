use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/stream-ciphers/chacha20/tests/data"]
pub struct OurVendorStreamCiphersChacha20TestsDataExtractor;
