use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rust-base64/.circleci"]
pub struct OurVendorRustBase64CircleciExtractor;
