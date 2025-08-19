use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tungstenite-rs/fuzz/seeds"]
pub struct OurVendorTungsteniteRsFuzzSeedsExtractor;
