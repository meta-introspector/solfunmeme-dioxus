### Key Command-Line Tools

This project includes several powerful command-line tools to help with development, analysis, and data processing. For detailed setup and usage instructions, see the [Development Setup](doc/development_setup.md) guide. Here are some of the most important ones:

*   **`solfunmeme_tools`**: A collection of essential utilities for managing and interacting with the Solfunmeme-Dioxus ecosystem.
    *   **`chat_processor`**: Processes and structures chat logs, extracting valuable information and insights.
*   **`solfunmeme_indexer`**: Manages the indexing of your codebase, enabling powerful search and analysis capabilities.
    *   **`full_indexer_cli`**: The command-line interface for the indexer, allowing you to build, manage, and query your search indices.
*   **`prepare_sources`**: Prepares your source code for analysis and indexing, ensuring it's in the optimal format for the Solfunmeme-Dioxus tools.
*   **`codebase_analyzer_cli`**: Provides a suite of tools for analyzing your indexed codebase, including word frequency, emoji usage, and semantic search.
*   **`plan_cli`**: Estimates the cost of indexing by analyzing file counts, lines, and estimated chunks.

---

## 🔍 Codebase Analysis Tools

### Codebase Analyzer CLI

The `codebase_analyzer_cli` provides powerful analysis capabilities for your indexed codebase using the existing Tantivy search infrastructure.

**Usage:**
```bash
# Show top words by frequency
cargo run --bin codebase_analyzer_cli word-freq 50

# Show top emojis by frequency  
cargo run --bin codebase_analyzer_cli emoji-freq 20

# Show file types by count
cargo run --bin codebase_analyzer_cli file-types 20

# Search codebase content
cargo run --bin codebase_analyzer_cli search "function" 10

# Show overall statistics
cargo run --bin codebase_analyzer_cli stats
```

**Features:**
- **Word Frequency Analysis:** Find most common terms in your codebase
- **Emoji Statistics:** Analyze emoji usage patterns across code
- **File Type Analysis:** Understand your project's file composition
- **Full-Text Search:** Search through indexed code content
- **Comprehensive Stats:** Get overview of indexed data

**Example Output:**
```
Top 20 words in codebase:
  1. function              - 1,234
  2. struct                - 987
  3. impl                  - 876
  4. pub                   - 654
  5. let                   - 543
  ...

Top 10 emojis in codebase:
  1. 🔧  - 45
  2. 📝  - 32
  3. 🚀  - 28
  4. 🐛  - 25
  5. ✨  - 22
  ...
```

This tool leverages your existing `codebase_index/` Tantivy index, providing fast analysis without reprocessing files.