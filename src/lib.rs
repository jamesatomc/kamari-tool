use eframe::egui;
use image::{ImageBuffer, Rgba};
use rfd::FileDialog;

const PIXEL_SIZE: f32 = 24.0;
const MAX_LAYERS: usize = 64; // Increased from 8
const MAX_FRAMES: usize = 64; // Increased from 16

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
    pub fn new(name: String, width: usize, height: usize, color: egui::Color32) -> Self {
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

pub struct PixelArtEditor {
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
    new_sprite_bg: egui::Color32,
    show_layers_panel: bool,
    show_frames_panel: bool,
    show_color_panel: bool,
    color_palette: Vec<egui::Color32>,
    animation_playing: bool,
    animation_frame: usize,
    animation_speed: f32,
    last_animation_time: f64,
    show_resize_dialog: bool,
    resize_width: usize,
    resize_height: usize,
    resize_anchor: usize, // 0=top-left, 1=center, 2=bottom-right
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
            egui::Color32::from_rgb(255, 165, 0),   // Orange
            egui::Color32::from_rgb(128, 0, 128),   // Purple
            egui::Color32::from_rgb(255, 192, 203), // Pink
            egui::Color32::from_rgb(165, 42, 42),   // Brown
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
            new_sprite_bg: egui::Color32::TRANSPARENT,
            show_layers_panel: true,
            show_frames_panel: true,
            show_color_panel: true,
            color_palette: default_palette,
            animation_playing: false,
            animation_frame: 0,
            animation_speed: 5.0,
            last_animation_time: 0.0,
            show_resize_dialog: false,
            resize_width: 16,
            resize_height: 16,
            resize_anchor: 1, // Center by default
        }
    }
}

impl PixelArtEditor {
    pub fn new() -> Self {
        Self::default()
    }

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
        // If we're in animation mode, use the animation frame
        let frame_idx = if self.animation_playing {
            self.animation_frame
        } else {
            self.current_frame
        };
        let frame = &self.frames[frame_idx];

        let width = frame.layers[0].width();
        let height = frame.layers[0].height();
        let mut composed = vec![vec![egui::Color32::TRANSPARENT; width]; height];

        for layer in &frame.layers {
            if !layer.visible {
                continue;
            }
            for y in 0..height {
                for x in 0..width {
                    let c = layer.grid[y][x];
                    if c.a() > 0 {
                        // Apply layer opacity
                        let new_color = if layer.opacity < 1.0 {
                            let alpha = (c.a() as f32 * layer.opacity) as u8;
                            egui::Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), alpha)
                        } else {
                            c
                        };

                        // Blend the color
                        if new_color.a() == 255 {
                            composed[y][x] = new_color;
                        } else if new_color.a() > 0 {
                            let bg = composed[y][x];
                            composed[y][x] = blend_colors(bg, new_color);
                        }
                    }
                }
            }
        }
        composed
    }

    // Helper function to blend colors with transparency
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

    fn flood_fill(
        &mut self,
        x: usize,
        y: usize,
        original_color: egui::Color32,
        new_color: egui::Color32,
    ) {
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

    fn resize_canvas(&mut self, new_width: usize, new_height: usize, anchor: usize) {
        self.push_undo();
        
        for frame in &mut self.frames {
            for layer in &mut frame.layers {
                let old_width = layer.width();
                let old_height = layer.height();
                let mut new_grid = vec![vec![egui::Color32::TRANSPARENT; new_width]; new_height];
                
                let (offset_x, offset_y) = match anchor {
                    0 => (0, 0), // Top-left
                    1 => { // Center
                        let x_offset = if new_width > old_width { (new_width - old_width) / 2 } else { 0 };
                        let y_offset = if new_height > old_height { (new_height - old_height) / 2 } else { 0 };
                        (x_offset, y_offset)
                    },
                    2 => { // Bottom-right
                        let x_offset = if new_width > old_width { new_width - old_width } else { 0 };
                        let y_offset = if new_height > old_height { new_height - old_height } else { 0 };
                        (x_offset, y_offset)
                    },
                    _ => (0, 0),
                };
                
                // Copy existing pixels to new grid
                for y in 0..old_height.min(new_height) {
                    for x in 0..old_width.min(new_width) {
                        let new_x = x + offset_x;
                        let new_y = y + offset_y;
                        if new_x < new_width && new_y < new_height {
                            new_grid[new_y][new_x] = layer.grid[y][x];
                        }
                    }
                }
                
                layer.grid = new_grid;
            }
        }
    }
}

