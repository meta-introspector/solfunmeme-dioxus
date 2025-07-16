# Bootstrap Stage 0 Quick Reference

## Core Concepts

| Concept | Description | Key Properties |
|---------|-------------|----------------|
| **Artifact** | Content-addressable data unit | Immutable, hash-identified |
| **Hash** | Generic hash representation | Algorithm-agnostic, multi-hash support |
| **Manifold** | 8D geometric space | Unit hypersphere, normalized coordinates |
| **Kernel** | Central orchestrator | 42-step cycle, component management |
| **Univalence** | Identity transformation tracking | Equivalence proofs, rewrite history |

## Quick Start

### Basic Usage
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

// Store artifact
let content = b"Hello, World!".to_vec();
let hash = kernel.store_artifact(content).unwrap();

// Get coordinate
let coord = kernel.get_artifact_coordinate(&hash);
```

### Component Replacement
```rust
// Replace hasher (advances cycle)
kernel.rewrite_hasher(Box::new(ChecksumHasher));

// Replace projector (advances cycle)
kernel.rewrite_projector(Box::new(DefaultProjector));

// Check cycle position
println!("Step: {}", kernel.step); // 0-41
```

## API Reference

### Kernel Methods
| Method | Description | Returns |
|--------|-------------|---------|
| `new()` | Create kernel with components | `Kernel` |
| `ingest(proof)` | Ingest cryptographic proof | `Result<(), &str>` |
| `get_coordinate(hash)` | Get manifold coordinate | `Option<Coordinate>` |
| `rewrite_hasher(hasher)` | Replace hasher component | `()` |
| `rewrite_projector(projector)` | Replace projector component | `()` |

### Core Types
| Type | Purpose | Key Fields |
|------|---------|------------|
| `Artifact` | Content storage | `content: Vec<u8>`, `hashes: Vec<Hash>` |
| `Hash` | Hash representation | `algorithm_id: u64`, `hash_bytes: Vec<u8>` |
| `Coordinate` | 8D position | `0: [f64; 8]` |
| `Proof` | Cryptographic proof | `hash: Hash`, `content: Vec<u8>`, `metadata` |
| `EquivalenceProof` | Identity history | `path: Vec<RewriteOp>` |

### Traits
| Trait | Purpose | Key Methods |
|-------|---------|-------------|
| `Hasher` | Hash algorithms | `algorithm_id()`, `hash(data)` |
| `ManifoldProjector` | Geometric projection | `project(hash)` |
| `ProofVerifier` | Proof verification | `ingest_proof(proof)` |
| `Describable` | Canonical description | `describe()` |

## Mathematical Constants

| Constant | Value | Purpose |
|----------|-------|---------|
| **Cycle Steps** | 42 | Ouroboros cycle length |
| **Manifold Dimensions** | 8 | Geometric space dimensions |
| **Default Algorithm ID** | 0 | Stage0 checksum hasher |
| **Unit Sphere** | ||x|| = 1 | Manifold normalization |

## Default Implementations

| Component | Implementation | Algorithm ID |
|-----------|----------------|--------------|
| **Hasher** | `ChecksumHasher` | 0 |
| **Projector** | `DefaultProjector` | N/A |
| **Verifier** | `DefaultProofVerifier` | N/A |

## Error Handling

### Common Errors
| Error | Cause | Solution |
|-------|-------|----------|
| `"Artifact has no hashes"` | Empty hash list | Ensure artifact has at least one hash |
| `"Invalid proof"` | Proof verification failed | Check proof content and metadata |
| `"Hash not found"` | Artifact not in storage | Store artifact before lookup |

### Error Patterns
```rust
// Handle proof ingestion
match kernel.ingest(proof) {
    Ok(()) => println!("Success"),
    Err(e) => println!("Error: {}", e),
}

