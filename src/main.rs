use eframe::egui;
use image::{ImageBuffer, Rgba};
use rfd::FileDialog;

const GRID_SIZE: usize = 16;
const PIXEL_SIZE: f32 = 24.0;
const MAX_LAYERS: usize = 8;
const MAX_FRAMES: usize = 16;

#[derive(Clone, Copy, PartialEq)]
enum Tool {
    Pencil,
    Eraser,
    Bucket,
    Eyedropper,
}

#[derive(Clone)]
struct Layer {
    name: String,
    visible: bool,
    opacity: f32,
    grid: Vec<Vec<egui::Color32>>,
}

impl Layer {
    fn new(name: String, width: usize, height: usize, color: egui::Color32) -> Self {
        Self {
            name,
            visible: true,
            opacity: 1.0,
            grid: vec![vec![color; width]; height],
        }
    }
    fn width(&self) -> usize {
        self.grid.first().map_or(0, |row| row.len())
    }
    fn height(&self) -> usize {
        self.grid.len()
    }
}

impl Default for Layer {
    fn default() -> Self {
        Self::new("Layer 1".to_string(), 16, 16, egui::Color32::TRANSPARENT)
    }
}

#[derive(Clone)]
struct Frame {
    layers: Vec<Layer>,
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            layers: vec![Layer::default()],
        }
    }
}

struct PixelArtEditor {
    frames: Vec<Frame>,
    current_frame: usize,
    current_layer: usize,
    selected_color: egui::Color32,
    secondary_color: egui::Color32,
    tool: Tool,
    last_state: Option<(Vec<Frame>, usize, usize)>,
    show_grid: bool,
    brush_size: usize,
    zoom: f32,
    show_new_sprite_dialog: bool,
    new_sprite_width: usize,
    new_sprite_height: usize,
    new_sprite_color_mode: usize,
    new_sprite_bg: egui::Color32,
    show_layers_panel: bool,
    show_frames_panel: bool,
    show_color_panel: bool,
    color_palette: Vec<egui::Color32>,
    animation_playing: bool,
    animation_frame: usize,
    animation_speed: f32,
    last_animation_time: f64,
}

impl Default for PixelArtEditor {
    fn default() -> Self {
        let default_palette = vec![
            egui::Color32::BLACK,
            egui::Color32::WHITE,
            egui::Color32::RED,
            egui::Color32::GREEN,
            egui::Color32::BLUE,
            egui::Color32::YELLOW,
            egui::Color32::LIGHT_BLUE,
            egui::Color32::from_rgb(255, 165, 0), // Orange
            egui::Color32::from_rgb(128, 0, 128), // Purple
            egui::Color32::from_rgb(255, 192, 203), // Pink
            egui::Color32::from_rgb(165, 42, 42), // Brown
            egui::Color32::GRAY,
            egui::Color32::DARK_GRAY,
            egui::Color32::LIGHT_GRAY,
            egui::Color32::from_rgb(0, 128, 128), // Teal
            egui::Color32::from_rgb(128, 128, 0), // Olive
        ];
        
        Self {
            frames: vec![Frame::default()],
            current_frame: 0,
            current_layer: 0,
            selected_color: egui::Color32::BLACK,
            secondary_color: egui::Color32::WHITE,
            tool: Tool::Pencil,
            last_state: None,
            show_grid: true,
            brush_size: 1,
            zoom: 1.0,
            show_new_sprite_dialog: false,
            new_sprite_width: 16,
            new_sprite_height: 16,
            new_sprite_color_mode: 0,
            new_sprite_bg: egui::Color32::TRANSPARENT,
            show_layers_panel: true,
            show_frames_panel: true,
            show_color_panel: true,
            color_palette: default_palette,
            animation_playing: false,
            animation_frame: 0,
            animation_speed: 5.0,
            last_animation_time: 0.0,
        }
    }
}

