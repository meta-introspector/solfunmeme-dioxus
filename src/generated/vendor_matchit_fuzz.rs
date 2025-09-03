use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/matchit/fuzz"]
pub struct OurVendorMatchitFuzzExtractor;
