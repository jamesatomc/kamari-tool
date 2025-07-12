use eframe::egui;
use image::{ImageBuffer, Rgba};
use rfd::FileDialog;
use crate::editor::PixelArtEditor;

impl PixelArtEditor {
    pub fn save_as_png(&self, path: &str) {
        let layer = &self.frames[self.current_frame].layers[self.current_layer];
        let width = layer.width() as u32;
        let height = layer.height() as u32;
        let mut img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);
        let composed = self.get_composed_grid();
        for y in 0..height as usize {
            for x in 0..width as usize {
                let c = composed[y][x];
                img.put_pixel(x as u32, y as u32, Rgba([c.r(), c.g(), c.b(), c.a()]));
            }
        }
        img.save(path).ok();
    }

    pub fn save_as_png_dialog(&self) {
        if let Some(path) = FileDialog::new()
            .add_filter("PNG Image", &["png"])
            .set_file_name("pixel_art.png")
            .save_file()
        {
            self.save_as_png(path.to_string_lossy().as_ref());
        }
    }

    pub fn resize_canvas(&mut self, new_width: usize, new_height: usize, anchor: usize) {
        self.push_undo();
        
        for frame in &mut self.frames {
            for layer in &mut frame.layers {
                let old_width = layer.width();
                let old_height = layer.height();
                let mut new_grid = vec![vec![egui::Color32::TRANSPARENT; new_width]; new_height];
                
                let (offset_x, offset_y) = match anchor {
                    0 => (0, 0), // Top-left
                    1 => { // Center
                        let x_offset = if new_width > old_width { (new_width - old_width) / 2 } else { 0 };
                        let y_offset = if new_height > old_height { (new_height - old_height) / 2 } else { 0 };
                        (x_offset, y_offset)
                    },
                    2 => { // Bottom-right
                        let x_offset = if new_width > old_width { new_width - old_width } else { 0 };
                        let y_offset = if new_height > old_height { new_height - old_height } else { 0 };
                        (x_offset, y_offset)
                    },
                    _ => (0, 0),
                };
                
                // Copy existing pixels to new grid
                for y in 0..old_height.min(new_height) {
                    for x in 0..old_width.min(new_width) {
                        let new_x = x + offset_x;
                        let new_y = y + offset_y;
                        if new_x < new_width && new_y < new_height {
                            new_grid[new_y][new_x] = layer.grid[y][x];
                        }
                    }
                }
                
                layer.grid = new_grid;
            }
        }
    }
}
