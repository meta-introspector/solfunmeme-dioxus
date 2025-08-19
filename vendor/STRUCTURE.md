# Vendor Hierarchical Submodule Structure

## Overview

This document outlines the hierarchical organization of vendor dependencies using an OSI-like 7-layer system with nested categorization (7→5→3→2).

## Architecture Principles

- **7 Layers**: Top-level categories reflecting system architecture layers
- **5 Subcategories**: Logical groupings within each layer
- **3 Components**: Implementation aspects within each subcategory  
- **2 Levels**: Stability/readiness levels within each component

## Layer Structure

### 1. `crypto/` - Cryptographic Layer
**Purpose**: Security, encryption, and cryptographic operations

#### 5 Subcategories:
- `primitives/` - Basic cryptographic operations
- `protocols/` - Cryptographic protocols and standards
- `keys/` - Key management and derivation
- `hashes/` - Hash functions and checksums
- `utils/` - Cryptographic utilities

#### 3 Components per subcategory:
- `core/` - Core implementations
- `bindings/` - Language bindings
- `extensions/` - Extended functionality

#### 2 Levels per component:
- `stable/` - Production-ready implementations
- `experimental/` - Experimental implementations

**Example mapping**:
```
crypto/
├── primitives/
│   ├── core/
│   │   ├── stable/          # block-ciphers, stream-ciphers
│   │   └── experimental/    # new cipher implementations
│   ├── bindings/
│   │   ├── stable/          # openssl bindings
│   │   └── experimental/    # new binding libraries
│   └── extensions/
│       ├── stable/          # AEADs, MACs
│       └── experimental/    # novel cryptographic extensions
├── protocols/
│   ├── core/
│   │   ├── stable/          # TLS, SSH implementations
│   │   └── experimental/    # new protocol implementations
│   ├── bindings/
│   │   ├── stable/          # rustls, rust-native-tls
│   │   └── experimental/    # new protocol bindings
│   └── extensions/
│       ├── stable/          # protocol extensions
│       └── experimental/    # experimental protocol features
├── keys/
│   ├── core/
│   │   ├── stable/          # ed25519-dalek, curve25519-dalek
│   │   └── experimental/    # new key algorithms
│   ├── bindings/
│   │   ├── stable/          # libsecp256k1 bindings
│   │   └── experimental/    # new key library bindings
│   └── extensions/
│       ├── stable/          # key derivation, BIP32
│       └── experimental/    # novel key management
├── hashes/
│   ├── core/
│   │   ├── stable/          # BLAKE3, SHA implementations
│   │   └── experimental/    # new hash functions
│   ├── bindings/
│   │   ├── stable/          # hash library bindings
│   │   └── experimental/    # new hash bindings
│   └── extensions/
│       ├── stable/          # password-hashes, HMAC
│       └── experimental/    # novel hash applications
└── utils/
    ├── core/
    │   ├── stable/          # subtle, constant-time operations
    │   └── experimental/    # new crypto utilities
    ├── bindings/
    │   ├── stable/          # crypto utility bindings
    │   └── experimental/    # new utility bindings
    └── extensions/
        ├── stable/          # crypto helper functions
        └── experimental/    # experimental crypto tools
```

### 2. `system/` - System Layer
**Purpose**: Operating system integration, system calls, and low-level operations

#### 5 Subcategories:
- `os/` - Operating system interfaces
- `io/` - Input/output operations
- `memory/` - Memory management
- `process/` - Process and thread management
- `utils/` - System utilities

**Example mapping**:
```
system/
├── os/
│   ├── core/
│   │   ├── stable/          # linux-raw-sys, rustix
│   │   └── experimental/    # new OS interfaces
│   ├── bindings/
│   │   ├── stable/          # system-configuration-rs
│   │   └── experimental/    # new OS bindings
│   └── extensions/
│       ├── stable/          # OS-specific utilities
│       └── experimental/    # experimental OS features
├── io/
│   ├── core/
│   │   ├── stable/          # mio, polling
│   │   └── experimental/    # new I/O implementations
│   ├── bindings/
│   │   ├── stable/          # I/O library bindings
│   │   └── experimental/    # new I/O bindings
│   └── extensions/
│       ├── stable/          # I/O utilities
│       └── experimental/    # experimental I/O features
├── memory/
│   ├── core/
│   │   ├── stable/          # memmap2-rs, slab
│   │   └── experimental/    # new memory implementations
│   ├── bindings/
│   │   ├── stable/          # memory library bindings
│   │   └── experimental/    # new memory bindings
│   └── extensions/
│       ├── stable/          # memory utilities
│       └── experimental/    # experimental memory features
├── process/
│   ├── core/
│   │   ├── stable/          # parking_lot, concurrent-queue
│   │   └── experimental/    # new process implementations
│   ├── bindings/
│   │   ├── stable/          # process library bindings
│   │   └── experimental/    # new process bindings
│   └── extensions/
│       ├── stable/          # process utilities
│       └── experimental/    # experimental process features
└── utils/
    ├── core/
    │   ├── stable/          # dunce, path-clean
    │   └── experimental/    # new system utilities
    ├── bindings/
    │   ├── stable/          # system utility bindings
    │   └── experimental/    # new utility bindings
    └── extensions/
        ├── stable/          # system helper functions
        └── experimental/    # experimental system tools
```

