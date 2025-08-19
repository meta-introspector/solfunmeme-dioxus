use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/nom/fuzz"]
pub struct OurVendorNomFuzzExtractor;
