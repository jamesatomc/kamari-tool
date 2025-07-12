use eframe::egui;
use crate::editor::PixelArtEditor;
use crate::types::Tool;
use crate::constants::PIXEL_SIZE;

impl PixelArtEditor {
    pub fn show_canvas(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::both()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                let composed = self.get_composed_grid();
                let height = composed.len();
                let width = if height > 0 { composed[0].len() } else { 0 };

                let pixel_size = PIXEL_SIZE * self.zoom;
                let canvas_size = egui::vec2(width as f32 * pixel_size, height as f32 * pixel_size);

                let (canvas_rect, _) = ui.allocate_exact_size(canvas_size, egui::Sense::hover());

                // Draw the canvas background and pixels
                for y in 0..height {
                    for x in 0..width {
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
                        
                        impl PixelArtEditor {
                            /// Applies a simple 2x2 Bayer dither pattern at the given pixel location.
                            pub fn apply_dither(&mut self, x: usize, y: usize, color: egui::Color32) {
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
                        }

                        // Draw grid lines if enabled
                        if self.show_grid {
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
            });
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
                    let layer = self.get_active_layer_mut();
                    if x < layer.width() && y < layer.height() {
                        layer.grid[y][x] = selected_color;
                    }
                }
                Tool::Eraser => {
                    self.push_undo();
                    let layer = self.get_active_layer_mut();
                    if x < layer.width() && y < layer.height() {
                        layer.grid[y][x] = egui::Color32::TRANSPARENT;
                    }
                }
                Tool::Bucket => {
                    self.push_undo();
                    // Simple flood fill implementation
                    let original_color = composed[y][x];
                    let new_color = self.selected_color;
                    if original_color != new_color {
                        let layer = self.get_active_layer_mut();
                        let width = layer.width();
                        let height = layer.height();
                        if x < width && y < height {
                            let mut stack = vec![(x, y)];
                            while let Some((cx, cy)) = stack.pop() {
                                if cx >= width || cy >= height || layer.grid[cy][cx] != original_color {
                                    continue;
                                }
                                layer.grid[cy][cx] = new_color;
                                if cx > 0 {
                                    stack.push((cx - 1, cy));
                                }
                                if cx < width - 1 {
                                    stack.push((cx + 1, cy));
                                }
                                if cy > 0 {
                                    stack.push((cx, cy - 1));
                                }
                                if cy < height - 1 {
                                    stack.push((cx, cy + 1));
                                }
                            }
                        }
                    }
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
            let layer = self.get_active_layer_mut();
            if x < layer.width() && y < layer.height() {
                layer.grid[y][x] = egui::Color32::TRANSPARENT;
            }
        }

        // Handle dragging
        if interact_rect.hovered() && pointer.primary_down() && !alt {
            match self.tool {
                Tool::Pencil => {
                    let selected_color = self.selected_color;
                    let layer = self.get_active_layer_mut();
                    if x < layer.width() && y < layer.height() {
                        layer.grid[y][x] = selected_color;
                    }
                }
                Tool::Eraser => {
                    let layer = self.get_active_layer_mut();
                    if x < layer.width() && y < layer.height() {
                        layer.grid[y][x] = egui::Color32::TRANSPARENT;
                    }
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
