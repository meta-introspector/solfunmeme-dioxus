use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wit-bindgen/.devcontainer"]
pub struct OurVendorWitBindgenDevcontainerExtractor;
