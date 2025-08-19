use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/signal-hook/src"]
pub struct OurVendorSignalHookSrcExtractor;
