# Bootstrap Stage 0 API Reference

## Overview

This document provides a comprehensive reference for all public APIs in the Bootstrap Stage 0 microkernel.

## Core Types

### Kernel

The central orchestrator of the microkernel system.

```rust
pub struct Kernel {
    pub step: u64,
    pub history: EquivalenceProof,
    // Private fields: store, verifier, projector, hasher
}
```

#### Methods

##### `new()`
Creates a new kernel with the specified components.

```rust
pub fn new(
    verifier: Box<dyn ProofVerifier>,
    projector: Box<dyn ManifoldProjector>,
    hasher: Box<dyn Hasher>,
) -> Self
```

**Parameters:**
- `verifier`: Cryptographic proof verifier
- `projector`: Geometric projection component
- `hasher`: Content hashing component

**Returns:** A new kernel instance with step 0 and empty history.

##### `ingest()`
Ingests a proof into the kernel's verifier and places it on the Chord.

```rust
pub fn ingest(&mut self, proof: Proof) -> Result<(), &'static str>
```

**Parameters:**
- `proof`: Cryptographic proof to ingest

**Returns:** `Ok(())` on success, error message on failure.

##### `get_coordinate()`
Retrieves the geometric coordinate of an entity on the manifold.

```rust
pub fn get_coordinate(&self, hash: &Hash) -> Option<Coordinate>
```

**Parameters:**
- `hash`: Hash of the entity to locate

**Returns:** `Some(Coordinate)` if found, `None` otherwise.

##### `rewrite_hasher()`
Replaces the kernel's hasher with a new one, advancing the cycle.

```rust
pub fn rewrite_hasher(&mut self, new_hasher: Box<dyn Hasher>)
```

**Parameters:**
- `new_hasher`: New hasher implementation

**Side Effects:**
- Advances the 42-step cycle
- Records rewrite operation in history

##### `rewrite_projector()`
Replaces the kernel's projector with a new one, advancing the cycle.

```rust
pub fn rewrite_projector(&mut self, new_projector: Box<dyn ManifoldProjector>)
```

**Parameters:**
- `new_projector`: New projector implementation

**Side Effects:**
- Advances the 42-step cycle
- Records rewrite operation in history

### Artifact

Content-addressable data units.

```rust
pub struct Artifact {
    pub content: Vec<u8>,
    pub hashes: Vec<Hash>,
}
```

#### Methods

##### `new()`
Creates a new artifact from content using the provided hasher.

```rust
pub fn new(content: Vec<u8>, hasher: &dyn Hasher) -> Self
```

**Parameters:**
- `content`: Raw binary content
- `hasher`: Hashing algorithm to use

**Returns:** New artifact with computed hash.

### Hash

Generic hash representation.

```rust
pub struct Hash {
    pub algorithm_id: u64,
    pub hash_bytes: Vec<u8>,
}
```

**Fields:**
- `algorithm_id`: Unique identifier for the hashing algorithm
- `hash_bytes`: Raw hash output bytes

### Coordinate

8-dimensional geometric coordinate.

```rust
pub struct Coordinate(pub [f64; 8]);
```

#### Methods

##### `new()`
Creates a new coordinate from an 8D vector.

```rust
pub fn new(vector: [f64; 8]) -> Self
```

##### `normalize()`
Normalizes the coordinate to lie on the surface of a unit 8-sphere.

```rust
pub fn normalize(&self) -> Self
```

**Returns:** Normalized coordinate with magnitude ≤ 1.

### Proof

Cryptographic proof structure.

```rust
pub struct Proof {
    pub hash: Hash,
    pub content: Vec<u8>,
    pub metadata: ProofMetadata,
}
```

### EquivalenceProof

Identity transformation history.

```rust
pub struct EquivalenceProof {
    pub path: Vec<RewriteOp>,
}
```

### RewriteOp

Types of rewrite operations.

```rust
pub enum RewriteOp {
    UpdateHasher,
    UpdateProjector,
}
```

## Traits

### Hasher

Contract for pluggable hashing algorithms.

```rust
pub trait Hasher {
    fn algorithm_id(&self) -> u64;
    fn hash(&self, data: &[u8]) -> Hash;
}
```

#### Methods

##### `algorithm_id()`
Returns the unique identifier for this hashing algorithm.

```rust
fn algorithm_id(&self) -> u64
```

**Returns:** Algorithm identifier (0 is reserved for stage0 default).

##### `hash()`
Hashes the given data and returns a Hash struct.

```rust
fn hash(&self, data: &[u8]) -> Hash
```

**Parameters:**
- `data`: Raw bytes to hash

**Returns:** Hash struct with algorithm ID and hash bytes.

### ManifoldProjector

Contract for geometric projection algorithms.

```rust
pub trait ManifoldProjector {
    fn project(&self, hash: &Hash) -> Coordinate;
}
```

#### Methods

##### `project()`
Projects a hash to a coordinate in the 8D manifold.

```rust
fn project(&self, hash: &Hash) -> Coordinate
```

**Parameters:**
- `hash`: Hash to project

**Returns:** 8D coordinate on the unit hypersphere.

### ProofVerifier

Contract for cryptographic proof verification.

```rust
pub trait ProofVerifier {
    fn ingest_proof(&mut self, proof: Proof) -> Result<(), &'static str>;
}
```

#### Methods

##### `ingest_proof()`
Ingests and verifies a cryptographic proof.

```rust
fn ingest_proof(&mut self, proof: Proof) -> Result<(), &'static str>
```

