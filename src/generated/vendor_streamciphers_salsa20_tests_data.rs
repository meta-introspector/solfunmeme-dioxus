use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/stream-ciphers/salsa20/tests/data"]
pub struct OurVendorStreamCiphersSalsa20TestsDataExtractor;
