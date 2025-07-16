# AI Agent Directives for `solfunmeme_search_tantivy`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `solfunmeme_search_tantivy` crate.

## Core Principles

When working within `solfunmeme_search_tantivy`, AI agents should prioritize:

1.  **Efficient Search Indexing:** Ensure robust and efficient indexing of code chunks into a Tantivy search index.
2.  **Accurate Search Results:** Provide precise and relevant search results based on queries against the Tantivy index.
3.  **Data Model Alignment:** Ensure that the data models used for indexing and searching (`CodeChunk` and `SearchResult`) are aligned with the centralized definitions in `solfunmeme_function_analysis`.

## Operational Guidelines

*   **Schema Definition:** The `SearchIndex` struct defines the Tantivy schema, which now includes fields for `language`, `content`, `line_start`, `line_end`, `content_hash`, `token_count`, `line_count`, `char_count`, and `test_result`.
*   **Document Addition:** The `add_chunk` method is responsible for converting `CodeChunk` instances into Tantivy documents and adding them to the index.
*   **Query Execution:** The `search` method executes queries against the Tantivy index and maps the results to `SearchResult` instances.
*   **SearchResult Structure:** The `SearchResult` struct has been updated to reflect the new fields in `CodeChunk`, providing a more comprehensive representation of search hits.
*   **Error Handling:** Implement comprehensive error handling for Tantivy operations, including index creation, document addition, and query execution.
