use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/crypto-bigint/src"]
pub struct OurVendorCryptoBigintSrcExtractor;
