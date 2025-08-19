use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wasm-bindgen/crates/test"]
pub struct OurVendorWasmBindgenCratesTestExtractor;
