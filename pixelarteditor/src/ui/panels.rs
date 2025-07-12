use eframe::egui;
use crate::editor::PixelArtEditor;
use crate::types::{Layer, Frame};

impl PixelArtEditor {
    pub fn show_layers_panel(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.set_min_width(180.0);
        ui.heading("Layers");
        ui.separator();

        let mut layer_to_remove = None;
        let mut add_layer = false;
        let mut layer_to_duplicate = None;
        let mut layer_to_clear = None;
        let mut should_move_up = false;
        let mut should_move_down = false;

        let current_frame = self.current_frame;
        let frame = &mut self.frames[current_frame];
        let layers_len = frame.layers.len();
        let current_layer = self.current_layer;

        egui::ScrollArea::vertical()
            .id_salt("layers_panel_scroll")
            .max_height(150.0)
            .show(ui, |ui| {
                for (i, layer) in frame.layers.iter_mut().enumerate().rev() {
                    let is_current = current_layer == i;
                    let is_renaming = self.renaming_layer == Some(i);

                    egui::Frame::group(ui.style())
                        .fill(if is_current {
                            ui.visuals().selection.bg_fill
                        } else {
                            ui.visuals().extreme_bg_color
                        })
                        .corner_radius(4.0)
                        .inner_margin(8.0)
                        .outer_margin(2.0)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                if ui.checkbox(&mut layer.visible, "")
                                    .on_hover_text("Toggle visibility")
                                    .changed() { 
                                    ctx.request_repaint(); 
                                }

                                if is_renaming {
                                    let text_edit = ui.text_edit_singleline(&mut self.rename_text);
                                    if text_edit.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                        if !self.rename_text.is_empty() {
                                            layer.name = self.rename_text.clone();
                                        }
                                        self.renaming_layer = None;
                                        self.rename_text.clear();
                                    }
                                    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                                        self.renaming_layer = None;
                                        self.rename_text.clear();
                                    }
                                    // Auto-focus when starting to rename
                                    if text_edit.gained_focus() {
                                        text_edit.request_focus();
                                    }
                                } else {
                                    if ui.selectable_label(is_current, &layer.name).clicked() { 
                                        self.current_layer = i; 
                                    }
                                }

                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let btn_size = egui::vec2(20.0, 18.0);
                                    if ui.add(egui::Button::new("Clear").min_size(btn_size)).on_hover_text("Clear Layer").clicked() { 
                                        layer_to_clear = Some(i);
                                    }
                                    if ui.add(egui::Button::new("Copy").min_size(btn_size)).on_hover_text("Duplicate").clicked() { 
                                        layer_to_duplicate = Some(i);
                                    }
                                    if layers_len > 1 && ui.add(egui::Button::new("Del").min_size(btn_size)).on_hover_text("Delete").clicked() { 
                                        layer_to_remove = Some(i);
                                    }
                                    if !is_renaming && ui.add(egui::Button::new("Edit").min_size(btn_size)).on_hover_text("Rename").clicked() {
                                        self.renaming_layer = Some(i);
                                        self.rename_text = layer.name.clone();
                                    }
                                });
                            });

                            ui.horizontal(|ui| {
                                ui.label("Opacity:");
                                if ui.add(egui::Slider::new(&mut layer.opacity, 0.0..=1.0).show_value(true).text("")).changed() { 
                                    ctx.request_repaint(); 
                                }
                            });
                        });

                    ui.add_space(2.0);
                }
            });

        ui.separator();        
        ui.horizontal(|ui| {
            let canvas_width = if !self.frames[self.current_frame].layers.is_empty() { 
                self.frames[self.current_frame].layers[0].width() 
            } else { 0 };
            let canvas_height = if !self.frames[self.current_frame].layers.is_empty() { 
                self.frames[self.current_frame].layers[0].height() 
            } else { 0 };
            let max_layers = crate::constants::get_max_layers_for_size(canvas_width, canvas_height);
            let current_layer_count = self.frames[self.current_frame].layers.len();
            
            if current_layer_count < max_layers && ui.button("+ Add Layer").clicked() { 
                add_layer = true;
            }
            if current_layer_count >= max_layers {
                ui.colored_label(egui::Color32::from_rgb(255, 165, 0), 
                    format!("Max layers ({}) reached for {}x{} canvas", max_layers, canvas_width, canvas_height));
            }
            if ui.button("▲").on_hover_text("Move Layer Up").clicked() && self.current_layer < current_layer_count - 1 { 
                should_move_up = true;
            }
            if ui.button("▼").on_hover_text("Move Layer Down").clicked() && self.current_layer > 0 { 
                should_move_down = true;
            }
        });

        // Handle layer operations
        if layer_to_remove.is_some() || add_layer || layer_to_duplicate.is_some() || layer_to_clear.is_some() { 
            self.push_undo(); 
        }

        let frame = &mut self.frames[self.current_frame];
        
        if let Some(i) = layer_to_remove { 
            frame.layers.remove(i); 
            if self.current_layer >= frame.layers.len() { 
                self.current_layer = frame.layers.len().saturating_sub(1); 
            } 
        }
        
        if let Some(i) = layer_to_duplicate { 
            let mut duplicated_layer = frame.layers[i].clone(); 
            duplicated_layer.name = format!("{} Copy", duplicated_layer.name); 
            frame.layers.insert(i + 1, duplicated_layer); 
            self.current_layer = i + 1; 
        }
        
        if let Some(i) = layer_to_clear { 
            let w = frame.layers[i].width(); 
            let h = frame.layers[i].height(); 
            frame.layers[i].grid = vec![vec![egui::Color32::TRANSPARENT; w]; h]; 
        }
        
        if add_layer { 
            let layer = Layer { 
                name: format!("Layer {}", frame.layers.len() + 1), 
                visible: true, 
                opacity: 1.0, 
                grid: { 
                    let w = frame.layers[0].width(); 
                    let h = frame.layers[0].height(); 
                    vec![vec![egui::Color32::TRANSPARENT; w]; h] 
                }, 
            }; 
            frame.layers.push(layer); 
            self.current_layer = frame.layers.len() - 1; 
        }

        if should_move_up { 
            self.push_undo(); 
            let frame = &mut self.frames[self.current_frame]; 
            frame.layers.swap(self.current_layer, self.current_layer + 1); 
            self.current_layer += 1; 
        }
        
        if should_move_down { 
            self.push_undo(); 
            let frame = &mut self.frames[self.current_frame]; 
            frame.layers.swap(self.current_layer, self.current_layer - 1); 
            self.current_layer -= 1; 
        }
    }

    pub fn show_frames_panel(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.set_min_width(180.0);
        ui.heading("Frames");
        ui.separator();

        let frame_infos: Vec<(usize, bool)> = (0..self.frames.len())
            .map(|i| (i, self.animation_playing && self.animation_frame == i))
            .collect();

        egui::ScrollArea::horizontal()
            .max_height(150.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    for (i, is_anim_frame) in frame_infos {
                        let is_current = self.current_frame == i;

                        egui::Frame::group(ui.style())
                            .fill(if is_current {
                                ui.visuals().selection.bg_fill
                            } else if is_anim_frame {
                                ui.visuals().faint_bg_color
                            } else {
                                ui.visuals().extreme_bg_color
                            })
                            .corner_radius(4.0)
                            .inner_margin(8.0)
                            .outer_margin(2.0)
                            .show(ui, |ui| {
                                let preview_size = 80.0;
                                {
                                    let frame = &self.frames[i];
                                    let width = frame.layers[0].width();
                                    let height = frame.layers[0].height();
                                    let scale = (preview_size / width as f32).min(preview_size / height as f32);

                                    ui.vertical(|ui| {
                                        ui.horizontal(|ui| {
                                            let frame_display = format!("Frame {}", i + 1);
                                            if ui.selectable_label(is_current, frame_display).clicked() { 
                                                self.current_frame = i; 
                                                self.current_layer = 0; 
                                            }
                                            if is_anim_frame { 
                                                ui.label("▶"); 
                                            }
                                        });

                                        let (rect, _) = ui.allocate_exact_size(
                                            egui::vec2(width as f32 * scale, height as f32 * scale),
                                            egui::Sense::hover(),
                                        );

                                        let mut composed = vec![vec![egui::Color32::TRANSPARENT; width]; height];
                                        for layer in &frame.layers { 
                                            if !layer.visible { continue; } 
                                            for y in 0..height { 
                                                for x in 0..width { 
                                                    let c = layer.grid[y][x]; 
                                                    if c.a() > 0 { 
                                                        composed[y][x] = c; 
                                                    } 
                                                } 
                                            } 
                                        }

                                        for y in 0..height { 
                                            for x in 0..width { 
                                                let pixel_color = composed[y][x]; 
                                                if pixel_color.a() > 0 { 
                                                    let pixel_rect = egui::Rect::from_min_size( 
                                                        rect.min + egui::vec2(x as f32 * scale, y as f32 * scale), 
                                                        egui::vec2(scale, scale), 
                                                    ); 
                                                    ui.painter().rect_filled(pixel_rect, 0.0, pixel_color); 
                                                } 
                                            } 
                                        }

                                        // Draw a simple frame around the preview
                                        ui.painter().rect_filled(
                                            egui::Rect::from_min_size(
                                                rect.min,
                                                egui::vec2(rect.width(), 1.0),
                                            ),
                                            0.0,
                                            ui.visuals().widgets.active.bg_stroke.color,
                                        );
                                    });
                                }

                                ui.horizontal(|ui| {
                                    let btn_size = egui::vec2(20.0, 18.0);
                                    if self.frames.len() > 1 && ui.add(egui::Button::new("Del").min_size(btn_size)).on_hover_text("Delete Frame").clicked() { 
                                        self.push_undo(); 
                                        self.frames.remove(i); 
                                        if self.current_frame >= self.frames.len() { 
                                            self.current_frame = self.frames.len() - 1; 
                                        } 
                                        self.current_layer = 0; 
                                    }
                                    if ui.add(egui::Button::new("Copy").min_size(btn_size)).on_hover_text("Duplicate Frame").clicked() { 
                                        self.push_undo(); 
                                        let new_frame = self.frames[i].clone(); 
                                        self.frames.insert(i + 1, new_frame); 
                                        self.current_frame = i + 1; 
                                        self.current_layer = 0; 
                                    }
                                });
                            });

                        ui.add_space(4.0);
                    }
                });
            });

        ui.separator();
        ui.horizontal(|ui| {
            let canvas_width = if !self.frames[self.current_frame].layers.is_empty() { 
                self.frames[self.current_frame].layers[0].width() 
            } else { 0 };
            let canvas_height = if !self.frames[self.current_frame].layers.is_empty() { 
                self.frames[self.current_frame].layers[0].height() 
            } else { 0 };
            let max_frames = crate::constants::get_max_frames_for_size(canvas_width, canvas_height);
            let current_frame_count = self.frames.len();
            
            if current_frame_count < max_frames && ui.button("+ Add Frame").clicked() { 
                self.push_undo(); 
                if self.frames.is_empty() { 
                    self.frames.push(Frame::default()); 
                } else { 
                    let new_frame = self.frames[self.current_frame].clone(); 
                    self.frames.insert(self.current_frame + 1, new_frame); 
                    self.current_frame += 1; 
                } 
                self.current_layer = 0; 
            }
            if current_frame_count >= max_frames {
                ui.colored_label(egui::Color32::from_rgb(255, 165, 0), 
                    format!("Max frames ({}) reached for {}x{} canvas", max_frames, canvas_width, canvas_height));
            }
            if ui.button("⏮").on_hover_text("First Frame").clicked() && !self.frames.is_empty() { 
                self.current_frame = 0; 
                self.current_layer = 0; 
            }
            if ui.button("⏭").on_hover_text("Last Frame").clicked() && !self.frames.is_empty() { 
                self.current_frame = self.frames.len() - 1; 
                self.current_layer = 0; 
            }

            if self.frames.len() > 1 {
                ui.separator();
                self.show_animation_controls(ctx, ui);
            }
        });
    }
}
