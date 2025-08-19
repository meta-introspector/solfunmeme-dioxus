//! # Bootstrap Stage 0
//!
//! The core, dependency-free foundation of the microkernel. This crate
//! defines the fundamental traits and data structures for a content-addressable,
//! extensible system.
//!
//! It includes:
//! - A generic `Hasher` trait and `Hash` struct.
//! - A default, non-cryptographic `ChecksumHasher`.
//! - A core `Artifact` struct.
//! - A default, `InMemoryStore` for artifacts.

//! # Bootstrap Stage 0
//!
//! The core, dependency-free foundation of the microkernel. This crate
//! defines the fundamental traits and data structures for a content-addressable,
//! extensible, and self-rewriting system.
//!
//! ## Core Concepts
//!
//! - **Content-Addressable Storage**: All data (`Artifact`s) is identified by the
//!   hash of its content.
//! - **Pluggable Components**: Core functionalities like hashing (`Hasher`) and
//!   geometric projection (`ManifoldProjector`) are defined by traits, allowing
//!   for their implementations to be dynamically replaced.
//! - **Geometric Projection**: Artifacts are mapped to coordinates on an 8D
//!   hypersphere (`Manifold`), providing a spatial representation of all data.
//! - **Univalence**: The system explicitly models the Univalent Axiom by treating
//!   identity as a path of transformations. The `Kernel` records its own rewrite
//!   history, forming an `EquivalenceProof` of its identity over time.
//! - **Ouroboros Cycle**: The `Kernel` operates on a 42-step cycle, advancing
//!   each time a component is rewritten, symbolizing perpetual self-renewal.
//!
//! This crate is intentionally kept pure and dependency-free. It provides the
//! abstract architecture and default, in-memory components.

pub mod artifact;
pub mod chord;
pub mod default_hasher;
pub mod default_verifier;
pub mod describable;
pub mod hash;
pub mod kernel;
pub mod manifold;
pub mod proof;
pub mod univalence;
pub mod verifier;
pub mod zos;

pub use artifact::Artifact;
pub use chord::ChordStore;
pub use default_hasher::ChecksumHasher;
pub use default_verifier::DefaultProofVerifier;
pub use describable::Describable;
pub use hash::{Hash, Hasher};
pub use kernel::Kernel;
pub use manifold::{Coordinate, DefaultProjector, ManifoldProjector};
pub use proof::Proof;
pub use univalence::{EquivalenceProof, RewriteOp};
pub use verifier::ProofVerifier;
pub use zos::ZosComponent;
