use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tinyvec/fuzz"]
pub struct OurVendorTinyvecFuzzExtractor;
