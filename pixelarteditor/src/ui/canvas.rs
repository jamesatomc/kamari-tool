use eframe::egui;
use crate::editor::PixelArtEditor;
use crate::types::Tool;
use crate::constants::PIXEL_SIZE;

impl PixelArtEditor {
    pub fn show_canvas(&mut self, ui: &mut egui::Ui) {
        let composed = self.get_composed_grid();
        let height = composed.len();
        let width = if height > 0 { composed[0].len() } else { 0 };

        let pixel_size = PIXEL_SIZE * self.zoom;
        let canvas_size = egui::vec2(width as f32 * pixel_size, height as f32 * pixel_size);

        // Create a scroll area that centers the canvas
        let scroll_area = egui::ScrollArea::both()
            .auto_shrink([false; 2])
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded);

        let scroll_output = scroll_area.show(ui, |ui| {
            // Handle mouse wheel zoom globally for the canvas area
            let scroll_delta = ui.input(|i| i.smooth_scroll_delta);
            if scroll_delta.y != 0.0 {
                if scroll_delta.y > 0.0 {
                    // Zoom in
                    self.zoom_in();
                } else {
                    // Zoom out
                    self.zoom_out();
                }
            }
            
            // Calculate the available space
            let available_size = ui.available_size();
            
            // Calculate padding to center the canvas
            let padding_x = (available_size.x - canvas_size.x).max(0.0) / 2.0;
            let padding_y = (available_size.y - canvas_size.y).max(0.0) / 2.0;
            
            // Add padding around the canvas to center it
            ui.allocate_space(egui::vec2(0.0, padding_y));
            
            ui.horizontal(|ui| {
                ui.allocate_space(egui::vec2(padding_x, 0.0));
                
                // Allocate the canvas area
                let (canvas_rect, _response) = ui.allocate_exact_size(canvas_size, egui::Sense::click_and_drag());
                
                // Handle canvas centering on first load
                if self.canvas_center_on_start {
                    self.canvas_center_on_start = false;
                    // Center the canvas by setting initial scroll position
                    if available_size.x > 0.0 && available_size.y > 0.0 {
                        let center_x = (canvas_size.x - available_size.x) / 2.0;
                        let center_y = (canvas_size.y - available_size.y) / 2.0;
                        ui.scroll_to_rect(egui::Rect::from_min_size(
                            egui::pos2(center_x, center_y),
                            available_size
                        ), Some(egui::Align::Center));
                    }
                }

                // Performance optimization: Only draw visible pixels
                let visible_rect = ui.clip_rect();
                let min_x = ((visible_rect.min.x - canvas_rect.min.x) / pixel_size).floor().max(0.0) as usize;
                let max_x = ((visible_rect.max.x - canvas_rect.min.x) / pixel_size).ceil().min(width as f32) as usize;
                let min_y = ((visible_rect.min.y - canvas_rect.min.y) / pixel_size).floor().max(0.0) as usize;
                let max_y = ((visible_rect.max.y - canvas_rect.min.y) / pixel_size).ceil().min(height as f32) as usize;

                // Draw the canvas background and pixels (only visible ones)
                for y in min_y..max_y {
                    for x in min_x..max_x {
                        let pixel_rect = egui::Rect::from_min_size(
                            canvas_rect.min + egui::vec2(x as f32 * pixel_size, y as f32 * pixel_size),
                            egui::vec2(pixel_size, pixel_size),
                        );

                        // Draw checkerboard background for transparent pixels
                        let bg_color = if (x + y) % 2 == 0 {
                            egui::Color32::from_gray(220)
                        } else {
                            egui::Color32::from_gray(240)
                        };

                        ui.painter().rect_filled(pixel_rect, 0.0, bg_color);

                        // Draw the pixel on top if not transparent
                        let pixel_color = composed[y][x];
                        if pixel_color.a() > 0 {
                            ui.painter().rect_filled(pixel_rect, 0.0, pixel_color);
                        }

                        // Draw animation effects
                        self.draw_pixel_animation_effects(ui, pixel_rect, x, y, pixel_size);

                        // Draw grid lines if enabled (only at certain zoom levels)
                        if self.show_grid && pixel_size > 2.0 {
                            ui.painter().rect_filled(
                                egui::Rect::from_min_size(
                                    pixel_rect.min,
                                    egui::vec2(pixel_size, 1.0),
                                ),
                                0.0,
                                egui::Color32::from_gray(128),
                            );
                            ui.painter().rect_filled(
                                egui::Rect::from_min_size(
                                    pixel_rect.min,
                                    egui::vec2(1.0, pixel_size),
                                ),
                                0.0,
                                egui::Color32::from_gray(128),
                            );
                        }

                        // Handle pixel interaction
                        self.handle_pixel_interaction(ui, pixel_rect, x, y, width, height, &composed);
                    }
                }

                // Draw tool overlays
                self.draw_tool_overlays(ui, &canvas_rect, width, height, pixel_size);
                
                // Draw tool animations
                self.draw_tool_animations(ui, &canvas_rect, width, height, pixel_size);
                
                ui.allocate_space(egui::vec2(padding_x, 0.0));
            });
            
            ui.allocate_space(egui::vec2(0.0, padding_y));
        });

