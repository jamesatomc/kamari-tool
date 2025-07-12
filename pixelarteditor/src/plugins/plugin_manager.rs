use eframe::egui;
use crate::plugins::{PluginContext, PluginRegistry, PluginParameter, PluginResult};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use rfd::FileDialog;
use serde_json;

/// Plugin manager handles plugin installation, loading, and execution
pub struct PluginManager {
    pub registry: PluginRegistry,
    pub plugin_dir: PathBuf,
    pub show_plugin_dialog: bool,
    pub show_install_dialog: bool,
    pub active_plugin_params: HashMap<String, PluginParameter>,
    pub active_plugin_id: Option<String>,
    pub active_command_id: Option<String>,
    pub plugin_install_path: String,
}

impl PluginManager {
    pub fn new() -> Self {
        let plugin_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("kamari-tool")
            .join("plugins");
        
        Self {
            registry: PluginRegistry::new(),
            plugin_dir,
            show_plugin_dialog: false,
            show_install_dialog: false,
            active_plugin_params: HashMap::new(),
            active_plugin_id: None,
            active_command_id: None,
            plugin_install_path: String::new(),
        }
    }
    
    /// Initialize plugin manager and load plugins from disk
    pub fn initialize(&mut self) {
        // Create plugin directory if it doesn't exist
        if !self.plugin_dir.exists() {
            fs::create_dir_all(&self.plugin_dir).ok();
        }
        
        // Load built-in plugins
        self.load_builtin_plugins();
        
        // Load plugins from disk
        self.load_plugins_from_disk();
    }
    
    /// Load built-in plugins
    fn load_builtin_plugins(&mut self) {
        // Add built-in Aseprite-compatible plugins
        self.registry.register_plugin(Box::new(crate::plugins::aseprite_plugin::BlurPlugin::new()));
        self.registry.register_plugin(Box::new(crate::plugins::aseprite_plugin::NoisePlugin::new()));
        self.registry.register_plugin(Box::new(crate::plugins::aseprite_plugin::OutlinePlugin::new()));
        self.registry.register_plugin(Box::new(crate::plugins::aseprite_plugin::PixelatePlugin::new()));
        self.registry.register_plugin(Box::new(crate::plugins::aseprite_plugin::ColorReplacementPlugin::new()));
    }
    
