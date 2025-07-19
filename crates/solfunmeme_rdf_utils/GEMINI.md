# AI Agent Directives for `solfunmeme_rdf_utils`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `solfunmeme_rdf_utils` crate.

## Core Principles

When working within `solfunmeme_rdf_utils`, AI agents should prioritize:

1.  **Simplified RDF Interaction:** Provide a high-level, simplified API for common RDF operations, abstracting away the complexities of the underlying `sophia` library.
2.  **Consistency and Reusability:** Ensure consistent patterns for RDF graph manipulation and term creation, promoting reusability across the project.
3.  **Semantic Clarity:** Facilitate clear and intuitive representation of semantic data.

## Operational Guidelines

The `solfunmeme_rdf_utils` crate now serves as the primary interface for all RDF-related tasks within the `solfunmeme-dioxus` project. Direct usage of `sophia` types (e.g., `sophia_api::term::SimpleTerm`, `sophia_iri::Iri`) should be minimized in other crates and instead, the helper functions and structs provided here should be utilized.

### Key Components and Usage:

#### 1. `RdfGraph`

The `RdfGraph` struct is the central component for managing RDF data. It wraps `sophia_inmem::graph::FastGraph` and provides simplified methods for adding and querying triples.

**Example Usage:**

```rust
use solfunmeme_rdf_utils::rdf_graph::RdfGraph;
use solfunmeme_rdf_utils::namespace_manager::NamespaceManager;
use solfunmeme_rdf_utils::term_factory;

fn main() -> anyhow::Result<()> {
    let mut graph = RdfGraph::new();

    // Add namespaces
    graph.namespaces.add_namespace("ex", "http://example.org/ontology/")?;
    graph.namespaces.add_namespace("rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#")?;

    // Add a simple triple using string IRIs
    graph.add_triple(
        "http://example.org/subject1",
        "http://example.org/predicate1",
        "http://example.org/object1",
    )?;

    // Add a literal triple
    graph.add_literal_triple(
        "http://example.org/subject2",
        "http://example.org/predicate2",
        "Hello, World!",
        "http://www.w3.org/2001/XMLSchema#string",
    )?;

    // Serialize to Turtle string
    let turtle_string = graph.serialize_to_turtle_string()?;
    println!("{}", turtle_string);

    Ok(())
}
```

#### 2. `NamespaceManager`

The `NamespaceManager` helps in defining and retrieving IRIs and terms based on prefixes, reducing repetitive string concatenation and direct `IriRef` handling.

**Example Usage (within `RdfGraph` or standalone):**

```rust
use solfunmeme_rdf_utils::namespace_manager::NamespaceManager;

fn main() -> anyhow::Result<()> {
    let mut ns = NamespaceManager::new();
    ns.add_namespace("ex", "http://example.org/ontology/")?;
    ns.add_namespace("xsd", "http://www.w3.org/2001/XMLSchema#")?;

    let function_term = ns.get_term("ex", "Function")?;
    let string_type_iri = ns.get_iri("xsd", "string")?;

    println!("Function Term: {:?}", function_term);
    println!("String Type IRI: {:?}", string_type_iri);

    Ok(())
}
```

#### 3. `term_factory`

The `term_factory` module provides convenience functions for creating `sophia_api::term::SimpleTerm` instances from Rust native types, abstracting away the direct `sophia` constructors and lifetime management.

**Available Functions:**

*   `iri_term(iri_string: &str) -> anyhow::Result<SimpleTerm>`: Creates an IRI term.
*   `literal_term(value: &str) -> SimpleTerm`: Creates an untyped literal term.
*   `literal_term_typed(value: &str, datatype_iri: &str) -> anyhow::Result<SimpleTerm>`: Creates a typed literal term.
*   `bnode_term(id: &str) -> SimpleTerm`: Creates a blank node term.

**Example Usage:**

```rust
use solfunmeme_rdf_utils::term_factory;

fn main() -> anyhow::Result<()> {
    let my_iri = term_factory::iri_term("http://example.org/myResource")?;
    let my_literal = term_factory::literal_term("some value");
    let my_typed_literal = term_factory::literal_term_typed("123", "http://www.w3.org/2001/XMLSchema#integer")?;
    let my_bnode = term_factory::bnode_term("b1");

    println!("IRI: {:?}", my_iri);
    println!("Literal: {:?}", my_literal);
    println!("Typed Literal: {:?}", my_typed_literal);
    println!("BNode: {:?}", my_bnode);

    Ok(())
}
```

#### 4. `GraphBuilder`

The `GraphBuilder` provides a fluent interface for constructing `RdfGraph` instances, especially useful for complex graph creation scenarios.

**Example Usage:**

