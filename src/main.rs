#![allow(non_snake_case)]

use dioxus::prelude::*;

mod views;
use views::*;

mod header;
use header::*;

mod utils;
use utils::*;

mod fetch_parser;
use fetch_parser::*;

mod svg_assets;
pub(crate) use svg_assets::*;

mod dioxus_adapter;
pub(crate) use dioxus_adapter::*;

mod fetch_util;
pub(crate) use fetch_util::*;

mod app;
pub(crate) use app::*;

fn main() {
    launch(App);
}
