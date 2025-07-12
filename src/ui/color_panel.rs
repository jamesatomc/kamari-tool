use eframe::egui;
use crate::editor::PixelArtEditor;

impl PixelArtEditor {
    pub fn show_color_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("🎨 Colors");
        ui.separator();

        egui::Frame::group(ui.style())
            .fill(ui.style().visuals.extreme_bg_color)
            .corner_radius(5.0)
            .inner_margin(8.0)
            .outer_margin(5.0)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label("Primary Color:");
                    ui.horizontal(|ui| {
                        let color_preview_size = 30.0;
                        let (rect, _) = ui.allocate_exact_size(
                            egui::vec2(color_preview_size, color_preview_size),
                            egui::Sense::hover(),
                        );
                        ui.painter().rect_filled(rect, 4.0, self.selected_color);

                        egui::color_picker::color_edit_button_srgba(
                            ui,
                            &mut self.selected_color,
                            egui::color_picker::Alpha::BlendOrAdditive,
                        );
                    });

                    ui.label("Secondary Color:");
                    ui.horizontal(|ui| {
                        let color_preview_size = 30.0;
                        let (rect, _) = ui.allocate_exact_size(
                            egui::vec2(color_preview_size, color_preview_size),
                            egui::Sense::hover(),
                        );
                        ui.painter().rect_filled(rect, 4.0, self.secondary_color);

                        egui::color_picker::color_edit_button_srgba(
                            ui,
                            &mut self.secondary_color,
                            egui::color_picker::Alpha::BlendOrAdditive,
                        );
                    });

                    if ui.button("🔄 Swap Colors").clicked() {
                        std::mem::swap(&mut self.selected_color, &mut self.secondary_color);
                    }
                });
            });

        ui.add_space(10.0);
        ui.label("Color Palette:");

        let palette_size = 30.0;
        let columns = 4;

        egui::ScrollArea::vertical()
            .id_salt("color_palette_scroll")
            .show(ui, |ui| {
                let mut row = Vec::new();

                for (i, color) in self.color_palette.iter().enumerate() {
                    row.push(*color);

                    if (i + 1) % columns == 0 || i == self.color_palette.len() - 1 {
                        ui.horizontal(|ui| {
                            for color in &row {
                                let (rect, response) = ui.allocate_exact_size(
                                    egui::vec2(palette_size, palette_size),
                                    egui::Sense::click(),
                                );

                                if response.clicked() {
                                    self.selected_color = *color;
                                } else if response.secondary_clicked() {
                                    self.secondary_color = *color;
                                }

                                ui.painter().rect_filled(rect, 4.0, *color);

                                if *color == self.selected_color {
                                    ui.painter().rect_filled(
                                        rect.shrink(2.0),
                                        4.0,
                                        egui::Color32::WHITE,
                                    );
                                } else if *color == self.secondary_color {
                                    ui.painter().rect_filled(
                                        rect.shrink(2.0),
                                        4.0,
                                        egui::Color32::LIGHT_GRAY,
                                    );
                                }
                            }
                        });
                        row.clear();
                    }
                }
            });

        ui.separator();
        
        // Palette selection
        ui.horizontal(|ui| {
            ui.label("Palette:");
            egui::ComboBox::from_id_salt("palette_selector")
                .selected_text(&self.palette_names[self.active_palette])
                .show_ui(ui, |ui| {
                    for (i, name) in self.palette_names.iter().enumerate() {
                        ui.selectable_value(&mut self.active_palette, i, name);
                    }
                });
        });

        // Show active palette
        let palette = &self.custom_palettes[self.active_palette];
        ui.label("Palette Colors:");
        let colors_per_row = 8;
        let color_size = 20.0; // Smaller size for better performance
        
        // Use scrollable area for large palettes
        egui::ScrollArea::vertical()
            .max_height(200.0)
            .show(ui, |ui| {
                for (i, _color) in palette.iter().enumerate() {
                    if i % colors_per_row == 0 {
                        ui.horizontal(|ui| {
                            for j in 0..colors_per_row {
                                let idx = i + j;
                                if idx < palette.len() {
                                    let color = palette[idx];
                                    let (rect, response) = ui.allocate_exact_size(
                                        egui::vec2(color_size, color_size),
                                        egui::Sense::click(),
                                    );
                                    
                                    ui.painter().rect_filled(rect, 2.0, color);
                                    
                                    // Only draw stroke for selected colors to improve performance
                                    if self.selected_color == color || self.secondary_color == color {
                                        let stroke = egui::Stroke::new(1.0, 
                                            if self.selected_color == color {
                                                egui::Color32::WHITE
                                            } else {
                                                egui::Color32::GRAY
                                            }
                                        );
                                        ui.painter().rect_stroke(rect, 2.0, stroke, egui::epaint::StrokeKind::Outside);
                                    }
                                    
                                    if response.clicked() {
                                        self.selected_color = color;
                                    }
                                    if response.secondary_clicked() {
                                        self.secondary_color = color;
                                    }
                                }
                            }
                        });
                    }
                }
            });
        
        if ui.button("+ Add Current Color").clicked() {
            let palette = &mut self.custom_palettes[self.active_palette];
            if !palette.contains(&self.selected_color) {
                palette.push(self.selected_color);
            }
        }
    }
}
