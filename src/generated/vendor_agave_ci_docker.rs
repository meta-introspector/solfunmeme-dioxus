use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/agave/ci/docker"]
pub struct OurVendorAgaveCiDockerExtractor;
