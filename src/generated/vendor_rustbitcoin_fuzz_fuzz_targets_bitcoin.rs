use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rust-bitcoin/fuzz/fuzz_targets/bitcoin"]
pub struct OurVendorRustBitcoinFuzzFuzzTargetsBitcoinExtractor;
