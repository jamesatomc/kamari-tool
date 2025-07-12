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
                            .range(1..=4096)
                            .prefix("W: "),
                    );
                    ui.add(
                        egui::DragValue::new(&mut self.new_sprite_height)
                            .range(1..=4096)
                            .prefix("H: "),
                    );
                });

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
                            .range(1..=4096)
                            .prefix("W: "),
                    );
                    ui.add(
                        egui::DragValue::new(&mut self.resize_height)
                            .range(1..=4096)
                            .prefix("H: "),
                    );
                });

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
