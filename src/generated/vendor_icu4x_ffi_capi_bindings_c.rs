use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/icu4x/ffi/capi/bindings/c"]
pub struct OurVendorIcu4xFfiCapiBindingsCExtractor;
