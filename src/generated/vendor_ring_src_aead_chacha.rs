use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ring/src/aead/chacha"]
pub struct OurVendorRingSrcAeadChachaExtractor;
