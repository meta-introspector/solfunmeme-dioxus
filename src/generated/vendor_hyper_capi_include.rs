use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/hyper/capi/include"]
pub struct OurVendorHyperCapiIncludeExtractor;
