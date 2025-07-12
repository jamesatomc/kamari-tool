// Re-export main types and editor for public use
pub use editor::PixelArtEditor;
pub use types::{Tool, Layer, Frame};
pub use constants::*;

// Module declarations
mod constants;
mod types;
mod editor;
mod tools;
mod file_operations;
mod app;
mod ui;
pub mod plugins;