impl PixelArtEditor {
    fn save_as_png(&self, path: &str) {
        let layer = &self.frames[self.current_frame].layers[self.current_layer];
        let width = layer.width() as u32;
        let height = layer.height() as u32;
        let mut img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);
        let composed = self.get_composed_grid();
        for y in 0..height as usize {
            for x in 0..width as usize {
                let c = composed[y][x];
                img.put_pixel(x as u32, y as u32, Rgba([c.r(), c.g(), c.b(), c.a()]));
            }
        }
        img.save(path).ok();
    }

    fn save_as_png_dialog(&self) {
        if let Some(path) = FileDialog::new()
            .add_filter("PNG Image", &["png"])
            .set_file_name("pixel_art.png")
            .save_file()
        {
            self.save_as_png(path.to_string_lossy().as_ref());
        }
    }
    
    fn undo(&mut self) {
        if let Some((frames, cf, cl)) = self.last_state.take() {
            self.frames = frames;
            self.current_frame = cf;
            self.current_layer = cl;
        }
    }
    
    fn push_undo(&mut self) {
        self.last_state = Some((self.frames.clone(), self.current_frame, self.current_layer));
    }
    
    fn get_active_layer_mut(&mut self) -> &mut Layer {
        &mut self.frames[self.current_frame].layers[self.current_layer]
    }
    
    fn get_active_layer(&self) -> &Layer {
        &self.frames[self.current_frame].layers[self.current_layer]
    }
    
    fn get_composed_grid(&self) -> Vec<Vec<egui::Color32>> {
        let frame = &self.frames[self.current_frame];
        let layer = &frame.layers[self.current_layer];
        let width = layer.width();
        let height = layer.height();
        let mut composed = vec![vec![egui::Color32::TRANSPARENT; width]; height];
        
        for layer in &frame.layers {
            if !layer.visible {
                continue;
            }
            for y in 0..height {
                for x in 0..width {
                    let c = layer.grid[y][x];
                    if c.a() > 0 {
                        composed[y][x] = c;
                    }
                }
            }
        }
        composed
    }
    
    fn tool_icon(&self, tool: Tool) -> &'static str {
        match tool {
            Tool::Pencil => "🖊",
            Tool::Eraser => "🧽",
            Tool::Bucket => "🪣",
            Tool::Eyedropper => "👁",
        }
    }
    
    fn tool_name(&self, tool: Tool) -> &'static str {
        match tool {
            Tool::Pencil => "Pencil",
            Tool::Eraser => "Eraser",
            Tool::Bucket => "Bucket",
            Tool::Eyedropper => "Eyedropper",
        }
    }
    
    fn flood_fill(&mut self, x: usize, y: usize, original_color: egui::Color32, new_color: egui::Color32) {
        let layer = self.get_active_layer_mut();
        let width = layer.width();
        let height = layer.height();
        if x >= width || y >= height || layer.grid[y][x] != original_color {
            return;
        }
        let mut stack = vec![(x, y)];
        while let Some((cx, cy)) = stack.pop() {
            if cx >= width || cy >= height || layer.grid[cy][cx] != original_color {
                continue;
            }
            layer.grid[cy][cx] = new_color;
            if cx > 0 { stack.push((cx - 1, cy)); }
            if cx < width - 1 { stack.push((cx + 1, cy)); }
            if cy > 0 { stack.push((cx, cy - 1)); }
            if cy < height - 1 { stack.push((cx, cy + 1)); }
        }
    }
}

