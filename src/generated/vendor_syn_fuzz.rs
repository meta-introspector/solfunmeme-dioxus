use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/syn/fuzz"]
pub struct OurVendorSynFuzzExtractor;
