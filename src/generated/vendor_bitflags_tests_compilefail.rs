use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/bitflags/tests/compile-fail"]
pub struct OurVendorBitflagsTestsCompileFailExtractor;
