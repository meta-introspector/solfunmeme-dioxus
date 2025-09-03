use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/pki-types/fuzz/corpus/pem"]
pub struct OurVendorPkiTypesFuzzCorpusPemExtractor;
