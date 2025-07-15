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

        eframe::egui::Window::new("Export Options")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("Export Settings:");
                ui.separator();
                
                // Format selection
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
                            ui.selectable_value(&mut self.export_format, crate::types::ExportFormat::TGA, "TGA");
                            ui.selectable_value(&mut self.export_format, crate::types::ExportFormat::WEBP, "WEBP");
                        });
                });
                
                ui.separator();
                
                // Export options
                ui.label("Export Options:");
                ui.checkbox(&mut self.export_individual_layers, "Export individual layers");
                ui.checkbox(&mut self.export_all_frames, "Export all frames");
                
                if self.export_individual_layers {
                    ui.label("  └ Creates separate files for each layer");
                }
                if self.export_all_frames {
                    ui.label("  └ Creates separate files for each frame");
                }
                
                ui.separator();
                
                // File naming preview
                ui.label("File naming preview:");
                ui.group(|ui| {
                    ui.label(format!("Current frame: composed_001.{}", self.get_file_extension()));
                    if self.export_all_frames {
                        ui.label(format!("All frames: composed_001.{}, composed_002.{}, ...", 
                                        self.get_file_extension(), self.get_file_extension()));
                    }
                    if self.export_individual_layers {
                        ui.label(format!("Layers: layer_01_Layer_1.{}, layer_02_Layer_2.{}, ...", 
                                        self.get_file_extension(), self.get_file_extension()));
                    }
                });
                
                ui.separator();
                
                // Export buttons
                ui.horizontal(|ui| {
                    if ui.button("Export Single").on_hover_text("Export current frame only").clicked() {
                        let filename = format!("pixel_art.{}", self.get_file_extension());
                        
                        if let Err(e) = self.save_image(&filename) {
                            eprintln!("Failed to export: {}", e);
                        } else {
                            println!("Exported: {}", filename);
                        }
                        
                        self.show_export_dialog = false;
                    }
                    
                    if ui.button("Export All").on_hover_text("Export all frames and layers to folder").clicked() {
                        self.save_all_dialog();
                        self.show_export_dialog = false;
                    }
                    
                    if ui.button("Cancel").clicked() {
                        self.show_export_dialog = false;
                    }
                });
                
                ui.separator();
                
                // Quick export buttons
                ui.label("Quick Export:");
                ui.horizontal(|ui| {
                    if ui.button("PNG").clicked() {
                        self.export_format = crate::types::ExportFormat::PNG;
                        let filename = format!("pixel_art.png");
                        if let Err(e) = self.save_image(&filename) {
                            eprintln!("Failed to export PNG: {}", e);
                        } else {
                            println!("Exported: {}", filename);
                        }
                        self.show_export_dialog = false;
                    }
                    
                    if ui.button("JPG").clicked() {
                        self.export_format = crate::types::ExportFormat::JPG;
                        let filename = format!("pixel_art.jpg");
                        if let Err(e) = self.save_image(&filename) {
                            eprintln!("Failed to export JPG: {}", e);
                        } else {
                            println!("Exported: {}", filename);
                        }
                        self.show_export_dialog = false;
                    }
                    
                    if ui.button("BMP").clicked() {
                        self.export_format = crate::types::ExportFormat::BMP;
                        let filename = format!("pixel_art.bmp");
                        if let Err(e) = self.save_image(&filename) {
                            eprintln!("Failed to export BMP: {}", e);
                        } else {
                            println!("Exported: {}", filename);
                        }
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

    /// Save all frames and layers in selected format
    pub fn save_all_dialog(&mut self) {
        if let Some(folder) = rfd::FileDialog::new()
            .set_title("Select folder to save all files")
            .pick_folder() {
            
            if let Err(e) = self.save_all_to_folder(&folder) {
                eprintln!("Failed to save all: {}", e);
            }
        }
    }
    
    /// Save all frames and layers to a specific folder
    pub fn save_all_to_folder(&self, folder: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        // Create subfolders if needed
        let frames_folder = folder.join("frames");
        let layers_folder = folder.join("layers");
        let composed_folder = folder.join("composed");
        
        std::fs::create_dir_all(&frames_folder)?;
        std::fs::create_dir_all(&layers_folder)?;
        std::fs::create_dir_all(&composed_folder)?;
        
        // Save composed frames
        if self.export_all_frames {
            self.save_all_frames(&frames_folder)?;
        }
        
        // Save individual layers
        if self.export_individual_layers {
            self.save_all_layers(&layers_folder)?;
        }
        
        // Save composed images (all layers combined)
        self.save_all_composed(&composed_folder)?;
        
        println!("All files saved to: {:?}", folder);
        Ok(())
    }
    
    /// Save all frames as separate files
    fn save_all_frames(&self, folder: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        for (frame_idx, frame) in self.frames.iter().enumerate() {
            let filename = format!("frame_{:03}.{}", frame_idx + 1, self.get_file_extension());
            let filepath = folder.join(filename);
            
            // Compose this frame
            let composed = self.compose_frame(frame);
            self.save_composed_image(&filepath, &composed)?;
        }
        Ok(())
    }
    
    /// Save all layers as separate files
    fn save_all_layers(&self, folder: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        for (frame_idx, frame) in self.frames.iter().enumerate() {
            let frame_folder = folder.join(format!("frame_{:03}", frame_idx + 1));
            std::fs::create_dir_all(&frame_folder)?;
            
            for (layer_idx, layer) in frame.layers.iter().enumerate() {
                if !layer.visible {
                    continue;
                }
                
                let layer_name = layer.name.replace(" ", "_").replace("/", "_").replace("\\", "_");
                let filename = format!("layer_{:02}_{}.{}", layer_idx + 1, layer_name, self.get_file_extension());
                let filepath = frame_folder.join(filename);
                
                self.save_layer_image(&filepath, layer)?;
            }
        }
        Ok(())
    }
    
    /// Save all composed images (all layers combined)
    fn save_all_composed(&self, folder: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        for (frame_idx, frame) in self.frames.iter().enumerate() {
            let filename = format!("composed_{:03}.{}", frame_idx + 1, self.get_file_extension());
            let filepath = folder.join(filename);
            
            let composed = self.compose_frame(frame);
            self.save_composed_image(&filepath, &composed)?;
        }
        Ok(())
    }
    
    /// Compose a single frame (combine all visible layers)
    fn compose_frame(&self, frame: &crate::types::Frame) -> Vec<Vec<eframe::egui::Color32>> {
        if frame.layers.is_empty() {
            return vec![];
        }
        
        let width = frame.layers[0].width();
        let height = frame.layers[0].height();
        let mut composed = vec![vec![eframe::egui::Color32::TRANSPARENT; width]; height];
        
        for layer in &frame.layers {
            if !layer.visible {
                continue;
            }
            
            for y in 0..height {
                for x in 0..width {
                    let pixel = layer.grid[y][x];
                    if pixel.a() > 0 {
                        // Simple alpha blending
                        if composed[y][x].a() == 0 {
                            composed[y][x] = pixel;
                        } else {
                            // Blend with existing pixel
                            let alpha = pixel.a() as f32 / 255.0;
                            let inv_alpha = 1.0 - alpha;
                            let existing = composed[y][x];
                            
                            let r = (pixel.r() as f32 * alpha + existing.r() as f32 * inv_alpha) as u8;
                            let g = (pixel.g() as f32 * alpha + existing.g() as f32 * inv_alpha) as u8;
                            let b = (pixel.b() as f32 * alpha + existing.b() as f32 * inv_alpha) as u8;
                            let a = (pixel.a() as f32 + existing.a() as f32 * inv_alpha).min(255.0) as u8;
                            
                            composed[y][x] = eframe::egui::Color32::from_rgba_unmultiplied(r, g, b, a);
                        }
                    }
                }
            }
        }
        
        composed
    }
    
    /// Save composed image data to file
    fn save_composed_image(&self, filepath: &std::path::Path, composed: &Vec<Vec<eframe::egui::Color32>>) -> Result<(), Box<dyn std::error::Error>> {
        if composed.is_empty() {
            return Err("No image data to save".into());
        }
        
        let width = composed[0].len() as u32;
        let height = composed.len() as u32;
        let mut image_data = vec![0u8; (width * height * 4) as usize];
        
        for y in 0..height {
            for x in 0..width {
                let pixel = composed[y as usize][x as usize];
                let idx = ((y * width + x) * 4) as usize;
                image_data[idx] = pixel.r();
                image_data[idx + 1] = pixel.g();
                image_data[idx + 2] = pixel.b();
                image_data[idx + 3] = pixel.a();
            }
        }
        
        let filepath_str = filepath.to_string_lossy().to_string();
        self.save_image_data(&filepath_str, &image_data, width, height)
    }
    
    /// Save single layer to file
    fn save_layer_image(&self, filepath: &std::path::Path, layer: &crate::types::Layer) -> Result<(), Box<dyn std::error::Error>> {
        let width = layer.width() as u32;
        let height = layer.height() as u32;
        let mut image_data = vec![0u8; (width * height * 4) as usize];
        
        for y in 0..height {
            for x in 0..width {
                let pixel = layer.grid[y as usize][x as usize];
                let idx = ((y * width + x) * 4) as usize;
                image_data[idx] = pixel.r();
                image_data[idx + 1] = pixel.g();
                image_data[idx + 2] = pixel.b();
                image_data[idx + 3] = pixel.a();
            }
        }
        
        let filepath_str = filepath.to_string_lossy().to_string();
        self.save_image_data(&filepath_str, &image_data, width, height)
    }
    
    /// Save image data with current format
    fn save_image_data(&self, filepath: &str, image_data: &[u8], width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
        match self.export_format {
            crate::types::ExportFormat::PNG => {
                self.save_as_png_data(filepath, image_data, width, height)?;
            }
            crate::types::ExportFormat::JPG | crate::types::ExportFormat::JPEG => {
                self.save_as_jpg_data(filepath, image_data, width, height)?;
            }
            crate::types::ExportFormat::BMP => {
                self.save_as_bmp_data(filepath, image_data, width, height)?;
            }
            crate::types::ExportFormat::GIF => {
                self.save_as_gif_data(filepath, image_data, width, height)?;
            }
            _ => {
                // Default to PNG for unsupported formats
                self.save_as_png_data(filepath, image_data, width, height)?;
            }
        }
        Ok(())
    }
    
    /// Get file extension based on current export format
    fn get_file_extension(&self) -> &str {
        match self.export_format {
            crate::types::ExportFormat::PNG => "png",
            crate::types::ExportFormat::JPG => "jpg",
            crate::types::ExportFormat::JPEG => "jpeg",
            crate::types::ExportFormat::BMP => "bmp",
            crate::types::ExportFormat::GIF => "gif",
            crate::types::ExportFormat::ICO => "ico",
            crate::types::ExportFormat::PCX => "pcx",
            crate::types::ExportFormat::POC => "poc",
            crate::types::ExportFormat::QOI => "qoi",
            crate::types::ExportFormat::SVG => "svg",
            crate::types::ExportFormat::TGA => "tga",
            crate::types::ExportFormat::WEBP => "webp",
            crate::types::ExportFormat::ASE => "ase",
            crate::types::ExportFormat::ASEPRITE => "aseprite",
            crate::types::ExportFormat::CSS => "css",
            crate::types::ExportFormat::FLC => "flc",
            crate::types::ExportFormat::FLI => "fli",
        }
    }
    
    /// Save PNG with image data
    fn save_as_png_data(&self, filepath: &str, image_data: &[u8], width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
        let mut img = image::RgbaImage::new(width, height);
        
        for y in 0..height {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;
                let r = image_data[idx];
                let g = image_data[idx + 1];
                let b = image_data[idx + 2];
                let a = image_data[idx + 3];
                img.put_pixel(x, y, image::Rgba([r, g, b, a]));
            }
        }
        
        img.save(filepath)?;
        Ok(())
    }
    
    /// Save JPG with image data
    fn save_as_jpg_data(&self, filepath: &str, image_data: &[u8], width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
        let mut img = image::RgbImage::new(width, height);
        
        for y in 0..height {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;
                let r = image_data[idx];
                let g = image_data[idx + 1];
                let b = image_data[idx + 2];
                // JPG doesn't support alpha, so we ignore it
                img.put_pixel(x, y, image::Rgb([r, g, b]));
            }
        }
        
        img.save(filepath)?;
        Ok(())
    }
    
    /// Save BMP with image data
    fn save_as_bmp_data(&self, filepath: &str, image_data: &[u8], width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
        let mut img = image::RgbImage::new(width, height);
        
        for y in 0..height {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;
                let r = image_data[idx];
                let g = image_data[idx + 1];
                let b = image_data[idx + 2];
                img.put_pixel(x, y, image::Rgb([r, g, b]));
            }
        }
        
        img.save(filepath)?;
        Ok(())
    }
    
    /// Save GIF with image data
    fn save_as_gif_data(&self, filepath: &str, image_data: &[u8], width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
        // For GIF, we'll use PNG as fallback since GIF requires palette conversion
        self.save_as_png_data(filepath, image_data, width, height)
    }
    
    /// Quick save current frame as PNG
    pub fn quick_save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let filename = format!("pixel_art_{}.png", timestamp);
        let composed = self.get_composed_grid();
        
        if composed.is_empty() {
            return Err("No canvas data to save".into());
        }
        
        let width = composed[0].len() as u32;
        let height = composed.len() as u32;
        let mut image_data = vec![0u8; (width * height * 4) as usize];
        
        for y in 0..height {
            for x in 0..width {
                let pixel = composed[y as usize][x as usize];
                let idx = ((y * width + x) * 4) as usize;
                image_data[idx] = pixel.r();
                image_data[idx + 1] = pixel.g();
                image_data[idx + 2] = pixel.b();
                image_data[idx + 3] = pixel.a();
            }
        }
        
        self.save_as_png_data(&filename, &image_data, width, height)?;
        println!("Quick saved as: {}", filename);
        Ok(())
    }
    
    /// Save project file (all frames and layers)
    pub fn save_project_file(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let project_data = serde_json::to_string_pretty(&self.frames)?;
        std::fs::write(filepath, project_data)?;
        println!("Project saved as: {}", filepath);
        Ok(())
    }
    
    /// Load project file
    pub fn load_project_file(&mut self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let project_data = std::fs::read_to_string(filepath)?;
        self.frames = serde_json::from_str(&project_data)?;
        self.current_frame = 0;
        self.current_layer = 0;
        println!("Project loaded from: {}", filepath);
        Ok(())
    }
    
    // ...existing code...
}
