//! Generic ZOS Plugin with self-rendering GUI + erdfa/fractran/DA51 APIs.
//!
//! Each plugin:
//! 1. Implements ZOSPlugin (commands)
//! 2. Renders its own UI as erdfa Components (semantic voxels)
//! 3. Produces DA51 CBOR shards for every output
//! 4. Has a FRACTRAN state (Gödel number) that evolves with each action

use serde::{Serialize, Deserialize};
use serde_json::Value;
use sha2::{Sha256, Digest};

// ── ZOS Plugin trait ────────────────────────────────────────────

pub trait ZOSPlugin: Send + Sync {
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str;
    fn commands(&self) -> Vec<&'static str>;
    fn execute(&self, command: &str, args: Vec<String>) -> Result<Value, String>;

    /// Render plugin UI as erdfa semantic components
    fn render(&self) -> Vec<GuiComponent> { vec![] }

    /// Current FRACTRAN state (Gödel number as prime factorization)
    fn state(&self) -> Vec<(u64, u32)> { vec![] }

    /// FRACTRAN ratios (actions that transform state)
    fn ratios(&self) -> Vec<FractranRatio> { vec![] }
}

// ── GUI Component (erdfa-compatible) ────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GuiComponent {
    Heading { level: u8, text: String },
    Paragraph { text: String },
    Code { language: String, source: String },
    Table { headers: Vec<String>, rows: Vec<Vec<String>> },
    Button { label: String, command: String },
    KeyValue { pairs: Vec<(String, String)> },
    Group { role: String, children: Vec<GuiComponent> },
}

// ── FRACTRAN ────────────────────────────────────────────────────

/// 20 Monster primes — universal semantic dimensions
pub const PRIMES: [(u64, &str); 20] = [
    (2, "position"), (3, "credits"), (5, "crypto"), (7, "network"),
    (11, "count"), (13, "peers"), (17, "turn"), (19, "health"),
    (23, "cargo"), (29, "monitor"), (31, "build"), (37, "deploy"),
    (41, "test"), (43, "render"), (47, "agent"), (53, "stego"),
    (59, "tunnel"), (61, "record"), (67, "shard"), (71, "meta"),
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FractranRatio {
    pub name: String,
    pub num: Vec<(u64, u32)>,   // multiply by these primes
    pub den: Vec<(u64, u32)>,   // if divisible by these primes
}

pub fn fractran_step(state: u64, ratio: &FractranRatio) -> Option<u64> {
    let mut s = state;
    for &(p, e) in &ratio.den {
        for _ in 0..e {
            if s % p != 0 { return None; }
            s /= p;
        }
    }
    for &(p, e) in &ratio.num {
        for _ in 0..e { s *= p; }
    }
    Some(s)
}

// ── DA51 CBOR shard ─────────────────────────────────────────────

pub const DASL_TAG: u64 = 55889;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DA51Shard {
    pub plugin: String,
    pub command: String,
    pub cid: String,
    pub dasl: String,
    pub orbifold: [u64; 3],
    pub bott: u8,
    pub data: Value,
}

impl DA51Shard {
    pub fn from_result(plugin: &str, command: &str, data: &Value) -> Self {
        let json = serde_json::to_vec(data).unwrap_or_default();
        let hash = Sha256::digest(&json);
        let cid = format!("bafk{}", hex::encode(&hash[..16]));
        let dasl = format!("0xda51{}", hex::encode(&hash[..8]));
        let n = u64::from_le_bytes(hash[..8].try_into().unwrap_or([0; 8]));
        DA51Shard {
            plugin: plugin.into(),
            command: command.into(),
            cid,
            dasl,
            orbifold: [n % 71, n % 59, n % 47],
            bott: (hash[2] % 8) as u8,
            data: data.clone(),
        }
    }

    pub fn to_cbor(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        ciborium::into_writer(self, &mut buf).unwrap_or_default();
        buf
    }
}

// ── zkperf witness ──────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginWitness {
    pub plugin: String,
    pub command: String,
    pub timestamp: i64,
    pub duration_ms: u64,
    pub commitment: String,
    pub orbifold: [u64; 3],
    pub crown_product: u64,
    pub shard: DA51Shard,
}

/// Execute a plugin command, produce DA51 shard + zkperf witness
pub fn execute_witnessed(plugin: &dyn ZOSPlugin, command: &str, args: Vec<String>) -> (Result<Value, String>, PluginWitness) {
    let start = std::time::Instant::now();
    let result = plugin.execute(command, args);
    let ms = start.elapsed().as_millis() as u64;
    let now = chrono::Utc::now().timestamp();

    let data = result.as_ref().cloned().unwrap_or(Value::Null);
    let shard = DA51Shard::from_result(plugin.name(), command, &data);

    let mut h = Sha256::new();
    h.update(format!("{}:{}:{}", plugin.name(), command, now));
    let commitment = hex::encode(h.finalize());

    let witness = PluginWitness {
        plugin: plugin.name().into(),
        command: command.into(),
        timestamp: now,
        duration_ms: ms,
        commitment,
        orbifold: shard.orbifold,
        crown_product: 196_883,
        shard,
    };
    (result, witness)
}
