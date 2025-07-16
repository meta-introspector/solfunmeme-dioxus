//mod project_algebra;

use crate::project_algebra::Feature;
use std::collections::HashMap;

/// Reflection information for a declaration in code, linking its symbolic name to algebraic value.
#[derive(Debug)]
pub struct DeclarationReflection {
    pub name: String,
    pub features: Vec<Feature>,
    pub numeric_signature: u64,
}

impl DeclarationReflection {
    pub fn new(name: impl Into<String>, features: Vec<Feature>) -> Self {
        let numeric_signature = features.iter().map(|f| f.prime()).product();
        DeclarationReflection {
            name: name.into(),
            features,
            numeric_signature,
        }
    }
}

/// Holds all reflected declarations for the project
#[derive(Debug)]
pub struct ProjectReflection {
    pub declarations: HashMap<String, DeclarationReflection>,
}

impl ProjectReflection {
    pub fn new() -> Self {
        ProjectReflection {
            declarations: HashMap::new(),
        }
    }

    /// Add a new reflected declaration
    pub fn add_declaration(&mut self, name: impl Into<String>, features: Vec<Feature>) {
        let decl = DeclarationReflection::new(name, features);
        self.declarations.insert(decl.name.clone(), decl);
    }

    /// Find a declaration's numeric signature by name
    pub fn signature_for(&self, name: &str) -> Option<u64> {
        self.declarations.get(name).map(|d| d.numeric_signature)
    }

    /// List all declarations and their algebraic signatures
    pub fn list_declarations(&self) -> Vec<(&String, u64)> {
        self.declarations
            .iter()
            .map(|(k, v)| (k, v.numeric_signature))
            .collect()
    }

    /// Find declaration(s) by numeric signature
    pub fn declarations_by_signature(&self, signature: u64) -> Vec<&DeclarationReflection> {
        self.declarations
            .values()
            .filter(|d| d.numeric_signature == signature)
            .collect()
    }
}

// Example usage (not run in library)
#[cfg(test)]
mod tests {
    use super::*;
    use crate::project_algebra::Feature;

    #[test]
    fn reflection_works() {
        let mut reflector = ProjectReflection::new();

        // Add some declarations
        reflector.add_declaration(
            "CoreProject",
            vec![
                Feature::Developer,
                Feature::Package,
                Feature::Modular,
                Feature::RustLang,
                Feature::Web,
            ],
        );
        reflector.add_declaration(
            "FunLayer",
            vec![Feature::Fun, Feature::Introspection, Feature::Docs],
        );
        reflector.add_declaration(
            "WebComponents",
            vec![Feature::Web, Feature::Modular, Feature::RustLang],
        );

        // Get signature for CoreProject
        assert_eq!(reflector.signature_for("CoreProject"), Some(4862));
        // Find declarations by signature
        let found = reflector.declarations_by_signature(31);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].name, "FunLayer");
    }
}
