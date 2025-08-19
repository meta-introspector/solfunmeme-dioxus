use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/sharded-slab/src"]
pub struct OurVendorShardedSlabSrcExtractor;
