use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/openssl-probe/src"]
pub struct OurVendorOpensslProbeSrcExtractor;
