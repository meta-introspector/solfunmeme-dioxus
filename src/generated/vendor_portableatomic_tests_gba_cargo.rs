use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/portable-atomic/tests/gba/.cargo"]
pub struct OurVendorPortableAtomicTestsGbaCargoExtractor;
