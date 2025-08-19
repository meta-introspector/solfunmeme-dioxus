use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ring/src/polyfill/once_cell"]
pub struct OurVendorRingSrcPolyfillOnceCellExtractor;
