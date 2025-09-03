use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/data-encoding/lib/fuzz"]
pub struct OurVendorDataEncodingLibFuzzExtractor;
