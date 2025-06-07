use serde::Deserialize;
use crate::model::Parsed;
#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenData {
    pub parsed: Parsed,
}
