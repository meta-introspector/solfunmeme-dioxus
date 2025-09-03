use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ring/src/aead/gcm"]
pub struct OurVendorRingSrcAeadGcmExtractor;
