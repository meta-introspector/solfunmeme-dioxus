# `solfunmeme_function_analysis`

This crate serves as the centralized repository for core data models and analysis logic related to code. It defines the fundamental structures used across the `solfunmeme-dioxus` project for representing code chunks, analyzed functions, and various processing statistics.

## Key Data Structures

### Structs:

*   **`ClosestEmojiInfo`**: Represents information about the closest emoji to a given multivector, including the emoji symbol, its category, and the distance.
*   **`AnalyzedFunction`**: Stores detailed information about an analyzed Rust function, including its name, code snippet, semantic summary, file path, multivector string, sieve address, closest emojis, and an optional orbital path.
*   **`FunctionInfo`**: Holds basic information about a function, similar to `AnalyzedFunction` but with a single closest emoji.
*   **`AnalyzedDocument`**: Represents an analyzed document, containing its file path, extracted code snippets, text chunks, and analyzed function snippets.
*   **`AnalyzedToken`**: Describes an analyzed token, including the token string, its count, multivector string, and an optional orbital path.
*   **`CodeChunk`**: Defines a chunk of code with its language, content, line range, content hash, token/line/character counts, and an optional test result.
*   **`Multivector`**: A simplified representation of a multivector with a scalar and a 3-element vector.
*   **`ExtractedFile`**: Represents a file that has been extracted, containing its name, a list of code snippets, and the total number of lines.
*   **`ProcessingFile`**: Stores information about a file currently being processed, including its name, progress, total lines, current content, and an optional document summary.
*   **`TestResult`**: Encapsulates the result of a test, indicating whether it passed, an error message if any, execution time, and output.
*   **`DocumentSummary`**: Provides a summary of a document, including total turns, code snippets, tokens, languages found, and content hashes.
*   **`ConversationTurn`**: Represents a turn in a conversation, including the author, content, and associated code snippets.
*   **`UploadedFile`**: Describes an uploaded file, including its name, contents, generated program, an optional summary, and a zip URL.
*   **`AnnotatedWord`**: Represents a word with associated semantic information, including primary/secondary emojis, Wikidata link, embedding, and multivector.
*   **`ProcessingStats`**: Stores statistics about the processing of files, such as the number of files processed, snippets extracted, lines processed, processing time, and languages detected.
*   **`LanguageConfig`**: Defines configuration for a programming language, including its name, file extension, comment prefix, testing support, and syntax highlighter.

### Enums:

*   **`ProcessingError`**: Enumerates possible errors during processing, such as file read errors, parse errors, test execution errors, and invalid format errors.