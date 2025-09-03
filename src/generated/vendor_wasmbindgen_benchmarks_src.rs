use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wasm-bindgen/benchmarks/src"]
pub struct OurVendorWasmBindgenBenchmarksSrcExtractor;
