use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/quick-xml/fuzz/fuzz_targets"]
pub struct OurVendorQuickXmlFuzzFuzzTargetsExtractor;
