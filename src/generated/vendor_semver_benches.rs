use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/semver/benches"]
pub struct OurVendorSemverBenchesExtractor;
