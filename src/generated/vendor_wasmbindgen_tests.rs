use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wasm-bindgen/tests"]
pub struct OurVendorWasmBindgenTestsExtractor;
