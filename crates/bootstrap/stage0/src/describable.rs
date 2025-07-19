//! Defines the `Describable` trait for canonical self-description.

use std::borrow::Cow;

/// A trait for types that can produce a canonical, byte-based representation
/// of themselves. This description is used for hashing and unique identification.
///
/// The core idea is that any object that can be uniquely identified must be able
/// to present itself as a stable sequence of bytes. This sequence is then used
/// by a `Hasher` to generate a content-based `Hash`.
pub trait Describable {
    /// Returns the canonical byte representation of the object.
    ///
    /// This method should be deterministic and produce the same byte slice for
    /// the same object content every time.
fn describe(&self) -> Cow<'static, [u8]>;
}
