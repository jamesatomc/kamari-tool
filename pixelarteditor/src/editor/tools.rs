use crate::editor::core::PixelArtEditor;
use crate::types::Tool;

impl PixelArtEditor {
    pub fn tool_icon(&self, tool: Tool) -> &'static str {
        match tool {
            Tool::Pencil => "P",
            Tool::Eraser => "E",
            Tool::Bucket => "B",
            Tool::Eyedropper => "I",
            Tool::Move => "M",
            Tool::Line => "L",
            Tool::Rectangle => "R",
            Tool::Circle => "C",
            Tool::Select => "S",
            Tool::Lasso => "A",
            Tool::Spray => "Y",
            Tool::Dither => "D",
        }
    }
    
    pub fn tool_icon_normalized(&self, tool: Tool) -> String {
        use unicode_normalization::UnicodeNormalization;
        let icon = self.tool_icon(tool);
        icon.nfc().collect::<String>()
    }
    
    pub fn tool_icon_safe(&self, tool: Tool) -> String {
        // Return simple text icons that work reliably
        match tool {
            Tool::Pencil => "[P]".to_string(),
            Tool::Eraser => "[E]".to_string(),
            Tool::Bucket => "[B]".to_string(),
            Tool::Eyedropper => "[I]".to_string(),
            Tool::Move => "[M]".to_string(),
            Tool::Line => "[L]".to_string(),
            Tool::Rectangle => "[R]".to_string(),
            Tool::Circle => "[C]".to_string(),
            Tool::Select => "[S]".to_string(),
            Tool::Lasso => "[A]".to_string(),
            Tool::Spray => "[Y]".to_string(),
            Tool::Dither => "[D]".to_string(),
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
