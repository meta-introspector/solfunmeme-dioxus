use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/syscall/src/arch"]
pub struct OurVendorSyscallSrcArchExtractor;
