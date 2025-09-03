use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/dioxus/packages/mobile/.cargo"]
pub struct OurVendorDioxusPackagesMobileCargoExtractor;
