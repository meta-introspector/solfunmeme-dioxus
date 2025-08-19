# Crates Overview

This directory contains all the Rust crates that constitute the `solfunmeme-dioxus` project. Each crate is designed with a specific purpose, contributing to the overall functionality of the Code-Math Manifold.

For detailed information on each crate, please refer to its individual `README.md` file located within its respective directory.

---

## Semantic Index of Crate Categories

### Core Application Crates
- Manage the UI, workflows, and overall project state.

### Data & Analysis Core
- Define fundamental data structures and provide core business logic for processing and analyzing data.

### Code Analysis & Processing
- Parse, extract, and analyze source code files into structured data.

### Mathematical & Embedding Systems
- Handle mathematical transformations, including Clifford algebra operations and BERT embedding generation.

### Semantic & RDF Processing
- Convert structured data into RDF triples and handle JSON-LD for semantic representation.

### UI & Component System
- Provide reusable UI components, state management, and tools for building interactive elements.

### Blockchain & Solana Integration
- Facilitate interaction with the Solana blockchain, including wallet integration, account management, and on-chain data storage.

### NLP & Language Processing Plugins
- Provide natural language processing capabilities, from tokenization and embedding to grammar checking and keyword extraction.

### Search & Indexing
- Manage full-text search functionalities and indexing of code and data.

### Storage & I/O
- Handle interactions with external storage systems and file operations.

### Scripting & Automation
- Integrate scripting engines for automation and custom logic.

### Emoji & Workflow System
- Enable the use of emojis for defining and managing workflows.

### Micro-Component Protocol
- Contain foundational types and traits for the Micro-Component Protocol.

---

## Example: `solfunmeme_tools`

This crate provides a collection of general utility functions and helper tools used across various modules within the Solfunmeme project.

- **Purpose:** General utility functions and helper tools used across modules.
- **Core Functionalities:**
  - Data conversion
  - String manipulation
  - Error handling utilities
  - Logging utilities

---

## Glossary

**DocIndexer**: A trait implemented by each indexer crate, defining how to scan and process documents.

**Orchestrator**: The main program that loads, runs, and aggregates results from all indexers.

**Vendor Directory**: The `vendor/` directory contains all indexer crates as submodules or subcrates.

**Node**: A document, log, or artifact indexed by the system (e.g., a daily gemini log, chat log, image, or audio file).

**Processed Docs**: Documents that have been chunked, vectorized, emoji-mapped, or otherwise semantically enriched by the system.

**Semantic Map**: The output index or graph showing the relationships and metadata of all nodes in the documentation.

**Glossary**: This section, listing and defining key terms used in the project and documentation system.

---

This documentation will be updated as the system evolves and new crates or features are added.
