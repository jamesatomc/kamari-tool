use crate::editor::core::PixelArtEditor;

impl PixelArtEditor {
    pub fn resize_canvas(&mut self, new_width: usize, new_height: usize, anchor: usize) {
        self.push_undo();
        
        for frame in &mut self.frames {
            for layer in &mut frame.layers {
                let old_width = layer.width();
                let old_height = layer.height();
                let mut new_grid = vec![vec![eframe::egui::Color32::TRANSPARENT; new_width]; new_height];
                
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

    pub fn save_as_png(&self, filename: &str) {
        // Implementation for saving as PNG
        let composed = self.get_composed_grid();
        if let Some(first_row) = composed.first() {
            let width = first_row.len();
            let height = composed.len();
            
            // Create image buffer
            let mut img = image::RgbaImage::new(width as u32, height as u32);
            
            for y in 0..height {
                for x in 0..width {
                    let pixel = composed[y][x];
                    img.put_pixel(x as u32, y as u32, image::Rgba([pixel.r(), pixel.g(), pixel.b(), pixel.a()]));

                }
            }
            
            // Save the image
            if let Err(e) = img.save(filename) {
                eprintln!("Failed to save PNG: {}", e);
            }
        }
    }

    pub fn save_as_png_dialog(&mut self) {
        // Implementation for PNG save dialog
        // This would typically open a file dialog
        self.save_as_png("pixel_art.png");
    }

    pub fn show_export_dialog(&mut self, ctx: &eframe::egui::Context) {
        if !self.show_export_dialog {
            return;
        }

        eframe::egui::Window::new("Export")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("Export Settings:");
                
                ui.horizontal(|ui| {
                    ui.label("Format:");
                    eframe::egui::ComboBox::from_id_salt("export_format")
                        .selected_text(format!("{:?}", self.export_format))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.export_format, crate::types::ExportFormat::PNG, "PNG");
                            ui.selectable_value(&mut self.export_format, crate::types::ExportFormat::JPG, "JPG");
                            ui.selectable_value(&mut self.export_format, crate::types::ExportFormat::JPEG, "JPEG");
                            ui.selectable_value(&mut self.export_format, crate::types::ExportFormat::BMP, "BMP");
                            ui.selectable_value(&mut self.export_format, crate::types::ExportFormat::GIF, "GIF");
                            ui.selectable_value(&mut self.export_format, crate::types::ExportFormat::ICO, "ICO");
                            ui.selectable_value(&mut self.export_format, crate::types::ExportFormat::PCX, "PCX");
                            ui.selectable_value(&mut self.export_format, crate::types::ExportFormat::POC, "POC");
                            ui.selectable_value(&mut self.export_format, crate::types::ExportFormat::QOI, "QOI");
                            ui.selectable_value(&mut self.export_format, crate::types::ExportFormat::SVG, "SVG");
                            ui.selectable_value(&mut self.export_format, crate::types::ExportFormat::TGA, "TGA");
                            ui.selectable_value(&mut self.export_format, crate::types::ExportFormat::WEBP, "WEBP");
                            ui.selectable_value(&mut self.export_format, crate::types::ExportFormat::ASE, "ASE");
                            ui.selectable_value(&mut self.export_format, crate::types::ExportFormat::ASEPRITE, "ASEPRITE");
                            ui.selectable_value(&mut self.export_format, crate::types::ExportFormat::CSS, "CSS");
                            ui.selectable_value(&mut self.export_format, crate::types::ExportFormat::FLC, "FLC");
                            ui.selectable_value(&mut self.export_format, crate::types::ExportFormat::FLI, "FLI");
                        });
                });
                
                ui.checkbox(&mut self.export_individual_layers, "Export individual layers");
                ui.checkbox(&mut self.export_all_frames, "Export all frames");
                
                ui.horizontal(|ui| {
                    if ui.button("Export").clicked() {
                        let filename = match self.export_format {
                            crate::types::ExportFormat::PNG => "exported_pixel_art.png",
                            crate::types::ExportFormat::JPG => "exported_pixel_art.jpg",
                            crate::types::ExportFormat::JPEG => "exported_pixel_art.jpeg",
                            crate::types::ExportFormat::BMP => "exported_pixel_art.bmp",
                            crate::types::ExportFormat::GIF => "exported_pixel_art.gif",
                            crate::types::ExportFormat::ICO => "exported_pixel_art.ico",
                            crate::types::ExportFormat::PCX => "exported_pixel_art.pcx",
                            crate::types::ExportFormat::POC => "exported_pixel_art.poc",
                            crate::types::ExportFormat::QOI => "exported_pixel_art.qoi",
                            crate::types::ExportFormat::SVG => "exported_pixel_art.svg",
                            crate::types::ExportFormat::TGA => "exported_pixel_art.tga",
                            crate::types::ExportFormat::WEBP => "exported_pixel_art.webp",
                            crate::types::ExportFormat::ASE => "exported_pixel_art.ase",
                            crate::types::ExportFormat::ASEPRITE => "exported_pixel_art.aseprite",
                            crate::types::ExportFormat::CSS => "exported_pixel_art.css",
                            crate::types::ExportFormat::FLC => "exported_pixel_art.flc",
                            crate::types::ExportFormat::FLI => "exported_pixel_art.fli",
                        };
                        
                        if let Err(e) = self.save_image(filename) {
                            eprintln!("Failed to export: {}", e);
                        }
                        
                        self.show_export_dialog = false;
                    }
                    
                    if ui.button("Cancel").clicked() {
                        self.show_export_dialog = false;
                    }
                });
            });
    }

    pub fn save_image(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        use std::fs::File;
        use std::io::Write;
        
        // Get composed grid and convert to image data
        let composed = self.get_composed_grid();
        if composed.is_empty() {
            return Err("No canvas data to export".into());
        }
        
        let width = composed[0].len() as u32;
        let height = composed.len() as u32;
        let mut image_data = vec![0u8; (width * height * 4) as usize];
        
        for y in 0..height {
            for x in 0..width {
                let pixel = composed[y as usize][x as usize];
                let idx = ((y * width + x) * 4) as usize;
                image_data[idx] = pixel.r();     // R
                image_data[idx + 1] = pixel.g(); // G
                image_data[idx + 2] = pixel.b(); // B
                image_data[idx + 3] = pixel.a(); // A
            }
        }
        
        match self.export_format {
            crate::types::ExportFormat::PNG => {
                self.save_as_png_new(filepath, &image_data, width, height)?;
            }
            crate::types::ExportFormat::JPG | crate::types::ExportFormat::JPEG => {
                self.save_as_jpg(filepath, &image_data, width, height)?;
            }
            crate::types::ExportFormat::BMP => {
                self.save_as_bmp(filepath, &image_data, width, height)?;
            }
            crate::types::ExportFormat::GIF => {
                self.save_as_gif(filepath, &image_data, width, height)?;
            }
            crate::types::ExportFormat::ICO => {
                self.save_as_ico(filepath, &image_data, width, height)?;
            }
            crate::types::ExportFormat::PCX => {
                self.save_as_pcx(filepath, &image_data, width, height)?;
            }
            crate::types::ExportFormat::POC => {
                self.save_as_poc(filepath, &image_data, width, height)?;
            }
            crate::types::ExportFormat::QOI => {
                self.save_as_qoi(filepath, &image_data, width, height)?;
            }
            crate::types::ExportFormat::SVG => {
                self.save_as_svg(filepath, &image_data, width, height)?;
            }
            crate::types::ExportFormat::TGA => {
                self.save_as_tga(filepath, &image_data, width, height)?;
            }
            crate::types::ExportFormat::WEBP => {
                self.save_as_webp(filepath, &image_data, width, height)?;
            }
            crate::types::ExportFormat::ASE | crate::types::ExportFormat::ASEPRITE => {
                self.save_as_aseprite(filepath, &image_data, width, height)?;
            }
            crate::types::ExportFormat::CSS => {
                self.save_as_css(filepath, &image_data, width, height)?;
            }
            crate::types::ExportFormat::FLC | crate::types::ExportFormat::FLI => {
                self.save_as_flic(filepath, &image_data, width, height)?;
            }
        }
        
        Ok(())
    }
    
    fn save_as_png_new(&self, filepath: &str, image_data: &[u8], width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
        // Use the existing PNG method but with new signature
        let composed = self.get_composed_grid();
        if let Some(first_row) = composed.first() {
            let width = first_row.len();
            let height = composed.len();
            
            // Create image buffer
            let mut img = image::RgbaImage::new(width as u32, height as u32);
            
            for y in 0..height {
                for x in 0..width {
                    let pixel = composed[y][x];
                    img.put_pixel(x as u32, y as u32, image::Rgba([pixel.r(), pixel.g(), pixel.b(), pixel.a()]));
                }
            }
            
            // Save the image
            img.save(filepath)?;
        }
        Ok(())
    }
    
    fn save_as_jpg(&self, filepath: &str, image_data: &[u8], width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
        // Convert RGBA to RGB (JPG doesn't support alpha)
        let composed = self.get_composed_grid();
        if let Some(first_row) = composed.first() {
            let width = first_row.len();
            let height = composed.len();
            
            // Create RGB image buffer
            let mut img = image::RgbImage::new(width as u32, height as u32);
            
            for y in 0..height {
                for x in 0..width {
                    let pixel = composed[y][x];
                    img.put_pixel(x as u32, y as u32, image::Rgb([pixel.r(), pixel.g(), pixel.b()]));
                }
            }
            
            // Save the image
            img.save(filepath)?;
        }
        Ok(())
    }
    
    fn save_as_bmp(&self, filepath: &str, image_data: &[u8], width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
        use std::fs::File;
        use std::io::Write;
        
        let file = File::create(filepath)?;
        let mut writer = std::io::BufWriter::new(file);
        
        // BMP header
        let file_size = 54 + (width * height * 3) as u32;
        
        // BMP file header
        writer.write_all(b"BM")?; // Signature
        writer.write_all(&file_size.to_le_bytes())?; // File size
        writer.write_all(&[0, 0, 0, 0])?; // Reserved
        writer.write_all(&54u32.to_le_bytes())?; // Data offset
        
        // BMP info header
        writer.write_all(&40u32.to_le_bytes())?; // Header size
        writer.write_all(&width.to_le_bytes())?; // Width
        writer.write_all(&height.to_le_bytes())?; // Height
        writer.write_all(&1u16.to_le_bytes())?; // Planes
        writer.write_all(&24u16.to_le_bytes())?; // Bits per pixel
        writer.write_all(&[0; 24])?; // Compression and other fields
        
        // Write pixel data (BMP is bottom-up)
        for y in (0..height).rev() {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;
                writer.write_all(&[image_data[idx + 2], image_data[idx + 1], image_data[idx]])?; // BGR
            }
        }
        
        Ok(())
    }
    
    fn save_as_gif(&self, filepath: &str, image_data: &[u8], width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
        // GIF format implementation (using fallback for now)
        self.write_raw_rgb(filepath, image_data, width, height)?;
        Ok(())
    }
    
    fn save_as_ico(&self, filepath: &str, image_data: &[u8], width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
        // ICO format implementation (using fallback for now)
        self.write_raw_rgb(filepath, image_data, width, height)?;
        Ok(())
    }
    
    fn save_as_pcx(&self, filepath: &str, image_data: &[u8], width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
        // PCX format implementation (using fallback for now)
        self.write_raw_rgb(filepath, image_data, width, height)?;
        Ok(())
    }
    
    fn save_as_poc(&self, filepath: &str, image_data: &[u8], width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
        // POC format implementation (using fallback for now)
        self.write_raw_rgb(filepath, image_data, width, height)?;
        Ok(())
    }
    
    fn save_as_qoi(&self, filepath: &str, image_data: &[u8], width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
        // QOI format implementation (using fallback for now)
        self.write_raw_rgb(filepath, image_data, width, height)?;
        Ok(())
    }
    
    fn save_as_svg(&self, filepath: &str, image_data: &[u8], width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
        use std::fs::File;
        use std::io::Write;
        
        let file = File::create(filepath)?;
        let mut writer = std::io::BufWriter::new(file);
        
        // SVG header
        writeln!(writer, r#"<?xml version="1.0" encoding="UTF-8"?>"#)?;
        writeln!(writer, r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}">"#, width, height)?;
        
        // Write pixels as rectangles
        for y in 0..height {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;
                let r = image_data[idx];
                let g = image_data[idx + 1];
                let b = image_data[idx + 2];
                let a = image_data[idx + 3];
                
                if a > 0 { // Only write non-transparent pixels
                    writeln!(writer, r#"  <rect x="{}" y="{}" width="1" height="1" fill="rgb({},{},{})" opacity="{}"/>"#, 
                             x, y, r, g, b, a as f32 / 255.0)?;
                }
            }
        }
        
        writeln!(writer, "</svg>")?;
        Ok(())
    }
    
    fn save_as_tga(&self, filepath: &str, image_data: &[u8], width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
        // TGA format implementation (using fallback for now)
        self.write_raw_rgb(filepath, image_data, width, height)?;
        Ok(())
    }
    
    fn save_as_webp(&self, filepath: &str, image_data: &[u8], width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
        // WEBP format implementation (using fallback for now)
        self.write_raw_rgb(filepath, image_data, width, height)?;
        Ok(())
    }
    
    fn save_as_aseprite(&self, filepath: &str, image_data: &[u8], width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
        // Aseprite format implementation (using fallback for now)
        self.write_raw_rgb(filepath, image_data, width, height)?;
        Ok(())
    }
    
    fn save_as_css(&self, filepath: &str, image_data: &[u8], width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
        use std::fs::File;
        use std::io::Write;
        
        let file = File::create(filepath)?;
        let mut writer = std::io::BufWriter::new(file);
        
        // CSS pixel art representation
        writeln!(writer, ".pixel-art {{")?;
        writeln!(writer, "  width: {}px;", width)?;
        writeln!(writer, "  height: {}px;", height)?;
        writeln!(writer, "  background-image:")?;
        
        // Create a CSS gradient representation
        write!(writer, "    linear-gradient(to bottom")?;
        for y in 0..height {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;
                let r = image_data[idx];
                let g = image_data[idx + 1];
                let b = image_data[idx + 2];
                let a = image_data[idx + 3];
                
                if a > 0 {
                    write!(writer, ", rgba({}, {}, {}, {}) {}px {}px", r, g, b, a as f32 / 255.0, x, y)?;
                }
            }
        }
        writeln!(writer, ");")?;
        writeln!(writer, "  background-size: 1px 1px;")?;
        writeln!(writer, "}}")?;
        Ok(())
    }
    
    fn save_as_flic(&self, filepath: &str, image_data: &[u8], width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
        // FLC/FLI format implementation (using fallback for now)
        self.write_raw_rgb(filepath, image_data, width, height)?;
        Ok(())
    }
    
    fn write_raw_rgb(&self, filepath: &str, image_data: &[u8], width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
        use std::fs::File;
        use std::io::Write;
        
        let file = File::create(filepath)?;
        let mut writer = std::io::BufWriter::new(file);
        
        // Write raw RGB data as fallback
        for chunk in image_data.chunks(4) {
            writer.write_all(&[chunk[0], chunk[1], chunk[2]])?; // RGB only
        }
        
        Ok(())
    }

    // ...existing code...
}
