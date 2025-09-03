use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/reqwest/src/wasm"]
pub struct OurVendorReqwestSrcWasmExtractor;
