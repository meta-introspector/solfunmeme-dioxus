use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/iana-time-zone"]
pub struct OurVendorIanaTimeZoneExtractor;
