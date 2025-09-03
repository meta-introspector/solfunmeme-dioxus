use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/darling/tests/compile-fail"]
pub struct OurVendorDarlingTestsCompileFailExtractor;
