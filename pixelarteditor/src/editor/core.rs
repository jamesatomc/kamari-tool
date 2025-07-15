use eframe::egui;
use crate::types::{Tool, Layer, Frame, ExportFormat, ToolAnimation, AnimationType};
use crate::constants::*;
use crate::plugins::PluginManager;
use std::collections::HashMap;
use std::time::Instant;

/// Core editor state and data
pub struct PixelArtEditor {
    // Core editing state
    pub frames: Vec<Frame>,
    pub current_frame: usize,
    pub current_layer: usize,
    pub selected_color: egui::Color32,
    pub secondary_color: egui::Color32,
    pub tool: Tool,
    
    // Undo/Redo system
    pub undo_stack: Vec<(Vec<Frame>, usize, usize)>,
    pub redo_stack: Vec<(Vec<Frame>, usize, usize)>,
    pub last_state: Option<(Vec<Frame>, usize, usize)>, // Keep for compatibility
    
    // View settings
    pub show_grid: bool,
    pub zoom: f32,
    pub show_layers_panel: bool,
    pub show_frames_panel: bool,
    pub show_color_panel: bool,
    
    // Canvas scroll position
    pub canvas_scroll_offset: egui::Vec2,
    pub canvas_center_on_start: bool,
    
    // Tool settings
    pub brush_size: usize,
    pub spray_size: usize,
    pub spray_intensity: f32,
    pub dither_pattern: usize,
    pub fill_shape: bool,
    pub fill_outline: bool,
    pub pixel_perfect_mode: bool,
    pub symmetry_mode: bool,
    pub symmetry_axis: (bool, bool), // (horizontal, vertical)
    
    // Color palette
    pub color_palette: Vec<egui::Color32>,
    pub custom_palettes: Vec<Vec<egui::Color32>>,
    pub active_palette: usize,
    pub palette_names: Vec<String>,
    
    // Animation settings
    pub animation_playing: bool,
    pub animation_frame: usize,
    pub animation_speed: f32,
    pub last_animation_time: f64,
    
    // Onion skinning
    pub onion_skinning: bool,
    pub onion_prev_frames: usize,
    pub onion_next_frames: usize,
    pub onion_opacity: f32,
    
    // Dialog states
    pub show_new_sprite_dialog: bool,
    pub new_sprite_width: usize,
    pub new_sprite_height: usize,
    pub new_sprite_bg: egui::Color32,
    pub show_resize_dialog: bool,
    pub resize_width: usize,
    pub resize_height: usize,
    pub resize_anchor: usize,
    pub show_export_dialog: bool,
    pub export_format: ExportFormat,
    pub export_individual_layers: bool,
    pub export_all_frames: bool,
    
    // Tool-specific state
    pub move_drag_start: Option<(usize, usize)>,
    pub move_layer_snapshot: Option<Vec<Vec<egui::Color32>>>,
    pub move_last_offset: Option<(isize, isize)>,
    pub line_start: Option<(usize, usize)>,
    pub line_end: Option<(usize, usize)>,
    pub rectangle_start: Option<(usize, usize)>,
    pub rectangle_end: Option<(usize, usize)>,
    pub circle_start: Option<(usize, usize)>,
    pub circle_center: Option<(usize, usize)>,
    pub circle_radius: Option<usize>,
    pub selection_start: Option<(usize, usize)>,
    pub selection_rect: Option<(usize, usize, usize, usize)>,
    pub selection_area: Option<(usize, usize, usize, usize)>,
    pub selection_pixels: Option<Vec<Vec<egui::Color32>>>,
    pub lasso_points: Vec<(usize, usize)>,
    pub lasso_active: bool,
    pub lasso_selection: Option<Vec<(usize, usize)>>,
    pub preview_overlay: Option<Vec<Vec<egui::Color32>>>,
    
    // Layer management
    pub renaming_layer: Option<usize>,
    pub rename_text: String,
    
    // Performance optimization
    pub render_cache: Option<Vec<Vec<egui::Color32>>>,
    pub cache_dirty: bool,
    pub last_frame_time: f64,
    pub frame_skip: usize,
    
