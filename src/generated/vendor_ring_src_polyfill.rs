use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ring/src/polyfill"]
pub struct OurVendorRingSrcPolyfillExtractor;
