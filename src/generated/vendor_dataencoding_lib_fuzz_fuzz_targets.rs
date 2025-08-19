use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/data-encoding/lib/fuzz/fuzz_targets"]
pub struct OurVendorDataEncodingLibFuzzFuzzTargetsExtractor;
