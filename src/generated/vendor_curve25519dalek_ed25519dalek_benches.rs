use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/curve25519-dalek/ed25519-dalek/benches"]
pub struct OurVendorCurve25519DalekEd25519DalekBenchesExtractor;
