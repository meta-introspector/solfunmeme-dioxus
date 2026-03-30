//! ZOS Plugin trait + zkperf witness interface.
//! Each playground voxel implements this to become a loadable plugin.

use serde_json::Value;
use serde::{Serialize, Deserialize};

/// ZOS plugin interface (mirrors ~/zos-server/src/traits.rs)
pub trait ZOSPlugin: Send + Sync {
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str;
    fn commands(&self) -> Vec<&'static str>;
    fn execute(&self, command: &str, args: Vec<String>) -> Result<Value, String>;
}

/// zkperf witness for any plugin execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginWitness {
    pub plugin: String,
    pub command: String,
    pub timestamp: i64,
    pub duration_ms: u64,
    pub commitment: String,
    pub orbifold: [u64; 3],
    pub crown_product: u64,
}

impl PluginWitness {
    pub fn new(plugin: &str, command: &str, duration_ms: u64) -> Self {
        use sha2::{Sha256, Digest};
        let now = chrono::Utc::now().timestamp();
        let mut h = Sha256::new();
        h.update(format!("{}:{}:{}", plugin, command, now));
        let commitment = hex::encode(h.finalize());
        Self {
            plugin: plugin.into(),
            command: command.into(),
            timestamp: now,
            duration_ms,
            commitment,
            orbifold: [(now as u64) % 71, (now as u64) % 59, (now as u64) % 47],
            crown_product: 196_883,
        }
    }
}

/// Run a plugin command and produce a zkperf witness
pub fn execute_witnessed(plugin: &dyn ZOSPlugin, command: &str, args: Vec<String>) -> (Result<Value, String>, PluginWitness) {
    let start = std::time::Instant::now();
    let result = plugin.execute(command, args);
    let witness = PluginWitness::new(plugin.name(), command, start.elapsed().as_millis() as u64);
    (result, witness)
}

pub mod mcp_tools;
pub mod rust_parser;
pub mod monster;
pub mod orbits;
pub mod charts;
pub mod bert;
pub mod registry;
