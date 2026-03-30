//! Plugin registry — all voxels as ZOS plugins with zkperf witnesses

pub mod mcp_tools;
pub mod rust_parser;
pub mod monster;
pub mod orbits;
pub mod charts;
pub mod bert;

use serde_json::Value;

pub fn all_plugins() -> Vec<Box<dyn ZOSPlugin>> {
    vec![
        Box::new(mcp_tools::McpPlugin),
        Box::new(rust_parser::RustParserPlugin),
        Box::new(monster::MonsterPlugin),
        Box::new(orbits::OrbitsPlugin),
        Box::new(charts::ChartsPlugin),
        Box::new(bert::BertPlugin),
    ]
}

pub fn list_all_commands() -> Vec<(&'static str, &'static str)> {
    let plugins = all_plugins();
    let mut cmds = vec![];
    for p in &plugins {
        for c in p.commands() {
            cmds.push((p.name(), c));
        }
    }
    cmds
}
