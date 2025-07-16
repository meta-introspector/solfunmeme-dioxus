# Algorithm Documentation: Rust Code to RDF Ontology Generation

This document details the algorithm employed by the `prepare_sources` tool to analyze Rust source code, extract semantic information, transform it into a Clifford algebra representation, map it to emojis, and finally generate an RDF ontology.

## 1. Overall Purpose

The primary goal of this algorithm is to bridge the gap between Rust source code and a semantic, machine-readable representation in the form of an RDF (Resource Description Framework) ontology. This ontology captures the structural and semantic essence of Rust functions, their relationships to files, and their conceptual proximity to a predefined emoji-based semantic space.

## 2. Key Steps and Components

The algorithm proceeds through several distinct stages, leveraging various Rust crates for specialized tasks:

### 2.1. Project Analysis (`project_analyzer.rs`)

This module orchestrates the entire analysis process.
*   **File Discovery**: It begins by recursively scanning the specified project root directory to identify all Rust source files (`.rs` extension) using the `walkdir` crate.
*   **Function Extraction**: For each Rust file found, it invokes the `analyze_rust_file` function from `function_analyzer.rs`.
*   **Error Handling**: Gracefully handles errors during file reading or parsing, logging the issues and skipping problematic files without crashing the entire process.
*   **Progress Indication**: Utilizes the `indicatif` crate to display a dynamic progress bar, providing visual feedback during the potentially long analysis of large codebases.

### 2.2. Rust Code Parsing and Semantic Summary Extraction (`function_analyzer.rs`)

This module is responsible for dissecting Rust files.
*   **Syntax Parsing**: It uses the `syn` crate to parse the Rust source code into an Abstract Syntax Tree (AST).
*   **Function Identification**: It iterates through the AST to identify and extract individual function definitions (`syn::Item::Fn`).
*   **Code Snippet Capture**: For each function, it captures the raw code snippet using the `quote` crate.
*   **Semantic Summary Generation**: The `extract_semantic_summary` function generates a simplified semantic representation of the function. Currently, this involves concatenating the function's identifier, input arguments, and return type. This summary serves as the input for the embedding model.
*   **Path Normalization**: Ensures that file paths are normalized to use forward slashes (`/`) for consistency across different operating systems (e.g., Windows backslashes are replaced).

### 2.3. BERT Embedding Generation (`embedding.rs`)

This module converts textual semantic summaries into high-dimensional numerical vectors.
*   **Model Retrieval**: It fetches the `sentence-transformers/all-MiniLM-L6-v2` BERT model and its associated tokenizer from the Hugging Face Hub (`hf-hub` crate).
*   **Tokenization**: The `tokenizers` crate is used to tokenize the `semantic_summary` into numerical `token_ids` and `attention_mask`.
*   **Input Truncation**: It truncates the tokenized input to the BERT model's maximum sequence length (`max_position_embeddings`, typically 512) to prevent out-of-bounds errors.
*   **Model Inference**: The `candle` machine learning framework is used to load the BERT model weights and perform a forward pass, generating a 384-dimensional embedding vector for the input text.
*   **Mean Pooling**: Aggregates the token-level embeddings into a single sentence-level embedding using mean pooling.

### 2.4. Clifford Multivector Conversion (`clifford.rs`)

This module transforms the BERT embeddings into a Clifford algebra representation.
*   **`BertCliffordEncoder`**: This custom encoder takes the 384-dimensional BERT embedding and projects it into an 8-dimensional Clifford algebra multivector (`SolMultivector`).
*   **Projection Matrix**: A randomly initialized projection matrix (8x384) is used to map the high-dimensional BERT space to the 8-dimensional generator space of the Clifford algebra.
*   **`tclifford` Crate**: The `tclifford` crate is used to define the 8D Clifford algebra (`SolCl`) and perform multivector operations.

### 2.5. Sieve Addressing (`sieve.rs`)

This module generates a unique binary address for each multivector.
*   **Sign-Based Addressing**: For each 8-dimensional multivector, an 8-bit binary "sieve address" is generated based on the sign (positive or negative) of its 8 basis vector components. A positive component maps to '1', and a negative component maps to '0'.

### 2.6. Emoji Mapping and Ontology Integration (`project_analyzer.rs`, `load_emoji_multivectors.rs`)

This stage connects the numerical representations to a semantic emoji space.
*   **Emoji Ontology Loading**: The `load_emoji_multivectors` function reads an existing RDF ontology (e.g., `ontologies/zos/v1.ttl`) that defines a semantic space of emojis, each associated with a BERT embedding and a category.
*   **Closest Emoji Calculation**: For each analyzed function's multivector:
    *   It calculates the Euclidean distance to all emoji multivectors in the loaded ontology.
    *   It identifies the 3 closest emojis, along with their categories and distances.
    *   If a semantic summary is already present in the ontology, it uses that mapping directly.
*   **`AnalyzedFunction` Enrichment**: The `AnalyzedFunction` struct is enriched with the `multivector_str`, `sieve_address`, and a list of `ClosestEmojiInfo` (emoji character, category, and distance).

### 2.7. RDF Ontology Generation (`ontology_generator/mod.rs`, `process_function.rs`, `serialize.rs`)

This final stage constructs the RDF graph and serializes it.
*   **Graph Construction**: The `sophia` RDF library is used to build an in-memory RDF graph (`sophia::inmem::graph::LightGraph`).
*   **Namespace Definition**: Standard RDF, RDFS, and custom `ex` (example) and `em` (emoji) namespaces are defined.
*   **Triple Insertion**: For each `AnalyzedFunction`, a set of RDF triples is generated and inserted into the graph. These triples link functions to their file paths, code snippets, semantic summaries, multivector embeddings, sieve addresses, and the details of their closest emojis.
*   **Serialization**: The `serialize_graph_to_file` function serializes the constructed RDF graph into a Turtle (`.ttl`) file. It uses `sophia_turtle` with `TurtleConfig` to manage prefixes and ensure pretty-printing.

## 3. Error Handling and Robustness

*   **`anyhow::Result`**: Extensive use of `anyhow::Result` for simplified error propagation throughout the pipeline.
*   **File System Operations**: Errors during file reading, parsing, or writing are caught and logged, preventing crashes and allowing partial results.
*   **BERT Model Loading**: Handles potential errors during model and tokenizer loading from Hugging Face Hub.

## 4. Future Considerations

*   **Performance Optimization**: Further optimizations could include parallel processing of files, caching BERT embeddings for frequently encountered semantic summaries, and optimizing distance calculations for large emoji ontologies.
*   **Semantic Summary Refinement**: The `extract_semantic_summary` function could be enhanced to produce more nuanced and comprehensive summaries of Rust functions, potentially using more advanced AST traversal and analysis techniques.
*   **Customizable Emoji Ontology**: Allow users to provide their own emoji ontologies or extend the existing one.
*   **Advanced Distance Metrics**: Explore more sophisticated distance metrics for multivectors beyond simple Euclidean distance.
*   **Error Handling for `unwrap()`**: Replace `.unwrap()` calls with proper error handling (e.g., `?` operator or `match` statements) for increased robustness.
