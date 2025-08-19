use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/stream-ciphers/salsa20/src/backends"]
pub struct OurVendorStreamCiphersSalsa20SrcBackendsExtractor;
