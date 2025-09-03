use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/icu4x/ffi/capi/src"]
pub struct OurVendorIcu4xFfiCapiSrcExtractor;
