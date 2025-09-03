use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/AEADs/aes-siv/src"]
pub struct OurVendorAEADsAesSivSrcExtractor;
