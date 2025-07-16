# `bm25_plugin`

This crate provides a wrapper around the `bm25` library, offering functionalities for calculating BM25 scores for document ranking.

## Purpose

The primary purpose of this plugin is to enable efficient text search and relevance ranking within the Solfunmeme ecosystem, leveraging the BM25 algorithm.

## Core Functionalities

-   **Calculate Score**: Compute the BM25 score for a given query and document.
-   **Search**: Perform a search against a corpus and return ranked results.

## Usage (Conceptual)

```rust
use bm25_plugin::Bm25Plugin;

fn main() {
    let corpus = vec![
        vec!["hello".to_string(), "world".to_string()],
        vec!["foo".to_string(), "bar".to_string(), "hello".to_string()],
    ];
    let plugin = Bm25Plugin::new(corpus).expect("Failed to create plugin");
    let query = vec!["hello".to_string()];
    let results = plugin.search(&query);
    println!("Search results (conceptual): {:?}", results);
}
```