impl eframe::App for PixelArtEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle animation
        if self.animation_playing && self.frames.len() > 1 {
            let current_time = ctx.input(|i| i.time);
            if current_time - self.last_animation_time > 1.0 / self.animation_speed as f64 {
                self.animation_frame = (self.animation_frame + 1) % self.frames.len();
                self.last_animation_time = current_time;
            }
        }
        
        // Menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("🆕 New Sprite").on_hover_text("Create new sprite").clicked() {
                        self.show_new_sprite_dialog = true;
                        ui.close_menu();
                    }
                    if ui.button("💾 Save as PNG").on_hover_text("Save as pixel_art.png").clicked() {
                        self.save_as_png("pixel_art.png");
                        ui.close_menu();
                    }
                    if ui.button("💾 Save As...").on_hover_text("Save as... (choose file)").clicked() {
                        self.save_as_png_dialog();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("🔄 Undo").on_hover_text("Undo (Ctrl+Z)").clicked() {
                        self.undo();
                        ui.close_menu();
                    }
                });
                
                ui.menu_button("View", |ui| {
                    ui.checkbox(&mut self.show_layers_panel, "Show Layers");
                    ui.checkbox(&mut self.show_frames_panel, "Show Frames");
                    ui.checkbox(&mut self.show_color_panel, "Show Colors");
                    ui.separator();
                    ui.checkbox(&mut self.show_grid, "Show Grid");
                });
                
                ui.menu_button("Help", |ui| {
                    ui.label("🖱️ Left Click: Paint");
                    ui.label("🖱️ Right Click: Erase");
                    ui.label("⌨️ Alt + Click: Pick Color");
                    ui.label("⌨️ Ctrl + Z: Undo");
                });

                // ปุ่ม Help ที่มุมขวาบน
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("❓").on_hover_text("Show Help").clicked() {
                        egui::Window::new("Help")
                            .collapsible(false)
                            .resizable(false)
                            .show(ctx, |ui| {
                                ui.label("🖊 Pencil: Draw");
                                ui.label("🧽 Eraser: Erase");
                                ui.label("🪣 Bucket: Fill area");
                                ui.label("👁 Eyedropper: Pick color");
                                ui.label("Ctrl+Z: Undo");
                                ui.label("Alt+Click: Eyedropper");
                                ui.label("Right click: Erase");
                                ui.label("Zoom: Use slider or +/- buttons");
                            });
                    }
                });
            });
        });
        
        // Main toolbar
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                // Tools section
                ui.vertical(|ui| {
                    ui.label("Tools:");
                    ui.horizontal(|ui| {
                        for tool in [Tool::Pencil, Tool::Eraser, Tool::Bucket, Tool::Eyedropper] {
                            let selected = self.tool == tool;
                            if ui.selectable_label(selected, format!("{} {}", self.tool_icon(tool), self.tool_name(tool)))
                                .on_hover_text(self.tool_name(tool))
                                .clicked()
                            {
                                self.tool = tool;
                            }
                        }
                    });
                });

                ui.separator();

                // Brush settings
                ui.vertical(|ui| {
                    ui.label("Brush:");
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(&mut self.brush_size, 1..=10).text("Size"));
                        // ปุ่ม Zoom in/out/reset
                        if ui.button("➖").on_hover_text("Zoom Out").clicked() {
                            self.zoom = (self.zoom - 0.25).max(0.5);
                        }
                        if ui.button("➕").on_hover_text("Zoom In").clicked() {
                            self.zoom = (self.zoom + 0.25).min(4.0);
                        }
                        if ui.button("🔄").on_hover_text("Reset Zoom").clicked() {
                            self.zoom = 1.0;
                        }
                        ui.add(egui::Slider::new(&mut self.zoom, 0.5..=4.0).text("Zoom"));
                    });
                });

                ui.separator();

                // Animation controls
                if self.frames.len() > 1 {
                    ui.vertical(|ui| {
                        ui.label("Animation:");
                        ui.horizontal(|ui| {
                            let play_icon = if self.animation_playing { "⏸" } else { "▶" };
                            if ui.button(play_icon).on_hover_text("Play/Pause Animation").clicked() {
                                self.animation_playing = !self.animation_playing;
                            }
                            ui.add(egui::Slider::new(&mut self.animation_speed, 1.0..=30.0).text("FPS"));
                        });
                    });
                }

                ui.separator();

                // ปุ่ม Center Canvas
                if ui.button("🎯 Center Canvas").on_hover_text("Scroll to center").clicked() {
                    ctx.memory_mut(|mem| mem.reset_areas());
                }
            });
            ui.add_space(5.0);
        });
        
        // New Sprite Dialog
        if self.show_new_sprite_dialog {
            egui::Window::new("🆕 New Sprite")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Canvas Size:");
                        ui.add(egui::DragValue::new(&mut self.new_sprite_width).range(1..=512).prefix("W: "));
                        ui.add(egui::DragValue::new(&mut self.new_sprite_height).range(1..=512).prefix("H: "));
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Background:");
                        egui::color_picker::color_edit_button_srgba(ui, &mut self.new_sprite_bg, egui::color_picker::Alpha::BlendOrAdditive);
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
                            
                            self.frames = vec![Frame { layers: vec![layer] }];
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
        
        // Color Panel
        if self.show_color_panel {
            egui::SidePanel::left("color_panel").resizable(true).show(ctx, |ui| {
                ui.heading("🎨 Colors");
                ui.separator();

                ui.vertical(|ui| {
                    ui.label("Primary Color:");
                    egui::color_picker::color_edit_button_srgba(ui, &mut self.selected_color, egui::color_picker::Alpha::BlendOrAdditive);

                    ui.label("Secondary Color:");
                    egui::color_picker::color_edit_button_srgba(ui, &mut self.secondary_color, egui::color_picker::Alpha::BlendOrAdditive);

                    if ui.button("🔄 Swap").clicked() {
                        std::mem::swap(&mut self.selected_color, &mut self.secondary_color);
                    }
                });

                ui.separator();
                ui.label("Palette:");

                let palette_size = 24.0;
                let columns = 4;

                for (i, color) in self.color_palette.iter().enumerate() {
                    if i % columns == 0 {
                        ui.horizontal(|ui| {
                            for j in 0..columns {
                                let idx = i + j;
                                if idx < self.color_palette.len() {
                                    let color = self.color_palette[idx];
                                    let (rect, response) = ui.allocate_exact_size(
                                        egui::vec2(palette_size, palette_size),
                                        egui::Sense::click()
                                    );

                                    if response.clicked() {
                                        self.selected_color = color;
                                    }

                                    ui.painter().rect_filled(rect, 2.0, color);
                                    ui.painter().rect_stroke(
                                        rect,
                                        egui::CornerRadius::ZERO,
                                        egui::Stroke::new(1.0, egui::Color32::GRAY),
                                        egui::epaint::StrokeKind::Middle,
                                    );
                                }
                            }
                        });
                    }
                }
            });
        }
        
        // Layers Panel
        if self.show_layers_panel {
            let mut layer_to_remove = None;
            let mut add_layer = false;
            let mut layer_to_duplicate = None;
            let mut layer_to_clear = None;

            egui::SidePanel::left("layers_panel").resizable(true).show(ctx, |ui| {
                ui.heading("📚 Layers");

                let frame = &mut self.frames[self.current_frame];
                let layers_len = frame.layers.len();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (i, layer) in frame.layers.iter_mut().enumerate().rev() {
                        let is_current = self.current_layer == i;

                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                if ui.selectable_label(is_current, &layer.name).clicked() {
                                    self.current_layer = i;
                                }

                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if ui.button("🧹").on_hover_text("Clear Layer").clicked() {
                                        layer_to_clear = Some(i);
                                    }
                                    if ui.button("📋").on_hover_text("Duplicate").clicked() {
                                        layer_to_duplicate = Some(i);
                                    }
                                    if layers_len > 1 && ui.button("🗑").on_hover_text("Delete").clicked() {
                                        layer_to_remove = Some(i);
                                    }
                                    ui.checkbox(&mut layer.visible, "👁").on_hover_text("Toggle visibility");
                                });
                            });

                            ui.horizontal(|ui| {
                                ui.label("Opacity:");
                                ui.add(egui::Slider::new(&mut layer.opacity, 0.0..=1.0).show_value(false));
                            });
                        });
                    }
                });

                ui.separator();
                if frame.layers.len() < MAX_LAYERS && ui.button("➕ Add Layer").on_hover_text("Add new layer").clicked() {
                    add_layer = true;
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
                let duplicated_layer = frame.layers[i].clone();
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
        }
        
        // Frames Panel
        if self.show_frames_panel {
            egui::SidePanel::right("frames_panel").resizable(true).show(ctx, |ui| {
                ui.heading("🎬 Frames");

                // Collect frame info to avoid borrow checker issues
                let frame_infos: Vec<(usize, bool)> = (0..self.frames.len())
                    .map(|i| (i, self.animation_playing && self.animation_frame == i))
                    .collect();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (i, is_anim_frame) in frame_infos {
                        let is_current = self.current_frame == i;
                        let frame_display = if is_anim_frame {
                            format!("▶ Frame {}", i + 1)
                        } else {
                            format!("Frame {}", i + 1)
                        };

                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                if ui.selectable_label(is_current, frame_display).clicked() {
                                    self.current_frame = i;
                                    self.current_layer = 0;
                                }

                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if self.frames.len() > 1 && ui.button("🗑").on_hover_text("Delete").clicked() {
                                        self.push_undo();
                                        self.frames.remove(i);
                                        if self.current_frame >= self.frames.len() {
                                            self.current_frame = self.frames.len() - 1;
                                        }
                                        self.current_layer = 0;
                                    }
                                    if ui.button("📋").on_hover_text("Duplicate").clicked() {
                                        self.push_undo();
                                        let new_frame = self.frames[i].clone();
                                        self.frames.insert(i + 1, new_frame);
                                        self.current_frame = i + 1;
                                        self.current_layer = 0;
                                    }
                                });
                            });
                        });
                    }
                });
                
                ui.separator();
                if self.frames.len() < MAX_FRAMES && ui.button("➕ Add Frame").clicked() {
                    self.push_undo();
                    let new_frame = self.frames[self.current_frame].clone();
                    self.frames.insert(self.current_frame + 1, new_frame);
                    self.current_frame += 1;
                    self.current_layer = 0;
                }
            });
        }
        
        // Central canvas
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                let composed = self.get_composed_grid();
                let height = composed.len();
                let width = if height > 0 { composed[0].len() } else { 0 };
                
                let pixel_size = PIXEL_SIZE * self.zoom;
                let canvas_size = egui::vec2(width as f32 * pixel_size, height as f32 * pixel_size);
                
                ui.allocate_ui_with_layout(canvas_size, egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    for y in 0..height {
                        ui.horizontal(|ui| {
                            for x in 0..width {
                                let (rect, response) = ui.allocate_exact_size(
                                    egui::vec2(pixel_size, pixel_size),
                                    egui::Sense::click_and_drag(),
                                );
                                
                                let pointer = ui.input(|i| i.pointer.clone());
                                let alt = ui.input(|i| i.modifiers.alt);
                                
                                // Handle drawing
                                if (response.clicked() || response.dragged()) && pointer.primary_down() {
                                    if !alt {
                                        self.push_undo();
                                        let brush = self.brush_size;
                                        let min_y = y.saturating_sub(brush / 2);
                                        let min_x = x.saturating_sub(brush / 2);
                                        let max_y = (y + brush / 2).min(height - 1);
                                        let max_x = (x + brush / 2).min(width - 1);
                                        
                                        for by in min_y..=max_y {
                                            for bx in min_x..=max_x {
                                                match self.tool {
                                                    Tool::Pencil => {
                                                        self.get_active_layer_mut().grid[by][bx] = self.selected_color;
                                                    }
                                                    Tool::Eraser => {
                                                        self.get_active_layer_mut().grid[by][bx] = egui::Color32::TRANSPARENT;
                                                    }
                                                    Tool::Bucket => {
                                                        // Simple bucket fill implementation
                                                        let original_color = self.get_active_layer().grid[by][bx];
                                                        if original_color != self.selected_color {
                                                            self.flood_fill(bx, by, original_color, self.selected_color);
                                                        }
                                                    }
                                                    Tool::Eyedropper => {
                                                        self.selected_color = composed[by][bx];
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                
                                // Right click erase
                                if response.secondary_clicked() || (response.hovered() && pointer.secondary_down()) {
                                    self.push_undo();
                                    let brush = self.brush_size;
                                    let min_y = y.saturating_sub(brush / 2);
                                    let min_x = x.saturating_sub(brush / 2);
                                    let max_y = (y + brush / 2).min(height - 1);
                                    let max_x = (x + brush / 2).min(width - 1);
                                    
                                    for by in min_y..=max_y {
                                        for bx in min_x..=max_x {
                                            self.get_active_layer_mut().grid[by][bx] = egui::Color32::TRANSPARENT;
                                        }
                                    }
                                }
                                
                                // Alt+click eyedropper
                                if response.clicked() && alt {
                                    self.selected_color = composed[y][x];
                                }
                                
                                // Draw pixel
                                let pixel_color = composed[y][x];
                                let display_color = if pixel_color.a() == 0 {
                                    // Checkered pattern for transparency
                                    if (x + y) % 2 == 0 {
                                        egui::Color32::from_gray(220)
                                    } else {
                                        egui::Color32::from_gray(240)
                                    }
                                } else {
                                    pixel_color
                                };
                                
                                ui.painter().rect_filled(rect, 1.0, display_color);
                                
                                if self.show_grid {
                                    ui.painter().rect_stroke(
                                        rect,
                                        egui::CornerRadius::ZERO,
                                        egui::Stroke::new(0.5, egui::Color32::from_gray(128)),
                                        egui::epaint::StrokeKind::Middle,
                                    );
                                }
                            }
                        });
                    }
                });
            });
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Pixel Art Editor",
        options,
        Box::new(|_cc| Ok(Box::new(PixelArtEditor::default()))),
    );
}