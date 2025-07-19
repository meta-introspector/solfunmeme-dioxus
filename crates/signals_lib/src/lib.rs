// Signal Management Library - Simplified pattern for Dioxus signals
// This provides a scalable pattern for managing signals in Dioxus components

pub mod common;
pub mod example_usage;
pub mod module_interface;
pub mod rdf_signal;

// Re-export the main signal management types
pub use common::{SignalState, SignalManager, use_signal_manager};
pub use example_usage::{WalletSignalManager, WalletState, ConnectionInfo};
pub use module_interface::{ModuleInterface, ModuleWrapper, ModuleRegistry}; 