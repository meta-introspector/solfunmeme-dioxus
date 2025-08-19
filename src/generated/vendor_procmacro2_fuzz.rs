use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/proc-macro2/fuzz"]
pub struct OurVendorProcMacro2FuzzExtractor;
