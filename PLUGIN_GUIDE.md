# Kamari Tool Plugin System

## Overview
Kamari Tool supports Aseprite-compatible plugins and provides a flexible plugin system for extending the editor's functionality.

## Plugin Types

### 1. Built-in Plugins
- **Blur**: Apply blur effects to layers
- **Noise**: Add random noise to images
- **Outline**: Create outlines around sprites
- **Pixelate**: Apply pixelation effects
- **Color Replace**: Replace specific colors in images

### 2. External Plugins
- Aseprite-compatible plugins (ZIP format)
- Custom plugins following Kamari plugin API

## Plugin Installation

### Method 1: Plugin Manager
1. Open Kamari Tool
2. Go to **Plugins** → **Plugin Manager**
3. Click **Install Plugin**
4. Select your plugin ZIP file
5. Click **Install**

### Method 2: Manual Installation
1. Create a folder in your plugins directory:
   - Windows: `%APPDATA%\kamari-tool\plugins\`
   - macOS: `~/Library/Application Support/kamari-tool/plugins/`
   - Linux: `~/.config/kamari-tool/plugins/`
2. Extract your plugin files into a subfolder
3. Restart Kamari Tool

## Plugin Directory Structure

```
plugins/
├── your-plugin-name/
│   ├── manifest.json
│   ├── plugin.js (or plugin.lua)
│   └── icons/
│       └── icon.png
```

## Plugin Manifest Format

```json
{
  "name": "My Plugin",
  "version": "1.0.0",
  "author": "Your Name",
  "description": "Description of what your plugin does",
  "category": "Filter",
  "aseprite_version": "1.0",
  "entry_point": "plugin.js"
}
```

## Plugin Categories

- **Tool**: Drawing tools and utilities
- **Filter**: Image processing filters
- **Animation**: Animation-related functionality
- **Import**: File import capabilities
- **Export**: File export capabilities
- **Utility**: General utility functions

## Using Built-in Plugins

### Blur Plugin
1. Select a layer
2. Go to **Plugins** → **Blur**
3. Adjust the radius parameter
4. Click **Apply**

### Noise Plugin
1. Select a layer
2. Go to **Plugins** → **Add Noise**
3. Adjust the intensity parameter
4. Click **Apply**

### Outline Plugin
1. Select a layer with a sprite
2. Go to **Plugins** → **Add Outline**
3. Choose outline color and thickness
4. Click **Apply**

### Pixelate Plugin
1. Select a layer
2. Go to **Plugins** → **Pixelate**
3. Adjust the block size
4. Click **Apply**

### Color Replace Plugin
1. Select a layer
2. Go to **Plugins** → **Replace Color**
3. Choose the "From" color and "To" color
4. Adjust tolerance if needed
5. Click **Apply**

## Plugin Development

### Creating a Plugin
1. Create a new directory for your plugin
2. Create a `manifest.json` file with plugin metadata
3. Implement your plugin using the Kamari Plugin API
4. Test your plugin in Kamari Tool
5. Package as a ZIP file for distribution

### Plugin API Reference
The plugin system provides access to:
- Current layer and canvas
- Color selection
- Undo/redo system
- UI components
- File operations

## Aseprite Compatibility

Kamari Tool aims to support Aseprite plugins where possible. Currently supported features:
- Basic filter operations
- Layer manipulation
- Color operations
- Simple UI dialogs

## Troubleshooting

### Plugin Not Loading
- Check plugin manifest syntax
- Verify plugin directory structure
- Check for script errors in logs

### Plugin Crashes
- Ensure plugin is compatible with current Kamari Tool version
- Check for missing dependencies
- Review plugin code for errors

## Examples

See the `examples/` directory for sample plugins and templates.

## Contributing

To contribute to the plugin system:
1. Fork the repository
2. Create a feature branch
3. Implement your changes
4. Add tests
5. Submit a pull request

## License

Plugin system is part of Kamari Tool and follows the same license terms.
