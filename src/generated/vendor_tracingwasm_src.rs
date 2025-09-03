use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tracing-wasm/src"]
pub struct OurVendorTracingWasmSrcExtractor;
