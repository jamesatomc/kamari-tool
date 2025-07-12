use serde::{Deserialize, Serialize};

/// Plugin metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub plugin_type: PluginType,
    pub parameters: Vec<PluginParameter>,
}

/// Plugin types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PluginType {
    Tool,
    Filter,
    Effect,
    Import,
    Export,
    Utility,
}

/// Plugin parameter values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginParameterValue {
    Integer(i32),
    Float(f32),
    String(String),
    Boolean(bool),
    Color(Color),
}

/// Plugin parameter types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginParameter {
    Integer(String, PluginParameterValue),
    Float(String, PluginParameterValue),
    String(String, PluginParameterValue),
    Boolean(String, PluginParameterValue),
    Color(String, PluginParameterValue),
}

impl PluginParameter {
    pub fn name(&self) -> &str {
        match self {
            PluginParameter::Integer(name, _) => name,
            PluginParameter::Float(name, _) => name,
            PluginParameter::String(name, _) => name,
            PluginParameter::Boolean(name, _) => name,
            PluginParameter::Color(name, _) => name,
        }
    }
    
    pub fn value(&self) -> &PluginParameterValue {
        match self {
            PluginParameter::Integer(_, value) => value,
            PluginParameter::Float(_, value) => value,
            PluginParameter::String(_, value) => value,
            PluginParameter::Boolean(_, value) => value,
            PluginParameter::Color(_, value) => value,
        }
    }
}

impl PluginParameterValue {
    pub fn as_integer(&self) -> Option<i32> {
        match self {
            PluginParameterValue::Integer(val) => Some(*val),
            _ => None,
        }
    }
    
    pub fn as_float(&self) -> Option<f32> {
        match self {
            PluginParameterValue::Float(val) => Some(*val),
            _ => None,
        }
    }
    
    pub fn as_string(&self) -> Option<&str> {
        match self {
            PluginParameterValue::String(val) => Some(val),
            _ => None,
        }
    }
    
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            PluginParameterValue::Boolean(val) => Some(*val),
            _ => None,
        }
    }
    
    pub fn as_color(&self) -> Option<Color> {
        match self {
            PluginParameterValue::Color(val) => Some(*val),
            _ => None,
        }
    }
}

/// Color representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };
    pub const RED: Color = Color { r: 255, g: 0, b: 0, a: 255 };
    pub const GREEN: Color = Color { r: 0, g: 255, b: 0, a: 255 };
    pub const BLUE: Color = Color { r: 0, g: 0, b: 255, a: 255 };
    pub const TRANSPARENT: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    
    pub fn to_rgba(&self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }
}

/// Canvas information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasInfo {
    pub width: u32,
    pub height: u32,
    pub scale: f32,
    pub background_color: Color,
}

/// Layer information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerInfo {
    pub current_layer: u32,
    pub layer_count: u32,
    pub layer_name: String,
    pub opacity: f32,
    pub blend_mode: String,
}

/// Plugin execution context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginContext {
    canvas_info: CanvasInfo,
    layer_info: LayerInfo,
    selected_color: Color,
    secondary_color: Color,
    pixel_data: Vec<Vec<Color>>,
}

impl PluginContext {
    pub fn new(
        canvas_info: CanvasInfo,
        layer_info: LayerInfo,
        selected_color: Color,
        secondary_color: Color,
        pixel_data: Vec<Vec<Color>>,
    ) -> Self {
        Self {
            canvas_info,
            layer_info,
            selected_color,
            secondary_color,
            pixel_data,
        }
    }
    
    /// Get the pixel data (read-only access)
    pub fn get_pixel_data(&self) -> &Vec<Vec<Color>> {
        &self.pixel_data
    }
    
    /// Set the entire pixel data
    pub fn set_pixel_data(&mut self, data: Vec<Vec<Color>>) {
        self.pixel_data = data;
    }
    
    /// Get canvas width
    pub fn width(&self) -> usize {
        self.canvas_info.width as usize
    }
    
    /// Get canvas height
    pub fn height(&self) -> usize {
        self.canvas_info.height as usize
    }
    
    /// Get pixel at position
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<Color> {
        self.pixel_data.get(y)?.get(x).copied()
    }
    
    /// Set pixel at position
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        if y < self.pixel_data.len() && x < self.pixel_data[y].len() {
            self.pixel_data[y][x] = color;
        }
    }
}
