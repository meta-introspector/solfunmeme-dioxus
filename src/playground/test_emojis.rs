// Re-export the component builder from the new modular crates
pub use component_builder_lib::ComponentBuilderEmojiApp;

// For backward compatibility, we can also re-export the types if needed
pub use component_registry_lib::{ComponentName, ComponentInstance, PropValue};
pub use component_emoji_lib::{get_emoji, get_emoji_style};
