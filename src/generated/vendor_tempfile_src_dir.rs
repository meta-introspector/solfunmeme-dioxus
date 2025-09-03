use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tempfile/src/dir"]
pub struct OurVendorTempfileSrcDirExtractor;
