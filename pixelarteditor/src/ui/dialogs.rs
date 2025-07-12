use eframe::egui;
use crate::editor::PixelArtEditor;
use crate::types::Layer;

impl PixelArtEditor {
    pub fn show_new_sprite_dialog(&mut self, ctx: &egui::Context) {
        egui::Window::new("🆕 New Sprite")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Canvas Size:");
                    ui.add(
                        egui::DragValue::new(&mut self.new_sprite_width)
                            .range(1..=1024)
                            .prefix("W: "),
                    );
                    ui.add(
                        egui::DragValue::new(&mut self.new_sprite_height)
                            .range(1..=1024)
                            .prefix("H: "),
                    );
                });

                // Warning for large canvases
                if self.new_sprite_width > 200 || self.new_sprite_height > 200 {
                    ui.colored_label(
                        egui::Color32::from_rgb(255, 165, 0),
                        format!("⚠️ Large canvas ({}x{}) may affect performance", 
                               self.new_sprite_width, self.new_sprite_height)
                    );
                }

                if self.new_sprite_width > 500 || self.new_sprite_height > 500 {
                    ui.colored_label(
                        egui::Color32::from_rgb(255, 80, 80),
                        "❌ Very large canvas - may cause lag"
                    );
                }

                ui.horizontal(|ui| {
                    ui.label("Background:");
                    egui::color_picker::color_edit_button_srgba(
                        ui,
                        &mut self.new_sprite_bg,
                        egui::color_picker::Alpha::BlendOrAdditive,
                    );
                });

                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("✅ Create").clicked() {
                        let w = self.new_sprite_width;
                        let h = self.new_sprite_height;
                        let bg = self.new_sprite_bg;

                        let layer = Layer {
                            name: "Background".to_string(),
                            visible: true,
                            opacity: 1.0,
                            grid: vec![vec![bg; w]; h],
                        };

                        self.frames = vec![crate::types::Frame {
                            layers: vec![layer],
                        }];
                        self.current_frame = 0;
                        self.current_layer = 0;
                        self.show_new_sprite_dialog = false;
                    }
                    if ui.button("❌ Cancel").clicked() {
                        self.show_new_sprite_dialog = false;
                    }
                });
            });
    }

    pub fn show_resize_dialog(&mut self, ctx: &egui::Context) {
        egui::Window::new("📐 Resize Canvas")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("New Size:");
                    ui.add(
                        egui::DragValue::new(&mut self.resize_width)
                            .range(1..=1024)
                            .prefix("W: "),
                    );
                    ui.add(
                        egui::DragValue::new(&mut self.resize_height)
                            .range(1..=1024)
                            .prefix("H: "),
                    );
                });

                // Warning for large canvases
                if self.resize_width > 200 || self.resize_height > 200 {
                    ui.colored_label(
                        egui::Color32::from_rgb(255, 165, 0),
                        format!("⚠️ Large canvas ({}x{}) may affect performance", 
                               self.resize_width, self.resize_height)
                    );
                }

                if self.resize_width > 500 || self.resize_height > 500 {
                    ui.colored_label(
                        egui::Color32::from_rgb(255, 80, 80),
                        "❌ Very large canvas - may cause lag"
                    );
                }

                ui.horizontal(|ui| {
                    ui.label("Anchor:");
                    ui.radio_value(&mut self.resize_anchor, 0, "Top-Left");
                    ui.radio_value(&mut self.resize_anchor, 1, "Center");
                    ui.radio_value(&mut self.resize_anchor, 2, "Bottom-Right");
                });

                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("✅ Resize").clicked() {
                        self.resize_canvas(self.resize_width, self.resize_height, self.resize_anchor);
                        self.show_resize_dialog = false;
                    }
                    if ui.button("❌ Cancel").clicked() {
                        self.show_resize_dialog = false;
                    }
                });
            });
    }
}
