use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wayland-rs/wayland-server/src"]
pub struct OurVendorWaylandRsWaylandServerSrcExtractor;
