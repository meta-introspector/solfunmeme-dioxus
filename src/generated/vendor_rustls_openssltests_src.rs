use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustls/openssl-tests/src"]
pub struct OurVendorRustlsOpensslTestsSrcExtractor;
