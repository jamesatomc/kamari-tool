use eframe::egui;
use std::time::Instant;
use rand::Rng;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Clone, Copy, PartialEq)]
pub enum AnimationType {
    None,
    Pulse,
    Bounce,
    Rotate,
    Scale,
    Sparkle,
    Glow,
    Fade,
    Shake,
    Wobble,
}

#[derive(Clone)]
pub struct ToolAnimation {
    pub tool: Tool,
    pub animation_type: AnimationType,
    pub duration: f32,
    pub start_time: Option<Instant>,
    pub is_active: bool,
    pub intensity: f32,
    pub color: egui::Color32,
    pub particles: Vec<Particle>,
}

#[derive(Clone)]
pub struct Particle {
    pub position: egui::Vec2,
    pub velocity: egui::Vec2,
    pub color: egui::Color32,
    pub size: f32,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub alpha: f32,
}

impl Default for ToolAnimation {
    fn default() -> Self {
        Self {
            tool: Tool::Pencil,
            animation_type: AnimationType::None,
            duration: 1.0,
            start_time: None,
            is_active: false,
            intensity: 1.0,
            color: egui::Color32::WHITE,
            particles: Vec::new(),
        }
    }
}

impl ToolAnimation {
    pub fn new(tool: Tool, animation_type: AnimationType, duration: f32) -> Self {
        Self {
            tool,
            animation_type,
            duration,
            start_time: None,
            is_active: false,
            intensity: 1.0,
            color: egui::Color32::WHITE,
            particles: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.is_active = true;
    }

    pub fn stop(&mut self) {
        self.is_active = false;
        self.start_time = None;
        self.particles.clear();
    }

    pub fn get_progress(&self) -> f32 {
        if let Some(start_time) = self.start_time {
            let elapsed = start_time.elapsed().as_secs_f32();
            (elapsed / self.duration).min(1.0)
        } else {
            0.0
        }
    }

    pub fn is_finished(&self) -> bool {
        self.get_progress() >= 1.0
    }

    pub fn update(&mut self, dt: f32) {
        if !self.is_active {
            return;
        }

        // Update particles
        self.particles.retain_mut(|particle| {
            particle.position += particle.velocity * dt;
            particle.lifetime -= dt;
            particle.alpha = (particle.lifetime / particle.max_lifetime).max(0.0);
            particle.lifetime > 0.0
        });

        // Generate new particles based on animation type
        match self.animation_type {
            AnimationType::Sparkle => {                    if rand::random::<f32>() < 0.1 {
                    self.particles.push(Particle {
                        position: egui::Vec2::new(
                            rand::random::<f32>() * 100.0,
                            rand::random::<f32>() * 100.0,
                        ),
                        velocity: egui::Vec2::new(
                            (rand::random::<f32>() - 0.5) * 50.0,
                            (rand::random::<f32>() - 0.5) * 50.0,
                        ),
                        color: self.color,
                        size: rand::random::<f32>() * 3.0 + 1.0,
                        lifetime: 1.0,
                        max_lifetime: 1.0,
                        alpha: 1.0,
                    });
                }
            }
            _ => {}
        }

        if self.is_finished() {
            self.stop();
        }
    }
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
