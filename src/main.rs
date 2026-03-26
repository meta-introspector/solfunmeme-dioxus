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

#[cfg(not(target_arch = "wasm32"))]
pub mod extractor;
pub mod fetch_parser;
#[cfg(not(target_arch = "wasm32"))]
mod mcp_gateway;
pub mod playground;
pub mod state;

pub mod core;
pub mod embedself;
pub mod plugin;

#[cfg(not(target_arch = "wasm32"))]
fn spawn_mcp_gateway() {
    std::thread::spawn(|| {
        let runtime = match tokio::runtime::Runtime::new() {
            Ok(runtime) => runtime,
            Err(error) => {
                eprintln!("failed to create tokio runtime for MCP gateway: {error}");
                return;
            }
        };

        if let Err(error) =
            runtime.block_on(async { crate::mcp_gateway::run_default_mcp_gateway().await })
        {
            eprintln!("MCP gateway failed to start: {error}");
        }
    });
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    spawn_mcp_gateway();

    // Use the memes App component from views
    embedself::printall();

    launch(PlaygroundApp);
}
