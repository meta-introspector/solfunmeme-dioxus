use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ryu/fuzz"]
pub struct OurVendorRyuFuzzExtractor;
