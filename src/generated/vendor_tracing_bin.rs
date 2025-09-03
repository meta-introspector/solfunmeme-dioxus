use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tracing/bin"]
pub struct OurVendorTracingBinExtractor;
