use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/crossbeam/crossbeam-channel/benches"]
pub struct OurVendorCrossbeamCrossbeamChannelBenchesExtractor;
