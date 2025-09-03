use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ring/src/rsa/padding"]
pub struct OurVendorRingSrcRsaPaddingExtractor;