```rust
use solfunmeme_rdf_utils::rdf_graph::GraphBuilder;

fn main() -> anyhow::Result<()> {
    let graph = GraphBuilder::new()
        .with_namespace("ex", "http://example.org/data/")?
        .add_triple("http://example.org/data/subjectA", "http://example.org/data/predicateA", "http://example.org/data/objectA")?
        .add_literal_triple("http://example.org/data/subjectB", "http://example.org/data/predicateB", "42", "http://www.w3.org/2001/XMLSchema#integer")?
        .build();

    let turtle_string = graph.serialize_to_turtle_string()?;
    println!("{}", turtle_string);

    Ok(())
}
```

### Why this simplifies `sophia` usage:

The primary goal of `solfunmeme_rdf_utils` is to reduce the boilerplate and cognitive load associated with direct `sophia` API usage.

*   **Abstracted Term Creation:** Instead of manually constructing `SimpleTerm` with `Iri::new_unchecked` or `SimpleTerm::new_literal_dt`, `term_factory` provides simple functions that handle the underlying `sophia` types and error handling.
*   **Simplified Triple Addition:** `RdfGraph::add_triple` and `RdfGraph::add_literal_triple` now directly accept `&str` for IRIs, removing the need for manual term conversion before adding triples.
*   **Centralized Namespace Management:** `NamespaceManager` ensures consistent IRI generation and avoids hardcoding namespace URIs throughout the codebase.
*   **Fluent Graph Construction:** The `GraphBuilder` pattern allows for a more readable and chainable way to build RDF graphs.

### Refactoring Impact:

The refactoring efforts involved updating several crates (e.g., `rdf_processing_lib`, `semweb_lib`, `solfunmeme_lean4`) to leverage this new simplified API. This has resulted in cleaner code, fewer direct `sophia` imports outside of `solfunmeme_rdf_utils`, and a more maintainable RDF layer for the project.

## Lessons Learned: Sophia 0.8.0 API Migration

Migrating to Sophia 0.8.0 introduced significant breaking changes, primarily due to the adoption of Generic Associated Types (GATs) and a redesign of the core API. Here are key lessons learned and tips for working with Sophia 0.8.0+:

1.  **`SimpleTerm` Construction:**
    *   **Old Way (0.7.x):** Direct constructors like `SimpleTerm::new_iri()`, `SimpleTerm::new_literal_untyped()`, `SimpleTerm::new_literal_dt()`, `SimpleTerm::new_bnode()`. These are **removed** in 0.8.0.
    *   **New Way (0.8.0+):** Use `SimpleTerm::new_iri(Iri::new(...)?)`, `SimpleTerm::new_literal_untyped(...)`, `SimpleTerm::new_literal_dt(...)`, `SimpleTerm::new_bnode(BNode::new(...))`. Note that `Iri` and `BNode` are now distinct types that need to be constructed first.

2.  **IRI and BNode Imports:**
    *   **Old Way (0.7.x):** `sophia_iri::Iri`, `sophia_api::term::BNode`.
    *   **New Way (0.8.0+):** `sophia_api::iri::Iri` and `sophia_api::term::BNode` (for the struct, not the enum variant). Ensure you import `sophia_api::iri::Iri` for IRI construction and `sophia_api::term::BNode` for blank node construction.

3.  **Graph Query Methods:**
    *   **Old Way (0.7.x):** Specific methods like `graph.triples_with_sp(s, p)` or `graph.triples_with_po(p, o)`.
    *   **New Way (0.8.0+):** Use `graph.triples_matching(subject_matcher, predicate_matcher, object_matcher)`. This method is more flexible and replaces many specific query methods.

4.  **Parser and Serializer Instantiation:**
    *   **Old Way (0.7.x):** `TurtleParser::new()`, `TurtleSerializer::new()`.
    *   **New Way (0.8.0+):** Parsers and serializers often require arguments in their `new()` constructors (e.g., `TurtleParser::new(content.as_bytes(), None)`). Always check the specific `new()` signature in the 0.8.0+ documentation.

5.  **Asynchronous Operations (JSON-LD):**
    *   `JsonLdParser::parse_json()` returns a `Future`. You must `await` the result and then call `collect_triples()` on the awaited value. This is a common pattern for asynchronous operations in Rust.

6.  **StreamSerializer `flush()` method:**
    *   The `flush()` method on `TurtleSerializer` (and other serializers implementing `StreamSerializer`) is still available, but ensure `use sophia_api::serializer::StreamSerializer;` is present.

By centralizing `sophia` interactions within `solfunmeme_rdf_utils`, we aim to minimize the impact of future upstream API changes on the rest of the codebase. Always refer to the official `sophia` 0.8.0+ documentation for the most accurate and up-to-date API usage.