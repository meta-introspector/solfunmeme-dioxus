use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/httpdate/fuzz"]
pub struct OurVendorHttpdateFuzzExtractor;
