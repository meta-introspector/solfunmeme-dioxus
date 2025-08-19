use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/chrono/fuzz"]
pub struct OurVendorChronoFuzzExtractor;
