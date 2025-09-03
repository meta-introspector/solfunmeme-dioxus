use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wit-bindgen/src"]
pub struct OurVendorWitBindgenSrcExtractor;
