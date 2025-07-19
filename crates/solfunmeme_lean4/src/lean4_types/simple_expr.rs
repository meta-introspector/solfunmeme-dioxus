use serde::{Deserialize, Serialize};
use super::lean4_constant_info_b::Lean4ConstantInfoB;
use crate::ontology_resolver::OntologyResolver;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimpleExpr {
    pub kind: String,
    #[serde(rename = "cnstInfB")]
    pub cnst_inf_b: Lean4ConstantInfoB,
}

impl SimpleExpr {
    pub fn get_name(&self) -> &str {
        &self.cnst_inf_b.name
    }

    pub fn get_emoji_representation(&self, resolver: &OntologyResolver) -> String {
        let concept_uri = format!("http://example.org/lean4_code#{}", self.get_name());
        resolver.resolve_emoji(&concept_uri).cloned().unwrap_or_else(|| "❓".to_string())
    }
}
