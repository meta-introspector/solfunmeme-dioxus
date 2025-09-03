use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/winnow/benches"]
pub struct OurVendorWinnowBenchesExtractor;
