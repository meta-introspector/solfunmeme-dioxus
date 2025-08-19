use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/zip/fuzz"]
pub struct OurVendorZipFuzzExtractor;
