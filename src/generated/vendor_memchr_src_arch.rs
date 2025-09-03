use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/memchr/src/arch"]
pub struct OurVendorMemchrSrcArchExtractor;
