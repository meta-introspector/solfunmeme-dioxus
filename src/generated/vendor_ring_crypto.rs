use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ring/crypto"]
pub struct OurVendorRingCryptoExtractor;
