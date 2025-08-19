use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/icu4x/ffi/dart/lib"]
pub struct OurVendorIcu4xFfiDartLibExtractor;
