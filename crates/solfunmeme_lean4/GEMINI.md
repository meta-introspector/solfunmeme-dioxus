# AI Agent Directives for `solfunmeme_lean4`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `solfunmeme_lean4` crate.

## Core Principles

When working within `solfunmeme_lean4`, AI agents should prioritize:

1.  **Lean 4 Integration:** Facilitate seamless integration with Lean 4 theorem prover concepts and data structures.
2.  **Ontology Resolution:** Provide robust mechanisms for resolving Lean 4-related concepts against defined ontologies.
3.  **Type Fidelity:** Maintain strict type fidelity when translating between Lean 4 and Rust data models.

## Operational Guidelines

The `solfunmeme_lean4` crate primarily focuses on bridging Lean 4 concepts with the project's RDF-based ontology. Its key component is the `OntologyResolver`.

### Key Components and Usage:

#### 1. `ontology_resolver::OntologyResolver`

The `OntologyResolver` is responsible for loading an ontology (currently expected to be a Turtle file) and providing a mechanism to resolve concept URIs to their associated emoji representations.

**Example Usage:**

```rust
use solfunmeme_lean4::ontology_resolver::OntologyResolver;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    // Assuming you have an ontology file named `test_ontology.ttl`
    // with triples like: <http://example.org/concept/foo> <https://rdf.solfunmeme.com/spec/2025/07/17/emoji.ttl#emoji> "💡" .
    let ontology_path = PathBuf::from("test_ontology.ttl");

    // Create a dummy ontology file for demonstration if it doesn't exist
    #[cfg(test)]
    {
        use std::fs::File;
        use std::io::Write;
        if !ontology_path.exists() {
            let mut file = File::create(&ontology_path)?;
            file.write_all(b"@prefix : <http://example.org/concept/> .\n")?;
            file.write_all(b"@prefix em: <https://rdf.solfunmeme.com/spec/2025/07/17/emoji.ttl#> .\n")?;
            file.write_all(b":foo em:emoji \"💡\" .\n")?;
            file.write_all(b":bar em:emoji \"🚀\" .\n")?;
        }
    }

    let resolver = OntologyResolver::load(&ontology_path)?;

    let emoji_for_foo = resolver.resolve_emoji("http://example.org/concept/foo");
    println!("Emoji for foo: {:?}", emoji_for_foo);

    let emoji_for_bar = resolver.resolve_emoji("http://example.org/concept/bar");
    println!("Emoji for bar: {:?}", emoji_for_bar);

    let emoji_for_baz = resolver.resolve_emoji("http://example.org/concept/baz");
    println!("Emoji for baz: {:?}", emoji_for_baz);

    Ok(())
}
```

### Refactoring Impact:

The `solfunmeme_lean4` crate now leverages `solfunmeme_rdf_utils::rdf_graph::RdfGraph` for loading and querying ontologies. This simplifies the underlying RDF operations and ensures consistency with the project's centralized RDF utility.

AI agents should use the `OntologyResolver` for any Lean 4-related semantic lookups that involve the project's RDF ontology.

```