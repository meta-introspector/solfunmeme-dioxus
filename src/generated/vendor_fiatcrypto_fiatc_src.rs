use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/fiat-crypto/fiat-c/src"]
pub struct OurVendorFiatCryptoFiatCSrcExtractor;
