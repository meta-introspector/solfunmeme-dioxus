use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustls/.cargo"]
pub struct OurVendorRustlsCargoExtractor;
