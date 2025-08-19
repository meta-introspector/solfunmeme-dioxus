# `rdf_processing_lib`

This crate provides utilities for processing and manipulating RDF (Resource Description Framework) data.

## Purpose

It enables the Solfunmeme project to work with semantic graphs, allowing for operations such as parsing, querying, and serializing RDF data, which is crucial for managing and understanding the project's knowledge base.

## Core Functionalities

-   **Parse RDF**: Parse RDF data from various formats (e.g., Turtle, N-Triples) into an in-memory graph.
-   **Query RDF**: Query RDF graphs using SPARQL-like patterns.
-   **Serialize RDF**: Serialize RDF graphs into different formats.

## Usage (Conceptual)

```rust
// use rdf_processing_lib::{RdfProcessor, RdfFormat};

// fn main() {
//     let processor = RdfProcessor::new();
//     let turtle_data = "@prefix ex: <http://example.org/> . ex:subject ex:predicate ex:object .";

//     // Example: Parse RDF data
//     // let graph = processor.parse_data(turtle_data, RdfFormat::Turtle).expect("Failed to parse RDF");

//     // Example: Query the graph
//     // let results = processor.query_graph(&graph, "SELECT ?s ?p ?o WHERE { ?s ?p ?o }").expect("Failed to query");
//     // println!("Query results (conceptual): {:?}", results);
// }
```
