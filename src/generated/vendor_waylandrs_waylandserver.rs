use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wayland-rs/wayland-server"]
pub struct OurVendorWaylandRsWaylandServerExtractor;
