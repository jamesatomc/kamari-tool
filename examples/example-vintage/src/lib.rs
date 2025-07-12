use kamari_plugin_api::*;

pub struct VintageFilter;

impl KamariPlugin for VintageFilter {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "Vintage Filter".to_string(),
            version: "1.0.0".to_string(),
            description: "Applies a vintage photo effect with sepia and vignette".to_string(),
            author: "Kamari Team".to_string(),
            plugin_type: PluginType::Filter,
            parameters: vec![
                PluginParameter::Float("sepia_strength".to_string(), PluginParameterValue::Float(0.5)),
                PluginParameter::Float("vignette_strength".to_string(), PluginParameterValue::Float(0.3)),
            ],
        }
    }
    
    fn execute(&self, context: &mut PluginContext, params: &[PluginParameter]) -> Result<(), String> {
        let sepia_strength = get_param!(params, "sepia_strength", Float);
        let vignette_strength = get_param!(params, "vignette_strength", Float);
        
        let width = context.width();
        let height = context.height();
        let center_x = width as f32 / 2.0;
        let center_y = height as f32 / 2.0;
        let max_distance = (center_x * center_x + center_y * center_y).sqrt();
        
        for y in 0..height {
            for x in 0..width {
                if let Some(pixel) = context.get_pixel(x, y) {
                    // Skip transparent pixels
                    if pixel.a == 0 {
                        continue;
                    }
                    
                    // Apply sepia effect
                    let r = pixel.r as f32;
                    let g = pixel.g as f32;
                    let b = pixel.b as f32;
                    
                    let sepia_r = (r * 0.393 + g * 0.769 + b * 0.189) * sepia_strength + r * (1.0 - sepia_strength);
                    let sepia_g = (r * 0.349 + g * 0.686 + b * 0.168) * sepia_strength + g * (1.0 - sepia_strength);
                    let sepia_b = (r * 0.272 + g * 0.534 + b * 0.131) * sepia_strength + b * (1.0 - sepia_strength);
                    
                    // Apply vignette effect
                    let distance = ((x as f32 - center_x).powi(2) + (y as f32 - center_y).powi(2)).sqrt();
                    let vignette_factor = (1.0 - (distance / max_distance) * vignette_strength).max(0.0);
                    
                    let final_r = (sepia_r * vignette_factor).clamp(0.0, 255.0) as u8;
                    let final_g = (sepia_g * vignette_factor).clamp(0.0, 255.0) as u8;
                    let final_b = (sepia_b * vignette_factor).clamp(0.0, 255.0) as u8;
                    
                    context.set_pixel(x, y, Color::new(final_r, final_g, final_b, pixel.a));
                }
            }
        }
        
        Ok(())
    }
}
