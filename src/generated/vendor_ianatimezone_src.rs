use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/iana-time-zone/src"]
pub struct OurVendorIanaTimeZoneSrcExtractor;
