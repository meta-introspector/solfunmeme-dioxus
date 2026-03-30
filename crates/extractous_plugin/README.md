# `extractous_plugin`

This crate provides a wrapper around the `extractous` library, offering functionalities for extracting clean text content from HTML documents.

## Purpose

It enables the Solfunmeme project to process web content by stripping away HTML tags and extracting only the human-readable text, which can then be used for further analysis or indexing.

## Core Functionalities

-   **Extract Text**: Takes an HTML string as input and returns its plain text representation.

## Usage (Conceptual)

```rust
use extractous_plugin::ExtractousPlugin;

fn main() {
    let plugin = ExtractousPlugin::new().expect("Failed to create plugin");
    let html_content = "<html><body><h1>Hello</h1><p>World</p></body></html>";
    let text = plugin.extract_text(html_content).expect("Failed to extract text");
    println!("Extracted text: {}", text);
}
```