**Parameters:**
- `proof`: Proof to verify

**Returns:** `Ok(())` if valid, error message if invalid.

### Describable

Contract for objects that can provide canonical descriptions.

```rust
pub trait Describable {
    fn describe(&self) -> &[u8];
}
```

#### Methods

##### `describe()`
Returns the canonical description of the object.

```rust
fn describe(&self) -> &[u8]
```

**Returns:** Canonical byte representation for hashing.

## Default Implementations

### ChecksumHasher

Default, non-cryptographic hasher for stage0.

```rust
pub struct ChecksumHasher;
```

**Algorithm ID:** 0 (reserved for stage0 default)

**Implementation:** Simple checksum using byte addition with wrapping arithmetic.

### DefaultProjector

Default geometric projector for stage0.

```rust
pub struct DefaultProjector;
```

**Implementation:** Splits hash bytes into 8 chunks, interprets as u64 values, normalizes to unit hypersphere.

### DefaultProofVerifier

Default proof verifier for stage0.

```rust
pub struct DefaultProofVerifier;
```

**Implementation:** Basic verification that proof hash matches content hash.

## Usage Examples

### Basic Kernel Usage

```rust
use bootstrap_stage0::{
    Kernel, ChecksumHasher, DefaultProjector, DefaultProofVerifier
};

// Create kernel
let mut kernel = Kernel::new(
    Box::new(DefaultProofVerifier),
    Box::new(DefaultProjector),
    Box::new(ChecksumHasher)
);

// Check initial state
assert_eq!(kernel.step, 0);
assert!(kernel.history.path.is_empty());
```

### Artifact Creation and Storage

```rust
use bootstrap_stage0::{Artifact, ChecksumHasher};

let hasher = ChecksumHasher;
let content = b"Hello, World!".to_vec();

// Create artifact
let artifact = Artifact::new(content.clone(), &hasher);

// Verify properties
assert_eq!(artifact.content, content);
assert_eq!(artifact.hashes.len(), 1);
assert_eq!(artifact.hashes[0], hasher.hash(&content));
```

### Component Rewriting

```rust
// Rewrite hasher
kernel.rewrite_hasher(Box::new(ChecksumHasher));
assert_eq!(kernel.step, 1);
assert_eq!(kernel.history.path.len(), 1);
assert_eq!(kernel.history.path[0], RewriteOp::UpdateHasher);

// Rewrite projector
kernel.rewrite_projector(Box::new(DefaultProjector));
assert_eq!(kernel.step, 2);
assert_eq!(kernel.history.path.len(), 2);
```

### Custom Hash Implementation

```rust
use bootstrap_stage0::{Hash, Hasher};

struct CustomHasher;

impl Hasher for CustomHasher {
    fn algorithm_id(&self) -> u64 {
        42 // Custom algorithm ID
    }
    
    fn hash(&self, data: &[u8]) -> Hash {
        // Custom hashing logic
        let mut sum = 0u64;
        for &byte in data {
            sum = sum.wrapping_add(byte as u64);
        }
        
        Hash {
            algorithm_id: self.algorithm_id(),
            hash_bytes: sum.to_be_bytes().to_vec(),
        }
    }
}
```

### Custom Projector Implementation

```rust
use bootstrap_stage0::{Coordinate, Hash, ManifoldProjector};

struct CustomProjector;

impl ManifoldProjector for CustomProjector {
    fn project(&self, hash: &Hash) -> Coordinate {
        // Custom projection logic
        let mut coords = [0.0; 8];
        for (i, &byte) in hash.hash_bytes.iter().take(8).enumerate() {
            coords[i] = byte as f64 / 255.0;
        }
        Coordinate::new(coords).normalize()
    }
}
```

## Error Handling

### Common Error Patterns

1. **Invalid Proof**: Proof verification fails
2. **Missing Artifact**: Hash not found in storage
3. **Algorithm Mismatch**: Hash algorithm ID not recognized
4. **Invalid Coordinates**: Projection produces invalid geometry

### Error Recovery

```rust
// Handle proof ingestion errors
match kernel.ingest(proof) {
    Ok(()) => println!("Proof ingested successfully"),
    Err(e) => println!("Proof ingestion failed: {}", e),
}

// Handle missing coordinates
match kernel.get_coordinate(&hash) {
    Some(coord) => println!("Found coordinate: {:?}", coord),
    None => println!("No coordinate found for hash"),
}
```

## Performance Considerations

### Memory Usage
- Artifacts store full content in memory
- Hash storage is O(1) per artifact
- History grows linearly with rewrite operations

### Computational Complexity
- Hash generation: O(n) where n is content size
- Manifold projection: O(1) for fixed-size hashes
- Component rewrite: O(1) for cycle advancement

### Optimization Tips
1. Reuse hasher instances when possible
2. Batch artifact operations when feasible
3. Consider content size for memory usage
4. Monitor history size for long-running systems

## Thread Safety

### Current Limitations
- Kernel is not thread-safe by default
- Components should be synchronized externally
- History modifications require exclusive access

### Future Considerations
- Thread-safe kernel implementation
- Concurrent component replacement
- Atomic rewrite operations

## Versioning

### API Stability
- Core traits are stable
- Default implementations may evolve
- New trait methods will be added with default implementations

### Migration Guide
- Algorithm IDs are reserved for stage0 default (0)
- Custom implementations should use IDs ≥ 1
- Coordinate normalization is always applied
- Proof verification is required for all ingested proofs 