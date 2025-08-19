use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/icu4x/ffi/capi"]
pub struct OurVendorIcu4xFfiCapiExtractor;
