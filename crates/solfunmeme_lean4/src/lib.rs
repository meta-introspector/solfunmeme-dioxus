pub mod lean4_types;
pub mod ontology_resolver;

pub use lean4_types::simple_expr::SimpleExpr;
pub use lean4_types::lean4_constant_info_b::Lean4ConstantInfoB;
pub use lean4_types::lean4_constant_kind::Lean4ConstantKind;
pub use lean4_types::lean4_constant_val::Lean4ConstantVal;
pub use lean4_types::lean4_level::Lean4Level;
pub use lean4_types::lean4_type::Lean4Type;
pub use ontology_resolver::OntologyResolver;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_ontology_resolver() -> anyhow::Result<()> {
        let ontology_path = PathBuf::from("test_ontology_lean4.ttl");

        // Create a dummy ontology file for testing
        let mut file = File::create(&ontology_path)?;
        file.write_all(b"@prefix : <http://example.org/concept/> .\n")?;
        file.write_all(b"@prefix em: <https://rdf.solfunmeme.com/spec/2025/07/17/emoji.ttl#> .\n")?;
        file.write_all(b":foo em:emoji \"💡\" .\n")?;
        file.write_all(b":bar em:emoji \"🚀\" .\n")?;
        file.flush()?;

        let resolver = OntologyResolver::load(&ontology_path)?;

        assert_eq!(resolver.resolve_emoji("http://example.org/concept/foo"), Some(&"💡".to_string()));
        assert_eq!(resolver.resolve_emoji("http://example.org/concept/bar"), Some(&"🚀".to_string()));
        assert_eq!(resolver.resolve_emoji("http://example.org/concept/baz"), None);
        
        // Clean up the dummy file
        std::fs::remove_file(&ontology_path)?;

        Ok(())
    }
}
