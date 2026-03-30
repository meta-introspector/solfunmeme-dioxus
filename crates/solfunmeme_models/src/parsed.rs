use crate::parseinfo::ParseInfo;
use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parsed {
    pub info: ParseInfo,
} 