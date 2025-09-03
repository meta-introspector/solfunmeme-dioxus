use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ring/src/polyfill/slice"]
pub struct OurVendorRingSrcPolyfillSliceExtractor;
