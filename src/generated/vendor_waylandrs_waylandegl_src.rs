use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wayland-rs/wayland-egl/src"]
pub struct OurVendorWaylandRsWaylandEglSrcExtractor;
