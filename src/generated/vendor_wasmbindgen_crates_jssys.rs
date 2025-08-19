use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wasm-bindgen/crates/js-sys"]
pub struct OurVendorWasmBindgenCratesJsSysExtractor;
