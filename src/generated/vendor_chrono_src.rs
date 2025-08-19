use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/chrono/src"]
pub struct OurVendorChronoSrcExtractor;
