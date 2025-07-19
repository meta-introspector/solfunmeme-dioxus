use serde::{Serialize, Deserialize, Serializer, Deserializer};
use solfunmeme_clifford::{SolCliffordAlgebra, SolMultivector};

use tclifford::algebra::ClAlgebraBase;
use tclifford::types::IndexType;

/// A wrapper around `SolMultivector` that provides `Serialize` and `Deserialize` implementations.
#[derive(Debug, Clone)]
pub struct SerializableMultivector(pub SolMultivector);

impl Serialize for SerializableMultivector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut coeffs = vec![0.0f32; 1 << SolCliffordAlgebra::dim()];
        for (idx, coeff) in self.0.coeff_enumerate() {
            coeffs[idx as usize] = *coeff;
        }
        coeffs.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SerializableMultivector {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let coefficients = Vec::<f32>::deserialize(deserializer)?;
        if coefficients.len() != (1 << SolCliffordAlgebra::dim()) {
            return Err(serde::de::Error::custom(format!(
                "Invalid number of coefficients. Expected {}, got {}",
                (1 << SolCliffordAlgebra::dim()),
                coefficients.len()
            )));
        }
        let mv = SolMultivector::from_indexed_iter(
            coefficients
                .iter()
                .enumerate()
                .filter(|(_, &coeff)| coeff != 0.0)
                .map(|(idx, &coeff)| (idx as IndexType, coeff))
        ).map_err(|e| serde::de::Error::custom(format!("{:?}", e)))?;
        Ok(SerializableMultivector(mv))
    }
}