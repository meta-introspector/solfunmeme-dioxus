use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/sharded-slab/bin"]
pub struct OurVendorShardedSlabBinExtractor;
