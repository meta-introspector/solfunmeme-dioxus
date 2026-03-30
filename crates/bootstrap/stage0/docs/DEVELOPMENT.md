# Bootstrap Stage 0 Development Guide

## Getting Started

### Prerequisites

- Rust 1.70+ (nightly recommended for latest features)
- Basic understanding of Rust traits and generics
- Familiarity with cryptographic concepts
- Understanding of geometric projections

### Setup

```bash
# Clone the repository
git clone <repository-url>
cd solfunmeme-dioxus

# Navigate to stage0
cd crates/bootstrap/stage0

# Build the project
cargo build

# Run tests
cargo test

# Run with verbose output
cargo test --verbose
```

## Project Structure

```
crates/bootstrap/stage0/
├── src/
│   ├── lib.rs              # Main library file with exports
│   ├── kernel.rs           # Central orchestrator
│   ├── artifact.rs         # Content-addressable data units
│   ├── hash.rs             # Hash representation and traits
│   ├── manifold.rs         # 8D geometric projection
│   ├── chord.rs            # Distributed hash table
│   ├── proof.rs            # Cryptographic proofs
│   ├── verifier.rs         # Proof verification
│   ├── univalence.rs       # Identity transformation tracking
│   ├── default_hasher.rs   # Default checksum implementation
│   ├── default_verifier.rs # Default proof verifier
│   ├── describable.rs      # Canonical description trait
│   └── zos.rs              # Zero ontology system
├── docs/                   # Documentation
├── Cargo.toml             # Dependencies (empty for stage0)
├── components.manifest    # Component registry
└── README.md              # Project overview
```

## Development Principles

### 1. Zero Dependencies
Stage 0 must remain completely dependency-free. All functionality must be implemented using only the Rust standard library.

**Guidelines:**
- No external crates in `Cargo.toml`
- Use only `std::` and `core::` imports
- Implement cryptographic primitives from scratch if needed
- Avoid complex algorithms that require external libraries

### 2. Trait-Based Design
All major components are defined by traits to enable pluggability.

**Guidelines:**
- Define clear trait contracts
- Provide sensible default implementations
- Ensure traits are object-safe when possible
- Document trait requirements thoroughly

### 3. Univalence Principle
The system maintains identity through transformation paths.

**Guidelines:**
- Record all component changes in history
- Provide cryptographic proof of identity continuity
- Ensure transformations are reversible or traceable
- Maintain equivalence proofs for all operations

### 4. 42-Step Cycle
The Ouroboros cycle advances with each component rewrite.

**Guidelines:**
- Increment cycle counter on every rewrite
- Wrap cycle at 42 steps (0-41)
- Record cycle position in history
- Provide cycle-aware operations

## Adding New Components

### Step 1: Define the Trait

Create a new trait in an appropriate module or create a new module:

```rust
// src/new_component.rs
pub trait NewComponent {
    fn operation(&self, input: &[u8]) -> Result<Vec<u8>, &'static str>;
    fn component_id(&self) -> u64;
}
```

### Step 2: Implement Default Version

```rust
pub struct DefaultNewComponent;

impl NewComponent for DefaultNewComponent {
    fn operation(&self, input: &[u8]) -> Result<Vec<u8>, &'static str> {
        // Simple, dependency-free implementation
        Ok(input.to_vec())
    }
    
    fn component_id(&self) -> u64 {
        0 // Reserved for stage0 default
    }
}
```

### Step 3: Add to Kernel

Update the kernel to include the new component:

```rust
// In kernel.rs
pub struct Kernel {
    // ... existing fields ...
    new_component: Box<dyn NewComponent>,
}

impl Kernel {
    pub fn new(
        verifier: Box<dyn ProofVerifier>,
        projector: Box<dyn ManifoldProjector>,
        hasher: Box<dyn Hasher>,
        new_component: Box<dyn NewComponent>, // Add parameter
    ) -> Self {
        // ... implementation ...
    }
    
    pub fn rewrite_new_component(&mut self, new_component: Box<dyn NewComponent>) {
        self.new_component = new_component;
        self.history.path.push(RewriteOp::UpdateNewComponent);
        self.advance_cycle();
    }
}
```

### Step 4: Update RewriteOp Enum

```rust
// In univalence.rs
pub enum RewriteOp {
    UpdateHasher,
    UpdateProjector,
    UpdateNewComponent, // Add new variant
}
```

### Step 5: Add to Library Exports

```rust
// In lib.rs
pub mod new_component;

pub use new_component::{NewComponent, DefaultNewComponent};
```

### Step 6: Update Components Manifest

```rust
// In components.manifest
new_component
default_new_component
```

## Testing Guidelines

### Unit Tests