// Helper function to blend colors with transparency
fn blend_colors(bg: egui::Color32, fg: egui::Color32) -> egui::Color32 {
    if fg.a() == 0 {
        return bg;
    }
    if fg.a() == 255 || bg.a() == 0 {
        return fg;
    }

    let a_fg = fg.a() as f32 / 255.0;
    let a_bg = bg.a() as f32 / 255.0;
    let a_out = a_fg + a_bg * (1.0 - a_fg);

    if a_out < 0.001 {
        return egui::Color32::TRANSPARENT;
    }

    let r = ((fg.r() as f32 * a_fg + bg.r() as f32 * a_bg * (1.0 - a_fg)) / a_out) as u8;
    let g = ((fg.g() as f32 * a_fg + bg.g() as f32 * a_bg * (1.0 - a_fg)) / a_out) as u8;
    let b = ((fg.b() as f32 * a_fg + bg.b() as f32 * a_bg * (1.0 - a_fg)) / a_out) as u8;
    let a = (a_out * 255.0) as u8;

    egui::Color32::from_rgba_unmultiplied(r, g, b, a)
}

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
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui
                        .button("🆕 New Sprite")
                        .on_hover_text("Create new sprite")
                        .clicked()
                    {
                        self.show_new_sprite_dialog = true;
                        ui.close_menu();
                    }
                    if ui
                        .button("💾 Save as PNG")
                        .on_hover_text("Save as pixel_art.png")
                        .clicked()
                    {
                        self.save_as_png("pixel_art.png");
                        ui.close_menu();
                    }
                    if ui
                        .button("💾 Save As...")
                        .on_hover_text("Save as... (choose file)")
                        .clicked()
                    {
                        self.save_as_png_dialog();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui
                        .button("🔄 Undo")
                        .on_hover_text("Undo (Ctrl+Z)")
                        .clicked()
                    {
                        self.undo();
                        ui.close_menu();
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui
                        .button("📐 Resize Canvas")
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
                        .button("🔄 Undo")
                        .on_hover_text("Undo (Ctrl+Z)")
                        .clicked()
                    {
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

                // Help button in the top right corner
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("❓").on_hover_text("Show Help").clicked() {
                        egui::Window::new("Help")
                            .collapsible(false)
                            .resizable(false)
                            .show(ctx, |ui| {
                                ui.heading("Keyboard Shortcuts");
                                ui.label("Ctrl+Z: Undo");
                                ui.label("Alt+Click: Pick color (eyedropper)");

                                ui.heading("Mouse Controls");
                                ui.label("Left click: Draw with selected tool");
                                ui.label("Right click: Erase");
                                ui.label("Drag: Continue drawing/erasing");

                                ui.heading("Tools");
                                ui.label("🖊 Pencil: Draw with selected color");
                                ui.label("🧽 Eraser: Make pixels transparent");
                                ui.label("🪣 Bucket: Fill connected area with color");
                                ui.label("👁 Eyedropper: Pick color from canvas");
                            });
                    }
                });
            });
        });

        // Improved toolbar with better organization
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                // Group tools in a nice looking panel
                egui::Frame::group(ui.style())
                    .stroke(egui::Stroke::new(1.0, ui.visuals().widgets.active.bg_fill))
                    .corner_radius(5.0) // Changed from rounding to corner_radius
                    .inner_margin(8.0)
                    .outer_margin(5.0)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.label("Tools");
                            ui.horizontal(|ui| {
                                for tool in
                                    [Tool::Pencil, Tool::Eraser, Tool::Bucket, Tool::Eyedropper]
                                {
                                    let selected = self.tool == tool;
                                    let response = ui
                                        .selectable_label(selected, self.tool_icon(tool))
                                        .on_hover_text(self.tool_name(tool));

                                    if response.clicked() {
                                        self.tool = tool;
                                    }
                                }
                            });
                        });
                    });

                ui.separator();

                // Brush settings
                egui::Frame::group(ui.style())
                    .stroke(egui::Stroke::new(1.0, ui.visuals().widgets.active.bg_fill))
                    .corner_radius(5.0) // Changed from rounding to corner_radius
                    .inner_margin(8.0)
                    .outer_margin(5.0)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.label("Brush Size");
                            ui.add(egui::Slider::new(&mut self.brush_size, 1..=10).text(""));
                        });
                    });

                ui.separator();

                // Zoom controls
                egui::Frame::group(ui.style())
                    .stroke(egui::Stroke::new(1.0, ui.visuals().widgets.active.bg_fill))
                    .corner_radius(5.0) // Changed from rounding to corner_radius
                    .inner_margin(8.0)
                    .outer_margin(5.0)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.label("Zoom");
                            ui.horizontal(|ui| {
                                if ui.button("➖").on_hover_text("Zoom Out").clicked() {
                                    self.zoom = (self.zoom - 0.25).max(0.5);
                                }
                                ui.label(format!("{:.0}%", self.zoom * 100.0));
                                if ui.button("➕").on_hover_text("Zoom In").clicked() {
                                    self.zoom = (self.zoom + 0.25).min(4.0);
                                }
                                if ui.button("🔄").on_hover_text("Reset Zoom").clicked() {
                                    self.zoom = 1.0;
                                }
                            });
                        });
                    });

                // Animation controls - only show when we have multiple frames
                if self.frames.len() > 1 {
                    ui.separator();

                    egui::Frame::group(ui.style())
                        .stroke(egui::Stroke::new(1.0, ui.visuals().widgets.active.bg_fill))
                        .corner_radius(5.0) // Changed from rounding to corner_radius
                        .inner_margin(8.0)
                        .outer_margin(5.0)
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                ui.label("Animation");
                                ui.horizontal(|ui| {
                                    let play_icon =
                                        if self.animation_playing { "⏸" } else { "▶" };
                                    if ui
                                        .button(play_icon)
                                        .on_hover_text("Play/Pause Animation")
                                        .clicked()
                                    {
                                        self.animation_playing = !self.animation_playing;
                                        // Reset animation frame when starting playback
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

                // Center canvas button
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .button("🎯 Center")
                        .on_hover_text("Center the canvas view")
                        .clicked()
                    {
                        ctx.memory_mut(|mem| mem.reset_areas());
                    }
                });
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
                        ui.add(
                            egui::DragValue::new(&mut self.new_sprite_width)
                                .range(1..=4096) // Increased from 512
                                .prefix("W: "),
                        );
                        ui.add(
                            egui::DragValue::new(&mut self.new_sprite_height)
                                .range(1..=4096) // Increased from 512
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

                            self.frames = vec![Frame {
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

        // Canvas Resize Dialog
        if self.show_resize_dialog {
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

        // Color Panel with improved layout
        if self.show_color_panel {
            egui::SidePanel::left("color_panel")
                .resizable(true)
                .default_width(180.0)
                .show(ctx, |ui| {
                    ui.heading("🎨 Colors");
                    ui.separator();

                    // Current colors with better UI
                    egui::Frame::group(ui.style())
                        .fill(ui.style().visuals.extreme_bg_color)
                        .corner_radius(5.0) // Changed from rounding to corner_radius
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
                                    ui.painter().rect_stroke(
                                        rect,
                                        4.0,
                                        egui::Stroke::new(
                                            1.0,
                                            ui.visuals().widgets.active.bg_stroke.color,
                                        ),
                                        egui::epaint::StrokeKind::Middle, // Using correct StrokeKind
                                    );

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
                                    ui.painter().rect_stroke(
                                        rect,
                                        4.0,
                                        egui::Stroke::new(
                                            1.0,
                                            ui.visuals().widgets.active.bg_stroke.color,
                                        ),
                                        egui::epaint::StrokeKind::Middle, // Using correct StrokeKind
                                    );

                                    egui::color_picker::color_edit_button_srgba(
                                        ui,
                                        &mut self.secondary_color,
                                        egui::color_picker::Alpha::BlendOrAdditive,
                                    );
                                });

                                if ui.button("🔄 Swap Colors").clicked() {
                                    std::mem::swap(
                                        &mut self.selected_color,
                                        &mut self.secondary_color,
                                    );
                                }
                            });
                        });

                    ui.add_space(10.0);
                    ui.label("Color Palette:");

                    // Improved palette layout
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
                                        // Fixed: Remove StrokeKind parameter
                                        ui.painter().rect_stroke(
                                            rect,
                                            4.0,
                                            egui::Stroke::new(1.0, egui::Color32::GRAY),
                                            egui::epaint::StrokeKind::Middle, // Using correct StrokeKind
                                        );

                                        // Highlight selected colors
                                        if *color == self.selected_color {
                                            // Fixed: Remove StrokeKind parameter
                                            ui.painter().rect_stroke(
                                                rect.shrink(2.0),
                                                4.0,
                                                egui::Stroke::new(2.0, egui::Color32::WHITE),
                                                egui::epaint::StrokeKind::Middle, // Using correct StrokeKind
                                            );
                                        } else if *color == self.secondary_color {
                                            // Fixed: Remove StrokeKind parameter
                                            ui.painter().rect_stroke(
                                                rect.shrink(2.0),
                                                4.0,
                                                egui::Stroke::new(1.0, egui::Color32::WHITE),
                                                egui::epaint::StrokeKind::Middle, // Using correct StrokeKind
                                            );
                                        }
                                    }
                                });
                                row.clear();
                            }
                        }
                    });
                });
        }

        // Central canvas with improved pixel rendering
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    let composed = self.get_composed_grid();
                    let height = composed.len();
                    let width = if height > 0 { composed[0].len() } else { 0 };

                    let pixel_size = PIXEL_SIZE * self.zoom;
                    let canvas_size =
                        egui::vec2(width as f32 * pixel_size, height as f32 * pixel_size);

                    // Track if undo was pushed for this drag/paint
                    let mut undo_pushed = false;

                    let (canvas_rect, _) =
                        ui.allocate_exact_size(canvas_size, egui::Sense::hover());

                    // Draw the canvas background (checker pattern for transparent pixels)
                    for y in 0..height {
                        for x in 0..width {
                            let pixel_rect = egui::Rect::from_min_size(
                                canvas_rect.min
                                    + egui::vec2(x as f32 * pixel_size, y as f32 * pixel_size),
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
                                // Fixed: Remove StrokeKind parameter
                                ui.painter().rect_stroke(
                                    pixel_rect,
                                    0.0,
                                    egui::Stroke::new(0.5, egui::Color32::from_gray(128)),
                                    egui::epaint::StrokeKind::Middle, // Using correct StrokeKind
                                );
                            }

                            // Handle pixel interaction - Fix Aseprite-like behavior
                            let interact_rect =
                                ui.allocate_rect(pixel_rect, egui::Sense::click_and_drag());
                            let pointer = ui.input(|i| i.pointer.clone());
                            let alt = ui.input(|i| i.modifiers.alt);

                            // Handle drawing - Aseprite-like tool behavior
                            if interact_rect.hovered() && pointer.primary_down() {
                                if alt || self.tool == Tool::Eyedropper {
                                    // Eyedropper tool
                                    self.selected_color = composed[y][x];
                                } else {
                                    // Only push_undo once per stroke
                                    if interact_rect.clicked() && !undo_pushed {
                                        self.push_undo();
                                        undo_pushed = true;
                                    }

                                    let brush = self.brush_size;
                                    let min_y = y.saturating_sub(brush / 2);
                                    let min_x = x.saturating_sub(brush / 2);
                                    let max_y = (y + brush / 2).min(height - 1);
                                    let max_x = (x + brush / 2).min(width - 1);

                                    for by in min_y..=max_y {
                                        for bx in min_x..=max_x {
                                            match self.tool {
                                                Tool::Pencil => {
                                                    self.get_active_layer_mut().grid[by][bx] =
                                                        self.selected_color;
                                                }
                                                Tool::Eraser => {
                                                    self.get_active_layer_mut().grid[by][bx] =
                                                        egui::Color32::TRANSPARENT;
                                                }
                                                Tool::Bucket => {
                                                    // Only fill on click, not drag
                                                    if interact_rect.clicked() {
                                                        let original_color =
                                                            self.get_active_layer().grid[by][bx];
                                                        if original_color != self.selected_color {
                                                            self.flood_fill(
                                                                bx,
                                                                by,
                                                                original_color,
                                                                self.selected_color,
                                                            );
                                                        }
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

                            // Right click erase - Aseprite behavior
                            if interact_rect.hovered() && pointer.secondary_down() {
                                if interact_rect.secondary_clicked() && !undo_pushed {
                                    self.push_undo();
                                    undo_pushed = true;
                                }

                                let brush = self.brush_size;
                                let min_y = y.saturating_sub(brush / 2);
                                let min_x = x.saturating_sub(brush / 2);
                                let max_y = (y + brush / 2).min(height - 1);
                                let max_x = (x + brush / 2).min(width - 1);

                                for by in min_y..=max_y {
                                    for bx in min_x..=max_x {
                                        self.get_active_layer_mut().grid[by][bx] =
                                            egui::Color32::TRANSPARENT;
                                    }
                                }
                            }
                        }
                    }
                });
        }); // <-- Add this line to properly close the CentralPanel closure

        // Combined Layers and Frames Panel at the bottom
        if self.show_layers_panel || self.show_frames_panel {
            egui::TopBottomPanel::bottom("bottom_panel")
                .resizable(true)
                .default_height(200.0)
                .show(ctx, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        // Layers Panel
                        if self.show_layers_panel {
                            ui.vertical(|ui| {
                                ui.set_min_width(250.0);
                                ui.heading("📚 Layers");
                                ui.separator();

                                let mut layer_to_remove = None;
                                let mut add_layer = false;
                                let mut layer_to_duplicate = None;
                                let mut layer_to_clear = None;
                                let mut layer_to_rename = None;
                                let mut new_layer_name = String::new();
                                let mut should_move_up = false;
                                let mut should_move_down = false;

                                let current_frame = self.current_frame;
                                let frame = &mut self.frames[current_frame];
                                let layers_len = frame.layers.len();
                                let current_layer = self.current_layer;

                                egui::ScrollArea::vertical()
                                    .id_salt("layers_panel_scroll")
                                    .max_height(120.0)
                                    .show(ui, |ui| {
                                    // Display layers in reverse order (top to bottom like Aseprite)
                                    for (i, layer) in frame.layers.iter_mut().enumerate().rev() {
                                        let is_current = current_layer == i;

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
                                                    // Layer visibility checkbox first
                                                    if ui.checkbox(&mut layer.visible, "")
                                                        .on_hover_text("Toggle visibility")
                                                        .changed() {
                                                        ctx.request_repaint();
                                                    }

                                                    // Layer name (clickable to select)
                                                    if ui.selectable_label(is_current, &layer.name).clicked() {
                                                        self.current_layer = i;
                                                    }

                                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                        let btn_size = egui::vec2(18.0, 18.0);

                                                        if ui.add(egui::Button::new("🧹").min_size(btn_size))
                                                            .on_hover_text("Clear Layer")
                                                            .clicked() {
                                                            layer_to_clear = Some(i);
                                                        }

                                                        if ui.add(egui::Button::new("📋").min_size(btn_size))
                                                            .on_hover_text("Duplicate")
                                                            .clicked() {
                                                            layer_to_duplicate = Some(i);
                                                        }

                                                        if layers_len > 1
                                                            && ui.add(egui::Button::new("🗑").min_size(btn_size))
                                                                .on_hover_text("Delete")
                                                                .clicked() {
                                                            layer_to_remove = Some(i);
                                                        }

                                                        if ui.add(egui::Button::new("✏️").min_size(btn_size))
                                                            .on_hover_text("Rename")
                                                            .clicked() {
                                                            layer_to_rename = Some(i);
                                                            new_layer_name = layer.name.clone();
                                                        }
                                                    });
                                                });

                                                // Opacity slider
                                                ui.horizontal(|ui| {
                                                    ui.label("Opacity:");
                                                    if ui.add(egui::Slider::new(&mut layer.opacity, 0.0..=1.0)
                                                        .show_value(true)
                                                        .text(""))
                                                        .changed() {
                                                        ctx.request_repaint();
                                                    }
                                                });
                                            });

                                        ui.add_space(2.0);
                                    }
                                });

                                ui.separator();

                                // Move layer up/down buttons
                                ui.horizontal(|ui| {
                                    if frame.layers.len() < MAX_LAYERS && ui.button("➕ Add Layer").clicked() {
                                        add_layer = true;
                                    }

                                    if ui.button("🔼").on_hover_text("Move Layer Up").clicked()
                                        && self.current_layer < frame.layers.len() - 1 {
                                        should_move_up = true;
                                    }

                                    if ui.button("🔽").on_hover_text("Move Layer Down").clicked()
                                        && self.current_layer > 0 {
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

                                // Fix layer movement - moving up means higher in layer stack
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

                                // Handle layer renaming dialog
                                if let Some(layer_idx) = layer_to_rename {
                                    egui::Window::new("Rename Layer")
                                        .collapsible(false)
                                        .resizable(false)
                                        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                                        .show(ctx, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label("Name:");
                                                ui.text_edit_singleline(&mut new_layer_name);
                                            });

                                            ui.separator();
                                            ui.horizontal(|ui| {
                                                if ui.button("✅ Rename").clicked() {
                                                    if !new_layer_name.is_empty() {
                                                        self.frames[self.current_frame].layers[layer_idx].name =
                                                            new_layer_name.clone();
                                                    }
                                                    layer_to_rename = None;
                                                }
                                                if ui.button("❌ Cancel").clicked() {
                                                    layer_to_rename = None;
                                                }
                                            });
                                        });
                                }
                            });
                            
                            ui.separator();
                        }

                        // Frames Panel
                        if self.show_frames_panel {
                            ui.vertical(|ui| {
                                ui.set_min_width(250.0);
                                ui.heading("🎬 Frames");
                                ui.separator();

                                // Collect frame info to avoid borrow checker issues
                                let frame_infos: Vec<(usize, bool)> = (0..self.frames.len())
                                    .map(|i| (i, self.animation_playing && self.animation_frame == i))
                                    .collect();

                                egui::ScrollArea::horizontal().show(ui, |ui| {
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
                                                    // Preview of the frame
                                                    let preview_size = 80.0;
                                                    // Only borrow frame for preview
                                                    {
                                                        let frame = &self.frames[i];
                                                        let width = frame.layers[0].width();
                                                        let height = frame.layers[0].height();

                                                        // Calculate pixel size to fit within preview
                                                        let scale = (preview_size / width as f32)
                                                            .min(preview_size / height as f32);

                                                        ui.vertical(|ui| {
                                                            // Frame title and controls
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
                                                            
                                                            // Create a preview of the frame
                                                            let (rect, _) = ui.allocate_exact_size(
                                                                egui::vec2(width as f32 * scale, height as f32 * scale),
                                                                egui::Sense::hover(),
                                                            );

                                                            // Get the composed frame (all visible layers)
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

                                                            // Draw the preview
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

                                                            // Draw a frame around the preview
                                                            ui.painter().rect_stroke(
                                                                rect,
                                                                0.0,
                                                                egui::Stroke::new(1.0, ui.visuals().widgets.active.bg_stroke.color),
                                                                egui::epaint::StrokeKind::Middle,
                                                            );
                                                        });
                                                    }

                                                    // Frame controls (no borrow of frame here)
                                                    ui.horizontal(|ui| {
                                                        let btn_size = egui::vec2(18.0, 18.0);
                                                        
                                                        if self.frames.len() > 1 && ui.add(egui::Button::new("🗑").min_size(btn_size))
                                                            .on_hover_text("Delete Frame")
                                                            .clicked() {
                                                            self.push_undo();
                                                            self.frames.remove(i);
                                                            if self.current_frame >= self.frames.len() {
                                                                self.current_frame = self.frames.len() - 1;
                                                            }
                                                            self.current_layer = 0;
                                                        }

                                                        if ui.add(egui::Button::new("📋").min_size(btn_size))
                                                            .on_hover_text("Duplicate Frame")
                                                            .clicked() {
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
                                    if self.frames.len() < MAX_FRAMES && ui.button("➕ Add Frame").clicked() {
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

                                    if ui.button("⏮").on_hover_text("First Frame").clicked() && !self.frames.is_empty() {
                                        self.current_frame = 0;
                                        self.current_layer = 0;
                                    }

                                    if ui.button("⏭").on_hover_text("Last Frame").clicked() && !self.frames.is_empty() {
                                        self.current_frame = self.frames.len() - 1;
                                        self.current_layer = 0;
                                    }
                                    
                                    // Animation controls in frame panel
                                    if self.frames.len() > 1 {
                                        ui.separator();
                                        let play_icon = if self.animation_playing { "⏸" } else { "▶" };
                                        if ui.button(play_icon).on_hover_text("Play/Pause Animation").clicked() {
                                            self.animation_playing = !self.animation_playing;
                                            if self.animation_playing {
                                                self.animation_frame = 0;
                                                self.last_animation_time = ctx.input(|i| i.time);
                                            }
                                        }
                                        
                                        ui.add(egui::Slider::new(&mut self.animation_speed, 1.0..=30.0).text("FPS"));
                                        
                                        if self.animation_playing {
                                            ui.label(format!(
                                                "Frame: {}/{}",
                                                self.animation_frame + 1,
                                                self.frames.len()
                                            ));
                                        }
                                    }
                                });
                            });
                        }
                    });
                });
        }
        
        // Handle keyboard shortcuts
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::Z)) {
            self.undo();
        }
    }
}
