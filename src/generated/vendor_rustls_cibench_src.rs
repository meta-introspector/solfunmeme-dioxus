use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustls/ci-bench/src"]
pub struct OurVendorRustlsCiBenchSrcExtractor;
