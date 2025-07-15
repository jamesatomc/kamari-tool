use crate::editor::core::PixelArtEditor;
use crate::types::{Tool, AnimationType};
use std::time::Instant;
use rand::{thread_rng, Rng};

impl PixelArtEditor {

    pub fn tool_icon_safe(&self, tool: Tool) -> String {
        // Return simple text icons that work reliably
        match tool {
            Tool::Pencil => "🖊".to_owned(),
            Tool::Eraser => "🧽".to_owned(),
            Tool::Bucket => "🪣".to_owned(),
            Tool::Eyedropper => "👁".to_owned(),
            Tool::Move => "↔".to_owned(),
            Tool::Line => "╱".to_owned(),
            Tool::Rectangle => "▭".to_owned(),
            Tool::Circle => "◯".to_owned(),
            Tool::Select => "⬚".to_owned(),
            Tool::Lasso => "➰".to_owned(),
            Tool::Spray => "💨".to_owned(),
            Tool::Dither => "▒".to_owned(),
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

    /// Applies a simple 2x2 Bayer dither pattern at the given pixel location.
    pub fn apply_dither(&mut self, x: usize, y: usize, color: eframe::egui::Color32) {
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
    pub fn paint_with_symmetry(&mut self, x: usize, y: usize, color: eframe::egui::Color32) {
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

    pub fn shift_layer_grid(grid: &Vec<Vec<eframe::egui::Color32>>, dx: isize, dy: isize) -> Vec<Vec<eframe::egui::Color32>> {
        let height = grid.len();
        let width = if height > 0 { grid[0].len() } else { 0 };
        let mut new_grid = vec![vec![eframe::egui::Color32::TRANSPARENT; width]; height];
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

impl PixelArtEditor {
    pub fn tool_icon_animated(&self, tool: Tool) -> (String, f32, f32, f32) {
        // Return icon with animation transform values
        let icon = self.tool_icon_safe(tool);
        let (scale, rotation, alpha) = self.get_tool_animation_transform(tool);
        (icon, scale, rotation, alpha)
    }
    
    pub fn use_tool_with_animation(&mut self, tool: Tool, x: usize, y: usize, color: eframe::egui::Color32) {
        // Start animation for the tool
        self.start_tool_animation(tool);
        
        // Create effect at position
        self.create_tool_effect(tool, eframe::egui::Vec2::new(x as f32, y as f32));
        
        // Use the tool based on its type
        match tool {
            Tool::Pencil => {
                self.paint_with_symmetry(x, y, color);
            }
            Tool::Eraser => {
                self.paint_with_symmetry(x, y, eframe::egui::Color32::TRANSPARENT);
            }
            Tool::Bucket => {
                self.flood_fill(x, y, color);
            }
            Tool::Eyedropper => {
                if let Some(layer) = self.frames.get(self.current_frame)
                    .and_then(|frame| frame.layers.get(self.current_layer)) {
                    if x < layer.width() && y < layer.height() {
                        self.selected_color = layer.grid[y][x];
                    }
                }
            }
            Tool::Spray => {
                self.paint_spray(x, y, color);
            }
            Tool::Dither => {
                self.apply_dither(x, y, color);
            }
            _ => {
                // For other tools, just use regular painting
                self.paint_with_symmetry(x, y, color);
            }
        }
    }
    
    pub fn paint_spray(&mut self, center_x: usize, center_y: usize, color: eframe::egui::Color32) {
        let spray_radius = self.spray_size as f32;
        let intensity = self.spray_intensity;
        
        // Create animated spray pattern
        let mut rng = thread_rng();
        let num_dots = (spray_radius * intensity * 8.0) as usize;
        
        for _ in 0..num_dots {
            let angle = rng.r#gen::<f32>() * std::f32::consts::PI * 2.0;
            let distance = rng.r#gen::<f32>() * spray_radius;
            
            let x = center_x as f32 + angle.cos() * distance;
            let y = center_y as f32 + angle.sin() * distance;
            
            let x = x.round() as usize;
            let y = y.round() as usize;
            
            if x < self.get_active_layer().width() && y < self.get_active_layer().height() {
                // Add some randomness to the spray effect
                if rng.r#gen::<f32>() < intensity {
                    self.paint_with_symmetry(x, y, color);
                }
            }
        }
    }
    
    pub fn flood_fill(&mut self, start_x: usize, start_y: usize, new_color: eframe::egui::Color32) {
        let layer = self.get_active_layer();
        let width = layer.width();
        let height = layer.height();
        
        if start_x >= width || start_y >= height {
            return;
        }
        
        let target_color = layer.grid[start_y][start_x];
        
        // Don't fill if colors are the same
        if target_color == new_color {
            return;
        }
        
        // Use a stack-based flood fill algorithm
        let mut stack = vec![(start_x, start_y)];
        let mut visited = vec![vec![false; width]; height];
        
        while let Some((x, y)) = stack.pop() {
            if x >= width || y >= height || visited[y][x] {
                continue;
            }
            
            let layer = self.get_active_layer_mut();
            if layer.grid[y][x] != target_color {
                continue;
            }
            
            visited[y][x] = true;
            layer.grid[y][x] = new_color;
            
            // Add neighboring pixels to stack
            if x > 0 {
                stack.push((x - 1, y));
            }
            if x < width - 1 {
                stack.push((x + 1, y));
            }
            if y > 0 {
                stack.push((x, y - 1));
            }
            if y < height - 1 {
                stack.push((x, y + 1));
            }
        }
    }
    
    pub fn draw_animated_line(&mut self, start_x: usize, start_y: usize, end_x: usize, end_y: usize, color: eframe::egui::Color32) {
        // Bresenham's line algorithm with animation
        let dx = (end_x as isize - start_x as isize).abs();
        let dy = (end_y as isize - start_y as isize).abs();
        let sx = if start_x < end_x { 1 } else { -1 };
        let sy = if start_y < end_y { 1 } else { -1 };
        let mut err = dx - dy;
        
        let mut x = start_x as isize;
        let mut y = start_y as isize;
        
        loop {
            if x >= 0 && y >= 0 && x < self.get_active_layer().width() as isize && y < self.get_active_layer().height() as isize {
                self.paint_with_symmetry(x as usize, y as usize, color);
                
                // Create sparkle effect along the line
                if rand::random::<f32>() < 0.3 {
                    self.create_tool_effect(Tool::Line, eframe::egui::Vec2::new(x as f32, y as f32));
                }
            }
            
            if x == end_x as isize && y == end_y as isize {
                break;
            }
            
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }
    
    pub fn draw_animated_rectangle(&mut self, start_x: usize, start_y: usize, end_x: usize, end_y: usize, color: eframe::egui::Color32, filled: bool) {
        let min_x = start_x.min(end_x);
        let max_x = start_x.max(end_x);
        let min_y = start_y.min(end_y);
        let max_y = start_y.max(end_y);
        
        if filled {
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    if x < self.get_active_layer().width() && y < self.get_active_layer().height() {
                        self.paint_with_symmetry(x, y, color);
                    }
                }
            }
        } else {
            // Draw outline
            for x in min_x..=max_x {
                if x < self.get_active_layer().width() {
                    if min_y < self.get_active_layer().height() {
                        self.paint_with_symmetry(x, min_y, color);
                    }
                    if max_y < self.get_active_layer().height() {
                        self.paint_with_symmetry(x, max_y, color);
                    }
                }
            }
            for y in min_y..=max_y {
                if y < self.get_active_layer().height() {
                    if min_x < self.get_active_layer().width() {
                        self.paint_with_symmetry(min_x, y, color);
                    }
                    if max_x < self.get_active_layer().width() {
                        self.paint_with_symmetry(max_x, y, color);
                    }
                }
            }
        }
        
        // Create animation effect
        self.create_tool_effect(Tool::Rectangle, eframe::egui::Vec2::new((min_x + max_x) as f32 / 2.0, (min_y + max_y) as f32 / 2.0));
    }
    
    pub fn draw_animated_circle(&mut self, center_x: usize, center_y: usize, radius: usize, color: eframe::egui::Color32, filled: bool) {
        let radius = radius as isize;
        
        if filled {
            for y in (center_y as isize - radius)..=(center_y as isize + radius) {
                for x in (center_x as isize - radius)..=(center_x as isize + radius) {
                    let dx = x - center_x as isize;
                    let dy = y - center_y as isize;
                    
                    if dx * dx + dy * dy <= radius * radius {
                        if x >= 0 && y >= 0 && x < self.get_active_layer().width() as isize && y < self.get_active_layer().height() as isize {
                            self.paint_with_symmetry(x as usize, y as usize, color);
                        }
                    }
                }
            }
        } else {
            // Midpoint circle algorithm
            let mut x = 0;
            let mut y = radius;
            let mut d = 1 - radius;
            
            while x <= y {
                // Plot 8 octants
                let points = [
                    (center_x as isize + x, center_y as isize + y),
                    (center_x as isize + x, center_y as isize - y),
                    (center_x as isize - x, center_y as isize + y),
                    (center_x as isize - x, center_y as isize - y),
                    (center_x as isize + y, center_y as isize + x),
                    (center_x as isize + y, center_y as isize - x),
                    (center_x as isize - y, center_y as isize + x),
                    (center_x as isize - y, center_y as isize - x),
                ];
                
                for (px, py) in points {
                    if px >= 0 && py >= 0 && px < self.get_active_layer().width() as isize && py < self.get_active_layer().height() as isize {
                        self.paint_with_symmetry(px as usize, py as usize, color);
                        
                        // Create sparkle effect
                        if rand::random::<f32>() < 0.2 {
                            self.create_tool_effect(Tool::Circle, eframe::egui::Vec2::new(px as f32, py as f32));
                        }
                    }
                }
                
                if d < 0 {
                    d += 2 * x + 3;
                } else {
                    d += 2 * (x - y) + 5;
                    y -= 1;
                }
                x += 1;
            }
        }
    }
}
