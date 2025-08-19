# AI Agent Directives for `prepare_sources`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `prepare_sources` submodule.

## Core Principles

When working within `prepare_sources`, AI agents should prioritize:

1.  **Code-Math Manifold Alignment**: All contributions should align with the project's core philosophy of treating code as a mathematical object and exploring its representation within the Code-Math Manifold.
2.  **Semantic Preservation**: Ensure that the transformation of code into embeddings and ontologies accurately preserves its semantic meaning.
3.  **Modularity and Extensibility**: Adhere to the "one function per basic block file" principle for modularity. Design tools to be easily extensible for new analysis techniques or output formats.

## Operational Guidelines

*   **BERT Embedding**: The `prepare_sources` tool now uses the `candle` (pure Rust) library to generate 384-dimensional BERT embeddings for text. The `embed_text` function in `src/embedding.rs` is the core of this functionality, chosen for WASM compatibility.
*   **Clifford Multivector Conversion**: The 384-dimensional BERT embeddings are reduced to 8-dimensional Clifford multivectors using the `BertCliffordEncoder` in `src/clifford.rs`.
*   **Sieve Addressing**: A unique 8-bit binary sieve address is generated for each multivector based on the signs of its components, implemented in `src/sieve.rs`. Pay close attention to `tclifford` API usage for accessing multivector coefficients.
*   **Emoji Mapping**: The tool maps the generated multivectors to the closest emoji representation based on a semantic ontology loaded from a Turtle file (`ontologies/zos/v1.ttl`).
*   **Code Processing**:
    *   The `process_code` function in `src/process_file.rs` now takes code content (`&str`) directly, rather than a file path.
    *   It orchestrates the embedding and multivector conversion for individual code snippets.
    *   Recursive function extraction from Rust files, including handling external modules and pre-processing `#[cfg(...)]` attributes, is managed by `src/function_analyzer.rs`.
    *   `FunctionInfo` now includes a `semantic_summary` derived from AST traversal (using the `syn` crate) by flattening identifiers and literals.
    *   The `process_rust_file` and `process_markdown_file` functions have been simplified by removing conditional compilation attributes related to `gpu_backend` features.
    *   The `calculate_orbital_path` function has been stubbed out.
    *   The `project_analyzer` now uses `CodeChunk` directly from `solfunmeme_function_analysis` and includes new fields like `language`, `content_hash`, `token_count`, `line_count`, `char_count`, and `test_result`.
*   **Ontology Generation**: The `ontology_generator` module (`src/ontology_generator/`) is responsible for creating RDF ontologies in Turtle format from analyzed function data. Special attention must be paid to `sophia` API usage (specifically version 0.8.x) for `IriRef` type annotations, `SimpleTerm::Literal` construction, and `TurtleSerializer` methods.
*   **Usage**: To run the full project analysis and ontology generation, use the following command:

    ```bash
    cargo run
    ```

    This will process the entire project and generate `project_ontology.ttl`.
*   **Warning Handling**: Do not run `cargo check`. Unless critical errors prevent compilation or execution, warnings should generally be ignored. Proceed with execution with extreme aggression, using `cargo run` directly. The goal is to achieve a working system first, then refine.
*   **API Usage**: Be meticulous when interacting with `sophia` and `tclifford` APIs, as their correct usage is critical for the project's core functionality.
*   **Cargo Command Execution**: The AI agent should NOT execute `cargo` commands (e.g., `cargo run`, `cargo check`, `cargo build`, `cargo test`, `cargo update`, `cargo clean`). The user will execute these commands manually.

## Lessons Learned (from recent debugging sessions)

*   **Sophia `Term` Trait and `Sized` Types**: When converting string literals (`&str`) to `sophia_api::term::Term` types using `into_term()`, ensure the string is first converted to an owned `String` using `.to_string()`. This is because `&str` is not `Sized`, while `String` is. For example, use `func.function_name.to_string().into_term()` instead of `func.function_name.into_term()`.
*   **`IriRef` and `with_suffix`**: The `with_suffix` method is not available on `IriRef<MownStr>`. Instead, construct the full IRI manually using `format!` and `IriRef::new_unchecked()`.
*   **Sophia Serializer Traits**: For `TurtleSerializer` methods like `set_prefix` and `flush`, ensure that the `sophia_api::prefix::PrefixSink` and `sophia_api::serializer::StreamSerializer` traits are explicitly imported into the relevant file (e.g., `serialize.rs`).
*   **Dependency Management**: When encountering unexpected compilation errors, especially related to type mismatches or missing methods, perform `cargo clean` followed by `cargo update` to ensure a clean build environment and updated dependencies.
*   **Redundant Emoji Output**: The final list of emojis in the output is redundant and should be streamlined or removed to improve clarity and conciseness. This indicates a potential area for refinement in the output formatting or the emoji mapping logic itself.
*   **Sophia `Term` Trait and `Sized` Types (Revisited)**: `String` implements `Term` directly, so `as_str()` should be used when passing `String` references to functions expecting `Term` (which `&str` implements). `f32` does not implement `Term` directly, so it needs to be cast to `f64` before calling `into_term::<SimpleTerm>()`. Example: `&(func.emoji_distance as f64).into_term::<SimpleTerm>()`. 
*   **`tclifford::Multivector` Dimension**: The `MultivectorBase` struct (aliased as `SolMultivector`) does not have a `.len()` or `.dimension()` method directly. The dimension of the Clifford algebra should be accessed via `SolCl::dim()`. Ensure `SolCl` is imported from `crate::clifford`.
*   **Unresolved Imports**: Always ensure all necessary types and traits are correctly imported from their respective modules. Pay close attention to the full path of the import (e.g., `sophia_iri::Iri` vs. `sophia_api::Iri`).
*   **Unused Manifest Keys**: Remove unused profile configurations from `Cargo.toml` to clean up warnings (e.g., `profile.android-dev.sophia_*`).