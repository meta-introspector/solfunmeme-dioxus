use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum PropValue {
    Bool(bool),
    String(String),
    StringVec(Vec<String>),
    // Note: We'll add more complex types as needed
    // ComponentMeme(ComponentMeme),
    // MemeCategory(MemeCategory),
    // WikidataMeme(WikidataMeme),
    // WorkflowMeme(WorkflowMeme),
    // WorkflowStep(WorkflowStep),
    // UseConnections(UseConnections),
} 