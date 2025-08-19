use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/fiat-crypto/src/Assembly"]
pub struct OurVendorFiatCryptoSrcAssemblyExtractor;
