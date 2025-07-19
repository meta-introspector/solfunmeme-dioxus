use serde::{Deserialize, Serialize};
use crate::serializable_multivector::SerializableMultivector;
use super::clifford_ops_models::CliffordOperationRequest;
use super::meme_definitions::MemeToken;

/// Represents a task for LLM-based code reflection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeReflectionTask {
    /// The code chunks to be reflected upon.
    pub code_chunks: Vec<String>,
    /// The multivector representation of the code chunks, encoding their geometric position in the Code-Math Manifold.
    pub multivector: SerializableMultivector,
}

/// Represents a task for LLM-driven code evolution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeEvolutionTask {
    /// The code snippet to be evolved.
    pub code_snippet: String,
    /// An optional MemeToken associated with the code snippet, providing semantic context for evolution.
    pub meme_token: Option<MemeToken>,
}

/// An enum representing the different types of payloads that an LLM task can carry.
/// This provides flexibility for the Market Maker to dispatch various kinds of tasks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LlmTaskPayload {
    /// A payload for code reflection tasks.
    CodeReflection(CodeReflectionTask),
    /// A payload for Clifford algebra operations.
    CliffordOperation(CliffordOperationRequest),
    /// A payload for code evolution tasks.
    CodeEvolution(CodeEvolutionTask),
}

/// Represents a group of LLM tasks to be dispatched by the Market Maker.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmTaskGroup {
    /// A high-level categorization of the task group (e.g., "code_reflection", "clifford_operation", "code_evolution").
    pub task_type: String,
    /// The specific payload of the LLM task, defined by the `LlmTaskPayload` enum.
    pub payload: LlmTaskPayload,
}
