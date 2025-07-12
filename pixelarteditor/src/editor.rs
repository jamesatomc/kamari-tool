use eframe::egui;
use crate::types::{Tool, Layer, Frame, ExportFormat};
use crate::constants::*;
use crate::plugins::PluginManager;

pub struct PixelArtEditor {
    pub frames: Vec<Frame>,
    pub current_frame: usize,
    pub current_layer: usize,
    pub selected_color: egui::Color32,
    pub secondary_color: egui::Color32,
    pub tool: Tool,
    pub last_state: Option<(Vec<Frame>, usize, usize)>,
    pub show_grid: bool,
    pub brush_size: usize,
    pub zoom: f32,
    pub show_new_sprite_dialog: bool,
    pub new_sprite_width: usize,
    pub new_sprite_height: usize,
    pub new_sprite_bg: egui::Color32,
    pub show_layers_panel: bool,
    pub show_frames_panel: bool,
    pub show_color_panel: bool,
    pub color_palette: Vec<egui::Color32>,
    pub animation_playing: bool,
    pub animation_frame: usize,
    pub animation_speed: f32,
    pub last_animation_time: f64,
    pub show_resize_dialog: bool,
    pub resize_width: usize,
    pub resize_height: usize,
    pub resize_anchor: usize,
    pub move_drag_start: Option<(usize, usize)>,
    pub move_layer_snapshot: Option<Vec<Vec<egui::Color32>>>,
    pub move_last_offset: Option<(isize, isize)>,
    // New fields for advanced tools
    pub line_start: Option<(usize, usize)>,
    pub line_end: Option<(usize, usize)>,
    pub rectangle_start: Option<(usize, usize)>,
    pub rectangle_end: Option<(usize, usize)>,
    pub circle_start: Option<(usize, usize)>,
    pub circle_center: Option<(usize, usize)>,
    pub circle_radius: Option<usize>,
    pub selection_start: Option<(usize, usize)>,
    pub selection_rect: Option<(usize, usize, usize, usize)>, // min_x, min_y, max_x, max_y
    pub selection_area: Option<(usize, usize, usize, usize)>, // x, y, width, height
    pub selection_pixels: Option<Vec<Vec<egui::Color32>>>,

    pub lasso_points: Vec<(usize, usize)>,
    pub lasso_active: bool,
    pub lasso_selection: Option<Vec<(usize, usize)>>,
    pub spray_size: usize,
    pub spray_intensity: f32,
    pub dither_pattern: usize,
    pub fill_shape: bool,
    pub fill_outline: bool,
    pub preview_overlay: Option<Vec<Vec<egui::Color32>>>,
    
    // Layer renaming state
    pub renaming_layer: Option<usize>,
    pub rename_text: String,
    
    // Multi-file saving state
    pub show_export_dialog: bool,
    pub export_format: ExportFormat,
    pub export_individual_layers: bool,
    pub export_all_frames: bool,
    
    // Onion skinning
    pub onion_skinning: bool,
    pub onion_prev_frames: usize,
    pub onion_next_frames: usize,
    pub onion_opacity: f32,
    
    // Advanced color palette
    pub custom_palettes: Vec<Vec<egui::Color32>>,
    pub active_palette: usize,
    pub palette_names: Vec<String>,
    
    // Pixel-perfect tools
    pub pixel_perfect_mode: bool,
    pub symmetry_mode: bool,
    pub symmetry_axis: (bool, bool), // (horizontal, vertical)
    
    // Performance optimization
    pub render_cache: Option<Vec<Vec<egui::Color32>>>,
    pub cache_dirty: bool,
    pub last_frame_time: f64,
    pub frame_skip: usize,
    
    // Plugin system
    pub plugin_manager: PluginManager,
}

