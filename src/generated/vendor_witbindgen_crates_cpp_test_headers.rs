use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wit-bindgen/crates/cpp/test_headers"]
pub struct OurVendorWitBindgenCratesCppTestHeadersExtractor;
