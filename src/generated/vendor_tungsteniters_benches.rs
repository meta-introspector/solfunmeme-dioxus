use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tungstenite-rs/benches"]
pub struct OurVendorTungsteniteRsBenchesExtractor;
