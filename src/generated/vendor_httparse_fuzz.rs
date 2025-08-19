use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/httparse/fuzz"]
pub struct OurVendorHttparseFuzzExtractor;
