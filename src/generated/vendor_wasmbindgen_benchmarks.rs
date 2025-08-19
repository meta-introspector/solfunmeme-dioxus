use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wasm-bindgen/benchmarks"]
pub struct OurVendorWasmBindgenBenchmarksExtractor;
