use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tracing/.cargo"]
pub struct OurVendorTracingCargoExtractor;
