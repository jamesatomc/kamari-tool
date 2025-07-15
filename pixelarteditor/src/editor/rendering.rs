use crate::editor::core::PixelArtEditor;
use crate::types::Frame;
use eframe::egui;

fn blend_colors(bg: egui::Color32, fg: egui::Color32) -> egui::Color32 {
    let bg_alpha = bg.a() as f32 / 255.0;
    let fg_alpha = fg.a() as f32 / 255.0;
    
    if fg_alpha == 0.0 {
        return bg;
    }
    
    let out_alpha = fg_alpha + bg_alpha * (1.0 - fg_alpha);
    
    if out_alpha == 0.0 {
        return egui::Color32::TRANSPARENT;
    }
    
    let r = ((fg.r() as f32 * fg_alpha + bg.r() as f32 * bg_alpha * (1.0 - fg_alpha)) / out_alpha) as u8;
    let g = ((fg.g() as f32 * fg_alpha + bg.g() as f32 * bg_alpha * (1.0 - fg_alpha)) / out_alpha) as u8;
    let b = ((fg.b() as f32 * fg_alpha + bg.b() as f32 * bg_alpha * (1.0 - fg_alpha)) / out_alpha) as u8;
    let a = (out_alpha * 255.0) as u8;
    
    egui::Color32::from_rgba_unmultiplied(r, g, b, a)
}

impl PixelArtEditor {
    pub fn get_composed_grid(&self) -> Vec<Vec<egui::Color32>> {
        // Use cache if available and not dirty
        if let Some(ref cache) = self.render_cache {
            if !self.cache_dirty {
                return cache.clone();
            }
        }

        let frame_idx = if self.animation_playing {
            self.animation_frame
        } else {
            self.current_frame
        };
        let frame = &self.frames[frame_idx];

        let width = frame.layers[0].width();
        let height = frame.layers[0].height();
        let mut composed = vec![vec![egui::Color32::TRANSPARENT; width]; height];

        // Add onion skinning if enabled
        if self.onion_skinning && !self.animation_playing {
            // Draw previous frames
            for i in 1..=self.onion_prev_frames {
                if frame_idx >= i {
                    let prev_frame = &self.frames[frame_idx - i];
                    let opacity = self.onion_opacity * (1.0 - (i as f32 * 0.2));
                    self.compose_frame_with_opacity(prev_frame, &mut composed, opacity, egui::Color32::BLUE);
                }
            }
            
            // Draw next frames
            for i in 1..=self.onion_next_frames {
                if frame_idx + i < self.frames.len() {
                    let next_frame = &self.frames[frame_idx + i];
                    let opacity = self.onion_opacity * (1.0 - (i as f32 * 0.2));
                    self.compose_frame_with_opacity(next_frame, &mut composed, opacity, egui::Color32::RED);
                }
            }
        }

        // Draw current frame
        for layer in &frame.layers {
            if !layer.visible {
                continue;
            }
            for y in 0..height {
                for x in 0..width {
                    let c = layer.grid[y][x];
                    if c.a() > 0 {
                        let new_color = if layer.opacity < 1.0 {
                            let alpha = (c.a() as f32 * layer.opacity) as u8;
                            egui::Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), alpha)
                        } else {
                            c
                        };

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

    fn compose_frame_with_opacity(&self, frame: &Frame, composed: &mut Vec<Vec<egui::Color32>>, opacity: f32, tint: egui::Color32) {
        let width = composed[0].len();
        let height = composed.len();
        
        for layer in &frame.layers {
            if !layer.visible {
                continue;
            }
            for y in 0..height {
                for x in 0..width {
                    let c = layer.grid[y][x];
                    if c.a() > 0 {
                        // Apply tint and opacity
                        let tinted = egui::Color32::from_rgba_unmultiplied(
                            ((c.r() as f32 * 0.7) + (tint.r() as f32 * 0.3)) as u8,
                            ((c.g() as f32 * 0.7) + (tint.g() as f32 * 0.3)) as u8,
                            ((c.b() as f32 * 0.7) + (tint.b() as f32 * 0.3)) as u8,
                            (c.a() as f32 * opacity) as u8
                        );
                        
                        if tinted.a() > 0 {
                            let bg = composed[y][x];
                            composed[y][x] = blend_colors(bg, tinted);
                        }
                    }
                }
            }
        }
    }
}
