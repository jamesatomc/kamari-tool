# Kamari Tool - Pixel Art Editor

A modern pixel art editor built with Rust and egui, featuring layers, frames, animation, and various drawing tools.

## Project Structure

The project has been refactored into a modular structure for better maintainability:

```
src/
├── lib.rs              # Main library file with module declarations
├── main.rs             # Entry point for the application
├── constants.rs        # Global constants and helper functions
├── types.rs            # Core data structures (Tool, Layer, Frame)
├── editor.rs           # Main PixelArtEditor struct and core methods
├── tools.rs            # Drawing tool implementations
├── file_operations.rs  # File I/O operations (save/load)
├── app.rs              # Main application loop and eframe::App implementation
└── ui/
    ├── mod.rs          # UI module declarations
    ├── menu.rs         # Menu bar and toolbar implementations
    ├── panels.rs       # Layers and frames panel implementations
    ├── color_panel.rs  # Color selection panel
    ├── dialogs.rs      # Dialog windows (new sprite, resize, etc.)
    └── canvas.rs       # Main canvas rendering and interaction
```

## Features

- **Multi-layer support**: Up to 64 layers with opacity and visibility controls
- **Animation**: Multi-frame animation support with up to 64 frames
- **Drawing tools**: Pencil, Eraser, Bucket fill, Eyedropper, and Move tools
- **Brush sizes**: Configurable brush sizes from 1-10 pixels
- **Color management**: Color palette and primary/secondary color selection
- **Zoom controls**: Zoom in/out with centered view
- **File operations**: Save as PNG with file dialog
- **Undo system**: Single-level undo for most operations
- **Grid display**: Toggle grid lines for pixel-perfect editing

## Building and Running

```bash
cargo run
```

## Controls

- **Left click**: Draw with selected tool
- **Right click**: Erase
- **Alt + Click**: Pick color (eyedropper)
- **Ctrl + Z**: Undo
- **Drag**: Continue drawing/erasing

## Tools

- 🖊 **Pencil**: Draw with selected color
- 🧽 **Eraser**: Make pixels transparent
- 🪣 **Bucket**: Fill connected area with color
- 👁 **Eyedropper**: Pick color from canvas
- ↔ **Move**: Move layer content around

## Architecture

The refactored code follows a clean separation of concerns:

- **Core logic** is in `editor.rs` and `types.rs`
- **UI components** are modularized in the `ui/` directory
- **File operations** are isolated in `file_operations.rs`
- **Tool implementations** are in `tools.rs`
- **Constants and utilities** are in `constants.rs`

This structure makes it easy to:
- Add new tools or features
- Modify UI components independently
- Maintain and debug the code
- Write tests for specific modules
- Extend functionality without affecting other parts

## Dependencies

- `eframe` and `egui`: GUI framework
- `image`: Image processing and PNG export
- `rfd`: File dialog support
