use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/bumpalo/tests/all"]
pub struct OurVendorBumpaloTestsAllExtractor;
