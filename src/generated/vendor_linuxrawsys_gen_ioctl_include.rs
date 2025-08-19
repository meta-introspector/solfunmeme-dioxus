use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/linux-raw-sys/gen/ioctl/include"]
pub struct OurVendorLinuxRawSysGenIoctlIncludeExtractor;
