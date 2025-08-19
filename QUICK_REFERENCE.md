# Solfunmeme-Dioxus Quick Reference Guide

## 🚀 Quick Start Commands

```bash
# Complete system setup and run
make setup && make all

# Interactive ZOS CLI
make zos

# Full processing pipeline (8 factorial steps)
make pipeline

# Quick system check
make quick-check
```

## 🎯 Core Workflow Commands

### 1. Setup & Build
```bash
make setup      # Initialize development environment
make build      # Build all crates and dependencies
make build-web  # Build for web deployment
make clean      # Clean build artifacts
```

### 2. Data Processing Pipeline
```bash
make index      # Index codebase for search and analysis
make analyze    # Run comprehensive code analysis
make extract    # Extract code chunks and functions
make embed      # Generate BERT embeddings and emoji vectors
make search     # Search indexed codebase
```

### 3. Semantic Processing
```bash
make rdf        # Generate RDF triples and semantic data
make semantic   # Process semantic relationships
```

### 4. Blockchain Integration
```bash
make blockchain # Setup Solana integration
make wallet     # Configure wallet integration
```

### 5. UI & Components
```bash
make ui         # Build UI components and views
make components # Build component system
make playground # Setup interactive playground
```

### 6. Tools & CLI
```bash
make tools      # Build CLI tools
make plugins    # Build and load plugins
make workflow   # Run workflow manager
```

## 🔧 ZOS CLI Commands

```bash
# Start interactive mode
make zos

# Run pipeline command
make zos-pipeline

# Direct CLI usage
cargo run --bin zos-driver interactive
cargo run --bin zos-driver pipeline "function" --sort --uniq --head 20
```

## 📊 Analysis Commands

```bash
# Codebase analysis
cargo run --bin codebase_analyzer_cli stats
cargo run --bin codebase_analyzer_cli word-freq 50
cargo run --bin codebase_analyzer_cli emoji-freq 20
cargo run --bin codebase_analyzer_cli file-types 20
cargo run --bin codebase_analyzer_cli search "function" 10

# Indexing
cargo run --bin full_indexer_cli build

# Planning
cargo run --bin plan_cli
```

## 🧩 Specialized Workflows

```bash
# Emoji workflow processing
make emoji-workflow

# Clifford algebra operations
make clifford

# Orbital simulation
make orbital

# NLP processing
make nlp
```

## 🔍 System Diagnostics

```bash
# System status
make status

# Dependency graph
make deps

# File structure
make tree

# Recent changes
make changes

# Backup current state
make backup
```

## 📚 Documentation

```bash
# Generate documentation
make docs

# Open documentation
make docs-open
```

## 🧪 Testing & Development

```bash
# Run tests
make test

# Development workflow
make dev

# Watch for changes
make watch

# Full integration test
make integration-test
```

## 🎯 Key System Components

### Core Crates
- **`solfunmeme_app`**: Main application entry point
- **`workflow_manager`**: Central workflow orchestration
- **`task_manager`**: Project state and task management
- **`playground_lib`**: Interactive sandbox environment

### Data Processing
- **`prepare_sources`**: Source code preparation
- **`solfunmeme_extractor`**: Code extraction
- **`solfunmeme_function_analysis`**: Function analysis
- **`solfunmeme_embedding`**: BERT embeddings
- **`solfunmeme_search_tantivy`**: Search indexing

### Semantic & RDF
- **`rdf_processing_lib`**: RDF processing utilities
- **`rdf_output`**: RDF triple generation
- **`jsonld_plugin`**: JSON-LD operations

### Blockchain & Solana
- **`solana_integration_lib`**: Solana blockchain integration
- **`solfunmeme_wallet_integration`**: Wallet integration
- **`gitaccount`**: Content-addressed Git storage

### UI & Components
- **`solfunmeme_views`**: UI components and views
- **`component_builder_lib`**: Component building
- **`component_registry_lib`**: Component registry
- **`component_emoji_lib`**: Emoji component management

### Mathematical & Embedding
- **`solfunmeme_clifford`**: Clifford algebra operations
- **`emoji_matrix_lib`**: Emoji matrix operations
- **`orbital_sim_lib`**: Orbital mechanics simulation

### NLP & Language Processing
- **`layered_nlp_plugin`**: Multi-layer NLP processing
- **`keyword_extraction_rs_plugin`**: Keyword extraction
- **`bm25_plugin`**: BM25 scoring
- **`eliza_rs_plugin`**: ELIZA-like chatbot

### Search & Indexing
- **`quickwit_plugin`**: Distributed search
- **`model2vec_rs_plugin`**: Model-to-vector conversion
- **`tongrams_rs_plugin`**: N-gram language models

### Storage & I/O
- **`s3_plugin`**: Amazon S3 integration
- **`zip_plugin`**: Zip archive operations
- **`git_plugin`**: Git repository operations

### Scripting & Automation
- **`rhai_plugin`**: Rhai scripting engine
- **`steel_plugin`**: Steel Scheme-like language
- **`gline_rs_plugin`**: Graphical line drawing

## 🔄 The 8 Factorial Steps

1. **Source Preparation** (`prepare_sources`)
2. **Code Extraction** (`solfunmeme_extractor`)
3. **Function Analysis** (`solfunmeme_function_analysis`)
4. **Embedding Generation** (`solfunmeme_embedding`)
5. **Semantic Indexing** (`solfunmeme_search_tantivy`)
6. **RDF Processing** (`rdf_processing_lib`)
7. **Blockchain Integration** (`solana_integration_lib`)
8. **UI Rendering** (`solfunmeme_views`)

## 🎯 Key Concepts

- **Autopoetic Rewrites**: Self-modifying code through LLM processing
- **Code-Math Manifold**: Mathematical representation of code structures
- **Emoji Vectors**: BERT embeddings converted to emoji representations
- **Semantic Processing**: RDF triple generation for semantic relationships
- **Blockchain Integration**: Content-addressed storage on Solana
- **Component System**: Dynamic UI component management

## 📋 File Locations

- **Systems Design**: `vendor/meta-meme.wiki/SystemsDesign.md`
- **Architecture**: `doc/architecture.md`
- **CLI Tools**: `doc/cli_tools_guide.md`
- **Main CLI**: `zos/src/main.rs`
- **Workflow**: `Makefile`
- **Workflow Diagram**: `WORKFLOW_DIAGRAM.md`
- **UML Diagrams**: `docs/uml/` - Complete set of architecture diagrams
- **Cross-References**: `docs/uml/CROSS_REFERENCES.md` - Documentation relationships
- **Ontologies**: `ontologies/` - Semantic models and relationships

## 🚨 Troubleshooting

```bash
# Check system status
make status

# Quick system check
make quick-check

# Clean and rebuild
make clean && make build

# Check for build issues
cargo check --workspace

# Run tests to verify functionality
make test
```

This quick reference provides the essential commands and information needed to work with the Solfunmeme-Dioxus system effectively. 