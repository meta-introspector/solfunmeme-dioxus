use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rust-openssl/openssl-errors/src"]
pub struct OurVendorRustOpensslOpensslErrorsSrcExtractor;
