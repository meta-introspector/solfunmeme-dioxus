use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/formats/tls_codec/fuzz/fuzz_targets"]
pub struct OurVendorFormatsTlsCodecFuzzFuzzTargetsExtractor;
