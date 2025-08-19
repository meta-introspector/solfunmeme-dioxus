use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wasm-bindgen/src/rt"]
pub struct OurVendorWasmBindgenSrcRtExtractor;
