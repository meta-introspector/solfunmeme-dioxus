use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wasm-bindgen/tests/crates"]
pub struct OurVendorWasmBindgenTestsCratesExtractor;
