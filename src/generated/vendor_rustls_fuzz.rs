use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustls/fuzz"]
pub struct OurVendorRustlsFuzzExtractor;
