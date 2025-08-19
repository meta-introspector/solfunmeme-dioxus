use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wit-bindgen/crates/test/src"]
pub struct OurVendorWitBindgenCratesTestSrcExtractor;
