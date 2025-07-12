use eframe::egui;
use crate::plugins::{Plugin, PluginMetadata, PluginCategory, PluginCommand, PluginResult, PluginContext, PluginParameter};
use std::collections::HashMap;
use rand::Rng;

/// Blur filter plugin
pub struct BlurPlugin {
    metadata: PluginMetadata,
}

impl BlurPlugin {
    pub fn new() -> Self {
        Self {
            metadata: PluginMetadata {
                name: "Blur".to_string(),
                version: "1.0.0".to_string(),
                author: "Kamari Tool".to_string(),
                description: "Apply blur effect to the image".to_string(),
                category: PluginCategory::Filter,
                aseprite_version: "1.0".to_string(),
                entry_point: "blur".to_string(),
            },
        }
    }
    
    pub fn apply_blur(&self, layer: &mut crate::types::Layer, radius: f32) {
        let width = layer.width();
        let height = layer.height();
        let mut new_grid = layer.grid.clone();
        
        for y in 0..height {
            for x in 0..width {
                let mut r_sum = 0.0;
                let mut g_sum = 0.0;
                let mut b_sum = 0.0;
                let mut a_sum = 0.0;
                let mut count = 0;
                
                let radius_i = radius as i32;
                for dy in -radius_i..=radius_i {
                    for dx in -radius_i..=radius_i {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        
                        if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                            let pixel = layer.grid[ny as usize][nx as usize];
                            r_sum += pixel.r() as f32;
                            g_sum += pixel.g() as f32;
                            b_sum += pixel.b() as f32;
                            a_sum += pixel.a() as f32;
                            count += 1;
                        }
                    }
                }
                
                if count > 0 {
                    new_grid[y][x] = egui::Color32::from_rgba_unmultiplied(
                        (r_sum / count as f32) as u8,
                        (g_sum / count as f32) as u8,
                        (b_sum / count as f32) as u8,
                        (a_sum / count as f32) as u8,
                    );
                }
            }
        }
        
        layer.grid = new_grid;
    }
}

impl Plugin for BlurPlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }
    
    fn commands(&self) -> Vec<PluginCommand> {
        vec![
            PluginCommand {
                id: "blur".to_string(),
                name: "Blur".to_string(),
                description: "Apply blur effect".to_string(),
                category: PluginCategory::Filter,
                shortcut: Some("Ctrl+B".to_string()),
            }
        ]
    }
    
    fn execute(&mut self, _command_id: &str, context: &mut PluginContext, params: &HashMap<String, PluginParameter>) -> PluginResult {
        context.save_state();
        
        if let Some(layer) = context.get_active_layer_mut() {
            let radius = if let Some(PluginParameter::Float { value, .. }) = params.get("radius") {
                *value
            } else {
                1.0
            };
            
            self.apply_blur(layer, radius);
            PluginResult::Success
        } else {
            PluginResult::Error("No active layer".to_string())
        }
    }
    
    fn show_dialog(&mut self, ui: &mut egui::Ui, params: &mut HashMap<String, PluginParameter>) -> bool {
        let mut should_close = false;
        
        ui.heading("Blur Filter");
        ui.separator();
        
        if let Some(PluginParameter::Float { value, min, max, .. }) = params.get_mut("radius") {
            ui.add(egui::Slider::new(value, *min..=*max).text("Radius"));
        }
        
        ui.separator();
        
        ui.horizontal(|ui| {
            if ui.button("Apply").clicked() {
                should_close = true;
            }
            if ui.button("Cancel").clicked() {
                should_close = true;
            }
        });
        
        should_close
    }
    
    fn get_parameters(&self) -> Vec<PluginParameter> {
        vec![
            PluginParameter::Float {
                name: "radius".to_string(),
                value: 1.0,
                min: 0.5,
                max: 10.0,
            }
        ]
    }
    
    fn can_execute(&self, command_id: &str) -> bool {
        command_id == "blur"
    }
}

/// Noise filter plugin
pub struct NoisePlugin {
    metadata: PluginMetadata,
}

