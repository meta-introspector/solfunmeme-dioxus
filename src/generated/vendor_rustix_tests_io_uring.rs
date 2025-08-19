use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/tests/io_uring"]
pub struct OurVendorRustixTestsIoUringExtractor;
