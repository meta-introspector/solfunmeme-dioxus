use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wasm-bindgen/src/cache"]
pub struct OurVendorWasmBindgenSrcCacheExtractor;
