# `json_ld_plugin`

This crate provides a wrapper around the `json-ld` library, offering functionalities for processing JSON-LD (JavaScript Object Notation for Linked Data).

## Purpose

It enables the Solfunmeme project to work with Linked Data by providing tools for JSON-LD compaction, expansion, flattening, and framing, facilitating semantic data interoperability.

## Core Functionalities

-   **Compact**: Compacts a JSON-LD document.
-   **Expand**: Expands a JSON-LD document.
-   **Flatten**: Flattens a JSON-LD document.
-   **Frame**: Frames a JSON-LD document according to a specified frame.

## Usage (Conceptual)

```rust
use json_ld_plugin::JsonLdPlugin;
use serde_json::json;

#[tokio::main]
async fn main() {
    let doc = json!({
        "@context": "http://schema.org/",
        "@id": "http://example.org/people/markus",
        "name": "Markus",
        "homepage": "http://markus.example.org/"
    });
    let context = json!({
        "name": "http://schema.org/name"
    });
    let options = json_ld::Options::default();

    // Example: Compact a JSON-LD document
    // let compacted = JsonLdPlugin::compact(doc.clone(), context.clone(), options.clone()).await.expect("Failed to compact");
    // println!("Compacted: {:#?}", compacted);

    // Example: Expand a JSON-LD document
    // let expanded = JsonLdPlugin::expand(doc.clone(), options.clone()).await.expect("Failed to expand");
    // println!("Expanded: {:#?}", expanded);
}
```
