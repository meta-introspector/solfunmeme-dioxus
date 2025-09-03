use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ryu/benches"]
pub struct OurVendorRyuBenchesExtractor;
