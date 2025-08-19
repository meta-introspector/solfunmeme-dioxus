use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/dioxus/.devcontainer"]
pub struct OurVendorDioxusDevcontainerExtractor;
