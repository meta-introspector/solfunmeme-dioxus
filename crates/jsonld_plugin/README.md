# `jsonld_plugin`

This crate provides generic input/output functionalities for JSON-LD (JavaScript Object Notation for Linked Data) files.

## Purpose

It serves as a utility for reading JSON-LD data from files and writing JSON-LD data to files, enabling seamless data exchange and persistence for semantic data within the Solfunmeme project.

## Core Functionalities

-   **Read JSON-LD**: Read JSON-LD data from a specified file path.
-   **Write JSON-LD**: Write JSON-LD data to a specified file path.

## Usage (Conceptual)

```rust
use jsonld_plugin::{read_jsonld_from_file, write_jsonld_to_file};
use serde_json::json;

fn main() {
    let file_path = "./data.jsonld";
    let data = json!({
        "@context": "http://schema.org/",
        "name": "Example",
        "description": "A test JSON-LD document"
    });

    // Example: Write JSON-LD to a file
    // write_jsonld_to_file(file_path, &data).expect("Failed to write JSON-LD");
    // println!("JSON-LD written to {}", file_path);

    // Example: Read JSON-LD from a file
    // let read_data = read_jsonld_from_file(file_path).expect("Failed to read JSON-LD");
    // println!("JSON-LD read: {:#?}", read_data);
}
```
