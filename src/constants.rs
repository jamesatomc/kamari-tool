use eframe::egui;

pub const PIXEL_SIZE: f32 = 24.0;
pub const MAX_LAYERS: usize = 64;
pub const MAX_FRAMES: usize = 64;

pub fn get_default_palette() -> Vec<egui::Color32> {
    vec![
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
    ]
}

// Helper function to blend colors with transparency
pub fn blend_colors(bg: egui::Color32, fg: egui::Color32) -> egui::Color32 {
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
