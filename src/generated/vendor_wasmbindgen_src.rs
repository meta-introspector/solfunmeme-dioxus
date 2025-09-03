use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wasm-bindgen/src"]
pub struct OurVendorWasmBindgenSrcExtractor;
