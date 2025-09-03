use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tinyvec/fuzz/src/bin"]
pub struct OurVendorTinyvecFuzzSrcBinExtractor;
