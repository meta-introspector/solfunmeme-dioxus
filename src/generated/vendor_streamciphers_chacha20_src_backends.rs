use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/stream-ciphers/chacha20/src/backends"]
pub struct OurVendorStreamCiphersChacha20SrcBackendsExtractor;
