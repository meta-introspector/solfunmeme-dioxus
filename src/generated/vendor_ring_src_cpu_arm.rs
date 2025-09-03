use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ring/src/cpu/arm"]
pub struct OurVendorRingSrcCpuArmExtractor;
