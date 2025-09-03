use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustls/bogo/keys"]
pub struct OurVendorRustlsBogoKeysExtractor;
