use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/borsh-rs/fuzz/fuzz-run"]
pub struct OurVendorBorshRsFuzzFuzzRunExtractor;
