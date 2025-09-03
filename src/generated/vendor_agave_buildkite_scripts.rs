use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/agave/.buildkite/scripts"]
pub struct OurVendorAgaveBuildkiteScriptsExtractor;