impl Default for PixelArtEditor {
    fn default() -> Self {
        Self {
            frames: vec![Frame::default()],
            current_frame: 0,
            current_layer: 0,
            selected_color: egui::Color32::BLACK,
            secondary_color: egui::Color32::WHITE,
            tool: Tool::Pencil,
            last_state: None,
            show_grid: true,
            brush_size: 1,
            zoom: 1.0,
            show_new_sprite_dialog: false,
            new_sprite_width: 16,
            new_sprite_height: 16,
            new_sprite_bg: egui::Color32::TRANSPARENT,
            show_layers_panel: true,
            show_frames_panel: true,
            show_color_panel: true,
            color_palette: get_default_palette(),
            animation_playing: false,
            animation_frame: 0,
            animation_speed: 5.0,
            last_animation_time: 0.0,
            show_resize_dialog: false,
            resize_width: 16,
            resize_height: 16,
            resize_anchor: 1,
            move_drag_start: None,
            move_layer_snapshot: None,
            move_last_offset: None,
            // Initialize new fields
            line_start: None,
            line_end: None,
            rectangle_start: None,
            rectangle_end: None,
            circle_start: None,
            circle_center: None,
            circle_radius: None,
            selection_start: None,
            selection_rect: None,
            selection_area: None,
            selection_pixels: None,
            lasso_points: Vec::new(),
            lasso_active: false,
            lasso_selection: None,
            spray_size: 3,
            spray_intensity: 0.5,
            dither_pattern: 0,
            fill_shape: false,
            fill_outline: false,
            preview_overlay: None,
            
            // Initialize new fields
            renaming_layer: None,
            rename_text: String::new(),
            show_export_dialog: false,
            export_format: ExportFormat::PNG,
            export_individual_layers: false,
            export_all_frames: false,
            
            // Initialize onion skinning
            onion_skinning: false,
            onion_prev_frames: 1,
            onion_next_frames: 1,
            onion_opacity: 0.3,
            
            // Initialize advanced color palette
            custom_palettes: vec![
                get_default_palette(),
                vec![egui::Color32::BLACK, egui::Color32::WHITE, egui::Color32::GRAY, egui::Color32::DARK_GRAY],
                vec![egui::Color32::RED, egui::Color32::GREEN, egui::Color32::BLUE, egui::Color32::YELLOW],
            ],
            active_palette: 0,
            palette_names: vec!["Default".to_string(), "Grayscale".to_string(), "Primary".to_string()],
            
            // Initialize pixel-perfect tools
            pixel_perfect_mode: true,
            symmetry_mode: false,
            symmetry_axis: (false, false),
            
            // Initialize performance optimization
            render_cache: None,
            cache_dirty: true,
            last_frame_time: 0.0,
            frame_skip: 0,
            
            // Initialize plugin manager
            plugin_manager: PluginManager::new(),
        }
    }
}

impl PixelArtEditor {
    pub fn new() -> Self {
        let mut editor = Self::default();
        editor.plugin_manager.initialize();
        editor
    }

    pub fn push_undo(&mut self) {
        self.last_state = Some((self.frames.clone(), self.current_frame, self.current_layer));
    }

    pub fn undo(&mut self) {
        if let Some((frames, cf, cl)) = self.last_state.take() {
            self.frames = frames;
            self.current_frame = cf;
            self.current_layer = cl;
        }
    }

    pub fn get_active_layer_mut(&mut self) -> &mut Layer {
        &mut self.frames[self.current_frame].layers[self.current_layer]
    }

    pub fn get_active_layer(&self) -> &Layer {
        &self.frames[self.current_frame].layers[self.current_layer]
    }

