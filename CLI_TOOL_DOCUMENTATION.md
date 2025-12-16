# Rust AST Emoji Analyzer CLI Tool

## Overview

The `prepare_sources.rs` CLI tool is a sophisticated Rust code analysis utility that transforms Rust source code into emoji-rich datasets for code understanding and visualization. It parses Rust source files into Abstract Syntax Trees (ASTs), maps code elements to emojis, and creates structured datasets suitable for machine learning and code analysis.

## What We Built

### Core Functionality

1. **Rust Source Code Discovery**: Recursively finds all `.rs` files in the `src/` directory
2. **AST Parsing**: Uses the `syn` crate to parse Rust code into detailed AST representations
3. **Emoji Mapping**: Maps AST node types and extracted words to emojis based on semantic categories
4. **Code Analysis**: Extracts string literals, word frequencies, and structural patterns
5. **Dataset Generation**: Creates Hugging Face-compatible datasets with proper chunking and organization

### Key Features

- **Cross-Platform Path Handling**: Supports both Windows (`\`) and Unix (`/`) path separators
- **Intelligent Chunking**: Splits large datasets into manageable 1MB chunks
- **Comprehensive Emoji Mapping**: 200+ emoji mappings across multiple categories
- **Domain Detection**: Identifies code domains (Crypto, Web, i18n, etc.) through emoji patterns
- **Structured Output**: Generates both individual reports and consolidated datasets

## Emoji Mapping Categories

### Rust Core (🦀⚙️)
- Function definitions: `fn` → 🦀⚙️
- Structs: `struct` → 🏛️🧱
- Enums: `enum` → 🎲
- Modules: `mod` → 📦
- Traits: `trait` → 🧩
- And 100+ more Rust language constructs

### Web/CSS (🎨)
- CSS units: `px`, `deg`, `em`, `rem` → 📏🧭🔠🔡
- Animation properties: `animation`, `transition` → 🎞️🔄
- Layout concepts: `absolute`, `align` → 📐📏

### Crypto/Security (🔒)
- Cryptography: `aead`, `aes`, `argon2` → 🔒🔑🧂
- Blockchain: `agave`, `helius` → 🌵🌞
- Security primitives: `curve25519`, `ed25519` → ➰📝

### Internationalization (🌐)
- i18n libraries: `icu4x`, `cldr` → 🌐🌍
- Language support: `chinese`, `hebrew` → 🀄✡️
- Date/time: `calendar`, `datetime` → 📅⏰

### Testing/Benchmarking (⏱️)
- Testing frameworks: `criterion`, `benches` → ⏱️🏋️
- Development tools: `fuzz`, `examples` → 🧪📚

## Dataset Structure

### Output Format

The tool generates a Hugging Face dataset with the following structure:

```
hf_dataset/
├── README.md                 # Dataset card with metadata
├── dataset_info.json         # Dataset configuration
├── data/                     # Chunked data files
│   ├── 000/                  # Subdirectories (max 10k files each)
│   │   ├── chunk_00000.json  # 1MB chunks
│   │   ├── chunk_00001.json
│   │   └── ...
│   └── 001/
│       └── ...
└── reports/                  # Analysis reports
    ├── summary_total.txt     # Global summary
    ├── emoji_mapping.txt     # Emoji mapping reference
    └── [compressed_dirs]/    # Per-file reports
```

### Data Fields

Each dataset example contains:

```json
{
  "file_path": "src/core/analyzer.rs",
  "timestamp": 1703123456,
  "ast": { /* Full AST representation */ },
  "summary": {
    "top_level_nodes": 15,
    "total_nodes": 234,
    "type_counts": { "fn": 5, "struct": 2, ... },
    "string_literals": ["hello", "world"],
    "word_counts": { "hello": 1, "world": 1 },
    "word_emoji_counts": { "🦀": 5, "🏛️": 2 },
    "emoji_counts_in_strings": { "🚀": 1 }
  }
}
```

## Usage

### Prerequisites

```bash
# Ensure you have Rust installed
rustc --version

# Install dependencies
cargo build --release
```

### Running the Tool

```bash
# Run from the project root
cargo run --bin prepare_sources

# Or run the compiled binary
./target/release/prepare_sources
```

### Output

The tool provides verbose progress output:

```
[INFO] Creating Hugging Face dataset structure...
[INFO] Initializing CodeAnalyzer ...
[INFO] Analyzing files ...
[INFO] Analysis complete. 15081 files analyzed.

src/core/analyzer.rs | 🦀⚙️(fn)×5 🏛️🧱(struct)×2 🎲(enum)×1 | 🦀⚙️🦀⚙️🦀⚙️🦀⚙️🦀⚙️🏛️🧱🏛️🧱🎲
[emojis in strings] 🚀×1
[words mapped to emojis] 🦀×5 🏛️×2

[INFO] Wrote chunk 0 to hf_dataset/data/000/chunk_00000.json (150 examples, 524288 bytes)
[INFO] Wrote chunk 1 to hf_dataset/data/000/chunk_00001.json (142 examples, 498123 bytes)
...
```

## Technical Implementation

### Architecture

1. **File Discovery**: Uses `walkdir` to recursively find Rust files
2. **Code Analysis**: Leverages custom `CodeAnalyzer` for AST parsing
3. **Emoji Processing**: Implements comprehensive emoji mapping system
4. **Chunking Logic**: Intelligent file size management for platform compatibility
5. **Path Handling**: Cross-platform path normalization and directory creation

### Key Algorithms

#### Emoji Mapping
```rust
fn emoji_for_type(ty: &str) -> (&'static str, &'static str) {
    for &(name, emoji, category) in EMOJI_TYPE_MAP {
        if ty == name {
            return (emoji, category);
        }
    }
    ("❓🤷", "Uncategorized")
}
```

#### Word Extraction
```rust
fn split_words(s: &str) -> Vec<String> {
    // Split on whitespace, punctuation, underscores
    // Handle CamelCase splitting
    // Return lowercase word tokens
}
```

#### Chunking Strategy
```rust
let max_file_size = 1024 * 1024; // 1MB chunks
let max_files_per_dir = 10000;    // Directory organization

// Check if adding example would exceed chunk size
if current_chunk_size + example_size > max_file_size {
    // Write current chunk and start new one
}
```

## Use Cases

### Code Understanding
- **Pattern Recognition**: Identify common coding patterns through emoji frequency
- **Domain Classification**: Automatically categorize code by domain (Crypto, Web, etc.)
- **Complexity Analysis**: Use emoji density as a proxy for code complexity

### Visualization
- **Emoji Summaries**: Create visual summaries of codebases
- **Trend Analysis**: Track emoji patterns across code evolution
- **Code Art**: Generate artistic representations of code structure

### Machine Learning
- **Code Classification**: Train models to classify code by domain
- **Similarity Detection**: Find similar code patterns using emoji signatures
- **Code Generation**: Use emoji patterns to guide code generation

## Performance Considerations

### Memory Efficiency
- **Streaming Processing**: Processes files one at a time to minimize memory usage
- **Compact JSON**: Uses compact JSON serialization to reduce file sizes
- **Intelligent Chunking**: Balances chunk size with accessibility

### Scalability
- **Directory Organization**: Limits files per directory to 10,000
- **Path Length Management**: Truncates long paths for Windows compatibility
- **Error Handling**: Graceful handling of parsing errors and file access issues

## Limitations and Future Improvements

### Current Limitations
- **Language Specific**: Currently only supports Rust
- **Emoji Coverage**: Limited to predefined mappings
- **Platform Constraints**: Windows path length limitations

### Potential Enhancements
- **Multi-language Support**: Extend to other programming languages
- **Dynamic Emoji Mapping**: Learn emoji mappings from code context
- **Real-time Analysis**: Support for live code analysis
- **Advanced Visualization**: Interactive emoji-based code explorers

## Contributing

### Adding New Emoji Mappings

To add new emoji mappings, edit the `EMOJI_TYPE_MAP` constant:

```rust
const EMOJI_TYPE_MAP: &[(&str, &str, &str)] = &[
    // Add new mappings here
    ("new_term", "🆕", "New Category"),
    // ...
];
```

### Extending Analysis

The tool is designed to be extensible. Key extension points:
- `extract_string_literals()`: Custom string extraction logic
- `split_words()`: Custom word tokenization
- `count_types_recursive()`: Custom AST traversal

## License

This tool is part of the solfunmeme-dioxus project and is licensed under AGPL-3.0.

## Acknowledgments

- Built with the `syn` crate for Rust AST parsing
- Inspired by creative approaches to code visualization
- Designed for the Hugging Face ecosystem 