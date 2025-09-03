use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ring/crypto/limbs"]
pub struct OurVendorRingCryptoLimbsExtractor;
