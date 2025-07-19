use serde::{Deserialize, Serialize};
use super::lean4_type::Lean4Type;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Lean4ConstantVal {
    #[serde(rename = "type")]
    pub type_field: Lean4Type,
    pub name: String,
    #[serde(rename = "levelParams")]
    pub level_params: Vec<String>,
    pub kind: String,
}
