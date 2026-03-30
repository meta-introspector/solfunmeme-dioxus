// Re-export core types that might be needed by other crates
pub use crate::component_name::ComponentName;
pub use crate::component_instance::ComponentInstance;
pub use crate::prop_value::PropValue;
pub use crate::registry::*;

mod component_name;
mod component_instance;
mod prop_value;
mod registry;
