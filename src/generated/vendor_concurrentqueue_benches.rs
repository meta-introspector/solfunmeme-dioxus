use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/concurrent-queue/benches"]
pub struct OurVendorConcurrentQueueBenchesExtractor;
