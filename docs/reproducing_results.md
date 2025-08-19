## Reproducing Results

To reproduce our current indexing and emoji analysis results, follow these steps:

1.  **Prerequisites**: Ensure you have Rust and Cargo installed. You may also need `git` if you haven't cloned the repository.

2.  **Clone the Repository (if not already done)**:
    ```bash
    git clone https://github.com/your-repo/solfunmeme-dioxus.git
    cd solfunmeme-dioxus
    ```

3.  **Build the Tools**: Navigate to the `solfunmeme_tools` crate and build the necessary binaries:
    ```bash
    cd crates/solfunmeme_tools
    cargo build --release --bins
    cd ../..
    ```

4.  **Index the Codebase**: Run the `full_indexer_cli` to index the `crates/` and `vendor/` directories. The `--overwrite` flag ensures a clean index, and `--debug-backtrace` provides detailed error logs if any issues occur.
    ```bash
    ./target/release/full_indexer_cli crates/ vendor/ --overwrite --debug-backtrace
    ```
    *Note: This step can take a significant amount of time due to the large number of files in the `vendor/` directory. Progress updates will be printed to stderr.*

5.  **Generate the Emoji Frequency Report**: Once indexing is complete, run the `codebase_analyzer_cli` to generate the emoji report. The `9999` limit ensures all unique emojis are captured.
    ```bash
    ./target/release/codebase_analyzer_cli emoji-freq 9999
    ```
    This will print the top emojis and their occurrences to your console.