// Handle missing coordinates
match kernel.get_coordinate(&hash) {
    Some(coord) => println!("Found: {:?}", coord),
    None => println!("Not found"),
}
```

## Performance Notes

### Time Complexity
- **Hash Generation**: O(n) where n = content size
- **Manifold Projection**: O(1) for fixed-size hashes
- **Chord Lookup**: O(log n) for n artifacts
- **Component Rewrite**: O(1) for cycle advancement

### Memory Usage
- **Artifact Storage**: O(n) for n artifacts
- **Hash Storage**: O(1) per artifact
- **History Tracking**: O(m) for m rewrite operations
- **Coordinates**: O(1) per artifact

## Development Rules

### 1. Zero Dependencies
- No external crates in `Cargo.toml`
- Use only `std::` and `core::` imports
- Implement algorithms from scratch

### 2. Trait-Based Design
- Define traits for all major components
- Provide default implementations
- Ensure object safety

### 3. Univalence Principle
- Record all component changes
- Maintain identity through transformations
- Provide equivalence proofs

### 4. Cycle Management
- Advance cycle on every rewrite
- Wrap at 42 steps (0-41)
- Track cycle position in history

## Testing Patterns

### Unit Tests
```rust
#[test]
fn test_component() {
    let component = DefaultComponent;
    let result = component.operation(b"test");
    assert!(result.is_ok());
}
```

### Integration Tests
```rust
#[test]
fn test_kernel_workflow() {
    let mut kernel = create_test_kernel();
    
    // Test artifact storage
    let hash = kernel.store_artifact(b"test".to_vec()).unwrap();
    assert!(kernel.get_artifact_coordinate(&hash).is_some());
    
    // Test component rewrite
    kernel.rewrite_hasher(Box::new(ChecksumHasher));
    assert_eq!(kernel.step, 1);
}
```

### Property Tests
```rust
#[test]
fn test_manifold_properties() {
    let projector = DefaultProjector;
    let hash = create_test_hash();
    let coord = projector.project(&hash);
    
    // Test normalization
    let mag_sq: f64 = coord.0.iter().map(|&x| x * x).sum();
    assert!((mag_sq - 1.0).abs() < 1e-9);
}
```

## Common Patterns

### Custom Hash Implementation
```rust
struct CustomHasher;

impl Hasher for CustomHasher {
    fn algorithm_id(&self) -> u64 { 42 }
    fn hash(&self, data: &[u8]) -> Hash {
        // Custom implementation
        Hash { algorithm_id: 42, hash_bytes: vec![] }
    }
}
```

### Custom Projector Implementation
```rust
struct CustomProjector;

impl ManifoldProjector for CustomProjector {
    fn project(&self, hash: &Hash) -> Coordinate {
        // Custom projection
        Coordinate::new([0.0; 8]).normalize()
    }
}
```

### Component Registration
```rust
// Add to kernel
kernel.rewrite_hasher(Box::new(CustomHasher));
kernel.rewrite_projector(Box::new(CustomProjector));

// Update history
assert_eq!(kernel.step, 2);
assert_eq!(kernel.history.path.len(), 2);
```

## Troubleshooting

### Build Issues
- **No dependencies**: Ensure `Cargo.toml` has no `[dependencies]` section
- **Import errors**: Check `lib.rs` exports
- **Trait bounds**: Ensure traits are object-safe

### Runtime Issues
- **Cycle overflow**: Check step wrapping at 42
- **Hash collisions**: Verify hash algorithm properties
- **Memory growth**: Monitor artifact storage

### Test Issues
- **Failing tests**: Run `cargo test --verbose`
- **Missing components**: Check component manifest
- **Trait implementations**: Verify all required methods

## Future Extensions

### Stage 1 Planning
- File I/O integration
- Persistent storage
- Basic networking
- Plugin system

### Component Ideas
- Advanced hash algorithms
- Alternative projections
- Custom storage backends
- Specialized verifiers

### Mathematical Enhancements
- Higher-dimensional spaces
- Non-Euclidean geometry
- Advanced cryptography
- Quantum-resistant algorithms

## Resources

### Documentation
- `README.md` - Project overview
- `ARCHITECTURE.md` - Detailed design
- `API.md` - Complete API reference
- `DEVELOPMENT.md` - Development guide
- `MATHEMATICS.md` - Mathematical foundations

### Code Examples
- Unit tests in each module
- Integration tests in kernel
- Property-based tests
- Documentation examples

### External References
- Homotopy Type Theory
- Distributed Hash Tables
- Geometric Projections
- Cryptographic Hash Functions 