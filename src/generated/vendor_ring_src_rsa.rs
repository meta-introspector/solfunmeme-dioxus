use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ring/src/rsa"]
pub struct OurVendorRingSrcRsaExtractor;
