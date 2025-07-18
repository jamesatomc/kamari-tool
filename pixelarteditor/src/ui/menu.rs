use eframe::egui;
use crate::editor::PixelArtEditor;
use crate::types::Tool;

impl PixelArtEditor {
    pub fn show_menu_bar(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui
                    .button("New Sprite")
                    .on_hover_text("Create new sprite")
                    .clicked()
                {
                    self.show_new_sprite_dialog = true;
                    ui.close_menu();
                }
                ui.separator();
                if ui
                    .button("Save As...")
                    .on_hover_text("Save current frame as single file")
                    .clicked()
                {
                    self.save_as_png_dialog();
                    ui.close_menu();
                }
                if ui
                    .button("Quick Save")
                    .on_hover_text("Quick save current frame as PNG")
                    .clicked()
                {
                    if let Err(e) = self.quick_save() {
                        eprintln!("Quick save failed: {}", e);
                    }
                    ui.close_menu();
                }
                if ui
                    .button("Save All...")
                    .on_hover_text("Save all frames and layers")
                    .clicked()
                {
                    self.save_all_dialog();
                    ui.close_menu();
                }
                if ui
                    .button("Export...")
                    .on_hover_text("Export with options")
                    .clicked()
                {
                    self.show_export_dialog = true;
                    ui.close_menu();
                }
                ui.separator();
                if ui
                    .button("Save Project...")
                    .on_hover_text("Save project file (.json)")
                    .clicked()
                {
                    if let Some(path) = rfd::FileDialog::new()
                        .set_title("Save Project")
                        .add_filter("JSON", &["json"])
                        .save_file() {
                        if let Err(e) = self.save_project_file(&path.to_string_lossy()) {
                            eprintln!("Failed to save project: {}", e);
                        }
                    }
                    ui.close_menu();
                }
                if ui
                    .button("Load Project...")
                    .on_hover_text("Load project file (.json)")
                    .clicked()
                {
                    if let Some(path) = rfd::FileDialog::new()
                        .set_title("Load Project")
                        .add_filter("JSON", &["json"])
                        .pick_file() {
                        if let Err(e) = self.load_project_file(&path.to_string_lossy()) {
                            eprintln!("Failed to load project: {}", e);
                        }
                    }
                    ui.close_menu();
                }
                ui.separator();
                if ui
                    .button("Undo")
                    .on_hover_text("Undo (Ctrl+Z)")
                    .clicked()
                {
                    self.undo();
                    ui.close_menu();
                }
                if ui
                    .add_enabled(self.can_redo(), egui::Button::new("Redo"))
                    .on_hover_text("Redo (Ctrl+Shift+Z)")
                    .clicked()
                {
                    self.redo();
                    ui.close_menu();
                }
            });

            ui.menu_button("Edit", |ui| {
                if ui
                    .button("Resize Canvas")
                    .on_hover_text("Resize the canvas")
                    .clicked()
                {
                    let (w, h) = {
                        let current_layer = self.get_active_layer();
                        (current_layer.width(), current_layer.height())
                    };
                    self.resize_width = w;
                    self.resize_height = h;
                    self.show_resize_dialog = true;
                    ui.close_menu();
                }
                ui.separator();
                if ui
                    .button("Undo")
                    .on_hover_text("Undo (Ctrl+Z)")
                    .clicked()
                {
                    self.undo();
                    ui.close_menu();
                }
                if ui
                    .add_enabled(self.can_redo(), egui::Button::new("Redo"))
                    .on_hover_text("Redo (Ctrl+Shift+Z)")
                    .clicked()
                {
                    self.redo();
                    ui.close_menu();
                }
            });

            ui.menu_button("View", |ui| {
                ui.checkbox(&mut self.show_layers_panel, "Show Layers");
                ui.checkbox(&mut self.show_frames_panel, "Show Frames");
                ui.checkbox(&mut self.show_color_panel, "Show Colors");
                ui.separator();
                ui.checkbox(&mut self.show_grid, "Show Grid");
                
                // Animation controls
                ui.separator();
                ui.label("Animation:");
                ui.horizontal(|ui| {
                    if ui.checkbox(&mut self.animation_enabled, "Enable Tool Animations").changed() {
                        if !self.animation_enabled {
                            self.current_tool_animation = None;
                            for (_, animation) in self.tool_animations.iter_mut() {
                                animation.stop();
                            }
                        }
                    }
                });
                
                // Zoom controls
                ui.separator();
                ui.label("Zoom:");
                ui.horizontal(|ui| {
                    if ui.button("-").clicked() {
                        self.zoom_out();
                    }
                    ui.label(format!("{:.1}%", self.zoom * 100.0));
                    if ui.button("+").clicked() {
                        self.zoom_in();
                    }
                    if ui.button("Reset").clicked() {
                        self.reset_zoom();
                    }
                });
                
                // Canvas controls
                ui.separator();
                if ui.button("Center Canvas").clicked() {
                    self.center_canvas();
                    ui.close_menu();
                }
                
                ui.separator();
                ui.checkbox(&mut self.onion_skinning, "Onion Skinning");
                if self.onion_skinning {
                    ui.horizontal(|ui| {
                        ui.label("Previous:");
                        ui.add(egui::DragValue::new(&mut self.onion_prev_frames).range(0..=5));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Next:");
                        ui.add(egui::DragValue::new(&mut self.onion_next_frames).range(0..=5));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Opacity:");
                        ui.add(egui::Slider::new(&mut self.onion_opacity, 0.1..=1.0));
                    });
                }
            });

            ui.menu_button("Tools", |ui| {
                ui.checkbox(&mut self.pixel_perfect_mode, "Pixel Perfect Mode");
                ui.separator();
                ui.checkbox(&mut self.symmetry_mode, "Symmetry Mode");
                if self.symmetry_mode {
                    ui.checkbox(&mut self.symmetry_axis.0, "Horizontal Symmetry");
                    ui.checkbox(&mut self.symmetry_axis.1, "Vertical Symmetry");
                }
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Brush Size:");
                    ui.add(egui::DragValue::new(&mut self.brush_size).range(1..=100));
                });
                ui.horizontal(|ui| {
                    ui.label("Spray Size:");
                    ui.add(egui::DragValue::new(&mut self.spray_size).range(1..=20));
                });
                ui.horizontal(|ui| {
                    ui.label("Spray Intensity:");
                    ui.add(egui::Slider::new(&mut self.spray_intensity, 0.1..=1.0));
                });
            });

            ui.menu_button("Plugins", |ui| {
                if ui.button("Plugin Manager").clicked() {
                    self.plugin_manager.show_plugin_dialog = true;
                    ui.close_menu();
                }
                ui.separator();
                
                // Show filter plugins
                ui.label("Filters");
                let filter_commands = self.plugin_manager.get_plugin_commands_by_category(crate::plugins::PluginCategory::Filter);
                for command in filter_commands {
                    if ui.button(&command.name).clicked() {
                        self.execute_plugin_command(&command.id);
                        ui.close_menu();
                    }
                }
                
                ui.separator();
                
                // Show utility plugins
                ui.label("Utilities");
                let utility_commands = self.plugin_manager.get_plugin_commands_by_category(crate::plugins::PluginCategory::Utility);
                for command in utility_commands {
                    if ui.button(&command.name).clicked() {
                        self.execute_plugin_command(&command.id);
                        ui.close_menu();
                    }
                }
            });

            ui.menu_button("Help", |ui| {
                ui.label("Left Click: Paint");
                ui.label("Right Click: Erase");
                ui.label("Alt + Click: Pick Color");
                ui.label("Ctrl+Z: Undo");
                ui.label("Ctrl+Shift+Z: Redo");
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("?").on_hover_text("Show Help").clicked() {
                    self.show_help_window(ctx);
                }
            });
        });
    }

    pub fn show_help_window(&self, ctx: &egui::Context) {
        egui::Window::new("Help")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Keyboard Shortcuts");
                ui.label("Ctrl+Z: Undo");
                ui.label("Ctrl+Shift+Z: Redo");
                ui.separator();
                ui.label("Ctrl+S: Quick Save");
                ui.label("Ctrl+Shift+S: Save All");
                ui.label("Ctrl+Alt+S: Export Options");
                ui.separator();
                ui.label("Ctrl+ +: Zoom In");
                ui.label("Ctrl+ -: Zoom Out");
                ui.label("Ctrl+0: Reset Zoom");
                ui.label("Ctrl+Home: Center Canvas");
                ui.separator();
                ui.label("Alt+Click: Pick color (eyedropper)");

                ui.heading("Mouse Controls");
                ui.label("Left click: Draw with selected tool");
                ui.label("Right click: Erase");
                ui.label("Drag: Continue drawing/erasing");
                ui.label("Mouse Wheel: Zoom in/out (unlimited)");

                ui.heading("Tools");
                ui.label(format!("{} Pencil: Draw with selected color", self.tool_icon_safe(Tool::Pencil)));
                ui.label(format!("{} Eraser: Make pixels transparent", self.tool_icon_safe(Tool::Eraser)));
                ui.label(format!("{} Bucket: Fill connected area with color", self.tool_icon_safe(Tool::Bucket)));
                ui.label(format!("{} Eyedropper: Pick color from canvas", self.tool_icon_safe(Tool::Eyedropper)));
                ui.label(format!("{} Move: Move layer content", self.tool_icon_safe(Tool::Move)));
                ui.label(format!("{} Line: Draw straight lines", self.tool_icon_safe(Tool::Line)));
                ui.label(format!("{} Rectangle: Draw rectangles", self.tool_icon_safe(Tool::Rectangle)));
                ui.label(format!("{} Circle: Draw circles", self.tool_icon_safe(Tool::Circle)));
                ui.label(format!("{} Select: Select rectangular area", self.tool_icon_safe(Tool::Select)));
                ui.label(format!("{} Lasso: Select freehand area", self.tool_icon_safe(Tool::Lasso)));
                ui.label(format!("{} Spray: Spray paint effect", self.tool_icon_safe(Tool::Spray)));
                ui.label(format!("{} Dither: Dithering patterns", self.tool_icon_safe(Tool::Dither)));
            });
    }

    pub fn show_toolbar(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Save buttons
            ui.group(|ui| {
                if ui.button("💾 Save").on_hover_text("Quick save (Ctrl+S)").clicked() {
                    if let Err(e) = self.quick_save() {
                        eprintln!("Quick save failed: {}", e);
                    }
                }
                
                if ui.button("📁 Save All").on_hover_text("Save all frames and layers (Ctrl+Shift+S)").clicked() {
                    self.save_all_dialog();
                }
                
                if ui.button("📤 Export").on_hover_text("Export with options (Ctrl+Alt+S)").clicked() {
                    self.show_export_dialog = true;
                }
            });
            
            ui.separator();
            
            // Undo/Redo buttons
            ui.group(|ui| {
                if ui.button("↶").on_hover_text("Undo (Ctrl+Z)").clicked() {
                    self.undo();
                }
                
                if ui.add_enabled(self.can_redo(), egui::Button::new("↷"))
                    .on_hover_text("Redo (Ctrl+Shift+Z)").clicked() {
                    self.redo();
                }
            });
            
            ui.separator();
            
            // Zoom controls
            ui.group(|ui| {
                if ui.button("-").on_hover_text("Zoom out (Ctrl+-)").clicked() {
                    self.zoom_out();
                }
                
                ui.label(format!("{:.0}%", self.zoom * 100.0));
                
                if ui.button("+").on_hover_text("Zoom in (Ctrl++)").clicked() {
                    self.zoom_in();
                }
                
                if ui.button("🎯").on_hover_text("Reset zoom (Ctrl+0)").clicked() {
                    self.reset_zoom();
                }
                
                if ui.button("⌂").on_hover_text("Center canvas (Ctrl+Home)").clicked() {
                    self.center_canvas();
                }
            });
            
            ui.separator();
            
            // Animation controls
            ui.group(|ui| {
                if self.frames.len() > 1 {
                    let icon = if self.animation_playing { "⏸" } else { "▶" };
                    let text = if self.animation_playing { "Pause" } else { "Play" };
                    
                    if ui.button(icon).on_hover_text(text).clicked() {
                        self.animation_playing = !self.animation_playing;
                    }
                    
                    ui.label(format!("Frame {}/{}", self.current_frame + 1, self.frames.len()));
                }
            });
            
            ui.separator();
            
            // Current tool and color
            ui.group(|ui| {
                let (tool_icon, scale, _rotation, alpha) = self.tool_icon_animated(self.tool);
                let tool_text = egui::RichText::new(tool_icon)
                    .size(16.0 * scale)
                    .color(egui::Color32::from_rgba_unmultiplied(255, 255, 255, (alpha * 255.0) as u8));
                
                ui.label(tool_text);
                ui.label(self.tool_name(self.tool));
                
                // Color preview
                let color_rect = ui.allocate_response(egui::vec2(20.0, 20.0), egui::Sense::hover());
                ui.painter().rect_filled(color_rect.rect, 2.0, self.selected_color);
                ui.painter().rect_stroke(color_rect.rect, 2.0, egui::Stroke::new(1.0, egui::Color32::WHITE), egui::epaint::StrokeKind::Middle);
            });
        });
    }
    
    pub fn show_animation_controls(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::Frame::group(ui.style())
            .stroke(egui::Stroke::new(1.0, ui.visuals().widgets.active.bg_fill))
            .corner_radius(5.0)
            .inner_margin(8.0)
            .outer_margin(5.0)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label("Animation");
                    ui.horizontal(|ui| {
                        let play_icon = if self.animation_playing { "Pause" } else { "Play" };
                        if ui
                            .button(play_icon)
                            .on_hover_text("Play/Pause Animation")
                            .clicked()
                        {
                            self.animation_playing = !self.animation_playing;
                            if self.animation_playing {
                                self.animation_frame = 0;
                                self.last_animation_time = ctx.input(|i| i.time);
                            }
                        }
                        ui.add(
                            egui::Slider::new(&mut self.animation_speed, 1.0..=30.0)
                                .text("FPS"),
                        );
                        if self.animation_playing {
                            ui.label(format!(
                                "Frame: {}/{}",
                                self.animation_frame + 1,
                                self.frames.len()
                            ));
                        }
                    });
                });
            });
    }

    pub fn show_tool_settings(&mut self, ui: &mut egui::Ui) {
        egui::Frame::group(ui.style())
            .stroke(egui::Stroke::new(1.0, ui.visuals().widgets.active.bg_fill))
            .corner_radius(5.0)
            .inner_margin(8.0)
            .outer_margin(5.0)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label("Tool Settings");
                    
                    match self.tool {
                        Tool::Spray => {
                            ui.label("Spray Intensity");
                            ui.add(egui::Slider::new(&mut self.spray_intensity, 0.1..=1.0).text(""));
                        },
                        Tool::Dither => {
                            ui.label("Dither Pattern");
                            ui.horizontal(|ui| {
                                for i in 0..4 {
                                    let selected = self.dither_pattern == i;
                                    if ui.selectable_label(selected, format!("P{}", i + 1)).clicked() {
                                        self.dither_pattern = i;
                                    }
                                }
                            });
                        },
                        Tool::Rectangle | Tool::Circle => {
                            ui.checkbox(&mut self.fill_outline, "Fill Shape");
                        },
                        Tool::Line => {
                            ui.label("Line tool: Click and drag to draw");
                        },
                        Tool::Select | Tool::Lasso => {
                            ui.label("Selection tool: Click and drag to select");
                            if self.selection_area.is_some() {
                                ui.horizontal(|ui| {
                                    if ui.button("Copy").clicked() {
                                        self.copy_selection();
                                    }
                                    if ui.button("Cut").clicked() {
                                        self.cut_selection();
                                    }
                                    if ui.button("Clear").clicked() {
                                        self.clear_selection();
                                    }
                                });
                            }
                        },
                        _ => {
                            ui.label("Standard tool settings");
                        }
                    }
                });
            });
    }

    fn copy_selection(&mut self) {
        // Implementation for copying selection
        if let Some((x, y, w, h)) = self.selection_area {
            let layer = self.get_active_layer();
            let mut selected_pixels = vec![vec![egui::Color32::TRANSPARENT; w]; h];
            
            for dy in 0..h {
                for dx in 0..w {
                    if x + dx < layer.width() && y + dy < layer.height() {
                        selected_pixels[dy][dx] = layer.grid[y + dy][x + dx];
                    }
                }
            }
            
            self.selection_pixels = Some(selected_pixels);
        }
    }

    fn cut_selection(&mut self) {
        self.push_undo();
        self.copy_selection();
        self.clear_selection();
    }

    fn clear_selection(&mut self) {
        if let Some((x, y, w, h)) = self.selection_area {
            let layer = self.get_active_layer_mut();
            
            for dy in 0..h {
                for dx in 0..w {
                    if x + dx < layer.width() && y + dy < layer.height() {
                        layer.grid[y + dy][x + dx] = egui::Color32::TRANSPARENT;
                    }
                }
            }
        }
    }
}
