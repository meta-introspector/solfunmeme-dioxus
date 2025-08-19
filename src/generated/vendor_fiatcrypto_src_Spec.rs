use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/fiat-crypto/src/Spec"]
pub struct OurVendorFiatCryptoSrcSpecExtractor;
