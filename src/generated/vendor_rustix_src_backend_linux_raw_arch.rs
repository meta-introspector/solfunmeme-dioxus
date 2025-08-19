use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/src/backend/linux_raw/arch"]
pub struct OurVendorRustixSrcBackendLinuxRawArchExtractor;
