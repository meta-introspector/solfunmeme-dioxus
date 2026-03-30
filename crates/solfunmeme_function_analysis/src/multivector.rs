use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Multivector {
    pub scalar: f32,
    pub vector: [f32; 3],
}
