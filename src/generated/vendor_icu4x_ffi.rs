use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/icu4x/ffi"]
pub struct OurVendorIcu4xFfiExtractor;
