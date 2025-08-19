use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/num-bigint/benches"]
pub struct OurVendorNumBigintBenchesExtractor;
