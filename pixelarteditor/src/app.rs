use eframe::egui;
use crate::editor::PixelArtEditor;

impl eframe::App for PixelArtEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle animation
        if self.animation_playing && self.frames.len() > 1 {
            let current_time = ctx.input(|i| i.time);
            if current_time - self.last_animation_time > 1.0 / self.animation_speed as f64 {
                self.animation_frame = (self.animation_frame + 1) % self.frames.len();
                self.last_animation_time = current_time;
                ctx.request_repaint();
            }
        }

        // Menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            self.show_menu_bar(ctx, ui);
        });

        // Toolbar
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            self.show_toolbar(ctx, ui);
        });

        // Show dialogs
        if self.show_new_sprite_dialog {
            self.show_new_sprite_dialog(ctx);
        }

        if self.show_resize_dialog {
            self.show_resize_dialog(ctx);
        }

        if self.show_export_dialog {
            self.show_export_dialog(ctx);
        }

        // Show plugin dialogs
        self.plugin_manager.show_plugin_manager(ctx);
        self.plugin_manager.show_plugin_config(ctx);

        // Color Panel
        if self.show_color_panel {
            egui::SidePanel::left("color_panel")
                .resizable(true)
                .default_width(180.0)
                .show(ctx, |ui| {
                    self.show_color_panel(ui);
                });
        }

        // Central canvas
        egui::CentralPanel::default().show(ctx, |ui| {
            self.show_canvas(ui);
        });

        // Combined Layers and Frames Panel
        if self.show_layers_panel || self.show_frames_panel {
            egui::SidePanel::right("right_panel")
                .resizable(true)
                .default_width(200.0)
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        if self.show_layers_panel {
                            self.show_layers_panel(ctx, ui);
                            ui.separator();
                        }

                        if self.show_frames_panel {
                            self.show_frames_panel(ctx, ui);
                        }
                    });
                });
        }

        // Handle keyboard shortcuts
        if ctx.input(|i| i.modifiers.ctrl && i.modifiers.shift && i.key_pressed(egui::Key::Z)) {
            self.redo();
        } else if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::Z)) {
            self.undo();
        }
    }
}
