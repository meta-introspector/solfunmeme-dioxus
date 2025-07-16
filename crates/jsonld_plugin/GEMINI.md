# AI Agent Directives for `jsonld_plugin`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `jsonld_plugin` crate.

## Core Principles

When working within `jsonld_plugin`, AI agents should prioritize:

1.  **JSON-LD Compliance:** Ensure that all JSON-LD processing (parsing, serialization) strictly adheres to the JSON-LD 1.1 specification.
2.  **Semantic Interoperability:** Facilitate seamless conversion between JSON-LD and other RDF formats to enhance semantic interoperability.
3.  **Data Transformation Integrity:** Maintain the integrity and correctness of data during JSON-LD transformations (e.g., compacting, expanding, flattening).

## Operational Guidelines

*   **Context Handling:** Pay close attention to the proper handling of JSON-LD contexts to ensure correct interpretation of terms and IRIs.
*   **Error Handling:** Implement robust error handling for all JSON-LD operations, providing clear feedback in case of failures.
*   **Security:** Be mindful of potential security implications when processing external JSON-LD contexts or data.