use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/winnow/fuzz"]
pub struct OurVendorWinnowFuzzExtractor;
