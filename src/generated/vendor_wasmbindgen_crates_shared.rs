use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wasm-bindgen/crates/shared"]
pub struct OurVendorWasmBindgenCratesSharedExtractor;
