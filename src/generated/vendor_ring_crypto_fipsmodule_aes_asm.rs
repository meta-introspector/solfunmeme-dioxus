use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ring/crypto/fipsmodule/aes/asm"]
pub struct OurVendorRingCryptoFipsmoduleAesAsmExtractor;
