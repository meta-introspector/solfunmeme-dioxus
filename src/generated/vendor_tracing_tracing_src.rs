use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tracing/tracing/src"]
pub struct OurVendorTracingTracingSrcExtractor;
