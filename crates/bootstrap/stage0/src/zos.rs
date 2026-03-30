//! Defines the `ZosComponent` trait.

use crate::proof::Proof;

/// A trait for a Zero Ontology System (ZOS) component.
///
/// This is the contract that all pluggable components in the system must adhere to.
/// It is intended to be implemented manually.
pub trait ZosComponent {
    /// A unique, human-readable name for the component.
    /// This name MUST be listed in `components.manifest` to ensure uniqueness.
    fn name(&self) -> &'static str;

    /// Produces a proof of the component's uniqueness.
    fn prove(&self) -> Proof;

    /// The core logic of the component.
    fn execute(&self);
}
