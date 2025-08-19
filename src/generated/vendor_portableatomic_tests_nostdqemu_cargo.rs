use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/portable-atomic/tests/no-std-qemu/.cargo"]
pub struct OurVendorPortableAtomicTestsNoStdQemuCargoExtractor;
