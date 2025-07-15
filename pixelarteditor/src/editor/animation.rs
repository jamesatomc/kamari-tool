use crate::editor::core::PixelArtEditor;

impl PixelArtEditor {
    /// Execute a plugin command safely
    pub fn execute_plugin_command(&mut self, command_id: &str) {
        // Create a temporary reference to avoid borrowing issues
        match command_id {
            "blur" => {
                self.push_undo();
                if let Some(layer) = self.frames.get_mut(self.current_frame)
                    .and_then(|frame| frame.layers.get_mut(self.current_layer)) {
                    let blur_plugin = crate::plugins::aseprite_plugin::BlurPlugin::new();
                    blur_plugin.apply_blur(layer, 1.0);
                }
            }
            "noise" => {
                self.push_undo();
                if let Some(layer) = self.frames.get_mut(self.current_frame)
                    .and_then(|frame| frame.layers.get_mut(self.current_layer)) {
                    let noise_plugin = crate::plugins::aseprite_plugin::NoisePlugin::new();
                    noise_plugin.apply_noise(layer, 10.0);
                }
            }
            "outline" => {
                self.push_undo();
                if let Some(layer) = self.frames.get_mut(self.current_frame)
                    .and_then(|frame| frame.layers.get_mut(self.current_layer)) {
                    let outline_plugin = crate::plugins::aseprite_plugin::OutlinePlugin::new();
                    outline_plugin.apply_outline(layer, eframe::egui::Color32::BLACK, 1);
                }
            }
            "pixelate" => {
                self.push_undo();
                if let Some(layer) = self.frames.get_mut(self.current_frame)
                    .and_then(|frame| frame.layers.get_mut(self.current_layer)) {
                    let pixelate_plugin = crate::plugins::aseprite_plugin::PixelatePlugin::new();
                    pixelate_plugin.apply_pixelate(layer, 2);
                }
            }
            "color_replace" => {
                self.push_undo();
                if let Some(layer) = self.frames.get_mut(self.current_frame)
                    .and_then(|frame| frame.layers.get_mut(self.current_layer)) {
                    let color_replace_plugin = crate::plugins::aseprite_plugin::ColorReplacementPlugin::new();
                    color_replace_plugin.replace_color(layer, eframe::egui::Color32::WHITE, eframe::egui::Color32::BLACK, 0);
                }
            }
            _ => {
                eprintln!("Unknown plugin command: {}", command_id);
            }
        }
    }
}
