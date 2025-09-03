use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/json/fuzz"]
pub struct OurVendorJsonFuzzExtractor;
