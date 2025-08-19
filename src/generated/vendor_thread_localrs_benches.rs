use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/thread_local-rs/benches"]
pub struct OurVendorThreadLocalRsBenchesExtractor;
