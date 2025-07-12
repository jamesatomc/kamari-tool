/// Macros for easier plugin development
///
/// These macros help reduce boilerplate code when creating plugins

/// Macro to create a simple plugin with basic metadata
#[macro_export]
macro_rules! create_plugin {
    ($name:expr, $version:expr, $description:expr, $plugin_type:ident) => {
        use kamari_plugin_api::*;
        
        pub struct Plugin;
        
        impl KamariPlugin for Plugin {
            fn metadata(&self) -> PluginMetadata {
                PluginMetadata {
                    name: $name.to_string(),
                    version: $version.to_string(),
                    description: $description.to_string(),
                    author: "Anonymous".to_string(),
                    plugin_type: PluginType::$plugin_type,
                    parameters: vec![],
                }
            }
            
            fn execute(&self, context: &mut PluginContext, _params: &[PluginParameter]) -> Result<(), String> {
                // Default implementation - override in your plugin
                Ok(())
            }
        }
    };
}

/// Macro to create a plugin with parameters
#[macro_export]
macro_rules! create_plugin_with_params {
    ($name:expr, $version:expr, $description:expr, $plugin_type:ident, $($param_name:expr => $param_type:ident),*) => {
        use kamari_plugin_api::*;
        
        pub struct Plugin;
        
        impl KamariPlugin for Plugin {
            fn metadata(&self) -> PluginMetadata {
                PluginMetadata {
                    name: $name.to_string(),
                    version: $version.to_string(),
                    description: $description.to_string(),
                    author: "Anonymous".to_string(),
                    plugin_type: PluginType::$plugin_type,
                    parameters: vec![
                        $(
                            PluginParameter::$param_type(
                                $param_name.to_string(),
                                match stringify!($param_type) {
                                    "Integer" => PluginParameterValue::Integer(0),
                                    "Float" => PluginParameterValue::Float(0.0),
                                    "String" => PluginParameterValue::String("".to_string()),
                                    "Boolean" => PluginParameterValue::Boolean(false),
                                    "Color" => PluginParameterValue::Color(Color::BLACK),
                                    _ => PluginParameterValue::String("".to_string()),
                                }
                            ),
                        )*
                    ],
                }
            }
            
            fn execute(&self, context: &mut PluginContext, _params: &[PluginParameter]) -> Result<(), String> {
                // Default implementation - override in your plugin
                Ok(())
            }
        }
    };
}

/// Macro to easily get parameter values
#[macro_export]
macro_rules! get_param {
    ($params:expr, $name:expr, Integer) => {
        $params
            .iter()
            .find(|p| p.name() == $name)
            .and_then(|p| p.value().as_integer())
            .ok_or_else(|| format!("Parameter '{}' not found or not an integer", $name))?
    };
    ($params:expr, $name:expr, Float) => {
        $params
            .iter()
            .find(|p| p.name() == $name)
            .and_then(|p| p.value().as_float())
            .ok_or_else(|| format!("Parameter '{}' not found or not a float", $name))?
    };
    ($params:expr, $name:expr, String) => {
        $params
            .iter()
            .find(|p| p.name() == $name)
            .and_then(|p| p.value().as_string())
            .ok_or_else(|| format!("Parameter '{}' not found or not a string", $name))?
    };
    ($params:expr, $name:expr, Boolean) => {
        $params
            .iter()
            .find(|p| p.name() == $name)
            .and_then(|p| p.value().as_boolean())
            .ok_or_else(|| format!("Parameter '{}' not found or not a boolean", $name))?
    };
    ($params:expr, $name:expr, Color) => {
        $params
            .iter()
            .find(|p| p.name() == $name)
            .and_then(|p| p.value().as_color())
            .ok_or_else(|| format!("Parameter '{}' not found or not a color", $name))?
    };
}

