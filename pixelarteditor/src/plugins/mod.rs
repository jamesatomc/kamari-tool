use eframe::egui;
use crate::editor::PixelArtEditor;
use crate::types::Layer;

pub mod aseprite_plugin;
pub mod plugin_manager;
pub mod plugin_types;

pub use aseprite_plugin::*;
pub use plugin_manager::*;
pub use plugin_types::*;

/// Plugin API context that provides access to editor state
pub struct PluginContext<'a> {
    pub editor: &'a mut PixelArtEditor,
    pub ui: &'a mut egui::Ui,
    pub ctx: &'a egui::Context,
}

impl<'a> PluginContext<'a> {
    pub fn new(editor: &'a mut PixelArtEditor, ui: &'a mut egui::Ui, ctx: &'a egui::Context) -> Self {
        Self { editor, ui, ctx }
    }
    
    /// Get current active layer
    pub fn get_active_layer(&self) -> Option<&Layer> {
        self.editor.frames.get(self.editor.current_frame)?
            .layers.get(self.editor.current_layer)
    }
    
    /// Get mutable reference to current active layer
    pub fn get_active_layer_mut(&mut self) -> Option<&mut Layer> {
        self.editor.frames.get_mut(self.editor.current_frame)?
            .layers.get_mut(self.editor.current_layer)
    }
    
    /// Get canvas dimensions
    pub fn get_canvas_size(&self) -> (usize, usize) {
        if let Some(layer) = self.get_active_layer() {
            (layer.width(), layer.height())
        } else {
            (16, 16)
        }
    }
    
    /// Get current selected color
    pub fn get_selected_color(&self) -> egui::Color32 {
        self.editor.selected_color
    }
    
    /// Set selected color
    pub fn set_selected_color(&mut self, color: egui::Color32) {
        self.editor.selected_color = color;
    }
    
    /// Add a new layer
    pub fn add_layer(&mut self, name: String, width: usize, height: usize) {
        if let Some(frame) = self.editor.frames.get_mut(self.editor.current_frame) {
            frame.layers.push(Layer::new(name, width, height, egui::Color32::TRANSPARENT));
        }
    }
    
    /// Save current state for undo
    pub fn save_state(&mut self) {
        self.editor.last_state = Some((
            self.editor.frames.clone(),
            self.editor.current_frame,
            self.editor.current_layer,
        ));
    }
}
