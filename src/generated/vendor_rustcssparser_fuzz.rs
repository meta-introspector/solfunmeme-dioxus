use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rust-cssparser/fuzz"]
pub struct OurVendorRustCssparserFuzzExtractor;
