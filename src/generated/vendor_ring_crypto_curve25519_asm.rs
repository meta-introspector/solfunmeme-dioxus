use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ring/crypto/curve25519/asm"]
pub struct OurVendorRingCryptoCurve25519AsmExtractor;
