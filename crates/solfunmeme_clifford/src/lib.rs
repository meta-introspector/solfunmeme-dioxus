use tclifford::{declare_algebra, Multivector};
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use serde::ser::{Serializer, SerializeTuple};
use serde::de::{Deserializer, Visitor, SeqAccess};
use std::fmt;

declare_algebra!(pub SolCliffordAlgebra, [+, +, +], []);

pub type SolMultivector = Multivector<f32, SolCliffordAlgebra>;
pub use SolCliffordAlgebra as SolCl;

pub fn get_multivector_norm(mv: &SolMultivector) -> f32 {
    mv.mag2().sqrt()
}

pub fn get_multivector_coefficients(mv: &SolMultivector) -> Vec<f32> {
    mv.coeff_array_view().to_vec()
}

pub fn generate_multivector_from_string(input: &str) -> SolMultivector {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();

    let mut coeffs = [0.0f32; 8];
    for i in 0..8 {
        // Use parts of the hash to generate coefficients
        coeffs[i] = (result[i] as f32 / 255.0) * 2.0 - 1.0; // Normalize to -1.0 to 1.0
    }
    SolMultivector::from_vector(coeffs.to_vec()).unwrap()
}

#[derive(Debug, Clone, PartialEq)]
pub struct SerializableMultivector(pub SolMultivector);

impl Serialize for SerializableMultivector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let coeffs = self.0.coeff_array_view();
        let mut tuple = serializer.serialize_tuple(coeffs.len())?;
        for coeff in coeffs.iter() {
            tuple.serialize_element(coeff)?;
        }
        tuple.end()
    }
}

impl<'de> Deserialize<'de> for SerializableMultivector {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SolMultivectorVisitor;

        impl<'de> Visitor<'de> for SolMultivectorVisitor {
            type Value = SerializableMultivector;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an 8-element array of f32")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut coeffs = Vec::with_capacity(8);
                while let Some(coeff) = seq.next_element()? {
                    coeffs.push(coeff);
                }
                if coeffs.len() != 8 {
                    return Err(serde::de::Error::invalid_length(
                        coeffs.len(),
                        &"8 elements",
                    ));
                }
                SolMultivector::from_vector(coeffs).map(SerializableMultivector).map_err(|e| serde::de::Error::custom(format!("{:?}", e)))
            }
        }

        deserializer.deserialize_tuple(8, SolMultivectorVisitor)
    }
}
