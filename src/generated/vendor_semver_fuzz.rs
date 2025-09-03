use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/semver/fuzz"]
pub struct OurVendorSemverFuzzExtractor;
