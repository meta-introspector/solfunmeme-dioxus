use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/libhermit-rs/.cargo"]
pub struct OurVendorLibhermitRsCargoExtractor;
