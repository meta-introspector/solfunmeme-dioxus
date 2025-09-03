use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/portable-atomic/bench/benches"]
pub struct OurVendorPortableAtomicBenchBenchesExtractor;
