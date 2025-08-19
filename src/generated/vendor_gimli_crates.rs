use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/gimli/crates"]
pub struct OurVendorGimliCratesExtractor;
