use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wasm-streams/.cargo"]
pub struct OurVendorWasmStreamsCargoExtractor;
