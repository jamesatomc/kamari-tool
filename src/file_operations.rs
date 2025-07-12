use eframe::egui;
use image::{ImageBuffer, Rgba, Rgb};
use rfd::FileDialog;
use crate::editor::PixelArtEditor;
use crate::types::ExportFormat;
use crate::constants::blend_colors;
use std::path::Path;

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

    pub fn export_project_dialog(&self) {
        if let Some(path) = FileDialog::new()
            .add_filter("PNG Image", &["png"])
            .add_filter("JPEG Image", &["jpg", "jpeg"])
            .add_filter("BMP Image", &["bmp"])
            .set_file_name("pixel_art")
            .save_file()
        {
            let path_str = path.to_string_lossy();
            let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("png");
            
            match extension.to_lowercase().as_str() {
                "png" => self.export_project(&path_str, ExportFormat::PNG),
                "jpg" | "jpeg" => self.export_project(&path_str, ExportFormat::JPG),
                "bmp" => self.export_project(&path_str, ExportFormat::BMP),
                _ => self.export_project(&path_str, ExportFormat::PNG),
            }
        }
    }

    pub fn export_project(&self, base_path: &str, format: ExportFormat) {
        let path = Path::new(base_path);
        let stem = path.file_stem().unwrap_or_default().to_string_lossy();
        let parent = path.parent().unwrap_or(Path::new("."));
        
        let extension = match format {
            ExportFormat::PNG => "png",
            ExportFormat::JPG => "jpg",
            ExportFormat::BMP => "bmp",
            ExportFormat::Aseprite => "aseprite",
        };

        // Export all frames as individual files
        for (frame_idx, frame) in self.frames.iter().enumerate() {
            let frame_name = if self.frames.len() > 1 {
                format!("{}_frame_{:03}", stem, frame_idx + 1)
            } else {
                stem.to_string()
            };

            // Export composed frame
            let composed_path = parent.join(format!("{}.{}", frame_name, extension));
            if let Some(path_str) = composed_path.to_str() {
                self.save_frame_as_image(frame_idx, path_str, format);
            }

            // Export individual layers if requested and more than one layer
            if self.export_individual_layers && frame.layers.len() > 1 {
                for (layer_idx, layer) in frame.layers.iter().enumerate() {
                    if layer.visible {
                        let layer_name = format!("{}_{}", frame_name, layer.name.replace(" ", "_"));
                        let layer_path = parent.join(format!("{}.{}", layer_name, extension));
                        if let Some(path_str) = layer_path.to_str() {
                            self.save_layer_as_image(frame_idx, layer_idx, path_str, format);
                        }
                    }
                }
            }
        }
    }

    pub fn save_frame_as_image(&self, frame_idx: usize, path: &str, format: ExportFormat) {
        if frame_idx >= self.frames.len() {
            return;
        }

        let frame = &self.frames[frame_idx];
        let width = frame.layers[0].width() as u32;
        let height = frame.layers[0].height() as u32;
        let mut img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);

        // Compose all visible layers
        let mut composed = vec![vec![egui::Color32::TRANSPARENT; width as usize]; height as usize];
        for layer in &frame.layers {
            if !layer.visible {
                continue;
            }
            for y in 0..height as usize {
                for x in 0..width as usize {
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

        // Convert to image
        for y in 0..height as usize {
            for x in 0..width as usize {
                let c = composed[y][x];
                img.put_pixel(x as u32, y as u32, Rgba([c.r(), c.g(), c.b(), c.a()]));
            }
        }

        match format {
            ExportFormat::PNG => {
                img.save_with_format(path, image::ImageFormat::Png).ok();
            }
            ExportFormat::JPG => {
                // Convert RGBA to RGB for JPEG
                let rgb_data: Vec<u8> = img.pixels()
                    .flat_map(|rgba| [rgba.0[0], rgba.0[1], rgba.0[2]])
                    .collect();
                let rgb_img: ImageBuffer<Rgb<u8>, Vec<u8>> = image::ImageBuffer::from_raw(width, height, rgb_data).unwrap();
                rgb_img.save_with_format(path, image::ImageFormat::Jpeg).ok();
            }
            ExportFormat::BMP => {
                img.save_with_format(path, image::ImageFormat::Bmp).ok();
            }
            ExportFormat::Aseprite => {
                // For now, save as PNG - later could implement actual .aseprite format
                img.save_with_format(path, image::ImageFormat::Png).ok();
            }
        }
    }

    pub fn save_layer_as_image(&self, frame_idx: usize, layer_idx: usize, path: &str, format: ExportFormat) {
        if frame_idx >= self.frames.len() || layer_idx >= self.frames[frame_idx].layers.len() {
            return;
        }

        let layer = &self.frames[frame_idx].layers[layer_idx];
        let width = layer.width() as u32;
        let height = layer.height() as u32;
        let mut img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);

        for y in 0..height as usize {
            for x in 0..width as usize {
                let c = layer.grid[y][x];
                let final_color = if layer.opacity < 1.0 {
                    let alpha = (c.a() as f32 * layer.opacity) as u8;
                    egui::Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), alpha)
                } else {
                    c
                };
                img.put_pixel(x as u32, y as u32, Rgba([final_color.r(), final_color.g(), final_color.b(), final_color.a()]));
            }
        }

        match format {
            ExportFormat::PNG => {
                img.save_with_format(path, image::ImageFormat::Png).ok();
            }
            ExportFormat::JPG => {
                // Convert RGBA to RGB for JPEG
                let rgb_data: Vec<u8> = img.pixels()
                    .flat_map(|rgba| [rgba.0[0], rgba.0[1], rgba.0[2]])
                    .collect();
                let rgb_img: ImageBuffer<Rgb<u8>, Vec<u8>> = image::ImageBuffer::from_raw(width, height, rgb_data).unwrap();
                rgb_img.save_with_format(path, image::ImageFormat::Jpeg).ok();
            }
            ExportFormat::BMP => {
                img.save_with_format(path, image::ImageFormat::Bmp).ok();
            }
            ExportFormat::Aseprite => {
                img.save_with_format(path, image::ImageFormat::Png).ok();
            }
        }
    }

    pub fn show_export_dialog(&mut self, ctx: &egui::Context) {
        egui::Window::new("Export Options")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.heading("Export Settings");
                    ui.separator();
                    
                    ui.horizontal(|ui| {
                        ui.label("Format:");
                        egui::ComboBox::from_label("")
                            .selected_text(format!("{:?}", self.export_format))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.export_format, ExportFormat::PNG, "PNG");
                                ui.selectable_value(&mut self.export_format, ExportFormat::JPG, "JPG");
                                ui.selectable_value(&mut self.export_format, ExportFormat::BMP, "BMP");
                                ui.selectable_value(&mut self.export_format, ExportFormat::Aseprite, "Aseprite");
                            });
                    });

                    ui.checkbox(&mut self.export_individual_layers, "Export individual layers");
                    ui.checkbox(&mut self.export_all_frames, "Export all frames");
                    
                    ui.separator();
                    
                    ui.horizontal(|ui| {
                        if ui.button("Export").clicked() {
                            // Close dialog first to prevent recursion
                            self.show_export_dialog = false;
                            
                            // Then show file dialog
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("PNG Image", &["png"])
                                .add_filter("JPEG Image", &["jpg", "jpeg"])
                                .add_filter("BMP Image", &["bmp"])
                                .set_file_name("pixel_art")
                                .save_file()
                            {
                                let path_str = path.to_string_lossy();
                                let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("png");
                                
                                let format = match extension.to_lowercase().as_str() {
                                    "png" => ExportFormat::PNG,
                                    "jpg" | "jpeg" => ExportFormat::JPG,
                                    "bmp" => ExportFormat::BMP,
                                    _ => ExportFormat::PNG,
                                };
                                
                                self.export_project(&path_str, format);
                            }
                        }
                        if ui.button("Cancel").clicked() {
                            self.show_export_dialog = false;
                        }
                    });
                });
            });
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
