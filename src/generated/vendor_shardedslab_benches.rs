use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/sharded-slab/benches"]
pub struct OurVendorShardedSlabBenchesExtractor;
