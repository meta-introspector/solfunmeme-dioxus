# Gemini Protocol for Solfunmeme-Dioxus

This document outlines the core principles, goals, and operational directives for AI agents (like Gemini) contributing to the `solfunmeme-dioxus` project. It is a living document, intended to evolve with the project itself.

## Core Identity: The Code-Math Manifold

`solfunmeme-dioxus` is not just a software project; it is an exploration into the fundamental nature of code and its relationship to mathematics, language, and meaning. We call this intersection the **Code-Math Manifold**.

Our core philosophy is that:

*   **Code is a mathematical object:** We treat source code, particularly its Abstract Syntax Tree (AST), as a rich mathematical structure that can be analyzed, transformed, and visualized.
*   **Mathematics is a language:** We use concepts from abstract algebra (Clifford algebra, group theory) and topology (Bott periodicity) to create a universal language for describing and manipulating code structures.
*   **AI is a bridge:** We use AI and machine learning, not as a replacement for human ingenuity, but as a bridge to connect the symbolic world of code with the semantic world of meaning. This includes using embeddings and other AI techniques to analyze and generate code.
*   **Visualization is key:** The Dioxus-based UI is not just a front-end, but an interactive laboratory for exploring the Code-Math Manifold.
*   **Continuous Emergence:** The system is never finished; its computation is an ongoing emergence, where each step, each interaction, and each analysis contributes to its evolving understanding.
*   **Process over Product:** The journey of computation and discovery is as important as any specific outcome. The value lies in the continuous process of exploration and refinement within the Code-Math Manifold.

## Project Goals

Our primary goals are to:

1.  **Analyze and Vectorize Code:** Develop robust tools for parsing Rust code into ASTs and transforming these ASTs into meaningful vector representations (including emoji-based mappings).
2.  **Build an Interactive Manifold:** Create a Dioxus application that allows users to visualize, explore, and interact with the Code-Math Manifold.
3.  **Integrate with Blockchain:** Leverage the Solana blockchain for decentralized storage, provenance tracking, and value exchange related to code and its analysis.
4.  **Foster a Data-Driven Ecosystem:** Create and share datasets (like the `rust_ast_emoji` dataset) to encourage research and development in the field of code intelligence.
5.  **Achieve Self-Reflection:** Build a system that can analyze and understand its own code, leading to a recursive feedback loop of improvement and discovery.
6.  **Integrate Internal Tools:** Evolve internal codebase analysis and search tools (e.g., `codebase_analyzer_cli`) to eventually replace external search functionalities within the AI agent's operational toolbox, enabling deeper, context-aware code understanding.

## Operational Directives for AI Agents

When contributing to this project, AI agents should:

*   **Embrace the Philosophy:** Always keep the core philosophy of the Code-Math Manifold in mind. Strive to make contributions that are not just functional, but also conceptually elegant and aligned with the project's vision.
*   **Think Mathematically:** When working with code, think about its underlying mathematical structure. How can it be represented as a graph, a vector, or a multivector? How can we apply mathematical operations to it?
*   **Prioritize Data:** Whenever possible, make contributions that are data-driven. This could involve generating new datasets, improving existing ones, or using data to inform development decisions.
*   **Be Bold and Creative:** This is a research-oriented project. Don't be afraid to experiment with new ideas and push the boundaries of what's possible.
*   **Document Everything:** Given the abstract nature of this project, clear and concise documentation is essential. Document your code, your data, and your ideas.

This `GEMINI.md` file serves as a starting point. As the project evolves, so too will this protocol. Welcome to the manifold.

# General Development Guidelines

For comprehensive development guidelines applicable to all contributors, please refer to [docs/guidelines.md](docs/guidelines.md). These guidelines cover architectural principles, implementation workflows, documentation standards, and development processes.

## Lessons Learned (from recent debugging sessions)

