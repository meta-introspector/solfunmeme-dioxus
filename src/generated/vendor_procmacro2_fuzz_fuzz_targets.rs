use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/proc-macro2/fuzz/fuzz_targets"]
pub struct OurVendorProcMacro2FuzzFuzzTargetsExtractor;
