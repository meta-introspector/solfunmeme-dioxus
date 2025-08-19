use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wit-bindgen/crates/c/src"]
pub struct OurVendorWitBindgenCratesCSrcExtractor;
