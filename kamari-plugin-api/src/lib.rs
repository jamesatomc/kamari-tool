pub mod api;
pub mod types;
pub mod macros;

pub use types::*;

/// Version of the Kamari Plugin API
pub const API_VERSION: &str = "1.0.0";

/// Plugin manifest version
pub const MANIFEST_VERSION: &str = "1.0";

/// Re-export commonly used types
pub use serde::{Deserialize, Serialize};
pub use serde_json;

/// Plugin trait that all external plugins must implement
pub trait KamariPlugin {
    /// Get plugin metadata
    fn metadata(&self) -> PluginMetadata;
    
    /// Execute the plugin with the given context and parameters
    fn execute(&self, context: &mut PluginContext, params: &[PluginParameter]) -> Result<(), String>;
    
    /// Called when the plugin is loaded (optional)
    fn on_load(&self) -> Result<(), String> {
        Ok(())
    }
    
    /// Called when the plugin is unloaded (optional)
    fn on_unload(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Entry point for plugins
pub trait PluginEntry {
    fn create_plugin() -> Box<dyn KamariPlugin>;
}
