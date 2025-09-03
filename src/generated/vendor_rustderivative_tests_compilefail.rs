use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rust-derivative/tests/compile-fail"]
pub struct OurVendorRustDerivativeTestsCompileFailExtractor;
