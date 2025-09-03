use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/openssl-probe"]
pub struct OurVendorOpensslProbeExtractor;
