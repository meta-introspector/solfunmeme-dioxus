use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ring/crypto/fipsmodule/ec/asm"]
pub struct OurVendorRingCryptoFipsmoduleEcAsmExtractor;
