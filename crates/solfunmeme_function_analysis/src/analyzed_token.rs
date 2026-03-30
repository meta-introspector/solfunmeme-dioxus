use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalyzedToken {
    pub token: String,
    pub count: usize,
    pub multivector_str: String,
    pub orbital_path: Option<Vec<(f64, f64)>>,
}