### 3. `network/` - Network Layer
**Purpose**: Networking, HTTP, and communication protocols

#### 5 Subcategories:
- `http/` - HTTP protocol implementations
- `tls/` - Transport Layer Security
- `websocket/` - WebSocket implementations
- `protocols/` - Other network protocols
- `utils/` - Network utilities

**Example mapping**:
```
network/
├── http/
│   ├── core/
│   │   ├── stable/          # hyper, axum
│   │   └── experimental/    # new HTTP implementations
│   ├── bindings/
│   │   ├── stable/          # HTTP library bindings
│   │   └── experimental/    # new HTTP bindings
│   └── extensions/
│       ├── stable/          # HTTP utilities, middleware
│       └── experimental/    # experimental HTTP features
├── tls/
│   ├── core/
│   │   ├── stable/          # rustls, ring
│   │   └── experimental/    # new TLS implementations
│   ├── bindings/
│   │   ├── stable/          # native-tls, schannel-rs
│   │   └── experimental/    # new TLS bindings
│   └── extensions/
│       ├── stable/          # TLS utilities
│       └── experimental/    # experimental TLS features
├── websocket/
│   ├── core/
│   │   ├── stable/          # tungstenite-rs
│   │   └── experimental/    # new WebSocket implementations
│   ├── bindings/
│   │   ├── stable/          # WebSocket library bindings
│   │   └── experimental/    # new WebSocket bindings
│   └── extensions/
│       ├── stable/          # WebSocket utilities
│       └── experimental/    # experimental WebSocket features
├── protocols/
│   ├── core/
│   │   ├── stable/          # h2, http-body
│   │   └── experimental/    # new protocol implementations
│   ├── bindings/
│   │   ├── stable/          # protocol library bindings
│   │   └── experimental/    # new protocol bindings
│   └── extensions/
│       ├── stable/          # protocol utilities
│       └── experimental/    # experimental protocol features
└── utils/
    ├── core/
    │   ├── stable/          # reqwest, tower
    │   └── experimental/    # new network utilities
    ├── bindings/
    │   ├── stable/          # network utility bindings
    │   └── experimental/    # new utility bindings
    └── extensions/
        ├── stable/          # network helper functions
        └── experimental/    # experimental network tools
```

### 4. `data/` - Data Layer
**Purpose**: Data structures, serialization, and storage

#### 5 Subcategories:
- `serialization/` - Data serialization formats
- `storage/` - Data storage and persistence
- `structures/` - Data structures and collections
- `formats/` - File and data formats
- `utils/` - Data utilities

**Example mapping**:
```
data/
├── serialization/
│   ├── core/
│   │   ├── stable/          # serde, bincode
│   │   └── experimental/    # new serialization formats
│   ├── bindings/
│   │   ├── stable/          # serialization library bindings
│   │   └── experimental/    # new serialization bindings
│   └── extensions/
│       ├── stable/          # serde extensions, formats
│       └── experimental/    # experimental serialization
├── storage/
│   ├── core/
│   │   ├── stable/          # tantivy, quickwit
│   │   └── experimental/    # new storage implementations
│   ├── bindings/
│   │   ├── stable/          # storage library bindings
│   │   └── experimental/    # new storage bindings
│   └── extensions/
│       ├── stable/          # storage utilities
│       └── experimental/    # experimental storage features
├── structures/
│   ├── core/
│   │   ├── stable/          # indexmap, dashmap
│   │   └── experimental/    # new data structures
│   ├── bindings/
│   │   ├── stable/          # structure library bindings
│   │   └── experimental/    # new structure bindings
│   └── extensions/
│       ├── stable/          # structure utilities
│       └── experimental/    # experimental structures
├── formats/
│   ├── core/
│   │   ├── stable/          # json, toml, csv
│   │   └── experimental/    # new format implementations
│   ├── bindings/
│   │   ├── stable/          # format library bindings
│   │   └── experimental/    # new format bindings
│   └── extensions/
│       ├── stable/          # format utilities
│       └── experimental/    # experimental formats
└── utils/
    ├── core/
    │   ├── stable/          # bytes, byteorder
    │   └── experimental/    # new data utilities
    ├── bindings/
    │   ├── stable/          # data utility bindings
    │   └── experimental/    # new utility bindings
    └── extensions/
        ├── stable/          # data helper functions
        └── experimental/    # experimental data tools
```

