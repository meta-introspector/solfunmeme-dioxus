use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rust-base64/fuzz"]
pub struct OurVendorRustBase64FuzzExtractor;
