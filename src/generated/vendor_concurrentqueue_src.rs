use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/concurrent-queue/src"]
pub struct OurVendorConcurrentQueueSrcExtractor;
