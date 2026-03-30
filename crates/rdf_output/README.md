# `rdf_output`

This crate is responsible for converting structured data, such as `CodeChunk`s, into RDF (Resource Description Framework) triples.

## Purpose

It enables the semantic representation of code and other project data, facilitating interoperability with Linked Data principles and allowing for advanced querying and reasoning over the codebase.

## Core Functionalities

-   **Code Chunks to RDF**: Transforms `CodeChunk` instances into an RDF graph, representing code elements and their properties as triples.

## Usage (Conceptual)

```rust
use rdf_output::code_chunks_to_rdf;
use shared_analysis_types::CodeChunk;

fn main() {
    let chunk = CodeChunk {
        path: "src/main.rs".to_string(),
        content: "fn main() {}\n".to_string(),
        emoji: "📄".to_string(),
        line_start: 1,
        line_end: 2,
        chunk_type: "code".to_string(),
    };

    // Example: Convert a CodeChunk to an RDF graph
    // let graph = code_chunks_to_rdf(vec![chunk]).expect("Failed to convert to RDF");
    // println!("Generated RDF graph (conceptual): {:#?}", graph);
}
```
