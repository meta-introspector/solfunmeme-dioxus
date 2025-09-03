use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/chrono/bench/src"]
pub struct OurVendorChronoBenchSrcExtractor;
