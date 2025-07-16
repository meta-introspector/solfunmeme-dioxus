# `solfunmeme_input_fs`

This crate provides the filesystem input layer for the `solfunmeme-dioxus` project. Its primary responsibility is to read code files from specified directories and convert their content into `CodeChunk` instances for further processing.

## Key Functionality

*   **`read_code_chunks`**: Traverses a given directory, reads the content of code files, and creates `CodeChunk` objects. It populates fields such as `language`, `content`, `line_start`, `line_end`, `content_hash`, `token_count`, `line_count`, `char_count`, and `test_result`.

This crate does not define any new structs or enums; it primarily utilizes the `CodeChunk` data model from `solfunmeme_function_analysis`.