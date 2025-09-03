use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/memchr/src/arch/aarch64/neon"]
pub struct OurVendorMemchrSrcArchAarch64NeonExtractor;
