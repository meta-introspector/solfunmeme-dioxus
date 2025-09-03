use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/utf8_iter/fuzz"]
pub struct OurVendorUtf8IterFuzzExtractor;
