use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wit-bindgen/ci/docker"]
pub struct OurVendorWitBindgenCiDockerExtractor;
