use eframe::egui;
use crate::types::{Tool, Layer, Frame};
use crate::constants::*;

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
    pub circle_center: Option<(usize, usize)>,
    pub circle_radius: Option<usize>,
    pub selection_area: Option<(usize, usize, usize, usize)>, // x, y, width, height
    pub selection_pixels: Option<Vec<Vec<egui::Color32>>>,

    pub lasso_points: Vec<(usize, usize)>,
    pub spray_intensity: f32,
    pub dither_pattern: usize,
    pub fill_outline: bool,
    pub preview_overlay: Option<Vec<Vec<egui::Color32>>>,
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
            circle_center: None,
            circle_radius: None,
            selection_area: None,
            selection_pixels: None,
            lasso_points: Vec::new(),
            spray_intensity: 0.5,
            dither_pattern: 0,
            fill_outline: false,
            preview_overlay: None,
        }
    }
}

impl PixelArtEditor {
    pub fn new() -> Self {
        Self::default()
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
        let frame_idx = if self.animation_playing {
            self.animation_frame
        } else {
            self.current_frame
        };
        let frame = &self.frames[frame_idx];

        let width = frame.layers[0].width();
        let height = frame.layers[0].height();
        let mut composed = vec![vec![egui::Color32::TRANSPARENT; width]; height];

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

    pub fn tool_icon(&self, tool: Tool) -> &'static str {
        match tool {
            Tool::Pencil => "✏️",
            Tool::Eraser => "🧽",
            Tool::Bucket => "🪣", 
            Tool::Eyedropper => "👁️",
            Tool::Move => "↔️",
            Tool::Line => "📏",
            Tool::Rectangle => "⬜",
            Tool::Circle => "⭕",
            Tool::Select => "🔲",
            Tool::Lasso => "🔗",
            Tool::Spray => "💨",
            Tool::Dither => "🌐",
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
}
