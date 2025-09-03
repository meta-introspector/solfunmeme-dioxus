use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tinyvec/fuzz/src"]
pub struct OurVendorTinyvecFuzzSrcExtractor;
