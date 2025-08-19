use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/icu4x/ffi/capi/bindings"]
pub struct OurVendorIcu4xFfiCapiBindingsExtractor;
