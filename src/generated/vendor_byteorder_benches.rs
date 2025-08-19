use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/byteorder/benches"]
pub struct OurVendorByteorderBenchesExtractor;
