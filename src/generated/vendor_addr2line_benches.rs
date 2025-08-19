use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/addr2line/benches"]
pub struct OurVendorAddr2lineBenchesExtractor;
