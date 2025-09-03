use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/agave/.buildkite/hooks"]
pub struct OurVendorAgaveBuildkiteHooksExtractor;