        // Store scroll position for future reference
        self.canvas_scroll_offset = scroll_output.state.offset;
    }

    fn draw_pixel_animation_effects(&self, ui: &mut egui::Ui, pixel_rect: egui::Rect, x: usize, y: usize, pixel_size: f32) {
        // Draw sparkle effects for specific tools
        for tool in [Tool::Spray, Tool::Circle, Tool::Dither].iter() {
            let effects = self.get_tool_effects(*tool);
            for effect_pos in effects {
                let effect_x = effect_pos.x as usize;
                let effect_y = effect_pos.y as usize;
                
                if effect_x == x && effect_y == y {
                    // Draw sparkle animation
                    let time = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs_f32();
                    
                    let sparkle_alpha = (time * 5.0).sin().abs();
                    let sparkle_color = egui::Color32::from_rgba_unmultiplied(255, 255, 255, (sparkle_alpha * 255.0) as u8);
                    
                    // Draw sparkle cross
                    let center = pixel_rect.center();
                    let size = pixel_size * 0.8;
                    
                    ui.painter().line_segment(
                        [center - egui::vec2(size/2.0, 0.0), center + egui::vec2(size/2.0, 0.0)],
                        egui::Stroke::new(2.0, sparkle_color)
                    );
                    ui.painter().line_segment(
                        [center - egui::vec2(0.0, size/2.0), center + egui::vec2(0.0, size/2.0)],
                        egui::Stroke::new(2.0, sparkle_color)
                    );
                }
            }
        }
    }

    fn draw_tool_animations(&self, ui: &mut egui::Ui, canvas_rect: &egui::Rect, width: usize, height: usize, pixel_size: f32) {
        // Draw current tool animation
        if let Some(ref animation) = self.current_tool_animation {
            if animation.is_active {
                let progress = animation.get_progress();
                let (scale, rotation, alpha) = self.get_tool_animation_transform(animation.tool);
                
                // Draw animation based on tool type
                match animation.tool {
                    Tool::Pencil => {
                        self.draw_pencil_animation(ui, canvas_rect, scale, alpha, progress);
                    }
                    Tool::Eraser => {
                        self.draw_eraser_animation(ui, canvas_rect, scale, alpha, progress);
                    }
                    Tool::Bucket => {
                        self.draw_bucket_animation(ui, canvas_rect, scale, alpha, progress);
                    }
                    Tool::Spray => {
                        self.draw_spray_animation(ui, canvas_rect, scale, alpha, progress);
                    }
                    Tool::Line => {
                        self.draw_line_animation(ui, canvas_rect, scale, alpha, progress);
                    }
                    Tool::Rectangle => {
                        self.draw_rectangle_animation(ui, canvas_rect, scale, rotation, alpha, progress);
                    }
                    Tool::Circle => {
                        self.draw_circle_animation(ui, canvas_rect, scale, alpha, progress);
                    }
                    Tool::Dither => {
                        self.draw_dither_animation(ui, canvas_rect, scale, alpha, progress);
                    }
                    _ => {}
                }
                
                // Draw particles
                for particle in &animation.particles {
                    let pos = canvas_rect.min + particle.position * pixel_size;
                    let color = egui::Color32::from_rgba_unmultiplied(
                        particle.color.r(),
                        particle.color.g(),
                        particle.color.b(),
                        (particle.alpha * 255.0) as u8
                    );
                    
                    ui.painter().circle_filled(pos, particle.size, color);
                }
            }
        }
    }

    fn draw_pencil_animation(&self, ui: &mut egui::Ui, canvas_rect: &egui::Rect, scale: f32, alpha: f32, progress: f32) {
        if let Some(hover_pos) = ui.input(|i| i.pointer.hover_pos()) {
            if canvas_rect.contains(hover_pos) {
                let pulse_radius = 10.0 * scale;
                let color = egui::Color32::from_rgba_unmultiplied(
                    self.selected_color.r(),
                    self.selected_color.g(),
                    self.selected_color.b(),
                    (alpha * 128.0) as u8
                );
                
                ui.painter().circle_stroke(hover_pos, pulse_radius, egui::Stroke::new(2.0, color));
            }
        }
    }

    fn draw_eraser_animation(&self, ui: &mut egui::Ui, canvas_rect: &egui::Rect, scale: f32, alpha: f32, progress: f32) {
        if let Some(hover_pos) = ui.input(|i| i.pointer.hover_pos()) {
            if canvas_rect.contains(hover_pos) {
                let fade_radius = 15.0 * scale;
                let color = egui::Color32::from_rgba_unmultiplied(255, 255, 255, (alpha * 100.0) as u8);
                
                ui.painter().circle_filled(hover_pos, fade_radius, color);
            }
        }
    }

    fn draw_bucket_animation(&self, ui: &mut egui::Ui, canvas_rect: &egui::Rect, scale: f32, alpha: f32, progress: f32) {
        if let Some(hover_pos) = ui.input(|i| i.pointer.hover_pos()) {
            if canvas_rect.contains(hover_pos) {
                let bounce_offset = (1.0 - progress).powi(2) * 10.0;
                let pos = hover_pos - egui::vec2(0.0, bounce_offset);
                let color = egui::Color32::from_rgba_unmultiplied(
                    self.selected_color.r(),
                    self.selected_color.g(),
                    self.selected_color.b(),
                    (alpha * 255.0) as u8
                );
                
                ui.painter().rect_filled(
                    egui::Rect::from_center_size(pos, egui::vec2(20.0 * scale, 20.0 * scale)),
                    5.0,
                    color
                );
            }
        }
    }

    fn draw_spray_animation(&self, ui: &mut egui::Ui, canvas_rect: &egui::Rect, scale: f32, alpha: f32, progress: f32) {
        if let Some(hover_pos) = ui.input(|i| i.pointer.hover_pos()) {
            if canvas_rect.contains(hover_pos) {
                let spray_radius = (self.spray_size as f32 * 20.0) * scale;
                
                // Draw spray particles
                for i in 0..8 {
                    let angle = (i as f32 / 8.0) * std::f32::consts::PI * 2.0 + progress * std::f32::consts::PI * 2.0;
                    let distance = spray_radius * (0.5 + 0.5 * (progress * 10.0 + i as f32).sin());
                    let particle_pos = hover_pos + egui::vec2(angle.cos(), angle.sin()) * distance;
                    
                    let color = egui::Color32::from_rgba_unmultiplied(
                        self.selected_color.r(),
                        self.selected_color.g(),
                        self.selected_color.b(),
                        (alpha * 128.0) as u8
                    );
                    
                    ui.painter().circle_filled(particle_pos, 2.0, color);
                }
            }
        }
    }

    fn draw_line_animation(&self, ui: &mut egui::Ui, canvas_rect: &egui::Rect, scale: f32, alpha: f32, progress: f32) {
        if let Some((start_x, start_y)) = self.line_start {
            if let Some(hover_pos) = ui.input(|i| i.pointer.hover_pos()) {
                if canvas_rect.contains(hover_pos) {
                    let start_pos = canvas_rect.min + egui::vec2(start_x as f32 * 20.0, start_y as f32 * 20.0);
                    let glow_color = egui::Color32::from_rgba_unmultiplied(255, 255, 255, (alpha * 100.0) as u8);
                    
                    ui.painter().line_segment(
                        [start_pos, hover_pos],
                        egui::Stroke::new(4.0 * scale, glow_color)
                    );
                }
            }
        }
    }

    fn draw_rectangle_animation(&self, ui: &mut egui::Ui, canvas_rect: &egui::Rect, scale: f32, rotation: f32, alpha: f32, progress: f32) {
        if let Some((start_x, start_y)) = self.rectangle_start {
            if let Some(hover_pos) = ui.input(|i| i.pointer.hover_pos()) {
                if canvas_rect.contains(hover_pos) {
                    let start_pos = canvas_rect.min + egui::vec2(start_x as f32 * 20.0, start_y as f32 * 20.0);
                    let center_vec = (start_pos.to_vec2() + hover_pos.to_vec2()) / 2.0;
                    let center = egui::pos2(center_vec.x, center_vec.y);
                    let size = (hover_pos - start_pos).abs() * scale;
                    
                    let color = egui::Color32::from_rgba_unmultiplied(
                        self.selected_color.r(),
                        self.selected_color.g(),
                        self.selected_color.b(),
                        (alpha * 128.0) as u8
                    );
                    
                    // Draw rotating rectangle outline
                    let rect = egui::Rect::from_center_size(center, size);
                    ui.painter().rect_stroke(rect, 0.0, egui::Stroke::new(3.0, color), egui::epaint::StrokeKind::Middle);
                }
            }
        }
    }

    fn draw_circle_animation(&self, ui: &mut egui::Ui, canvas_rect: &egui::Rect, scale: f32, alpha: f32, progress: f32) {
        if let Some((start_x, start_y)) = self.circle_start {
            if let Some(hover_pos) = ui.input(|i| i.pointer.hover_pos()) {
                if canvas_rect.contains(hover_pos) {
                    let start_pos = canvas_rect.min + egui::vec2(start_x as f32 * 20.0, start_y as f32 * 20.0);
                    let radius = (hover_pos - start_pos).length() * scale;
                    
                    let color = egui::Color32::from_rgba_unmultiplied(
                        self.selected_color.r(),
                        self.selected_color.g(),
                        self.selected_color.b(),
                        (alpha * 128.0) as u8
                    );
                    
                    ui.painter().circle_stroke(start_pos, radius, egui::Stroke::new(3.0, color));
                    
                    // Draw sparkle effects around the circle
                    for i in 0..6 {
                        let angle = (i as f32 / 6.0) * std::f32::consts::PI * 2.0 + progress * std::f32::consts::PI * 2.0;
                        let sparkle_pos = start_pos + egui::vec2(angle.cos(), angle.sin()) * radius;
                        ui.painter().circle_filled(sparkle_pos, 2.0, egui::Color32::WHITE);
                    }
                }
            }
        }
    }

    fn draw_dither_animation(&self, ui: &mut egui::Ui, canvas_rect: &egui::Rect, scale: f32, alpha: f32, progress: f32) {
        if let Some(hover_pos) = ui.input(|i| i.pointer.hover_pos()) {
            if canvas_rect.contains(hover_pos) {
                let glow_size = 30.0 * scale;
                let color = egui::Color32::from_rgba_unmultiplied(
                    self.selected_color.r(),
                    self.selected_color.g(),
                    self.selected_color.b(),
                    (alpha * 80.0) as u8
                );
                
                // Draw dither pattern preview
                for i in 0..4 {
                    for j in 0..4 {
                        let pattern_pos = hover_pos + egui::vec2(
                            (i as f32 - 1.5) * 8.0,
                            (j as f32 - 1.5) * 8.0
                        );
                        
                        if (i + j) % 2 == 0 {
                            ui.painter().rect_filled(
                                egui::Rect::from_center_size(pattern_pos, egui::vec2(6.0, 6.0)),
                                0.0,
                                color
                            );
                        }
                    }
                }
            }
        }
    }

    fn draw_tool_overlays(&self, ui: &mut egui::Ui, canvas_rect: &egui::Rect, width: usize, height: usize, pixel_size: f32) {
        // Draw line preview
        if let Some((start_x, start_y)) = self.line_start {
            if let Some(hover_pos) = ui.input(|i| i.pointer.hover_pos()) {
                if canvas_rect.contains(hover_pos) {
                    let rel_pos = hover_pos - canvas_rect.min;
                    let hover_x = (rel_pos.x / pixel_size) as usize;
                    let hover_y = (rel_pos.y / pixel_size) as usize;
                    
                    if hover_x < width && hover_y < height {
                        // Draw preview line
                        let start_pos = canvas_rect.min + egui::vec2(start_x as f32 * pixel_size, start_y as f32 * pixel_size);
                        let end_pos = canvas_rect.min + egui::vec2(hover_x as f32 * pixel_size, hover_y as f32 * pixel_size);
                        ui.painter().line_segment([start_pos, end_pos], egui::Stroke::new(2.0, egui::Color32::WHITE));
                    }
                }
            }
        }

        // Draw rectangle preview
        if let Some((start_x, start_y)) = self.rectangle_start {
            if let Some(hover_pos) = ui.input(|i| i.pointer.hover_pos()) {
                if canvas_rect.contains(hover_pos) {
                    let rel_pos = hover_pos - canvas_rect.min;
                    let hover_x = (rel_pos.x / pixel_size) as usize;
                    let hover_y = (rel_pos.y / pixel_size) as usize;
                    
                    if hover_x < width && hover_y < height {
                        let min_x = start_x.min(hover_x);
                        let min_y = start_y.min(hover_y);
                        let max_x = start_x.max(hover_x);
                        let max_y = start_y.max(hover_y);
                        
                        let rect = egui::Rect::from_min_max(
                            canvas_rect.min + egui::vec2(min_x as f32 * pixel_size, min_y as f32 * pixel_size),
                            canvas_rect.min + egui::vec2((max_x + 1) as f32 * pixel_size, (max_y + 1) as f32 * pixel_size)
                        );
                        ui.painter().rect_stroke(rect, 0.0, egui::Stroke::new(2.0, egui::Color32::WHITE), egui::epaint::StrokeKind::Middle);
                    }
                }
            }
        }

        // Draw circle preview
        if let Some((start_x, start_y)) = self.circle_start {
            if let Some(hover_pos) = ui.input(|i| i.pointer.hover_pos()) {
                if canvas_rect.contains(hover_pos) {
                    let rel_pos = hover_pos - canvas_rect.min;
                    let hover_x = (rel_pos.x / pixel_size) as usize;
                    let hover_y = (rel_pos.y / pixel_size) as usize;
                    
                    if hover_x < width && hover_y < height {
                        let center = canvas_rect.min + egui::vec2(start_x as f32 * pixel_size + pixel_size/2.0, start_y as f32 * pixel_size + pixel_size/2.0);
                        let radius = ((hover_x as f32 - start_x as f32).powi(2) + (hover_y as f32 - start_y as f32).powi(2)).sqrt() * pixel_size;
                        ui.painter().circle_stroke(center, radius, egui::Stroke::new(2.0, egui::Color32::WHITE));
                    }
                }
            }
        }

        // Draw selection rectangle
        if let Some((min_x, min_y, max_x, max_y)) = self.selection_rect {
            let rect = egui::Rect::from_min_max(
                canvas_rect.min + egui::vec2(min_x as f32 * pixel_size, min_y as f32 * pixel_size),
                canvas_rect.min + egui::vec2((max_x + 1) as f32 * pixel_size, (max_y + 1) as f32 * pixel_size)
            );
            ui.painter().rect_stroke(rect, 0.0, egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 255, 255)), egui::epaint::StrokeKind::Middle);
            ui.painter().rect_stroke(rect, 0.0, egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 0, 0)), egui::epaint::StrokeKind::Middle);
        }

        // Draw lasso selection
        if let Some(ref points) = self.lasso_selection {
            if points.len() > 1 {
                let path_points: Vec<egui::Pos2> = points.iter()
                    .map(|(x, y)| canvas_rect.min + egui::vec2(*x as f32 * pixel_size, *y as f32 * pixel_size))
                    .collect();
                
                for i in 0..path_points.len() {
                    let start = path_points[i];
                    let end = path_points[(i + 1) % path_points.len()];
                    ui.painter().line_segment([start, end], egui::Stroke::new(1.0, egui::Color32::WHITE));
                }
            }
        }

        // Draw active lasso path
        if self.lasso_active && self.lasso_points.len() > 1 {
            let path_points: Vec<egui::Pos2> = self.lasso_points.iter()
                .map(|(x, y)| canvas_rect.min + egui::vec2(*x as f32 * pixel_size, *y as f32 * pixel_size))
                .collect();
            
            for i in 0..path_points.len() - 1 {
                let start = path_points[i];
                let end = path_points[i + 1];
                ui.painter().line_segment([start, end], egui::Stroke::new(1.0, egui::Color32::YELLOW));
            }
        }

        // Draw brush preview
        if matches!(self.tool, Tool::Pencil | Tool::Eraser) && self.brush_size > 1 {
            if let Some(hover_pos) = ui.input(|i| i.pointer.hover_pos()) {
                if canvas_rect.contains(hover_pos) {
                    let rel_pos = hover_pos - canvas_rect.min;
                    let hover_x = (rel_pos.x / pixel_size) as usize;
                    let hover_y = (rel_pos.y / pixel_size) as usize;
                    
                    if hover_x < width && hover_y < height {
                        let brush_radius = (self.brush_size / 2) as f32;
                        let center = canvas_rect.min + egui::vec2(
                            hover_x as f32 * pixel_size + pixel_size / 2.0,
                            hover_y as f32 * pixel_size + pixel_size / 2.0
                        );
                        
                        // Draw brush preview circle
                        ui.painter().circle_stroke(
                            center,
                            brush_radius * pixel_size,
                            egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(255, 255, 255, 128))
                        );
                    }
                }
            }
        }
    }

    fn handle_pixel_interaction(
        &mut self,
        ui: &mut egui::Ui,
        pixel_rect: egui::Rect,
        x: usize,
        y: usize,
        _width: usize,
        _height: usize,
        composed: &Vec<Vec<egui::Color32>>,
    ) {
        let interact_rect = ui.allocate_rect(pixel_rect, egui::Sense::click_and_drag());
        let pointer = ui.input(|i| i.pointer.clone());
        let alt = ui.input(|i| i.modifiers.alt);

        // Move tool drag logic
        if self.tool == Tool::Move {
            if interact_rect.drag_started() {
                self.move_drag_start = Some((x, y));
                self.move_layer_snapshot = Some(self.get_active_layer().grid.clone());
                self.move_last_offset = Some((0, 0));
                self.push_undo();
            }
            if let (Some((start_x, start_y)), Some(snapshot), Some((last_dx, last_dy))) = (
                self.move_drag_start,
                &self.move_layer_snapshot,
                self.move_last_offset,
            ) {
                if interact_rect.dragged() {
                    let dx = x as isize - start_x as isize;
                    let dy = y as isize - start_y as isize;
                    if dx != last_dx || dy != last_dy {
                        let shifted = Self::shift_layer_grid(snapshot, dx, dy);
                        self.get_active_layer_mut().grid = shifted;
                        self.move_last_offset = Some((dx, dy));
                    }
                }
                if interact_rect.drag_stopped() {
                    self.move_drag_start = None;
                    self.move_layer_snapshot = None;
                    self.move_last_offset = None;
                }
            }
        }

        // Handle clicks
        if interact_rect.clicked() {
            match self.tool {
                Tool::Pencil => {
                    self.push_undo();
                    let selected_color = self.selected_color;
                    self.use_tool_with_animation(Tool::Pencil, x, y, selected_color);
                }
                Tool::Eraser => {
                    self.push_undo();
                    self.use_tool_with_animation(Tool::Eraser, x, y, eframe::egui::Color32::TRANSPARENT);
                }
                Tool::Bucket => {
                    self.push_undo();
                    let selected_color = self.selected_color;
                    self.use_tool_with_animation(Tool::Bucket, x, y, selected_color);
                }
                Tool::Spray => {
                    self.push_undo();
                    let selected_color = self.selected_color;
                    self.use_tool_with_animation(Tool::Spray, x, y, selected_color);
                }
                Tool::Dither => {
                    self.push_undo();
                    let selected_color = self.selected_color;
                    self.use_tool_with_animation(Tool::Dither, x, y, selected_color);
                }
                Tool::Eyedropper => {
                    self.selected_color = composed[y][x];
                }
                Tool::Line => {
                    if self.line_start.is_none() {
                        self.line_start = Some((x, y));
                    } else {
                        self.push_undo();
                        if let Some((start_x, start_y)) = self.line_start {
                            // Simple line drawing using Bresenham's algorithm
                            let selected_color = self.selected_color;
                            let layer = self.get_active_layer_mut();
                            let mut x0 = start_x as i32;
                            let mut y0 = start_y as i32;
                            let x1 = x as i32;
                            let y1 = y as i32;
                            
                            let dx = (x1 - x0).abs();
                            let dy = (y1 - y0).abs();
                            let sx = if x0 < x1 { 1 } else { -1 };
                            let sy = if y0 < y1 { 1 } else { -1 };
                            let mut err = dx - dy;
                            
                            loop {
                                if x0 >= 0 && x0 < layer.width() as i32 && y0 >= 0 && y0 < layer.height() as i32 {
                                    layer.grid[y0 as usize][x0 as usize] = selected_color;
                                }
                                if x0 == x1 && y0 == y1 {
                                    break;
                                }
                                let e2 = 2 * err;
                                if e2 > -dy {
                                    err -= dy;
                                    x0 += sx;
                                }
                                if e2 < dx {
                                    err += dx;
                                    y0 += sy;
                                }
                            }
                        }
                        self.line_start = None;
                    }
                }
                Tool::Rectangle => {
                    if self.rectangle_start.is_none() {
                        self.rectangle_start = Some((x, y));
                    } else {
                        self.push_undo();
                        if let Some((start_x, start_y)) = self.rectangle_start {
                            let selected_color = self.selected_color;
                            let fill_shape = self.fill_shape;
                            let layer = self.get_active_layer_mut();
                            let min_x = start_x.min(x);
                            let max_x = start_x.max(x);
                            let min_y = start_y.min(y);
                            let max_y = start_y.max(y);
                            
                            if fill_shape {
                                // Fill rectangle
                                for py in min_y..=max_y {
                                    for px in min_x..=max_x {
                                        if px < layer.width() && py < layer.height() {
                                            layer.grid[py][px] = selected_color;
                                        }
                                    }
                                }
                            } else {
                                // Draw rectangle outline
                                for px in min_x..=max_x {
                                    if px < layer.width() && min_y < layer.height() {
                                        layer.grid[min_y][px] = selected_color;
                                    }
                                    if px < layer.width() && max_y < layer.height() {
                                        layer.grid[max_y][px] = selected_color;
                                    }
                                }
                                for py in min_y..=max_y {
                                    if min_x < layer.width() && py < layer.height() {
                                        layer.grid[py][min_x] = selected_color;
                                    }
                                    if max_x < layer.width() && py < layer.height() {
                                        layer.grid[py][max_x] = selected_color;
                                    }
                                }
                            }
                        }
                        self.rectangle_start = None;
                    }
                }
                Tool::Circle => {
                    if self.circle_start.is_none() {
                        self.circle_start = Some((x, y));
                    } else {
                        self.push_undo();
                        if let Some((start_x, start_y)) = self.circle_start {
                            let selected_color = self.selected_color;
                            let layer = self.get_active_layer_mut();
                            let radius = ((x as f32 - start_x as f32).powi(2) + (y as f32 - start_y as f32).powi(2)).sqrt() as i32;
                            
                            // Simple circle drawing using midpoint algorithm
                            let mut cx = 0;
                            let mut cy = radius;
                            let mut d = 1 - radius;
                            
                            while cx <= cy {
                                // Plot 8 octants
                                let points = [
                                    (start_x as i32 + cx, start_y as i32 + cy),
                                    (start_x as i32 - cx, start_y as i32 + cy),
                                    (start_x as i32 + cx, start_y as i32 - cy),
                                    (start_x as i32 - cx, start_y as i32 - cy),
                                    (start_x as i32 + cy, start_y as i32 + cx),
                                    (start_x as i32 - cy, start_y as i32 + cx),
                                    (start_x as i32 + cy, start_y as i32 - cx),
                                    (start_x as i32 - cy, start_y as i32 - cx),
                                ];
                                
                                for (px, py) in points {
                                    if px >= 0 && px < layer.width() as i32 && py >= 0 && py < layer.height() as i32 {
                                        layer.grid[py as usize][px as usize] = selected_color;
                                    }
                                }
                                
                                if d < 0 {
                                    d += 2 * cx + 3;
                                } else {
                                    d += 2 * (cx - cy) + 5;
                                    cy -= 1;
                                }
                                cx += 1;
                            }
                        }
                        self.circle_start = None;
                    }
                }
                Tool::Select => {
                    if self.selection_start.is_none() {
                        self.selection_start = Some((x, y));
                    } else {
                        if let Some((start_x, start_y)) = self.selection_start {
                            self.selection_rect = Some((
                                start_x.min(x),
                                start_y.min(y),
                                start_x.max(x),
                                start_y.max(y),
                            ));
                        }
                        self.selection_start = None;
                    }
                }
                Tool::Lasso => {
                    if !self.lasso_active {
                        self.lasso_points.clear();
                        self.lasso_active = true;
                    }
                    self.lasso_points.push((x, y));
                }
                _ => {}
            }
        } else if interact_rect.secondary_clicked() {
            self.push_undo();
            self.erase_brush(x, y);
        }

        // Handle dragging
        if interact_rect.hovered() && pointer.primary_down() && !alt {
            match self.tool {
                Tool::Pencil => {
                    let selected_color = self.selected_color;
                    self.paint_brush(x, y, selected_color);
                }
                Tool::Eraser => {
                    self.erase_brush(x, y);
                }
                Tool::Spray => {
                    // Simple spray paint implementation
                    let size = self.spray_size;
                    let selected_color = self.selected_color;
                    let layer = self.get_active_layer_mut();
                    for _ in 0..size {
                        let offset_x = (rand::random::<f32>() - 0.5) * size as f32;
                        let offset_y = (rand::random::<f32>() - 0.5) * size as f32;
                        let spray_x = (x as f32 + offset_x) as i32;
                        let spray_y = (y as f32 + offset_y) as i32;
                        
                        if spray_x >= 0 && spray_x < layer.width() as i32 && spray_y >= 0 && spray_y < layer.height() as i32 {
                            if rand::random::<f32>() < 0.3 {
                                layer.grid[spray_y as usize][spray_x as usize] = selected_color;
                            }
                        }
                    }
                }
                Tool::Dither => {
                    self.apply_dither(x, y, self.selected_color);
                }
                Tool::Lasso => {
                    if self.lasso_active {
                        self.lasso_points.push((x, y));
                    }
                }
                _ => {}
            }
        }

        // Handle right-click erasing on drag
        if interact_rect.hovered() && pointer.secondary_down() {
            let layer = self.get_active_layer_mut();
            if x < layer.width() && y < layer.height() {
                layer.grid[y][x] = egui::Color32::TRANSPARENT;
            }
        }

        // Handle Alt+Click for Eyedropper
        if interact_rect.clicked() && alt {
            self.selected_color = composed[y][x];
        }

        // Handle mouse release for lasso tool
        if interact_rect.drag_stopped() && self.tool == Tool::Lasso && self.lasso_active {
            if !self.lasso_points.is_empty() {
                // Close the lasso path and create selection
                self.lasso_selection = Some(self.lasso_points.clone());
                self.lasso_active = false;
            }
        }
    }
}