    pub fn get_composed_grid(&self) -> Vec<Vec<egui::Color32>> {
        // Use cache if available and not dirty
        if let Some(ref cache) = self.render_cache {
            if !self.cache_dirty {
                return cache.clone();
            }
        }

        let frame_idx = if self.animation_playing {
            self.animation_frame
        } else {
            self.current_frame
        };
        let frame = &self.frames[frame_idx];

        let width = frame.layers[0].width();
        let height = frame.layers[0].height();
        let mut composed = vec![vec![egui::Color32::TRANSPARENT; width]; height];

        // Add onion skinning if enabled
        if self.onion_skinning && !self.animation_playing {
            // Draw previous frames
            for i in 1..=self.onion_prev_frames {
                if frame_idx >= i {
                    let prev_frame = &self.frames[frame_idx - i];
                    let opacity = self.onion_opacity * (1.0 - (i as f32 * 0.2));
                    self.compose_frame_with_opacity(prev_frame, &mut composed, opacity, egui::Color32::BLUE);
                }
            }
            
            // Draw next frames
            for i in 1..=self.onion_next_frames {
                if frame_idx + i < self.frames.len() {
                    let next_frame = &self.frames[frame_idx + i];
                    let opacity = self.onion_opacity * (1.0 - (i as f32 * 0.2));
                    self.compose_frame_with_opacity(next_frame, &mut composed, opacity, egui::Color32::RED);
                }
            }
        }

        // Draw current frame
        for layer in &frame.layers {
            if !layer.visible {
                continue;
            }
            for y in 0..height {
                for x in 0..width {
                    let c = layer.grid[y][x];
                    if c.a() > 0 {
                        let new_color = if layer.opacity < 1.0 {
                            let alpha = (c.a() as f32 * layer.opacity) as u8;
                            egui::Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), alpha)
                        } else {
                            c
                        };

                        if new_color.a() == 255 {
                            composed[y][x] = new_color;
                        } else if new_color.a() > 0 {
                            let bg = composed[y][x];
                            composed[y][x] = blend_colors(bg, new_color);
                        }
                    }
                }
            }
        }
        composed
    }

    fn compose_frame_with_opacity(&self, frame: &Frame, composed: &mut Vec<Vec<egui::Color32>>, opacity: f32, tint: egui::Color32) {
        let width = composed[0].len();
        let height = composed.len();
        
        for layer in &frame.layers {
            if !layer.visible {
                continue;
            }
            for y in 0..height {
                for x in 0..width {
                    let c = layer.grid[y][x];
                    if c.a() > 0 {
                        // Apply tint and opacity
                        let tinted = egui::Color32::from_rgba_unmultiplied(
                            ((c.r() as f32 * 0.7) + (tint.r() as f32 * 0.3)) as u8,
                            ((c.g() as f32 * 0.7) + (tint.g() as f32 * 0.3)) as u8,
                            ((c.b() as f32 * 0.7) + (tint.b() as f32 * 0.3)) as u8,
                            (c.a() as f32 * opacity) as u8
                        );
                        
                        if tinted.a() > 0 {
                            let bg = composed[y][x];
                            composed[y][x] = blend_colors(bg, tinted);
                        }
                    }
                }
            }
        }
    }

    pub fn tool_icon(&self, tool: Tool) -> &'static str {
        match tool {
            Tool::Pencil => "✎",      // Pencil icon
            Tool::Eraser => "⌫",      // Erase icon
            Tool::Bucket => "🪣",     // Bucket icon (if supported, else fallback)
            Tool::Eyedropper => "🎨",  // Eyedropper icon
            Tool::Move => "✥",        // Move icon
            Tool::Line => "📏",       // Line icon
            Tool::Rectangle => "▭",   // Rectangle icon
            Tool::Circle => "○",      // Circle icon
            Tool::Select => "⬚",      // Select icon
            Tool::Lasso => "◯",       // Lasso icon
            Tool::Spray => "💨",      // Spray icon
            Tool::Dither => "▦",      // Dither icon
        }
    }
    
    pub fn tool_icon_normalized(&self, tool: Tool) -> String {
        use unicode_normalization::UnicodeNormalization;
        let icon = self.tool_icon(tool);
        icon.nfc().collect::<String>()
    }
    
    pub fn tool_icon_safe(&self, tool: Tool) -> String {
        use unicode_segmentation::UnicodeSegmentation;
        let icon = self.tool_icon_normalized(tool);
        // Check if emoji is supported, fallback to text if not
        if icon.graphemes(true).count() > 0 {
            icon
        } else {
            // Fallback to simple text icons
            match tool {
                Tool::Pencil => "P".to_string(),
                Tool::Eraser => "E".to_string(),
                Tool::Bucket => "B".to_string(),
                Tool::Eyedropper => "I".to_string(),
                Tool::Move => "M".to_string(),
                Tool::Line => "L".to_string(),
                Tool::Rectangle => "R".to_string(),
                Tool::Circle => "C".to_string(),
                Tool::Select => "S".to_string(),
                Tool::Lasso => "A".to_string(),
                Tool::Spray => "Y".to_string(),
                Tool::Dither => "D".to_string(),
            }
        }
    }

    pub fn tool_name(&self, tool: Tool) -> &'static str {
        match tool {
            Tool::Pencil => "Pencil",
            Tool::Eraser => "Eraser",
            Tool::Bucket => "Bucket",
            Tool::Eyedropper => "Eyedropper",
            Tool::Move => "Move",
            Tool::Line => "Line",
            Tool::Rectangle => "Rectangle",
            Tool::Circle => "Circle",
            Tool::Select => "Select",
            Tool::Lasso => "Lasso",
            Tool::Spray => "Spray",
            Tool::Dither => "Dither",
        }
    }

    pub fn shift_layer_grid(grid: &Vec<Vec<egui::Color32>>, dx: isize, dy: isize) -> Vec<Vec<egui::Color32>> {
        let height = grid.len();
        let width = if height > 0 { grid[0].len() } else { 0 };
        let mut new_grid = vec![vec![egui::Color32::TRANSPARENT; width]; height];
        for y in 0..height {
            for x in 0..width {
                let nx = x as isize - dx;
                let ny = y as isize - dy;
                if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                    new_grid[y][x] = grid[ny as usize][nx as usize];
                }
            }
        }
        new_grid
    }

    /// Applies a simple 2x2 Bayer dither pattern at the given pixel location.
    pub fn apply_dither(&mut self, x: usize, y: usize, color: egui::Color32) {
        let layer = self.get_active_layer_mut();
        let width = layer.width();
        let height = layer.height();

        // 2x2 Bayer matrix
        let bayer = [[0, 2], [3, 1]];
        for dy in 0..2 {
            for dx in 0..2 {
                let px = x + dx;
                let py = y + dy;
                if px < width && py < height {
                    // Apply color based on threshold
                    if bayer[dy][dx] < 2 {
                        layer.grid[py][px] = color;
                    }
                }
            }
        }
    }

    /// Paint with symmetry if enabled
    pub fn paint_with_symmetry(&mut self, x: usize, y: usize, color: egui::Color32) {
        let width = self.get_active_layer().width();
        let height = self.get_active_layer().height();
        let symmetry_mode = self.symmetry_mode;
        let symmetry_axis = self.symmetry_axis;
        
        let layer = self.get_active_layer_mut();
        
        // Paint the main pixel
        if x < width && y < height {
            layer.grid[y][x] = color;
        }
        
        // Paint symmetry pixels if enabled
        if symmetry_mode {
            if symmetry_axis.0 { // Horizontal symmetry
                let sym_x = width - 1 - x;
                if sym_x < width && y < height {
                    layer.grid[y][sym_x] = color;
                }
            }
            
            if symmetry_axis.1 { // Vertical symmetry
                let sym_y = height - 1 - y;
                if x < width && sym_y < height {
                    layer.grid[sym_y][x] = color;
                }
            }
            
            if symmetry_axis.0 && symmetry_axis.1 { // Both axes
                let sym_x = width - 1 - x;
                let sym_y = height - 1 - y;
                if sym_x < width && sym_y < height {
                    layer.grid[sym_y][sym_x] = color;
                }
            }
        }
    }

    pub fn invalidate_cache(&mut self) {
        self.cache_dirty = true;
    }

    pub fn update_cache(&mut self, composed: Vec<Vec<egui::Color32>>) {
        self.render_cache = Some(composed);
        self.cache_dirty = false;
    }

    /// Execute a plugin command safely
    pub fn execute_plugin_command(&mut self, command_id: &str) {
        // This method allows safe execution of plugin commands without borrowing conflicts
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
                    outline_plugin.apply_outline(layer, egui::Color32::BLACK, 1);
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
                    color_replace_plugin.replace_color(layer, egui::Color32::BLACK, egui::Color32::WHITE, 0);
                }
            }
            _ => {
                eprintln!("Unknown plugin command: {}", command_id);
            }
        }
    }
}
