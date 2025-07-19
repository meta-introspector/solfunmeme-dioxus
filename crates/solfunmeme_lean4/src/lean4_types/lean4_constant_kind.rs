use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Lean4ConstantKind {
    pub value: String,
    pub kind: String,
}