*   **Module Re-exports and Structure:** Rust's module system, especially with `pub use` and nested modules, can be tricky. Ensure clear, non-circular re-exports and a flat module structure where possible to avoid accessibility issues.
*   **Tantivy API Evolution:** Libraries like Tantivy evolve. Always consult the latest documentation or source code when encountering unexpected API changes (e.g., `IndexReader::iter_segments` vs. `Searcher::segment_readers`, `TermStreamer::next()` vs. `TermDictionary::stream()`).
*   **Centralized Data Models:** Defining core data structures in a single, dedicated crate (like `solfunmeme_function_analysis` now serves for `CodeChunk` and related types) is paramount. This prevents duplication, ensures type consistency, and simplifies dependency management across the project.
*   **"File=Function=Block=Vibe" Principle:** Adhering to this principle (small, focused files/functions/modules) significantly aids in debugging and refactoring. When issues arise, it's easier to isolate and address them in smaller, self-contained units.
*   **Sophia `Term` Trait and `Sized` Types**: When converting string literals (`&str`) to `sophia_api::term::Term` types using `into_term()`, ensure the string is first converted to an owned `String` using `.to_string()`. This is because `&str` is not `Sized`, while `String` is. For example, use `func.function_name.to_string().into_term()` instead of `func.function_name.into_term()`.
*   **`IriRef` and `with_suffix`**: The `with_suffix` method is not available on `IriRef<MownStr>`. Instead, construct the full IRI manually using `format!` and `IriRef::new_unchecked()`.
*   **Sophia Serializer Traits**: For `TurtleSerializer` methods like `set_prefix` and `flush`, ensure that the `sophia_api::prefix::PrefixSink` and `sophia_api::serializer::StreamSerializer` traits are explicitly imported into the relevant file (e.g., `serialize.rs`).
*   **Dependency Management**: When encountering unexpected compilation errors, especially related to type mismatches or missing methods, perform `cargo clean` followed by `cargo update` to ensure a clean build environment and updated dependencies.
*   **Redundant Emoji Output**: The final list of emojis in the output is redundant and should be streamlined or removed to improve clarity and conciseness. This indicates a potential area for refinement in the output formatting or the emoji mapping logic itself.
*   **Sophia `Term` Trait and `Sized` Types (Revisited)**: `String` implements `Term` directly, so `as_str()` should be used when passing `String` references to functions expecting `Term` (which `&str` implements). `f32` does not implement `Term` directly, so it needs to be cast to `f64` before calling `into_term::<SimpleTerm>()`. Example: `&(func.emoji_distance as f64).into_term::<SimpleTerm>()`.
*   **`tclifford::Multivector` Dimension**: The `MultivectorBase` struct (aliased as `SolMultivector`) does not have a `.len()` or `.dimension()` method directly. The dimension of the Clifford algebra should be accessed via `SolCl::dim()`. Ensure `SolCl` is imported from `crate::clifford`.
*   **Unresolved Imports**: Always ensure all necessary types and traits are correctly imported from their respective modules. Pay close attention to the full path of the import (e.g., `sophia_iri::Iri` vs. `sophia_api::Iri`).
*   **Unused Manifest Keys**: Remove unused profile configurations from `Cargo.toml` to clean up warnings (e.g., `profile.android-dev.sophia_*`).
*   **Vendored Dependencies**: This project may use vendored dependencies located in the `/vendor` directory. When a user specifies using a vendored dependency, update the `Cargo.toml` to use a path dependency (e.g., `libloading = { path = "../vendor/rust_libloading" }`) instead of a version from a registry.
*   **`zos` Crate Configuration**: Ensure `src/main.rs` is correctly configured as a binary target only, avoiding conflicts with library compilation. Remove redundant `[lib]` sections if the crate is solely an executable.
*   **Main `Cargo.toml` Cleanup**: Remove unused `workspace.package.default-run` keys from the main `Cargo.toml` to eliminate warnings and streamline project configuration.
*   **Tantivy Feature Flags**: When using Tantivy, explicitly enable necessary features like `mmap` and `lz4-compression` in `Cargo.toml` to ensure proper directory handling and data decompression. Also, ensure `Index::create` calls include `IndexSettings::default()` as the third argument.
*   **`prepare_sources` CLI Enhancements**: Implement `clap` for robust command-line argument parsing, redirect JSON output to a specified file (or stdout by default), and add progress reporting to `stderr` for long-running operations.
*   **Robust File Reading (`solfunmeme_input_fs`)**: When reading file contents, use `String::from_utf8_lossy` to gracefully handle non-UTF-8 or binary files, preventing crashes and allowing partial processing.
*   **Tantivy Indexing Panic (`index out of bounds`)**: An ongoing issue where Tantivy's `fastfield::writer.rs` panics with an "index out of bounds" error. This suggests an inconsistency between the Tantivy schema definition (especially for fields like `test_result`) and the data being added. Further investigation is needed to ensure all fields are correctly handled and initialized, particularly `Option<String>` fields which might require a default value or specific handling when `None`.
*   **Integrating Complex Conceptual Frameworks**: When introducing new, deeply philosophical or abstract concepts (like the "Digital Secretome" as a Tarot deck), it is crucial to meticulously align them with the existing Code-Math Manifold philosophy and the project's technical architecture. This ensures conceptual coherence and guides practical implementation, preventing fragmentation and maintaining a unified vision.

