use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tungstenite-rs/fuzz/fuzz_targets"]
pub struct OurVendorTungsteniteRsFuzzFuzzTargetsExtractor;
