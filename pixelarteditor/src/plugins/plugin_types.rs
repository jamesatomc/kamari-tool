use eframe::egui;
use crate::plugins::PluginContext;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Plugin metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub category: PluginCategory,
    pub aseprite_version: String,
    pub entry_point: String,
}

/// Plugin categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PluginCategory {
    Tool,
    Filter,
    Animation,
    Import,
    Export,
    Utility,
}

/// Plugin execution result
#[derive(Debug)]
pub enum PluginResult {
    Success,
    Error(String),
    Warning(String),
}

/// Plugin command definition
#[derive(Debug, Clone)]
pub struct PluginCommand {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: PluginCategory,
    pub shortcut: Option<String>,
}

/// Plugin dialog configuration
#[derive(Debug, Clone)]
pub struct PluginDialog {
    pub title: String,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub resizable: bool,
    pub modal: bool,
}

/// Plugin parameters for configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginParameter {
    Integer { name: String, value: i32, min: i32, max: i32 },
    Float { name: String, value: f32, min: f32, max: f32 },
    Boolean { name: String, value: bool },
    String { name: String, value: String },
    Color { name: String, value: [u8; 4] }, // RGBA
    Range { name: String, value: i32, min: i32, max: i32 },
}

impl PluginParameter {
    pub fn name(&self) -> &str {
        match self {
            PluginParameter::Integer { name, .. } => name,
            PluginParameter::Float { name, .. } => name,
            PluginParameter::Boolean { name, .. } => name,
            PluginParameter::String { name, .. } => name,
            PluginParameter::Color { name, .. } => name,
            PluginParameter::Range { name, .. } => name,
        }
    }
}

/// Plugin trait that all plugins must implement
pub trait Plugin: Send + Sync {
    fn metadata(&self) -> &PluginMetadata;
    fn commands(&self) -> Vec<PluginCommand>;
    fn execute(&mut self, command_id: &str, context: &mut PluginContext, params: &HashMap<String, PluginParameter>) -> PluginResult;
    fn show_dialog(&mut self, ui: &mut egui::Ui, params: &mut HashMap<String, PluginParameter>) -> bool;
    fn get_parameters(&self) -> Vec<PluginParameter>;
    fn can_execute(&self, command_id: &str) -> bool;
}

/// Plugin registry for managing installed plugins
pub struct PluginRegistry {
    pub plugins: HashMap<String, Box<dyn Plugin>>,
    pub commands: HashMap<String, String>, // command_id -> plugin_id
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            commands: HashMap::new(),
        }
    }
    
    pub fn register_plugin(&mut self, plugin: Box<dyn Plugin>) {
        let metadata = plugin.metadata();
        let plugin_id = metadata.name.clone();
        
        // Register all commands
        for command in plugin.commands() {
            self.commands.insert(command.id.clone(), plugin_id.clone());
        }
        
        self.plugins.insert(plugin_id, plugin);
    }
    
    pub fn get_plugin(&self, plugin_id: &str) -> Option<&Box<dyn Plugin>> {
        self.plugins.get(plugin_id)
    }
    
    pub fn get_plugin_mut(&mut self, plugin_id: &str) -> Option<&mut Box<dyn Plugin>> {
        self.plugins.get_mut(plugin_id)
    }
    
    pub fn find_plugin_by_command(&self, command_id: &str) -> Option<&Box<dyn Plugin>> {
        if let Some(plugin_id) = self.commands.get(command_id) {
            self.get_plugin(plugin_id)
        } else {
            None
        }
    }
    
    pub fn find_plugin_by_command_mut(&mut self, command_id: &str) -> Option<&mut Box<dyn Plugin>> {
        if let Some(plugin_id) = self.commands.get(command_id).cloned() {
            self.get_plugin_mut(&plugin_id)
        } else {
            None
        }
    }
    
    pub fn list_plugins(&self) -> Vec<&PluginMetadata> {
        self.plugins.values().map(|p| p.metadata()).collect()
    }
    
    pub fn list_commands(&self) -> Vec<PluginCommand> {
        self.plugins.values()
            .flat_map(|p| p.commands())
            .collect()
    }
    
    pub fn list_commands_by_category(&self, category: PluginCategory) -> Vec<PluginCommand> {
        self.list_commands().into_iter()
            .filter(|cmd| cmd.category == category)
            .collect()
    }
}
