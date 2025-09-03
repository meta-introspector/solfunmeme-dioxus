use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/dioxus/.docker"]
pub struct OurVendorDioxusDockerExtractor;
