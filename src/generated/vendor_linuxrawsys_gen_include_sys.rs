use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/linux-raw-sys/gen/include/sys"]
pub struct OurVendorLinuxRawSysGenIncludeSysExtractor;