### 5. `compute/` - Compute Layer
**Purpose**: Computation, mathematics, and algorithms

#### 5 Subcategories:
- `math/` - Mathematical operations and libraries
- `algorithms/` - Algorithm implementations
- `parallel/` - Parallel and concurrent computation
- `optimization/` - Optimization and performance
- `utils/` - Compute utilities

**Example mapping**:
```
compute/
├── math/
│   ├── core/
│   │   ├── stable/          # nalgebra, ndarray
│   │   └── experimental/    # new math implementations
│   ├── bindings/
│   │   ├── stable/          # math library bindings
│   │   └── experimental/    # new math bindings
│   └── extensions/
│       ├── stable/          # math utilities, linfa
│       └── experimental/    # experimental math features
├── algorithms/
│   ├── core/
│   │   ├── stable/          # rayon, itertools
│   │   └── experimental/    # new algorithm implementations
│   ├── bindings/
│   │   ├── stable/          # algorithm library bindings
│   │   └── experimental/    # new algorithm bindings
│   └── extensions/
│       ├── stable/          # algorithm utilities
│       └── experimental/    # experimental algorithms
├── parallel/
│   ├── core/
│   │   ├── stable/          # crossbeam, parking_lot
│   │   └── experimental/    # new parallel implementations
│   ├── bindings/
│   │   ├── stable/          # parallel library bindings
│   │   └── experimental/    # new parallel bindings
│   └── extensions/
│       ├── stable/          # parallel utilities
│       └── experimental/    # experimental parallel features
├── optimization/
│   ├── core/
│   │   ├── stable/          # fastrand, ahash
│   │   └── experimental/    # new optimization implementations
│   ├── bindings/
│   │   ├── stable/          # optimization library bindings
│   │   └── experimental/    # new optimization bindings
│   └── extensions/
│       ├── stable/          # optimization utilities
│       └── experimental/    # experimental optimizations
└── utils/
    ├── core/
    │   ├── stable/          # num, approx
    │   └── experimental/    # new compute utilities
    ├── bindings/
    │   ├── stable/          # compute utility bindings
    │   └── experimental/    # new utility bindings
    └── extensions/
        ├── stable/          # compute helper functions
        └── experimental/    # experimental compute tools
```

### 6. `ui/` - User Interface Layer
**Purpose**: User interface, presentation, and interaction

#### 5 Subcategories:
- `framework/` - UI frameworks and libraries
- `components/` - UI components and widgets
- `rendering/` - Rendering and graphics
- `interaction/` - User interaction handling
- `utils/` - UI utilities

**Example mapping**:
```
ui/
├── framework/
│   ├── core/
│   │   ├── stable/          # dioxus, leptos
│   │   └── experimental/    # new UI frameworks
│   ├── bindings/
│   │   ├── stable/          # framework library bindings
│   │   └── experimental/    # new framework bindings
│   └── extensions/
│       ├── stable/          # framework utilities
│       └── experimental/    # experimental framework features
├── components/
│   ├── core/
│   │   ├── stable/          # dioxus-charts, dioxus-motion
│   │   └── experimental/    # new UI components
│   ├── bindings/
│   │   ├── stable/          # component library bindings
│   │   └── experimental/    # new component bindings
│   └── extensions/
│       ├── stable/          # component utilities
│       └── experimental/    # experimental components
├── rendering/
│   ├── core/
│   │   ├── stable/          # stylo, cssparser
│   │   └── experimental/    # new rendering implementations
│   ├── bindings/
│   │   ├── stable/          # rendering library bindings
│   │   └── experimental/    # new rendering bindings
│   └── extensions/
│       ├── stable/          # rendering utilities
│       └── experimental/    # experimental rendering features
├── interaction/
│   ├── core/
│   │   ├── stable/          # keyboard-types, cursor-icon
│   │   └── experimental/    # new interaction implementations
│   ├── bindings/
│   │   ├── stable/          # interaction library bindings
│   │   └── experimental/    # new interaction bindings
│   └── extensions/
│       ├── stable/          # interaction utilities
│       └── experimental/    # experimental interaction features
└── utils/
    ├── core/
    │   ├── stable/          # copypasta, clipboard-win
    │   └── experimental/    # new UI utilities
    ├── bindings/
    │   ├── stable/          # UI utility bindings
    │   └── experimental/    # new utility bindings
    └── extensions/
        ├── stable/          # UI helper functions
        └── experimental/    # experimental UI tools
```