impl NoisePlugin {
    pub fn new() -> Self {
        Self {
            metadata: PluginMetadata {
                name: "Noise".to_string(),
                version: "1.0.0".to_string(),
                author: "Kamari Tool".to_string(),
                description: "Add noise to the image".to_string(),
                category: PluginCategory::Filter,
                aseprite_version: "1.0".to_string(),
                entry_point: "noise".to_string(),
            },
        }
    }
    
    pub fn apply_noise(&self, layer: &mut crate::types::Layer, intensity: f32) {
        let width = layer.width();
        let height = layer.height();
        let mut rng = rand::thread_rng();
        
        for y in 0..height {
            for x in 0..width {
                let pixel = layer.grid[y][x];
                if pixel.a() > 0 {
                    let noise = rng.gen_range(-intensity..=intensity);
                    let r = (pixel.r() as f32 + noise).clamp(0.0, 255.0) as u8;
                    let g = (pixel.g() as f32 + noise).clamp(0.0, 255.0) as u8;
                    let b = (pixel.b() as f32 + noise).clamp(0.0, 255.0) as u8;
                    
                    layer.grid[y][x] = egui::Color32::from_rgba_unmultiplied(r, g, b, pixel.a());
                }
            }
        }
    }
}

impl Plugin for NoisePlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }
    
    fn commands(&self) -> Vec<PluginCommand> {
        vec![
            PluginCommand {
                id: "noise".to_string(),
                name: "Add Noise".to_string(),
                description: "Add noise to image".to_string(),
                category: PluginCategory::Filter,
                shortcut: Some("Ctrl+N".to_string()),
            }
        ]
    }
    
    fn execute(&mut self, _command_id: &str, context: &mut PluginContext, params: &HashMap<String, PluginParameter>) -> PluginResult {
        context.save_state();
        
        if let Some(layer) = context.get_active_layer_mut() {
            let intensity = if let Some(PluginParameter::Float { value, .. }) = params.get("intensity") {
                *value
            } else {
                10.0
            };
            
            self.apply_noise(layer, intensity);
            PluginResult::Success
        } else {
            PluginResult::Error("No active layer".to_string())
        }
    }
    
    fn show_dialog(&mut self, ui: &mut egui::Ui, params: &mut HashMap<String, PluginParameter>) -> bool {
        let mut should_close = false;
        
        ui.heading("Add Noise");
        ui.separator();
        
        if let Some(PluginParameter::Float { value, min, max, .. }) = params.get_mut("intensity") {
            ui.add(egui::Slider::new(value, *min..=*max).text("Intensity"));
        }
        
        ui.separator();
        
        ui.horizontal(|ui| {
            if ui.button("Apply").clicked() {
                should_close = true;
            }
            if ui.button("Cancel").clicked() {
                should_close = true;
            }
        });
        
        should_close
    }
    
    fn get_parameters(&self) -> Vec<PluginParameter> {
        vec![
            PluginParameter::Float {
                name: "intensity".to_string(),
                value: 10.0,
                min: 1.0,
                max: 50.0,
            }
        ]
    }
    
    fn can_execute(&self, command_id: &str) -> bool {
        command_id == "noise"
    }
}

/// Outline plugin
pub struct OutlinePlugin {
    metadata: PluginMetadata,
}

impl OutlinePlugin {
    pub fn new() -> Self {
        Self {
            metadata: PluginMetadata {
                name: "Outline".to_string(),
                version: "1.0.0".to_string(),
                author: "Kamari Tool".to_string(),
                description: "Add outline to sprites".to_string(),
                category: PluginCategory::Filter,
                aseprite_version: "1.0".to_string(),
                entry_point: "outline".to_string(),
            },
        }
    }
    
