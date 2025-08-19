use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wasm-bindgen/releases"]
pub struct OurVendorWasmBindgenReleasesExtractor;
