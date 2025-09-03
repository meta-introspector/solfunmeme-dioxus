use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tracing/.config"]
pub struct OurVendorTracingConfigExtractor;
