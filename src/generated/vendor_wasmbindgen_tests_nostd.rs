use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wasm-bindgen/tests/no-std"]
pub struct OurVendorWasmBindgenTestsNoStdExtractor;
