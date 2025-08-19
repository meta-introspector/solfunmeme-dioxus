# Chat Processing and Document Chunking

## Overview

We've implemented a comprehensive chat processing and document chunking system in the `solfunmeme_tools` crate. This system processes markdown chat files, extracts conversations, and splits documents into organized chunks with external files.

## Components

### 1. Chat Processor (`chat_processor`)

**Purpose**: Processes markdown chat files to extract clean conversation turns.

**Features**:
- HTML tag removal and entity decoding
- Conversation turn extraction (User/Grok AI)
- Code snippet preservation
- LaTeX block handling
- Cross-platform path normalization

**Usage**:
```bash
cargo run --bin chat_processor -- --help
```

### 2. Chunk Processor (`chunk_processor`) - NEW

**Purpose**: Splits documents into organized directory structures with external files.

**Features**:
- **Directory Structure**: `processed_docs/YYYY/MM/DD/filename_normalized/`
- **Date Extraction**: From filenames or file modification time
- **Chunk Types**: Conversation, CodeBlock, Documentation, Analysis, Other
- **External Files**: Large code blocks extracted to separate files
- **Parallel Processing**: Uses rayon for multi-threaded processing
- **Metadata**: JSON metadata for each chunk

**Directory Structure**:
```
processed_docs/
├── 2025/
│   ├── 01/
│   │   ├── 15/
│   │   │   ├── chat_session_1/
│   │   │   │   ├── index.md
│   │   │   │   ├── chunk_0/
│   │   │   │   │   ├── content.md
│   │   │   │   │   ├── metadata.json
│   │   │   │   │   └── code_0.rs
│   │   │   │   └── chunk_1/
│   │   │   │       ├── content.md
│   │   │   │       └── metadata.json
│   │   │   └── chat_session_2/
│   │   │       └── ...
```

**Usage**:
```bash
# Process a single file
cargo run --bin chunk_processor -- file --input founding_documents/chat-solfunmeme.md --output processed_docs

# Process all markdown files in a directory
cargo run --bin chunk_processor -- directory --input founding_documents --output processed_docs --pattern "*.md"

# Use specific number of threads
cargo run --bin chunk_processor -- directory --input founding_documents --output processed_docs --threads 8
```

## Implementation Details

### Chunk Detection

The chunk processor detects boundaries using:
- Markdown headers (`#`, `##`, `###`)
- Conversation markers (`User:`, `Grok AI:`)
- Code blocks (```)
- Section dividers (`---`)
- Part markers (`[START PART`, `[END PART`)

### External File Extraction

Code blocks larger than 100 characters are extracted to separate files:
- **File Naming**: `code_0.rs`, `code_1.py`, etc.
- **Extension Mapping**: Language-specific extensions
- **Content Replacement**: Links in markdown content

### Parallel Processing

- Uses `rayon` for parallel file processing
- Configurable thread count
- Error handling with success/failure reporting

## Dependencies

**Lightweight Dependencies**:
- `regex` - Pattern matching
- `walkdir` - File traversal
- `glob` - File pattern matching
- `chrono` - Date/time handling
- `serde_json` - JSON serialization
- `clap` - CLI argument parsing
- `rayon` - Parallel processing

**Optional Heavy Dependencies** (full feature):
- `solfunmeme_core_logic`
- `solfunmeme_models`
- `solfunmeme_extractor`
- `rust-embed`

## Next Steps

1. **Content-Based Naming**: Implement semantic chunk naming based on content analysis
2. **Pandoc Integration**: Use pandoc intermediate form for better parsing
3. **Vector Embeddings**: Generate embeddings for semantic search
4. **Index Generation**: Create searchable indexes across all chunks
5. **Web Interface**: Build a web UI for browsing and searching chunks

## File Structure

```
crates/solfunmeme_tools/
├── src/
│   ├── bin/
│   │   ├── chat_processor_cli.rs
│   │   └── chunk_processor_cli.rs
│   ├── chat_processing/
│   │   ├── mod.rs
│   │   ├── args.rs
│   │   ├── file_finder.rs
│   │   ├── content_processor.rs
│   │   ├── turn_processor.rs
│   │   └── chunk_processor.rs
│   └── utils.rs
└── Cargo.toml
```

## Status

✅ **Completed**:
- Chat processor with conversation extraction
- Document chunking with directory structure
- External file extraction
- Parallel processing
- Cross-platform path handling
- CLI interfaces

🔄 **In Progress**:
- Testing with founding documents
- Content-based chunk naming

📋 **Planned**:
- Pandoc integration
- Vector embeddings
- Search indexing
- Web interface 