    // Plugin system
    pub plugin_manager: PluginManager,
    
    // Tool animations
    pub tool_animations: HashMap<Tool, ToolAnimation>,
    pub current_tool_animation: Option<ToolAnimation>,
    pub animation_enabled: bool,
    pub last_tool_use: Option<Instant>,
    pub tool_effects: HashMap<Tool, Vec<egui::Vec2>>, // For tool-specific effects like shake, sparkle positions
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
            
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            last_state: None,
            
            show_grid: true,
            zoom: 1.0,
            show_layers_panel: true,
            show_frames_panel: true,
            show_color_panel: true,
            
            canvas_scroll_offset: egui::Vec2::ZERO,
            canvas_center_on_start: true,
            
            brush_size: 1,
            spray_size: 3,
            spray_intensity: 0.5,
            dither_pattern: 0,
            fill_shape: false,
            fill_outline: false,
            pixel_perfect_mode: true,
            symmetry_mode: false,
            symmetry_axis: (false, false),
            
            color_palette: get_default_palette(),
            custom_palettes: vec![
                get_default_palette(),
                vec![egui::Color32::BLACK, egui::Color32::WHITE, egui::Color32::GRAY, egui::Color32::DARK_GRAY],
                vec![egui::Color32::RED, egui::Color32::GREEN, egui::Color32::BLUE, egui::Color32::YELLOW],
            ],
            active_palette: 0,
            palette_names: vec!["Default".to_string(), "Grayscale".to_string(), "Primary".to_string()],
            
            animation_playing: false,
            animation_frame: 0,
            animation_speed: 5.0,
            last_animation_time: 0.0,
            
            onion_skinning: false,
            onion_prev_frames: 1,
            onion_next_frames: 1,
            onion_opacity: 0.3,
            
            show_new_sprite_dialog: false,
            new_sprite_width: 16,
            new_sprite_height: 16,
            new_sprite_bg: egui::Color32::TRANSPARENT,
            show_resize_dialog: false,
            resize_width: 16,
            resize_height: 16,
            resize_anchor: 1,
            show_export_dialog: false,
            export_format: ExportFormat::PNG,
            export_individual_layers: false,
            export_all_frames: false,
            
            move_drag_start: None,
            move_layer_snapshot: None,
            move_last_offset: None,
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
            preview_overlay: None,
            
            renaming_layer: None,
            rename_text: String::new(),
            
            render_cache: None,
            cache_dirty: true,
            last_frame_time: 0.0,
            frame_skip: 0,
            
            plugin_manager: PluginManager::new(),
            
            // Tool animations
            tool_animations: HashMap::new(),
            current_tool_animation: None,
            animation_enabled: true,
            last_tool_use: None,
            tool_effects: HashMap::new(),
        }
    }
}

impl PixelArtEditor {
    pub fn new() -> Self {
        let mut editor = Self::default();
        editor.plugin_manager.initialize();
        editor.setup_tool_animations();
        editor
    }

    pub fn get_active_layer_mut(&mut self) -> &mut Layer {
        &mut self.frames[self.current_frame].layers[self.current_layer]
    }

    pub fn get_active_layer(&self) -> &Layer {
        &self.frames[self.current_frame].layers[self.current_layer]
    }

    pub fn invalidate_cache(&mut self) {
        self.cache_dirty = true;
    }

    pub fn update_cache(&mut self, composed: Vec<Vec<egui::Color32>>) {
        self.render_cache = Some(composed);
        self.cache_dirty = false;
    }

    pub fn center_canvas(&mut self) {
        self.canvas_center_on_start = true;
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        // Remove upper limit for unlimited zoom, but keep reasonable lower limit
        self.zoom = zoom.max(0.01); // Minimum zoom to prevent division by zero
        self.invalidate_cache();
    }

    pub fn zoom_in(&mut self) {
        self.set_zoom(self.zoom * 1.05);
    }

    pub fn zoom_out(&mut self) {
        self.set_zoom(self.zoom * 0.95);
    }

