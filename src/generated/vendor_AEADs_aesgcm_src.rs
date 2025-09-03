use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/AEADs/aes-gcm/src"]
pub struct OurVendorAEADsAesGcmSrcExtractor;
