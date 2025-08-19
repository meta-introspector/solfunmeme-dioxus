use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/emojis/.cargo"]
pub struct OurVendorEmojisCargoExtractor;
