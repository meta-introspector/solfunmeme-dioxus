use crate::model::Parsed;
use serde::Deserialize;
#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenData {
    pub parsed: Parsed,
}
