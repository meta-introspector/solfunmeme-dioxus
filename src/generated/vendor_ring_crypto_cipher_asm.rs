use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ring/crypto/cipher/asm"]
pub struct OurVendorRingCryptoCipherAsmExtractor;
