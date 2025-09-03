use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/memchr/benchmarks/engines/libc"]
pub struct OurVendorMemchrBenchmarksEnginesLibcExtractor;
