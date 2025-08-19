use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/windows-rs/.cargo"]
pub struct OurVendorWindowsRsCargoExtractor;
