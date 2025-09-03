use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ring/crypto/chacha/asm"]
pub struct OurVendorRingCryptoChachaAsmExtractor;
