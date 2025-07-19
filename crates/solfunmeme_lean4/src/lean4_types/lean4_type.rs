use serde::{Deserialize, Serialize};
use super::lean4_level::Lean4Level;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Lean4Type {
    #[serde(rename = "type")]
    pub type_field: String,
    pub level: Lean4Level,
}
