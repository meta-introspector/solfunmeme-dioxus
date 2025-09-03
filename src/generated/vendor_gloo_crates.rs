use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/gloo/crates"]
pub struct OurVendorGlooCratesExtractor;
