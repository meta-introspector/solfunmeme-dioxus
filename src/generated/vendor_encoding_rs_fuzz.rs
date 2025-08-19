use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/encoding_rs/fuzz"]
pub struct OurVendorEncodingRsFuzzExtractor;
