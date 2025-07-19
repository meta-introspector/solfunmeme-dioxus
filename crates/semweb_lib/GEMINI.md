# AI Agent Directives for `semweb_lib`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `semweb_lib` crate.

## Core Principles

When working within `semweb_lib`, AI agents should prioritize:

1.  **Semantic Web Integration:** Provide robust and idiomatic Rust implementations for core Semantic Web concepts and operations.
2.  **Modularity and Extensibility:** Design components to be modular, allowing for easy extension with new reasoning rules, inference strategies, and data processing capabilities.
3.  **Clarity and Usability:** Ensure the API is clear, well-documented, and easy to use for building Semantic Web-enabled applications.

## Operational Guidelines

The `semweb_lib` crate serves as a foundational layer for semantic web interactions within the `solfunmeme-dioxus` project. It leverages `solfunmeme_rdf_utils` for underlying RDF graph management, providing higher-level abstractions for reasoning, inference, and ontology management.

### Key Components and Usage:

#### 1. `SemWebContext`

The `SemWebContext` struct is the entry point for most semantic web operations. It encapsulates an `RdfGraph`, ontology management, and reasoning/inference engines.

**Example Usage:**

```rust
use semweb_lib::{SemWebContext, SemWebResult, Ontology};
use tokio;

#[tokio::main]
async fn main() -> SemWebResult<()> {
    let mut context = SemWebContext::new();

    // Example: Load an ontology (assuming a Turtle file exists at this path)
    // context.load_ontology("path/to/your/ontology.ttl").await?;

    // Example: Add some triples directly to the graph
    context.graph.add_triple(
        "http://example.org/subject1",
        "http://example.org/predicate1",
        "http://example.org/object1",
    )?;
    context.graph.add_literal_triple(
        "http://example.org/subject2",
        "http://example.org/predicate2",
        "Hello, Semantic Web!",
        "http://www.w3.org/2001/XMLSchema#string",
    )?;

    // Example: Perform inference (if rules are defined)
    // context.infer()?;

    // Example: Export the graph to a Turtle file
    context.to_turtle("output.ttl")?;

    println!("Semantic Web operations completed. Check output.ttl");

    Ok(())
}
```

#### 2. Traits (`Timbl`, `Danbri`, `CWM`, `Eco`, `Hofstadter`, `McLuhan`, `Stallman`, `Torvalds`)

These traits define various semantic web functionalities, inspired by prominent figures in the field. Implementations (`TimblImpl`, `DanbriImpl`, etc.) provide concrete logic for these operations.

**Example Usage (within `SemWebContext` or directly):**

```rust
use semweb_lib::{Timbl, TimblImpl, SemWebResult, RdfGraph};
use std::collections::HashMap;

fn main() -> SemWebResult<()> {
    let timbl_agent = TimblImpl;
    let mut graph = RdfGraph::new();

    // Create a resource
    let mut properties = HashMap::new();
    properties.insert("http://purl.org/dc/elements/1.1/title".to_string(), "My Document".to_string());
    let resource = timbl_agent.create_resource("http://example.org/doc1", properties)?;
    println!("Created resource: {:?}", resource);

    // Add a statement
    let statement = timbl_agent.create_statement(
        "http://example.org/doc1",
        "http://purl.org/dc/elements/1.1/creator",
        "http://example.org/person/john_doe",
    )?;
    println!("Created statement: {:?}", statement);

    Ok(())
}
```

### Refactoring Impact:

The `semweb_lib` crate has been refactored to exclusively use `solfunmeme_rdf_utils::rdf_graph::RdfGraph` for all graph operations. This ensures a consistent and simplified approach to RDF data handling, reducing direct dependencies on `sophia` types and promoting a cleaner architecture.

AI agents should primarily interact with the `SemWebContext` and the provided traits for semantic web tasks, relying on the abstractions offered by `solfunmeme_rdf_utils` for low-level RDF manipulations.
