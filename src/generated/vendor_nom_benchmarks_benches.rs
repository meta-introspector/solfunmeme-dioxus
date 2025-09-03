use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/nom/benchmarks/benches"]
pub struct OurVendorNomBenchmarksBenchesExtractor;
