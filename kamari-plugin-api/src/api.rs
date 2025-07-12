use crate::types::{PluginContext, Color};

/// Plugin context operations
impl PluginContext {
    /// Apply blur effect to the entire canvas
    pub fn apply_blur(&mut self, radius: f32) {
        let width = self.width();
        let height = self.height();
        let mut new_data = self.get_pixel_data().clone();
        
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
                            if let Some(pixel) = self.get_pixel(nx as usize, ny as usize) {
                                r_sum += pixel.r as f32;
                                g_sum += pixel.g as f32;
                                b_sum += pixel.b as f32;
                                a_sum += pixel.a as f32;
                                count += 1;
                            }
                        }
                    }
                }
                
                if count > 0 {
                    new_data[y][x] = Color::new(
                        (r_sum / count as f32) as u8,
                        (g_sum / count as f32) as u8,
                        (b_sum / count as f32) as u8,
                        (a_sum / count as f32) as u8,
                    );
                }
            }
        }
        
        self.set_pixel_data(new_data);
    }
    
    /// Apply noise effect
    pub fn apply_noise(&mut self, intensity: f32) {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let width = self.width();
        let height = self.height();
        
        for y in 0..height {
            for x in 0..width {
                let pixel = self.get_pixel(x, y).unwrap_or(Color::TRANSPARENT);
                if pixel.a > 0 {
                    // Simple pseudo-random noise based on position
                    let mut hasher = DefaultHasher::new();
                    (x, y).hash(&mut hasher);
                    let hash = hasher.finish();
                    let noise = ((hash % 256) as f32 - 128.0) * intensity / 128.0;
                    
                    let r = (pixel.r as f32 + noise).clamp(0.0, 255.0) as u8;
                    let g = (pixel.g as f32 + noise).clamp(0.0, 255.0) as u8;
                    let b = (pixel.b as f32 + noise).clamp(0.0, 255.0) as u8;
                    
                    self.set_pixel(x, y, Color::new(r, g, b, pixel.a));
                }
            }
        }
    }
    
    /// Apply outline effect
    pub fn apply_outline(&mut self, color: Color, thickness: usize) {
        let width = self.width();
        let height = self.height();
        let mut new_data = self.get_pixel_data().clone();
        
        for y in 0..height {
            for x in 0..width {
                let pixel = self.get_pixel(x, y).unwrap_or(Color::TRANSPARENT);
                if pixel.a > 0 {
                    // Check surrounding pixels
                    for dy in -(thickness as i32)..=(thickness as i32) {
                        for dx in -(thickness as i32)..=(thickness as i32) {
                            if dx == 0 && dy == 0 { continue; }
                            
                            let nx = x as i32 + dx;
                            let ny = y as i32 + dy;
                            
                            if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                                let nx = nx as usize;
                                let ny = ny as usize;
                                
                                let neighbor = self.get_pixel(nx, ny).unwrap_or(Color::TRANSPARENT);
                                if neighbor.a == 0 {
                                    // Distance check for circular outline
                                    let dist = ((dx * dx + dy * dy) as f32).sqrt();
                                    if dist <= thickness as f32 {
                                        new_data[ny][nx] = color;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        self.set_pixel_data(new_data);
    }
    
    /// Apply pixelate effect
    pub fn apply_pixelate(&mut self, block_size: usize) {
        let width = self.width();
        let height = self.height();
        let mut new_data = self.get_pixel_data().clone();
        
        for y in (0..height).step_by(block_size) {
            for x in (0..width).step_by(block_size) {
                // Calculate average color of the block
                let mut r_sum = 0u32;
                let mut g_sum = 0u32;
                let mut b_sum = 0u32;
                let mut a_sum = 0u32;
                let mut count = 0;
                
                for by in y..std::cmp::min(y + block_size, height) {
                    for bx in x..std::cmp::min(x + block_size, width) {
                        if let Some(pixel) = self.get_pixel(bx, by) {
                            r_sum += pixel.r as u32;
                            g_sum += pixel.g as u32;
                            b_sum += pixel.b as u32;
                            a_sum += pixel.a as u32;
                            count += 1;
                        }
                    }
                }
                
                if count > 0 {
                    let avg_color = Color::new(
                        (r_sum / count) as u8,
                        (g_sum / count) as u8,
                        (b_sum / count) as u8,
                        (a_sum / count) as u8,
                    );
                    
                    // Apply average color to the entire block
                    for by in y..std::cmp::min(y + block_size, height) {
                        for bx in x..std::cmp::min(x + block_size, width) {
                            new_data[by][bx] = avg_color;
                        }
                    }
                }
            }
        }
        
        self.set_pixel_data(new_data);
    }
    
    /// Replace colors
    pub fn replace_color(&mut self, from_color: Color, to_color: Color, tolerance: u8) {
        let width = self.width();
        let height = self.height();
        
        for y in 0..height {
            for x in 0..width {
                if let Some(pixel) = self.get_pixel(x, y) {
                    // Check if pixel matches the from_color within tolerance
                    let r_diff = (pixel.r as i32 - from_color.r as i32).abs();
                    let g_diff = (pixel.g as i32 - from_color.g as i32).abs();
                    let b_diff = (pixel.b as i32 - from_color.b as i32).abs();
                    let a_diff = (pixel.a as i32 - from_color.a as i32).abs();
                    
                    if r_diff <= tolerance as i32 && g_diff <= tolerance as i32 && 
                       b_diff <= tolerance as i32 && a_diff <= tolerance as i32 {
                        self.set_pixel(x, y, to_color);
                    }
                }
            }
        }
    }
    
    /// Fill connected area with color (bucket fill)
    pub fn flood_fill(&mut self, start_x: usize, start_y: usize, fill_color: Color) {
        let target_color = self.get_pixel(start_x, start_y).unwrap_or(Color::TRANSPARENT);
        if target_color.to_rgba() == fill_color.to_rgba() {
            return; // Same color, nothing to do
        }
        
        let mut stack = vec![(start_x, start_y)];
        let width = self.width();
        let height = self.height();
        
        while let Some((x, y)) = stack.pop() {
            if x >= width || y >= height {
                continue;
            }
            
            let current = self.get_pixel(x, y).unwrap_or(Color::TRANSPARENT);
            if current.to_rgba() != target_color.to_rgba() {
                continue;
            }
            
            self.set_pixel(x, y, fill_color);
            
            // Add neighboring pixels
            if x > 0 { stack.push((x - 1, y)); }
            if x + 1 < width { stack.push((x + 1, y)); }
            if y > 0 { stack.push((x, y - 1)); }
            if y + 1 < height { stack.push((x, y + 1)); }
        }
    }
}
