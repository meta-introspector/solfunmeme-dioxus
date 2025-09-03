use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tokio/tokio-stream/fuzz/fuzz_targets"]
pub struct OurVendorTokioTokioStreamFuzzFuzzTargetsExtractor;
