use serde::{Deserialize, Serialize};
use super::lean4_constant_val::Lean4ConstantVal;
use super::lean4_constant_kind::Lean4ConstantKind;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Lean4ConstantInfoB {
    pub sig: Lean4ConstantVal,
    pub name: String,
    #[serde(rename = "kindB")]
    pub kind_b: String,
    pub kind: Lean4ConstantKind,
    pub cnst_inf: serde_json::Value, // This will hold the specific constant info (e.g., InductiveVal)
}
