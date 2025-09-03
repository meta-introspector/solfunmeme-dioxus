use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/proc-macro2/benches"]
pub struct OurVendorProcMacro2BenchesExtractor;
