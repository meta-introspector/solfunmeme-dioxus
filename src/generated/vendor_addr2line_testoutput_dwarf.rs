use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/addr2line/testoutput/dwarf"]
pub struct OurVendorAddr2lineTestoutputDwarfExtractor;
