use serde::Deserialize;
#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseWithContext<O> {
    pub value: O,
}