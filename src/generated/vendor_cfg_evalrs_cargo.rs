use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/cfg_eval.rs/.cargo"]
pub struct OurVendorCfgEvalRsCargoExtractor;
