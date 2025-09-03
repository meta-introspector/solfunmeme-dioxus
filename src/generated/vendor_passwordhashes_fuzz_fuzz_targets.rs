use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/password-hashes/fuzz/fuzz_targets"]
pub struct OurVendorPasswordHashesFuzzFuzzTargetsExtractor;
