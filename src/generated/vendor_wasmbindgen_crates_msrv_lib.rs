use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wasm-bindgen/crates/msrv/lib"]
pub struct OurVendorWasmBindgenCratesMsrvLibExtractor;
