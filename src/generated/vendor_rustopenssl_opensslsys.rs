use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rust-openssl/openssl-sys"]
pub struct OurVendorRustOpensslOpensslSysExtractor;
