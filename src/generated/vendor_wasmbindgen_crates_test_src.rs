use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wasm-bindgen/crates/test/src"]
pub struct OurVendorWasmBindgenCratesTestSrcExtractor;
