use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wasm-bindgen/.cargo"]
pub struct OurVendorWasmBindgenCargoExtractor;
