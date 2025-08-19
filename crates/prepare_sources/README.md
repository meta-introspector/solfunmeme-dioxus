# `prepare_sources`

This crate is responsible for processing source code files and extracting them into structured `CodeChunk`s.

## Purpose

It serves as the initial step in the code analysis pipeline, preparing raw source code for further processing, indexing, and semantic analysis within the Solfunmeme ecosystem.

## Usage

To run the tool, use the following command:

```bash
cargo run --bin prepare_sources -- <path_to_directory_or_file>
```

This will process the specified files and output `CodeChunk`s in JSON format to standard output.

Example Output (simplified):

```json
{"path":"src/lib.rs","content":"fn main() {\n    println!(\"Hello, world!\");\n}","emoji":"📄","line_start":1,"line_end":3,"chunk_type":"code"}
{"path":"src/main.rs","content":"// Another chunk\nfn some_func() {\n    // ...\n}","emoji":"📄","line_start":1,"line_end":4,"chunk_type":"code"}
```