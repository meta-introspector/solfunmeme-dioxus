use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/zerocopy/testdata"]
pub struct OurVendorZerocopyTestdataExtractor;
