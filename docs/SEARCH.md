# Code Search and Indexing

The Solfunmeme project employs a sophisticated, hybrid search and indexing system to enable comprehensive code analysis, semantic search, and "vibe-based" retrieval. This system is built on a combination of traditional text search, modern vector search, and a unique emoji-based indexing strategy.

## Core Architecture: A Hybrid Approach

The search functionality is built on three pillars:

1.  **Tantivy-based Text Search:** At its core, the system uses the `tantivy` crate to provide a fast, efficient, and feature-rich full-text search engine. This allows for precise keyword and phrase matching within the codebase.

2.  **Vector-based Semantic Search:** To enable searching for code based on its meaning and functionality (rather than just keywords), the system uses a vector search approach. This involves:
    *   **Embeddings:** Using `rust-bert` to generate high-dimensional vector embeddings of code snippets.
    *   **Vector Database:** The architecture is designed to integrate with a vector database like Qdrant or LanceDB for efficient similarity searches. (Note: This integration is planned as a next step).

3.  **Emoji-based "Vibe" Search:** A unique feature of this project is the ability to search for code based on its "vibe" or "personality." This is achieved by mapping code constructs and patterns to emojis, allowing for a more intuitive and context-aware search experience.

## Key Crates and Their Roles

The search and indexing functionality is distributed across several key crates:

*   **`crates/solfunmeme_indexer`:** This crate is the main entry point for the indexing process. It uses `solfunmeme_input_fs` to read files, `solfunmeme_search_tantivy` to manage the index, and `solfunmeme_tantivy_report` to generate reports.

*   **`crates/solfunmeme_search_tantivy`:** This is the heart of the text search functionality. It defines the Tantivy schema, manages the index, and provides the core search logic.

*   **`crates/solfunmeme_tantivy_report`:** This crate provides tools for generating reports and analyzing the data stored in the Tantivy index.

*   **`crates/solfunmeme_tools`:** This crate contains a collection of command-line tools for interacting with the search and indexing system.

## Command-Line Tools

The following CLI tools are available in `crates/solfunmeme_tools` for managing the search and indexing process:

*   **`full_indexer_cli`:** The primary tool for indexing the codebase. It reads source files, chunks them, and adds them to the Tantivy index.
    *   **Usage:** `cargo run --bin full_indexer_cli <directories_to_index>`
    *   **Features:**
        *   Automatic schema negotiation and index recreation if corruption is detected.
        *   `--overwrite` flag for clean re-indexing.
        *   `--sandbox` flag for testing in a temporary directory.

*   **`plan_cli`:** A helper tool to estimate the computational cost of a full indexing operation by analyzing the number of files, lines, and code chunks.
    *   **Usage:** `cargo run --bin plan_cli <directories_to_plan>`

*   **`tantivy_analyzer_cli`:** A tool for analyzing the Tantivy index itself.

*   **`recursive_index_cli`:** This tool provides an alternative, "recursive" indexing strategy.

*   **`zos` (Planned):** The long-term vision is to consolidate all of these tools into a single, unified `zos` command-line interface for a streamlined user experience.

## Future Development

The search and indexing system is under active development. Key future enhancements include:

*   **Vector Database Integration:** Completing the integration with Qdrant or LanceDB to enable full semantic search capabilities.
*   **Hybrid Search Engine:** Building a sophisticated fusion layer that combines the results from text, vector, and emoji searches into a single, ranked list.
*   **Unified `zos` CLI:** Consolidating all search and indexing commands into the `zos` tool.
