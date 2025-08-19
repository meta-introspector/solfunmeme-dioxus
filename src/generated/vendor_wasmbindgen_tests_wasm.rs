use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wasm-bindgen/tests/wasm"]
pub struct OurVendorWasmBindgenTestsWasmExtractor;