    pub fn apply_outline(&self, layer: &mut crate::types::Layer, color: egui::Color32, thickness: usize) {
        let width = layer.width();
        let height = layer.height();
        let mut new_grid = layer.grid.clone();
        
        for y in 0..height {
            for x in 0..width {
                if layer.grid[y][x].a() > 0 {
                    // Check surrounding pixels
                    for dy in -(thickness as i32)..=(thickness as i32) {
                        for dx in -(thickness as i32)..=(thickness as i32) {
                            if dx == 0 && dy == 0 { continue; }
                            
                            let nx = x as i32 + dx;
                            let ny = y as i32 + dy;
                            
                            if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                                let nx = nx as usize;
                                let ny = ny as usize;
                                
                                if layer.grid[ny][nx].a() == 0 {
                                    // Distance check for circular outline
                                    let dist = ((dx * dx + dy * dy) as f32).sqrt();
                                    if dist <= thickness as f32 {
                                        new_grid[ny][nx] = color;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        layer.grid = new_grid;
    }
}

impl Plugin for OutlinePlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }
    
    fn commands(&self) -> Vec<PluginCommand> {
        vec![
            PluginCommand {
                id: "outline".to_string(),
                name: "Add Outline".to_string(),
                description: "Add outline to sprites".to_string(),
                category: PluginCategory::Filter,
                shortcut: Some("Ctrl+O".to_string()),
            }
        ]
    }
    
    fn execute(&mut self, _command_id: &str, context: &mut PluginContext, params: &HashMap<String, PluginParameter>) -> PluginResult {
        context.save_state();
        
        if let Some(layer) = context.get_active_layer_mut() {
            let thickness = if let Some(PluginParameter::Integer { value, .. }) = params.get("thickness") {
                *value as usize
            } else {
                1
            };
            
            let color = if let Some(PluginParameter::Color { value, .. }) = params.get("color") {
                egui::Color32::from_rgba_unmultiplied(value[0], value[1], value[2], value[3])
            } else {
                egui::Color32::BLACK
            };
            
            self.apply_outline(layer, color, thickness);
            PluginResult::Success
        } else {
            PluginResult::Error("No active layer".to_string())
        }
    }
    
    fn show_dialog(&mut self, ui: &mut egui::Ui, params: &mut HashMap<String, PluginParameter>) -> bool {
        let mut should_close = false;
        
        ui.heading("Add Outline");
        ui.separator();
        
        if let Some(PluginParameter::Integer { value, min, max, .. }) = params.get_mut("thickness") {
            ui.add(egui::Slider::new(value, *min..=*max).text("Thickness"));
        }
        
        if let Some(PluginParameter::Color { value, .. }) = params.get_mut("color") {
            let mut color = egui::Color32::from_rgba_unmultiplied(value[0], value[1], value[2], value[3]);
            ui.color_edit_button_srgba(&mut color);
            *value = [color.r(), color.g(), color.b(), color.a()];
        }
        
        ui.separator();
        
        ui.horizontal(|ui| {
            if ui.button("Apply").clicked() {
                should_close = true;
            }
            if ui.button("Cancel").clicked() {
                should_close = true;
            }
        });
        
        should_close
    }
    
    fn get_parameters(&self) -> Vec<PluginParameter> {
        vec![
            PluginParameter::Integer {
                name: "thickness".to_string(),
                value: 1,
                min: 1,
                max: 5,
            },
            PluginParameter::Color {
                name: "color".to_string(),
                value: [0, 0, 0, 255],
            }
        ]
    }
    
    fn can_execute(&self, command_id: &str) -> bool {
        command_id == "outline"
    }
}

/// Pixelate plugin
pub struct PixelatePlugin {
    metadata: PluginMetadata,
}

impl PixelatePlugin {
    pub fn new() -> Self {
        Self {
            metadata: PluginMetadata {
                name: "Pixelate".to_string(),
                version: "1.0.0".to_string(),
                author: "Kamari Tool".to_string(),
                description: "Apply pixelation effect".to_string(),
                category: PluginCategory::Filter,
                aseprite_version: "1.0".to_string(),
                entry_point: "pixelate".to_string(),
            },
        }
    }
    
    pub fn apply_pixelate(&self, layer: &mut crate::types::Layer, block_size: usize) {
        let width = layer.width();
        let height = layer.height();
        let mut new_grid = layer.grid.clone();
        
        for y in (0..height).step_by(block_size) {
            for x in (0..width).step_by(block_size) {
                // Calculate average color of the block
                let mut r_sum = 0;
                let mut g_sum = 0;
                let mut b_sum = 0;
                let mut a_sum = 0;
                let mut count = 0;
                
                for by in y..std::cmp::min(y + block_size, height) {
                    for bx in x..std::cmp::min(x + block_size, width) {
                        let pixel = layer.grid[by][bx];
                        r_sum += pixel.r() as u32;
                        g_sum += pixel.g() as u32;
                        b_sum += pixel.b() as u32;
                        a_sum += pixel.a() as u32;
                        count += 1;
                    }
                }
                
                if count > 0 {
                    let avg_color = egui::Color32::from_rgba_unmultiplied(
                        (r_sum / count) as u8,
                        (g_sum / count) as u8,
                        (b_sum / count) as u8,
                        (a_sum / count) as u8,
                    );
                    
                    // Apply average color to the entire block
                    for by in y..std::cmp::min(y + block_size, height) {
                        for bx in x..std::cmp::min(x + block_size, width) {
                            new_grid[by][bx] = avg_color;
                        }
                    }
                }
            }
        }
        
        layer.grid = new_grid;
    }
}

impl Plugin for PixelatePlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }
    
    fn commands(&self) -> Vec<PluginCommand> {
        vec![
            PluginCommand {
                id: "pixelate".to_string(),
                name: "Pixelate".to_string(),
                description: "Apply pixelation effect".to_string(),
                category: PluginCategory::Filter,
                shortcut: Some("Ctrl+P".to_string()),
            }
        ]
    }
    
    fn execute(&mut self, _command_id: &str, context: &mut PluginContext, params: &HashMap<String, PluginParameter>) -> PluginResult {
        context.save_state();
        
        if let Some(layer) = context.get_active_layer_mut() {
            let block_size = if let Some(PluginParameter::Integer { value, .. }) = params.get("block_size") {
                *value as usize
            } else {
                2
            };
            
            self.apply_pixelate(layer, block_size);
            PluginResult::Success
        } else {
            PluginResult::Error("No active layer".to_string())
        }
    }
    
    fn show_dialog(&mut self, ui: &mut egui::Ui, params: &mut HashMap<String, PluginParameter>) -> bool {
        let mut should_close = false;
        
        ui.heading("Pixelate");
        ui.separator();
        
        if let Some(PluginParameter::Integer { value, min, max, .. }) = params.get_mut("block_size") {
            ui.add(egui::Slider::new(value, *min..=*max).text("Block Size"));
        }
        
        ui.separator();
        
        ui.horizontal(|ui| {
            if ui.button("Apply").clicked() {
                should_close = true;
            }
            if ui.button("Cancel").clicked() {
                should_close = true;
            }
        });
        
        should_close
    }
    
    fn get_parameters(&self) -> Vec<PluginParameter> {
        vec![
            PluginParameter::Integer {
                name: "block_size".to_string(),
                value: 2,
                min: 1,
                max: 16,
            }
        ]
    }
    
    fn can_execute(&self, command_id: &str) -> bool {
        command_id == "pixelate"
    }
}

/// Color replacement plugin
pub struct ColorReplacementPlugin {
    metadata: PluginMetadata,
}

impl ColorReplacementPlugin {
    pub fn new() -> Self {
        Self {
            metadata: PluginMetadata {
                name: "Color Replace".to_string(),
                version: "1.0.0".to_string(),
                author: "Kamari Tool".to_string(),
                description: "Replace colors in the image".to_string(),
                category: PluginCategory::Filter,
                aseprite_version: "1.0".to_string(),
                entry_point: "color_replace".to_string(),
            },
        }
    }
    
    pub fn replace_color(&self, layer: &mut crate::types::Layer, from_color: egui::Color32, to_color: egui::Color32, tolerance: u8) {
        let width = layer.width();
        let height = layer.height();
        
        for y in 0..height {
            for x in 0..width {
                let pixel = layer.grid[y][x];
                
                // Check if pixel matches the from_color within tolerance
                let r_diff = (pixel.r() as i32 - from_color.r() as i32).abs();
                let g_diff = (pixel.g() as i32 - from_color.g() as i32).abs();
                let b_diff = (pixel.b() as i32 - from_color.b() as i32).abs();
                let a_diff = (pixel.a() as i32 - from_color.a() as i32).abs();
                
                if r_diff <= tolerance as i32 && g_diff <= tolerance as i32 && 
                   b_diff <= tolerance as i32 && a_diff <= tolerance as i32 {
                    layer.grid[y][x] = to_color;
                }
            }
        }
    }
}

impl Plugin for ColorReplacementPlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }
    
    fn commands(&self) -> Vec<PluginCommand> {
        vec![
            PluginCommand {
                id: "color_replace".to_string(),
                name: "Replace Color".to_string(),
                description: "Replace colors in the image".to_string(),
                category: PluginCategory::Filter,
                shortcut: Some("Ctrl+R".to_string()),
            }
        ]
    }
    
    fn execute(&mut self, _command_id: &str, context: &mut PluginContext, params: &HashMap<String, PluginParameter>) -> PluginResult {
        context.save_state();
        
        if let Some(layer) = context.get_active_layer_mut() {
            let from_color = if let Some(PluginParameter::Color { value, .. }) = params.get("from_color") {
                egui::Color32::from_rgba_unmultiplied(value[0], value[1], value[2], value[3])
            } else {
                egui::Color32::BLACK
            };
            
            let to_color = if let Some(PluginParameter::Color { value, .. }) = params.get("to_color") {
                egui::Color32::from_rgba_unmultiplied(value[0], value[1], value[2], value[3])
            } else {
                egui::Color32::WHITE
            };
            
            let tolerance = if let Some(PluginParameter::Integer { value, .. }) = params.get("tolerance") {
                *value as u8
            } else {
                0
            };
            
            self.replace_color(layer, from_color, to_color, tolerance);
            PluginResult::Success
        } else {
            PluginResult::Error("No active layer".to_string())
        }
    }
    
    fn show_dialog(&mut self, ui: &mut egui::Ui, params: &mut HashMap<String, PluginParameter>) -> bool {
        let mut should_close = false;
        
        ui.heading("Replace Color");
        ui.separator();
        
        if let Some(PluginParameter::Color { value, .. }) = params.get_mut("from_color") {
            ui.label("From Color:");
            let mut color = egui::Color32::from_rgba_unmultiplied(value[0], value[1], value[2], value[3]);
            ui.color_edit_button_srgba(&mut color);
            *value = [color.r(), color.g(), color.b(), color.a()];
        }
        
        if let Some(PluginParameter::Color { value, .. }) = params.get_mut("to_color") {
            ui.label("To Color:");
            let mut color = egui::Color32::from_rgba_unmultiplied(value[0], value[1], value[2], value[3]);
            ui.color_edit_button_srgba(&mut color);
            *value = [color.r(), color.g(), color.b(), color.a()];
        }
        
        if let Some(PluginParameter::Integer { value, min, max, .. }) = params.get_mut("tolerance") {
            ui.add(egui::Slider::new(value, *min..=*max).text("Tolerance"));
        }
        
        ui.separator();
        
        ui.horizontal(|ui| {
            if ui.button("Apply").clicked() {
                should_close = true;
            }
            if ui.button("Cancel").clicked() {
                should_close = true;
            }
        });
        
        should_close
    }
    
    fn get_parameters(&self) -> Vec<PluginParameter> {
        vec![
            PluginParameter::Color {
                name: "from_color".to_string(),
                value: [0, 0, 0, 255],
            },
            PluginParameter::Color {
                name: "to_color".to_string(),
                value: [255, 255, 255, 255],
            },
            PluginParameter::Integer {
                name: "tolerance".to_string(),
                value: 0,
                min: 0,
                max: 255,
            }
        ]
    }
    
    fn can_execute(&self, command_id: &str) -> bool {
        command_id == "color_replace"
    }
}
