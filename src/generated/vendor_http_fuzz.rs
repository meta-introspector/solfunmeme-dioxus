use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/http/fuzz"]
pub struct OurVendorHttpFuzzExtractor;
