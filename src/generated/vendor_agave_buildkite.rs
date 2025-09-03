use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/agave/.buildkite"]
pub struct OurVendorAgaveBuildkiteExtractor;
