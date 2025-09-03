use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/memchr/fuzz"]
pub struct OurVendorMemchrFuzzExtractor;
