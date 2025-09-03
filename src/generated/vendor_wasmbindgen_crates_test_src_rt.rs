use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wasm-bindgen/crates/test/src/rt"]
pub struct OurVendorWasmBindgenCratesTestSrcRtExtractor;
