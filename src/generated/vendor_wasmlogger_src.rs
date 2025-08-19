use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wasm-logger/src"]
pub struct OurVendorWasmLoggerSrcExtractor;
