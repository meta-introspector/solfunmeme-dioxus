use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/icu4x/.devcontainer"]
pub struct OurVendorIcu4xDevcontainerExtractor;
