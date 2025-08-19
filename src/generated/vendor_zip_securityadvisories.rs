use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/zip/security-advisories"]
pub struct OurVendorZipSecurityAdvisoriesExtractor;
