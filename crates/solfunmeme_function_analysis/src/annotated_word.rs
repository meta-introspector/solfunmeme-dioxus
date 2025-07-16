use serde::{Deserialize, Serialize};
use super::multivector::Multivector;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AnnotatedWord {
    pub word: String,
    pub primary_emoji: String,
    pub secondary_emoji: String,
    pub wikidata: Option<String>,
    pub embedding: Vec<f32>,
    pub multivector: Multivector,
}
