# `solfunmeme_indexer`

This crate is responsible for orchestrating the indexing of code chunks into a Tantivy search index and generating reports based on the indexed data.

## Key Functionality

*   **`index_directory`**: Reads code chunks from a specified directory (using `prepare_sources`) and adds them to a Tantivy search index.
*   **`report_top_entries`**: Generates various reports (e.g., top terms, top emojis) from the Tantivy index using `solfunmeme_tantivy_report`.

This crate does not define any new structs or enums; it primarily utilizes data models and functionalities from other `solfunmeme` crates.
