use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ring/crypto/fipsmodule/bn/asm"]
pub struct OurVendorRingCryptoFipsmoduleBnAsmExtractor;