    pub fn zoom_in_at_point(&mut self, point: egui::Pos2, zoom_factor: f32) {
        let old_zoom = self.zoom;
        self.set_zoom(self.zoom * zoom_factor);
        
        // Calculate the offset change needed to keep the point under the cursor
        let zoom_change = self.zoom / old_zoom;
        let offset_change = point.to_vec2() * (1.0 - zoom_change);
        self.canvas_scroll_offset += offset_change;
    }

    pub fn zoom_out_at_point(&mut self, point: egui::Pos2, zoom_factor: f32) {
        let old_zoom = self.zoom;
        self.set_zoom(self.zoom / zoom_factor);
        
        // Calculate the offset change needed to keep the point under the cursor
        let zoom_change = self.zoom / old_zoom;
        let offset_change = point.to_vec2() * (1.0 - zoom_change);
        self.canvas_scroll_offset += offset_change;
    }

    pub fn reset_zoom(&mut self) {
        self.set_zoom(1.0);
    }

    pub fn paint_brush(&mut self, center_x: usize, center_y: usize, color: egui::Color32) {
        let brush_radius = (self.brush_size / 2) as isize;
        let symmetry_mode = self.symmetry_mode;
        let symmetry_axis = self.symmetry_axis;
        
        let layer = self.get_active_layer_mut();
        let width = layer.width();
        let height = layer.height();
        
        // Draw circular brush
        for dy in -brush_radius..=brush_radius {
            for dx in -brush_radius..=brush_radius {
                // Check if the pixel is within the circular brush
                if dx * dx + dy * dy <= brush_radius * brush_radius {
                    let x = center_x as isize + dx;
                    let y = center_y as isize + dy;
                    
                    if x >= 0 && x < width as isize && y >= 0 && y < height as isize {
                        let x = x as usize;
                        let y = y as usize;
                        
                        // Paint the main pixel
                        layer.grid[y][x] = color;
                        
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
                }
            }
        }
    }

    pub fn erase_brush(&mut self, center_x: usize, center_y: usize) {
        let brush_radius = (self.brush_size / 2) as isize;
        
        let layer = self.get_active_layer_mut();
        let width = layer.width();
        let height = layer.height();
        
        // Draw circular brush for erasing
        for dy in -brush_radius..=brush_radius {
            for dx in -brush_radius..=brush_radius {
                // Check if the pixel is within the circular brush
                if dx * dx + dy * dy <= brush_radius * brush_radius {
                    let x = center_x as isize + dx;
                    let y = center_y as isize + dy;
                    
                    if x >= 0 && x < width as isize && y >= 0 && y < height as isize {
                        let x = x as usize;
                        let y = y as usize;
                        layer.grid[y][x] = egui::Color32::TRANSPARENT;
                    }
                }
            }
        }
    }
}

impl PixelArtEditor {
    pub fn setup_tool_animations(&mut self) {
        // Set up animations for each tool
        self.tool_animations.insert(Tool::Pencil, ToolAnimation::new(Tool::Pencil, AnimationType::Pulse, 0.5));
        self.tool_animations.insert(Tool::Eraser, ToolAnimation::new(Tool::Eraser, AnimationType::Fade, 0.8));
        self.tool_animations.insert(Tool::Bucket, ToolAnimation::new(Tool::Bucket, AnimationType::Bounce, 0.6));
        self.tool_animations.insert(Tool::Eyedropper, ToolAnimation::new(Tool::Eyedropper, AnimationType::Scale, 0.4));
        self.tool_animations.insert(Tool::Move, ToolAnimation::new(Tool::Move, AnimationType::Wobble, 0.7));
        self.tool_animations.insert(Tool::Line, ToolAnimation::new(Tool::Line, AnimationType::Glow, 0.5));
        self.tool_animations.insert(Tool::Rectangle, ToolAnimation::new(Tool::Rectangle, AnimationType::Rotate, 0.8));
        self.tool_animations.insert(Tool::Circle, ToolAnimation::new(Tool::Circle, AnimationType::Sparkle, 1.0));
        self.tool_animations.insert(Tool::Select, ToolAnimation::new(Tool::Select, AnimationType::Pulse, 0.4));
        self.tool_animations.insert(Tool::Lasso, ToolAnimation::new(Tool::Lasso, AnimationType::Shake, 0.3));
        self.tool_animations.insert(Tool::Spray, ToolAnimation::new(Tool::Spray, AnimationType::Sparkle, 0.6));
        self.tool_animations.insert(Tool::Dither, ToolAnimation::new(Tool::Dither, AnimationType::Glow, 0.9));
    }
    
