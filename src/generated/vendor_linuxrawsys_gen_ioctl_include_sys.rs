use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/linux-raw-sys/gen/ioctl/include/sys"]
pub struct OurVendorLinuxRawSysGenIoctlIncludeSysExtractor;
