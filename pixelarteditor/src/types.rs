use eframe::egui;

#[derive(Clone, Copy, PartialEq)]
pub enum Tool {
    Pencil,
    Eraser,
    Bucket,
    Eyedropper,
    Move,
    Line,
    Rectangle,
    Circle,
    Select,
    Lasso,
    Spray,
    Dither,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ExportFormat {
    PNG,
    JPG,
    JPEG,
    BMP,
    GIF,
    ICO,
    PCX,
    POC,
    QOI,
    SVG,
    TGA,
    WEBP,
    ASE,
    ASEPRITE,
    CSS,
    FLC,
    FLI,
}

#[derive(Clone)]
pub struct Layer {
    pub name: String,
    pub visible: bool,
    pub opacity: f32,
    pub grid: Vec<Vec<egui::Color32>>,
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
    
    pub fn width(&self) -> usize {
        self.grid.first().map_or(0, |row| row.len())
    }
    
    pub fn height(&self) -> usize {
        self.grid.len()
    }
}

impl Default for Layer {
    fn default() -> Self {
        Self::new("Layer 1".to_string(), 16, 16, egui::Color32::TRANSPARENT)
    }
}

#[derive(Clone)]
pub struct Frame {
    pub layers: Vec<Layer>,
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            layers: vec![Layer::default()],
        }
    }
}
