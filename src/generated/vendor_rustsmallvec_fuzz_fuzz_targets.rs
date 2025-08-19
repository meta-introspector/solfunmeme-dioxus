use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rust-smallvec/fuzz/fuzz_targets"]
pub struct OurVendorRustSmallvecFuzzFuzzTargetsExtractor;
