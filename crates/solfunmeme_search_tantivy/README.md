# `solfunmeme_search_tantivy`

This crate provides full-text search capabilities for code chunks using the Tantivy search engine. It manages the Tantivy index, allowing for the addition of code chunks and the execution of search queries.

## Key Data Structures

### Structs:

*   **`SearchResult`**: Represents a single search result from the Tantivy index, including details like language, content, line numbers, content hash, token/line/character counts, test result, and a relevance score.
*   **`SearchIndex`**: Manages the Tantivy index, providing methods to create/open an index, add `CodeChunk` documents, commit changes, and perform searches.
*   **`BagOfWords`**: A utility for basic text analysis, used for counting word frequencies and calculating text similarity.

This crate primarily utilizes the `CodeChunk` data model from `solfunmeme_function_analysis` for indexing.
