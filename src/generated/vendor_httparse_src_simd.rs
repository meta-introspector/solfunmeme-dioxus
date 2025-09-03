use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/httparse/src/simd"]
pub struct OurVendorHttparseSrcSimdExtractor;
