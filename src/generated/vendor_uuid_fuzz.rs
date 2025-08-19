use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/uuid/fuzz"]
pub struct OurVendorUuidFuzzExtractor;