    pub fn start_tool_animation(&mut self, tool: Tool) {
        if !self.animation_enabled {
            return;
        }
        
        if let Some(animation) = self.tool_animations.get_mut(&tool) {
            animation.color = self.selected_color;
            animation.start();
            self.current_tool_animation = Some(animation.clone());
        }
        
        self.last_tool_use = Some(Instant::now());
    }
    
    pub fn update_tool_animations(&mut self, dt: f32) {
        if !self.animation_enabled {
            return;
        }
        
        // Update current tool animation
        if let Some(ref mut animation) = self.current_tool_animation {
            animation.update(dt);
            if animation.is_finished() {
                self.current_tool_animation = None;
            }
        }
        
        // Update all tool animations
        for (_, animation) in self.tool_animations.iter_mut() {
            animation.update(dt);
        }
    }
    
    pub fn get_tool_animation_transform(&self, tool: Tool) -> (f32, f32, f32) {
        // Returns (scale, rotation, alpha)
        if let Some(animation) = self.tool_animations.get(&tool) {
            if animation.is_active {
                let progress = animation.get_progress();
                match animation.animation_type {
                    AnimationType::Pulse => {
                        let scale = 1.0 + (progress * std::f32::consts::PI * 2.0).sin() * 0.2;
                        (scale, 0.0, 1.0)
                    }
                    AnimationType::Bounce => {
                        let scale = 1.0 + (1.0 - progress).powi(2i32) * 0.3;
                        (scale, 0.0, 1.0)
                    }
                    AnimationType::Rotate => {
                        let rotation = progress * std::f32::consts::PI * 2.0;
                        (1.0, rotation, 1.0)
                    }
                    AnimationType::Scale => {
                        let scale = 1.0 + progress * 0.5;
                        (scale, 0.0, 1.0)
                    }
                    AnimationType::Glow => {
                        let alpha = 1.0 + (progress * std::f32::consts::PI * 2.0).sin() * 0.3;
                        (1.0, 0.0, alpha)
                    }
                    AnimationType::Fade => {
                        let alpha = 1.0 - progress * 0.5;
                        (1.0, 0.0, alpha)
                    }
                    AnimationType::Shake => {
                        let offset = ((progress * 20.0).sin() * (1.0 - progress)) * 2.0;
                        (1.0, offset, 1.0)
                    }
                    AnimationType::Wobble => {
                        let wobble = (progress * std::f32::consts::PI * 4.0).sin() * (1.0 - progress) * 0.1;
                        (1.0 + wobble, 0.0, 1.0)
                    }
                    _ => (1.0, 0.0, 1.0),
                }
            } else {
                (1.0, 0.0, 1.0)
            }
        } else {
            (1.0, 0.0, 1.0)
        }
    }
    
    pub fn toggle_animations(&mut self) {
        self.animation_enabled = !self.animation_enabled;
        if !self.animation_enabled {
            self.current_tool_animation = None;
            for (_, animation) in self.tool_animations.iter_mut() {
                animation.stop();
            }
        }
    }
    
    pub fn create_tool_effect(&mut self, tool: Tool, position: egui::Vec2) {
        if !self.animation_enabled {
            return;
        }
        
        // Create sparkle effect for certain tools
        match tool {
            Tool::Spray | Tool::Circle | Tool::Dither => {
                let effects = self.tool_effects.entry(tool).or_insert_with(Vec::new);
                effects.push(position);
                if effects.len() > 10 {
                    effects.remove(0);
                }
            }
            _ => {}
        }
    }
    
    pub fn get_tool_effects(&self, tool: Tool) -> Vec<egui::Vec2> {
        self.tool_effects.get(&tool).cloned().unwrap_or_default()
    }
    
    pub fn clear_tool_effects(&mut self, tool: Tool) {
        self.tool_effects.remove(&tool);
    }
}
