use eframe::egui;
use crate::editor::PixelArtEditor;
use crate::types::Tool;
use crate::constants::PIXEL_SIZE;
use crate::tools::*;

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
            });
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
            // Simple tool handling for now
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
                Tool::Eyedropper => {
                    self.selected_color = composed[y][x];
                }
                _ => {
                    // For advanced tools, we'll implement later
                }
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
            if matches!(self.tool, Tool::Pencil | Tool::Eraser) {
                let selected_color = self.selected_color;
                let current_tool = self.tool;
                let layer = self.get_active_layer_mut();
                if x < layer.width() && y < layer.height() {
                    match current_tool {
                        Tool::Pencil => {
                            layer.grid[y][x] = selected_color;
                        }
                        Tool::Eraser => {
                            layer.grid[y][x] = egui::Color32::TRANSPARENT;
                        }
                        _ => {}
                    }
                }
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
    }
}
