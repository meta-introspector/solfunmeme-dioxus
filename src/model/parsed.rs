use serde::Deserialize;
use crate::model::ParseInfo;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parsed {
    pub info: ParseInfo,
}