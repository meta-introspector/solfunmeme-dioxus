use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustls/ci-bench"]
pub struct OurVendorRustlsCiBenchExtractor;
