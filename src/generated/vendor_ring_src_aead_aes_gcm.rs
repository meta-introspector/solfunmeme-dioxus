use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ring/src/aead/aes_gcm"]
pub struct OurVendorRingSrcAeadAesGcmExtractor;
