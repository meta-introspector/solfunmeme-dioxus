use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ring/src/aead/aes"]
pub struct OurVendorRingSrcAeadAesExtractor;
