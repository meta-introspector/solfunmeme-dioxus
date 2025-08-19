use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/src/backend"]
pub struct OurVendorRustixSrcBackendExtractor;
