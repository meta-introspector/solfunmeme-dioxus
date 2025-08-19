use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tokio/tokio/fuzz/fuzz_targets"]
pub struct OurVendorTokioTokioFuzzFuzzTargetsExtractor;
