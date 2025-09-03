use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/icu4x/ffi/freertos/src"]
pub struct OurVendorIcu4xFfiFreertosSrcExtractor;
