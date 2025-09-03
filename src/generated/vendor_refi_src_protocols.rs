use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/r-efi/src/protocols"]
pub struct OurVendorREfiSrcProtocolsExtractor;
