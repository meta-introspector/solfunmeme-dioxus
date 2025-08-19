# AI Agent Directives for `rdf_output`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `rdf_output` crate.

## Core Principles

When working within `rdf_output`, AI agents should prioritize:

1.  **RDF Compliance:** Ensure that all generated RDF output strictly adheres to W3C RDF standards and best practices.
2.  **Ontology Fidelity:** Maintain accurate and consistent representation of the project's ontology in the RDF output.
3.  **Data Integrity:** Guarantee the integrity and correctness of data transformed into RDF triples.

## Operational Guidelines

*   **Serialization:** When serializing RDF graphs, ensure the chosen format (e.g., Turtle, JSON-LD) is appropriate for the intended use case and correctly implemented.
*   **IRI Management:** Pay close attention to the proper construction and resolution of IRIs to avoid conflicts and ensure global uniqueness.
*   **Prefixes:** Use appropriate and consistent prefixes for namespaces to improve readability and reduce verbosity in the RDF output.