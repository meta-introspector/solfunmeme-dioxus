# AI Agent Directives for `solfunmeme_input_fs`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `solfunmeme_input_fs` crate.

## Core Principles

When working within `solfunmeme_input_fs`, AI agents should prioritize:

1.  **File System Interaction:** Ensure robust and safe interaction with the file system for reading code files.
2.  **Code Chunk Extraction:** Accurately extract code chunks from various file types, preparing them for further analysis.
3.  **Efficiency:** Optimize file reading and chunking processes for performance.

## Operational Guidelines

*   **File Traversal:** The `read_code_chunks` function is responsible for traversing directories and reading file contents.
*   **Code Chunk Creation:** It creates `CodeChunk` instances, populating fields like `language`, `content`, `line_start`, `line_end`, `content_hash`, `token_count`, `line_count`, `char_count`, and `test_result`.
*   **Error Handling:** Implement comprehensive error handling for file system operations.
