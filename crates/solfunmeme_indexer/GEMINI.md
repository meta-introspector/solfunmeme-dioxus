# AI Agent Directives for `solfunmeme_indexer`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `solfunmeme_indexer` crate.

## Core Principles

When working within `solfunmeme_indexer`, AI agents should prioritize:

1.  **Efficient Code Indexing:** Ensure robust and efficient indexing of code chunks into a Tantivy search index.
2.  **Data Flow Integrity:** Maintain the integrity of data as it flows from source code through `prepare_sources` to the Tantivy index.
3.  **Reporting Accuracy:** Provide accurate and insightful reports based on the indexed data.

## Operational Guidelines

*   **Indexing Process:** The `index_directory` function orchestrates the indexing process by calling `prepare_sources` to get code chunks and then adding them to the Tantivy index.
*   **Report Generation:** The `report_top_entries` function generates reports based on indexed data, utilizing `solfunmeme_tantivy_report`.
*   **Error Handling:** Implement comprehensive error handling for file operations, data serialization/deserialization, and Tantivy interactions.
*   **CLI Interface:** The crate provides a CLI for indexing and reporting. Ensure the CLI arguments and commands are well-defined and user-friendly.
