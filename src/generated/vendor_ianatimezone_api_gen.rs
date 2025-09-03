use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/iana-time-zone/api_gen"]
pub struct OurVendorIanaTimeZoneApiGenExtractor;
