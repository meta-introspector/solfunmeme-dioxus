use serde::{Deserialize, Serialize};
use crate::serializable_multivector::SerializableMultivector;

/// Represents a request to perform a Clifford algebra operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliffordOperationRequest {
    /// The name of the operation to perform (e.g., "create_scalar_multivector", "compose_prime_multivectors").
    pub operation: String,
    /// A scalar value to be used in the operation (e.g., for scalar multivector creation).
    #[serde(default)]
    pub scalar_value: f32,
    /// A list of vector component values to be used in the operation (e.g., for vector multivector creation or prime composition).
    #[serde(default)]
    pub vector_values: Vec<f32>,
    /// An optional input multivector to be used in operations like `update_flow_multivector`.
    #[serde(default)]
    pub input_multivector: Option<SerializableMultivector>,
}

/// Represents the response from a Clifford algebra operation.
#[derive(Debug, Serialize, Deserialize)]
pub struct CliffordOperationResponse {
    /// The result of the operation, typically a string representation of a multivector or other output.
    pub result: String,
    /// An optional error message if the operation failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
