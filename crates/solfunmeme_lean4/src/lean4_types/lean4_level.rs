use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Lean4Level {
    pub level: String,
    pub kind: String,
}
