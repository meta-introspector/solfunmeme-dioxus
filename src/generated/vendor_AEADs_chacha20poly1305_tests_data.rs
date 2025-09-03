use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/AEADs/chacha20poly1305/tests/data"]
pub struct OurVendorAEADsChacha20poly1305TestsDataExtractor;
