use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/memchr/src/arch/all"]
pub struct OurVendorMemchrSrcArchAllExtractor;
