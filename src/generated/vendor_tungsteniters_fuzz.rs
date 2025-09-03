use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tungstenite-rs/fuzz"]
pub struct OurVendorTungsteniteRsFuzzExtractor;