### 7. `ai/` - Artificial Intelligence Layer
**Purpose**: Machine learning, AI, and intelligent systems

#### 5 Subcategories:
- `models/` - AI models and implementations
- `nlp/` - Natural language processing
- `embedding/` - Vector embeddings and similarity
- `search/` - AI-powered search and retrieval
- `utils/` - AI utilities

**Example mapping**:
```
ai/
├── models/
│   ├── core/
│   │   ├── stable/          # candle, rustbert
│   │   └── experimental/    # new model implementations
│   ├── bindings/
│   │   ├── stable/          # model library bindings
│   │   └── experimental/    # new model bindings
│   └── extensions/
│       ├── stable/          # model utilities
│       └── experimental/    # experimental model features
├── nlp/
│   ├── core/
│   │   ├── stable/          # nlprule, vaporetto
│   │   └── experimental/    # new NLP implementations
│   ├── bindings/
│   │   ├── stable/          # NLP library bindings
│   │   └── experimental/    # new NLP bindings
│   └── extensions/
│       ├── stable/          # NLP utilities, layered-nlp
│       └── experimental/    # experimental NLP features
├── embedding/
│   ├── core/
│   │   ├── stable/          # fastembed-rs, rust-sbert
│   │   └── experimental/    # new embedding implementations
│   ├── bindings/
│   │   ├── stable/          # embedding library bindings
│   │   └── experimental/    # new embedding bindings
│   └── extensions/
│       ├── stable/          # embedding utilities
│       └── experimental/    # experimental embedding features
├── search/
│   ├── core/
│   │   ├── stable/          # bm25, keyword-extraction-rs
│   │   └── experimental/    # new search implementations
│   ├── bindings/
│   │   ├── stable/          # search library bindings
│   │   └── experimental/    # new search bindings
│   └── extensions/
│       ├── stable/          # search utilities
│       └── experimental/    # experimental search features
└── utils/
    ├── core/
    │   ├── stable/          # model2vec-rs, llms-from-scratch-rs
    │   └── experimental/    # new AI utilities
    ├── bindings/
    │   ├── stable/          # AI utility bindings
    │   └── experimental/    # new utility bindings
    └── extensions/
        ├── stable/          # AI helper functions
        └── experimental/    # experimental AI tools
```

## Implementation Strategy

### Phase 1: Structure Creation
1. Create the 7 top-level directories
2. Create the 5 subcategory directories within each layer
3. Create the 3 component directories within each subcategory
4. Create the 2 level directories within each component

### Phase 2: Dependency Migration
1. Analyze current vendor dependencies
2. Map each dependency to appropriate location in hierarchy
3. Move dependencies to new structure
4. Update `.gitmodules` with new paths

### Phase 3: Documentation and Tooling
1. Update documentation to reflect new structure
2. Create migration scripts and tools
3. Update build system to work with new structure
4. Create maintenance procedures

## Benefits

1. **Logical Organization**: Dependencies grouped by function and purpose
2. **Scalability**: Easy to add new dependencies in appropriate categories
3. **Maintainability**: Clear structure makes maintenance easier
4. **Performance**: Git operations can be more efficient with logical grouping
5. **Discoverability**: Developers can easily find relevant dependencies
6. **Architecture Reflection**: Structure mirrors the system's architectural principles

## Migration Notes

- Each dependency should be placed in the most specific appropriate category
- Some dependencies may span multiple categories - place in primary category
- Experimental implementations should be clearly marked
- Stable implementations should be production-ready and well-tested
- Bindings should be clearly separated from core implementations 