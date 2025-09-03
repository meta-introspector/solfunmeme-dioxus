use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/env_logger/.cargo"]
pub struct OurVendorEnvLoggerCargoExtractor;
