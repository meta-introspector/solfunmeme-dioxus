use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/gloo/crates/worker"]
pub struct OurVendorGlooCratesWorkerExtractor;