    /// Load plugins from disk
    fn load_plugins_from_disk(&mut self) {
        if let Ok(entries) = fs::read_dir(&self.plugin_dir) {
            for entry in entries.flatten() {
                if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                    self.load_plugin_from_directory(&entry.path());
                }
            }
        }
    }
    
    /// Load a plugin from a directory
    fn load_plugin_from_directory(&mut self, path: &Path) {
        let manifest_path = path.join("manifest.json");
        
        if let Ok(manifest_content) = fs::read_to_string(&manifest_path) {
            if let Ok(metadata) = serde_json::from_str::<crate::plugins::PluginMetadata>(&manifest_content) {
                // For now, we'll just log that we found a plugin
                // In a full implementation, we'd load JavaScript or Lua scripts
                println!("Found plugin: {} v{}", metadata.name, metadata.version);
            }
        }
    }
    
    /// Install a plugin from a file
    pub fn install_plugin(&mut self, plugin_path: &Path) -> Result<(), String> {
        // Check if it's a ZIP file (Aseprite plugin format)
        if plugin_path.extension().and_then(|ext| ext.to_str()) == Some("zip") {
            self.install_aseprite_plugin(plugin_path)
        } else {
            Err("Unsupported plugin format".to_string())
        }
    }
    
    /// Install an Aseprite plugin (ZIP format)
    fn install_aseprite_plugin(&mut self, zip_path: &Path) -> Result<(), String> {
        // Extract plugin name from filename
        let plugin_name = zip_path.file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown_plugin");
        
        let plugin_install_dir = self.plugin_dir.join(plugin_name);
        
        // Create plugin directory
        fs::create_dir_all(&plugin_install_dir)
            .map_err(|e| format!("Failed to create plugin directory: {}", e))?;
        
        // For now, we'll just copy the ZIP file
        // In a full implementation, we'd extract the ZIP
        let dest_path = plugin_install_dir.join("plugin.zip");
        fs::copy(zip_path, dest_path)
            .map_err(|e| format!("Failed to copy plugin file: {}", e))?;
        
        // Create a basic manifest
        let manifest = crate::plugins::PluginMetadata {
            name: plugin_name.to_string(),
            version: "1.0.0".to_string(),
            author: "Unknown".to_string(),
            description: "Installed Aseprite plugin".to_string(),
            category: crate::plugins::PluginCategory::Utility,
            aseprite_version: "1.0".to_string(),
            entry_point: "plugin.zip".to_string(),
        };
        
        let manifest_json = serde_json::to_string_pretty(&manifest)
            .map_err(|e| format!("Failed to serialize manifest: {}", e))?;
        
        fs::write(plugin_install_dir.join("manifest.json"), manifest_json)
            .map_err(|e| format!("Failed to write manifest: {}", e))?;
        
        Ok(())
    }
    
    /// Execute a plugin command
    pub fn execute_command(&mut self, command_id: &str, context: &mut PluginContext) -> PluginResult {
        if let Some(plugin) = self.registry.find_plugin_by_command_mut(command_id) {
            plugin.execute(command_id, context, &self.active_plugin_params)
        } else {
            PluginResult::Error(format!("Command not found: {}", command_id))
        }
    }
    
    /// Execute a plugin command without UI context (simpler version)
    pub fn execute_command_simple(&mut self, command_id: &str, editor: &mut crate::editor::PixelArtEditor) -> PluginResult {
        if let Some(plugin) = self.registry.find_plugin_by_command_mut(command_id) {
            // Create a minimal context for the plugin
            let mut temp_params = std::collections::HashMap::new();
            
            // Load default parameters for the plugin
            for param in plugin.get_parameters() {
                temp_params.insert(param.name().to_string(), param);
            }
            
            // Execute plugin directly on the editor
            // For now, we'll handle the common plugin operations directly
            match command_id {

                _ => PluginResult::Error(format!("Unknown command: {}", command_id))
            }
        } else {
            PluginResult::Error(format!("Command not found: {}", command_id))
        }
    }
    
    /// Execute a plugin by name
    pub fn execute_plugin(&self, plugin_name: &str, editor: &mut crate::editor::PixelArtEditor) -> PluginResult {
        // Check if it's a built-in plugin first
        if let Some(result) = self.execute_builtin_plugin(plugin_name, editor) {
            return result;
        }
        
        // For now, we only support built-in plugins
        PluginResult::Error(format!("Plugin '{}' not found", plugin_name))
    }
    
    /// Execute built-in plugins
    fn execute_builtin_plugin(&self, plugin_name: &str, editor: &mut crate::editor::PixelArtEditor) -> Option<PluginResult> {
        match plugin_name {

            _ => None
        }
    }
    
    /// Show plugin manager UI
    pub fn show_plugin_manager(&mut self, ctx: &egui::Context) {
        if self.show_plugin_dialog {
            egui::Window::new("Plugin Manager")
                .collapsible(false)
                .resizable(true)
                .default_size(egui::vec2(500.0, 400.0))
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Install Plugin").clicked() {
                            self.show_install_dialog = true;
                        }
                        ui.separator();
                        if ui.button("Refresh").clicked() {
                            self.load_plugins_from_disk();
                        }
                    });
                    
                    ui.separator();
                    
                    egui::ScrollArea::vertical()
                        .max_height(300.0)
                        .show(ui, |ui| {
                            for metadata in self.registry.list_plugins() {
                                ui.group(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.vertical(|ui| {
                                            ui.heading(&metadata.name);
                                            ui.label(format!("Version: {}", metadata.version));
                                            ui.label(format!("Author: {}", metadata.author));
                                            ui.label(&metadata.description);
                                        });
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            if ui.button("Configure").clicked() {
                                                self.active_plugin_id = Some(metadata.name.clone());
                                                // Load plugin parameters
                                                if let Some(plugin) = self.registry.get_plugin(&metadata.name) {
                                                    self.active_plugin_params.clear();
                                                    for param in plugin.get_parameters() {
                                                        self.active_plugin_params.insert(param.name().to_string(), param);
                                                    }
                                                }
                                            }
                                        });
                                    });
                                });
                            }
                        });
                    
                    ui.separator();
                    
                    ui.horizontal(|ui| {
                        if ui.button("Close").clicked() {
                            self.show_plugin_dialog = false;
                        }
                    });
                });
        }
        
        // Show install dialog
        if self.show_install_dialog {
            self.show_install_dialog(ctx);
        }
    }
    
    /// Show plugin installation dialog
    fn show_install_dialog(&mut self, ctx: &egui::Context) {
        egui::Window::new("Install Plugin")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("Select plugin file to install:");
                
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.plugin_install_path);
                    if ui.button("Browse").clicked() {
                        if let Some(path) = FileDialog::new()
                            .add_filter("Aseprite Plugin", &["zip"])
                            .pick_file()
                        {
                            self.plugin_install_path = path.to_string_lossy().to_string();
                        }
                    }
                });
                
                ui.separator();
                
                ui.horizontal(|ui| {
                    if ui.button("Install").clicked() {
                        let path_str = self.plugin_install_path.clone();
                        let path = Path::new(&path_str);
                        match self.install_plugin(path) {
                            Ok(_) => {
                                self.show_install_dialog = false;
                                self.plugin_install_path.clear();
                                self.load_plugins_from_disk();
                            }
                            Err(e) => {
                                // Show error message
                                eprintln!("Plugin installation failed: {}", e);
                            }
                        }
                    }
                    if ui.button("Cancel").clicked() {
                        self.show_install_dialog = false;
                        self.plugin_install_path.clear();
                    }
                });
            });
    }
    
    /// Show plugin configuration dialog
    pub fn show_plugin_config(&mut self, ctx: &egui::Context) {
        if let Some(plugin_id) = &self.active_plugin_id.clone() {
            if let Some(plugin) = self.registry.get_plugin_mut(plugin_id) {
                let should_close = egui::Window::new(format!("Configure {}", plugin.metadata().name))
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        plugin.show_dialog(ui, &mut self.active_plugin_params)
                    })
                    .map_or(false, |r| r.inner.unwrap_or(false));
                
                if should_close {
                    self.active_plugin_id = None;
                    self.active_plugin_params.clear();
                }
            }
        }
    }
    
    /// Get plugin commands for menu integration
    pub fn get_plugin_commands(&self) -> Vec<crate::plugins::PluginCommand> {
        self.registry.list_commands()
    }
    
    /// Get plugin commands by category
    pub fn get_plugin_commands_by_category(&self, category: crate::plugins::PluginCategory) -> Vec<crate::plugins::PluginCommand> {
        self.registry.list_commands_by_category(category)
    }
}
