use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/atomic-waker/benches"]
pub struct OurVendorAtomicWakerBenchesExtractor;
