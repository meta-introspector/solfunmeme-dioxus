use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/num-integer/benches"]
pub struct OurVendorNumIntegerBenchesExtractor;
