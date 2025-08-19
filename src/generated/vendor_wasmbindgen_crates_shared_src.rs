use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wasm-bindgen/crates/shared/src"]
pub struct OurVendorWasmBindgenCratesSharedSrcExtractor;
