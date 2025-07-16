# `shared_analysis_types`

This crate defines common data structures and types used across various code analysis and processing modules within the Solfunmeme project.

## Purpose

It ensures consistency and interoperability between different components of the analysis pipeline by providing a centralized definition for shared data models, such as code snippets, analyzed functions, and document summaries.

## Core Data Structures

-   **`CodeChunk`**: Represents a segment of code extracted from a source file, including its content, path, and line numbers.
-   **`AnalyzedFunction`**: Stores detailed information about a parsed and analyzed function, such as its name, code snippet, and semantic summary.
-   **`AnalyzedDocument`**: Represents a processed document, containing its file path, code snippets, and analyzed functions.
-   **`AnalyzedToken`**: Describes a token extracted during analysis, including its text, count, and associated multivector.
-   **`ClosestEmojiInfo`**: Provides information about the closest emoji representation for a given multivector, including the emoji character, category, and distance.
-   **`Multivector`**: A generic multivector structure used for representing embeddings in Clifford algebra.
-   **`ExtractedFile`**: Represents a file that has been processed and had its code snippets extracted.
-   **`ProcessingFile`**: Tracks the progress and state of a file undergoing processing.
-   **`TestResult`**: Stores the outcome of a test execution.
-   **`DocumentSummary`**: Provides a summary of an analyzed document, including token counts and languages found.
-   **`ConversationTurn`**: Represents a turn in a conversation, including author, content, and any associated code snippets.
-   **`UploadedFile`**: Describes a file that has been uploaded for processing.
-   **`AnnotatedWord`**: Represents a word with associated semantic annotations, such as emojis and embeddings.
-   **`ProcessingStats`**: Stores statistics about the overall processing pipeline.
-   **`ProcessingError`**: Defines various error types that can occur during processing.
-   **`LanguageConfig`**: Configuration for language-specific processing.

## Usage (Conceptual)

```rust
// use shared_analysis_types::{CodeChunk, AnalyzedFunction};

// fn main() {
//     let chunk = CodeChunk { /* ... */ };
//     let func = AnalyzedFunction { /* ... */ };
// }
```
