use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rust-bitcoin/fuzz/fuzz_targets/hashes"]
pub struct OurVendorRustBitcoinFuzzFuzzTargetsHashesExtractor;
