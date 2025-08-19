use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/uriparse-rs/fuzz"]
pub struct OurVendorUriparseRsFuzzExtractor;