Each component should have comprehensive unit tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_creation() {
        let component = DefaultNewComponent;
        assert_eq!(component.component_id(), 0);
    }

    #[test]
    fn test_component_operation() {
        let component = DefaultNewComponent;
        let input = b"test data";
        let result = component.operation(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), input);
    }

    #[test]
    fn test_edge_cases() {
        let component = DefaultNewComponent;
        
        // Test empty input
        let result = component.operation(b"");
        assert!(result.is_ok());
        
        // Test large input
        let large_input = vec![0u8; 10000];
        let result = component.operation(&large_input);
        assert!(result.is_ok());
    }
}
```

### Integration Tests

Test component interactions in the kernel:

```rust
#[test]
fn test_kernel_with_new_component() {
    let mut kernel = Kernel::new(
        Box::new(DefaultProofVerifier),
        Box::new(DefaultProjector),
        Box::new(ChecksumHasher),
        Box::new(DefaultNewComponent),
    );
    
    // Test component replacement
    kernel.rewrite_new_component(Box::new(DefaultNewComponent));
    assert_eq!(kernel.step, 1);
    assert_eq!(kernel.history.path.len(), 1);
}
```

### Property-Based Tests

Use property-based testing for mathematical properties:

```rust
#[test]
fn test_manifold_projection_properties() {
    let projector = DefaultProjector;
    
    // Test normalization property
    let hash = Hash {
        algorithm_id: 0,
        hash_bytes: vec![1, 2, 3, 4, 5, 6, 7, 8],
    };
    
    let coord = projector.project(&hash);
    let mag_sq: f64 = coord.0.iter().map(|&x| x * x).sum();
    assert!((mag_sq - 1.0).abs() < 1e-9);
}
```

## Performance Considerations

### Memory Management

- Use `Vec<u8>` for binary data (efficient for stage0)
- Avoid unnecessary allocations in hot paths
- Consider using `Cow<[u8]>` for borrowed data when possible
- Monitor artifact storage growth

### Computational Efficiency

- Keep hash algorithms simple for stage0
- Use deterministic algorithms for reproducibility
- Avoid complex geometric calculations
- Profile critical paths

### Optimization Techniques

```rust
// Use const generics for fixed-size operations
fn process_chunks<const N: usize>(data: &[u8]) -> [u8; N] {
    // Efficient fixed-size processing
}

// Use iterators for memory efficiency
fn hash_stream<I: Iterator<Item = u8>>(iter: I) -> Hash {
    // Stream processing without full materialization
}
```

## Error Handling

### Error Types

Define clear error types for your components:

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ComponentError {
    InvalidInput,
    ProcessingFailed,
    ResourceExhausted,
}

impl std::fmt::Display for ComponentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput => write!(f, "Invalid input provided"),
            Self::ProcessingFailed => write!(f, "Processing operation failed"),
            Self::ResourceExhausted => write!(f, "Resource limit exceeded"),
        }
    }
}
```

### Error Propagation

Use consistent error handling patterns:

```rust
pub trait Component {
    fn process(&self, input: &[u8]) -> Result<Vec<u8>, ComponentError>;
}

impl Component for DefaultComponent {
    fn process(&self, input: &[u8]) -> Result<Vec<u8>, ComponentError> {
        if input.is_empty() {
            return Err(ComponentError::InvalidInput);
        }
        
        // Processing logic...
        Ok(input.to_vec())
    }
}
```

## Documentation Standards

### Code Documentation

All public APIs must be documented:

```rust
/// A component that processes binary data.
///
/// This trait defines the contract for data processing components
/// that can be dynamically loaded by the kernel.
///
/// # Examples
///
/// ```
/// use bootstrap_stage0::Component;
///
/// let component = DefaultComponent;
/// let result = component.process(b"hello");
/// assert!(result.is_ok());
/// ```
pub trait Component {
    /// Processes the given input data.
    ///
    /// # Arguments
    ///
    /// * `input` - The binary data to process
    ///
    /// # Returns
    ///
    /// Returns the processed data on success, or an error if processing fails.
    ///
    /// # Errors
    ///
    /// Returns `ComponentError::InvalidInput` if the input is invalid.
    fn process(&self, input: &[u8]) -> Result<Vec<u8>, ComponentError>;
}
```

### README Updates

Update the main README when adding significant features:

- Document new components
- Update usage examples
- Add performance notes
- Update architecture diagrams

## Contributing Workflow

### 1. Fork and Clone

```bash
git clone <your-fork-url>
cd solfunmeme-dioxus
git checkout -b feature/new-component
```

### 2. Implement Changes

- Follow the development principles
- Add comprehensive tests
- Update documentation
- Ensure zero dependencies

### 3. Test Thoroughly

```bash
# Run all tests
cargo test

# Run with verbose output
cargo test --verbose

# Check for warnings
cargo check

# Run clippy (if available)
cargo clippy
```

### 4. Update Documentation

- Update API documentation
- Add usage examples
- Update architecture docs
- Update README if needed

### 5. Submit Pull Request

- Clear description of changes
- Link to related issues
- Include test results
- Update components manifest

## Common Pitfalls

### 1. Adding Dependencies

**Problem:** Accidentally adding external crates
**Solution:** Always check `Cargo.toml` before committing

### 2. Breaking Trait Contracts

**Problem:** Changing trait signatures without consideration
**Solution:** Use default implementations for new methods

### 3. Ignoring Cycle Management

**Problem:** Forgetting to advance the 42-step cycle
**Solution:** Always call `advance_cycle()` in rewrite methods

### 4. Incomplete Error Handling

**Problem:** Not handling all error cases
**Solution:** Use exhaustive pattern matching

### 5. Memory Leaks

**Problem:** Growing data structures without bounds
**Solution:** Monitor storage growth and add limits

## Future Considerations

### Stage 1 Preparation

When designing components, consider how they might evolve in stage1:

- File I/O integration
- Persistent storage
- Network communication
- Plugin loading

### Backward Compatibility

- Maintain trait compatibility
- Use default implementations for new methods
- Provide migration paths
- Document breaking changes

### Performance Monitoring

- Add performance benchmarks
- Monitor memory usage
- Track cycle advancement
- Profile critical paths

## Conclusion

The Bootstrap Stage 0 development process emphasizes simplicity, correctness, and extensibility. By following these guidelines, contributors can help build a robust foundation for the solfunmeme-dioxus ecosystem while maintaining the core principles of minimalism and self-renewal.

Remember: Stage 0 is the foundation - keep it simple, keep it correct, and keep it extensible. 