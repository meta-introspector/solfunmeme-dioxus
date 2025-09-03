#![allow(non_snake_case)]
use crate::playground::app::PlaygroundApp;
use dioxus::launch;
mod model;
mod views;
use model::*;
mod header;
mod utils;
use fetch_parser::*;
mod svg_assets;
pub(crate) use svg_assets::*;
mod fetch_util;
pub(crate) use fetch_util::*;
mod app;
use crate::model::NotificationInfo;
pub(crate) use app::{Route, LOGO};
mod password_manager;

pub mod extractor;
pub mod fetch_parser;
pub mod playground;
pub mod state;

pub mod core;
pub mod embedself;
pub mod generated;

pub mod project_algebra;
pub mod project_reflector;
pub mod solfunmeme_maps;
pub mod models;
fn main() {
    // Use the memes App component from views
//    embedself::printall();

    launch(PlaygroundApp);
}
