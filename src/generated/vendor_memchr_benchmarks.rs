use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/memchr/benchmarks"]
pub struct OurVendorMemchrBenchmarksExtractor;
