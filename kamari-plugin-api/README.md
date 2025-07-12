# Kamari Plugin API

The Kamari Plugin API provides a comprehensive framework for creating external plugins for the Kamari pixel art editor. This API allows developers to create powerful image processing filters, tools, and effects that can be loaded and executed within the editor.

## Features

- **Type-safe plugin development** with Rust's type system
- **Rich parameter system** supporting integers, floats, strings, booleans, and colors
- **Comprehensive canvas manipulation** with pixel-level access
- **Built-in image processing functions** (blur, noise, outline, etc.)
- **Easy-to-use macros** for common plugin patterns
- **Serialization support** for plugin configurations

## Quick Start

### Creating a Simple Plugin

```rust
use kamari_plugin_api::*;

pub struct MyPlugin;

impl KamariPlugin for MyPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "My First Plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "A simple example plugin".to_string(),
            author: "Your Name".to_string(),
            plugin_type: PluginType::Filter,
            parameters: vec![
                PluginParameter::Float("strength".to_string(), PluginParameterValue::Float(1.0)),
            ],
        }
    }
    
    fn execute(&self, context: &mut PluginContext, params: &[PluginParameter]) -> Result<(), String> {
        let strength = get_param!(params, "strength", Float);
        
        // Apply some effect to the canvas
        context.apply_blur(strength);
        
        Ok(())
    }
}
```

### Using Macros for Common Patterns

The API provides convenient macros for creating common plugin types:

```rust
use kamari_plugin_api::*;

// Create a simple blur plugin
blur_plugin!("My Blur");

// Create a noise plugin
noise_plugin!("My Noise");

// Create an outline plugin
outline_plugin!("My Outline");

// Create a color replace plugin
color_replace_plugin!("My Color Replace");
```

### Creating a Plugin with Custom Parameters

```rust
use kamari_plugin_api::*;

create_plugin_with_params!(
    "Advanced Filter",
    "1.0.0",
    "An advanced filter with multiple parameters",
    Filter,
    "intensity" => Float,
    "color" => Color,
    "enabled" => Boolean
);

impl KamariPlugin for Plugin {
    fn execute(&self, context: &mut PluginContext, params: &[PluginParameter]) -> Result<(), String> {
        let intensity = get_param!(params, "intensity", Float);
        let color = get_param!(params, "color", Color);
        let enabled = get_param!(params, "enabled", Boolean);
        
        if enabled {
            // Apply custom effect
            context.apply_noise(intensity);
            context.apply_outline(color, 1);
        }
        
        Ok(())
    }
}
```

## API Reference

### Core Types

#### PluginMetadata
Contains information about your plugin:
- `name`: Plugin display name
- `version`: Plugin version string
- `description`: Brief description of what the plugin does
- `author`: Plugin author name
- `plugin_type`: Type of plugin (Filter, Tool, Effect, etc.)
- `parameters`: List of configurable parameters

#### PluginParameter
Represents a configurable parameter:
- `Integer(name, value)`: Integer parameter
- `Float(name, value)`: Floating-point parameter
- `String(name, value)`: String parameter
- `Boolean(name, value)`: Boolean parameter
- `Color(name, value)`: Color parameter

#### PluginContext
Provides access to the canvas and manipulation functions:
- `get_pixel(x, y)`: Get pixel color at coordinates
- `set_pixel(x, y, color)`: Set pixel color at coordinates
- `width()`: Get canvas width
- `height()`: Get canvas height
- `apply_blur(radius)`: Apply blur effect
- `apply_noise(intensity)`: Apply noise effect
- `apply_outline(color, thickness)`: Apply outline effect
- `apply_pixelate(block_size)`: Apply pixelate effect
- `replace_color(from, to, tolerance)`: Replace colors
- `flood_fill(x, y, color)`: Bucket fill operation

#### Color
Represents an RGBA color:
- `new(r, g, b, a)`: Create new color
- `from_hex(hex_string)`: Create from hex string
- `to_rgba()`: Convert to RGBA tuple
- Constants: `BLACK`, `WHITE`, `RED`, `GREEN`, `BLUE`, `TRANSPARENT`

### Plugin Types

- `Filter`: Image processing filters (blur, sharpen, etc.)
- `Tool`: Drawing or editing tools
- `Effect`: Special effects and transformations
- `Import`: File import functionality
- `Export`: File export functionality
- `Utility`: General utility functions

### Macros

#### Parameter Extraction
```rust
// Extract parameters with type checking
let value = get_param!(params, "parameter_name", Type);
```

#### Plugin Creation
```rust
// Create simple plugin
create_plugin!("Name", "1.0.0", "Description", PluginType);

// Create plugin with parameters
create_plugin_with_params!("Name", "1.0.0", "Description", PluginType, 
    "param1" => Type1,
    "param2" => Type2
);
```

#### Specialized Plugins
```rust
blur_plugin!("My Blur");
noise_plugin!("My Noise");
outline_plugin!("My Outline");
color_replace_plugin!("My Color Replace");
```

## Building and Distribution

### Building Your Plugin

1. Create a new Rust library project:
```bash
cargo new --lib my_plugin
cd my_plugin
```

2. Add the Kamari Plugin API dependency to your `Cargo.toml`:
```toml
[dependencies]
kamari-plugin-api = "1.0"

[lib]
crate-type = ["cdylib"]
```

3. Implement your plugin in `src/lib.rs`

4. Build the plugin:
```bash
cargo build --release
```

### Plugin Manifest

Create a `plugin.json` file describing your plugin:
```json
{
    "name": "My Plugin",
    "version": "1.0.0",
    "description": "A sample plugin",
    "author": "Your Name",
    "main": "libmy_plugin.dll",
    "type": "filter",
    "parameters": [
        {
            "name": "strength",
            "type": "float",
            "default": 1.0,
            "min": 0.0,
            "max": 10.0
        }
    ]
}
```

## Examples

### Custom Filter Plugin

```rust
use kamari_plugin_api::*;

pub struct VintageFilter;

impl KamariPlugin for VintageFilter {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "Vintage Filter".to_string(),
            version: "1.0.0".to_string(),
            description: "Applies a vintage photo effect".to_string(),
            author: "Example Author".to_string(),
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
        let max_distance = ((center_x * center_x + center_y * center_y).sqrt());
        
        for y in 0..height {
            for x in 0..width {
                if let Some(pixel) = context.get_pixel(x, y) {
                    // Apply sepia effect
                    let r = pixel.r as f32;
                    let g = pixel.g as f32;
                    let b = pixel.b as f32;
                    
                    let sepia_r = (r * 0.393 + g * 0.769 + b * 0.189) * sepia_strength + r * (1.0 - sepia_strength);
                    let sepia_g = (r * 0.349 + g * 0.686 + b * 0.168) * sepia_strength + g * (1.0 - sepia_strength);
                    let sepia_b = (r * 0.272 + g * 0.534 + b * 0.131) * sepia_strength + b * (1.0 - sepia_strength);
                    
                    // Apply vignette effect
                    let distance = ((x as f32 - center_x).powi(2) + (y as f32 - center_y).powi(2)).sqrt();
                    let vignette_factor = 1.0 - (distance / max_distance) * vignette_strength;
                    
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
```

## License

This API is part of the Kamari pixel art editor project and is available under the same license terms.
