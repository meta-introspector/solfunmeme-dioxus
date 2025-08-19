use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/fiat-crypto/src"]
pub struct OurVendorFiatCryptoSrcExtractor;
