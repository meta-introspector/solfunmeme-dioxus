## Indexing the Codebase

The `full_indexer_cli` tool is used to index your codebase into a Tantivy search index. This process involves reading source files, chunking them, and adding them to the index for later analysis and querying. The indexer includes **automatic schema negotiation**: if an existing index is found to be corrupted or have a mismatched schema, it will be automatically recreated to ensure data integrity and compatibility.

**Usage:**
```bash
cargo run --bin full_indexer_cli <directory1> [directory2 ...] [--overwrite] [--sandbox] [--debug-backtrace]
```

**Options:**
*   `<directory1> [directory2 ...]`: One or more paths to directories you want to index (e.g., `crates/`, `vendor/`).
*   `--overwrite`: If specified, deletes the existing `codebase_index` directory before re-indexing. Use this for a clean re-index.
*   `--sandbox`: Creates the index in a temporary directory, preventing pollution of your main `codebase_index`. Useful for testing or one-off analyses.
*   `--debug-backtrace`: Enables `RUST_BACKTRACE=full` for subprocesses (like `prepare_sources`), providing detailed error traces if a crash occurs during indexing.

**Example:**
```bash
cargo run --bin full_indexer_cli crates/ vendor/ --overwrite --debug-backtrace
```
This command will re-index both the `crates/` and `vendor/` directories, overwriting any existing index, and provide detailed backtraces on errors.

---

## Estimating Indexing Cost

Before performing a full index, you can use the `plan_cli` tool to estimate the computational cost. This tool analyzes the number of files, lines, and estimates the number of code chunks that will be generated, giving you an idea of the indexing effort.

**Usage:**
```bash
cargo run --bin plan_cli <directory1> [directory2 ...]
```

**Example:**
```bash
cargo run --bin plan_cli crates/ vendor/
```
This command will provide a summary of files, lines, and estimated chunks for the specified directories.