/// Macro to create a simple blur plugin
#[macro_export]
macro_rules! blur_plugin {
    ($name:expr) => {
        use kamari_plugin_api::*;
        
        pub struct Plugin;
        
        impl KamariPlugin for Plugin {
            fn metadata(&self) -> PluginMetadata {
                PluginMetadata {
                    name: $name.to_string(),
                    version: "1.0.0".to_string(),
                    description: "Applies a blur effect to the canvas".to_string(),
                    author: "Kamari".to_string(),
                    plugin_type: PluginType::Filter,
                    parameters: vec![
                        PluginParameter::Float("radius".to_string(), PluginParameterValue::Float(2.0)),
                    ],
                }
            }
            
            fn execute(&self, context: &mut PluginContext, params: &[PluginParameter]) -> Result<(), String> {
                let radius = get_param!(params, "radius", Float);
                context.apply_blur(radius);
                Ok(())
            }
        }
    };
}

/// Macro to create a simple noise plugin
#[macro_export]
macro_rules! noise_plugin {
    ($name:expr) => {
        use kamari_plugin_api::*;
        
        pub struct Plugin;
        
        impl KamariPlugin for Plugin {
            fn metadata(&self) -> PluginMetadata {
                PluginMetadata {
                    name: $name.to_string(),
                    version: "1.0.0".to_string(),
                    description: "Adds noise to the canvas".to_string(),
                    author: "Kamari".to_string(),
                    plugin_type: PluginType::Filter,
                    parameters: vec![
                        PluginParameter::Float("intensity".to_string(), PluginParameterValue::Float(0.1)),
                    ],
                }
            }
            
            fn execute(&self, context: &mut PluginContext, params: &[PluginParameter]) -> Result<(), String> {
                let intensity = get_param!(params, "intensity", Float);
                context.apply_noise(intensity);
                Ok(())
            }
        }
    };
}

/// Macro to create a simple outline plugin
#[macro_export]
macro_rules! outline_plugin {
    ($name:expr) => {
        use kamari_plugin_api::*;
        
        pub struct Plugin;
        
        impl KamariPlugin for Plugin {
            fn metadata(&self) -> PluginMetadata {
                PluginMetadata {
                    name: $name.to_string(),
                    version: "1.0.0".to_string(),
                    description: "Adds an outline effect to the canvas".to_string(),
                    author: "Kamari".to_string(),
                    plugin_type: PluginType::Filter,
                    parameters: vec![
                        PluginParameter::Color("color".to_string(), PluginParameterValue::Color(Color::BLACK)),
                        PluginParameter::Integer("thickness".to_string(), PluginParameterValue::Integer(1)),
                    ],
                }
            }
            
            fn execute(&self, context: &mut PluginContext, params: &[PluginParameter]) -> Result<(), String> {
                let color = get_param!(params, "color", Color);
                let thickness = get_param!(params, "thickness", Integer) as usize;
                context.apply_outline(color, thickness);
                Ok(())
            }
        }
    };
}

/// Macro to create a color replace plugin
#[macro_export]
macro_rules! color_replace_plugin {
    ($name:expr) => {
        use kamari_plugin_api::*;
        
        pub struct Plugin;
        
        impl KamariPlugin for Plugin {
            fn metadata(&self) -> PluginMetadata {
                PluginMetadata {
                    name: $name.to_string(),
                    version: "1.0.0".to_string(),
                    description: "Replaces one color with another".to_string(),
                    author: "Kamari".to_string(),
                    plugin_type: PluginType::Filter,
                    parameters: vec![
                        PluginParameter::Color("from_color".to_string(), PluginParameterValue::Color(Color::WHITE)),
                        PluginParameter::Color("to_color".to_string(), PluginParameterValue::Color(Color::BLACK)),
                        PluginParameter::Integer("tolerance".to_string(), PluginParameterValue::Integer(0)),
                    ],
                }
            }
            
            fn execute(&self, context: &mut PluginContext, params: &[PluginParameter]) -> Result<(), String> {
                let from_color = get_param!(params, "from_color", Color);
                let to_color = get_param!(params, "to_color", Color);
                let tolerance = get_param!(params, "tolerance", Integer) as u8;
                context.replace_color(from_color, to_color, tolerance);
                Ok(())
            }
        }
    };
}
