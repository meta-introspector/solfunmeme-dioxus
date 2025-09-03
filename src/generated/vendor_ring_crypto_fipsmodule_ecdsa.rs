use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ring/crypto/fipsmodule/ecdsa"]
pub struct OurVendorRingCryptoFipsmoduleEcdsaExtractor;
