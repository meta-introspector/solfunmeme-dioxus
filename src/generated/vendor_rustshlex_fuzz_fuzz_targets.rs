use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rust-shlex/fuzz/fuzz_targets"]
pub struct OurVendorRustShlexFuzzFuzzTargetsExtractor;
