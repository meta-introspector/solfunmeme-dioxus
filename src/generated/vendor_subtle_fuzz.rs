use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/subtle/fuzz"]
pub struct OurVendorSubtleFuzzExtractor;
