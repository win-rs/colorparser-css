#![allow(dead_code)]
use core::fmt;
#[cfg(feature = "fast-hash")]
use fx_hash::FxHashMap as HashMap;
use serde_jsonc2::Value;
use serde_jsonc2::from_str;
#[cfg(feature = "theme_yml")]
use serde_yml::{Value as YamlValue, from_str as from_yml_str, to_string as yml_to_string};
#[cfg(not(feature = "fast-hash"))]
use std::collections::HashMap;
use std::ops::Deref;
use std::ops::DerefMut;

/// Represents a color as a string.
#[derive(Clone, Debug)]
pub struct Color(String);

impl Color {
    /// Constructs a new `Color` from a string.
    ///
    /// # Arguments
    ///
    /// * `s` - A string representing the color.
    ///
    /// # Returns
    ///
    /// A new `Color` instance.
    pub fn new(s: String) -> Self {
        Color(s)
    }

    /// Returns a reference to the inner string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0) // Use the inner String value
    }
}

impl PartialEq<&str> for Color {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<str> for Color {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<Color> for Color {
    fn eq(&self, other: &Color) -> bool {
        self.0 == other.0
    }
}

impl Deref for Color {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Debug)]
pub enum ThemeValue {
    Color(Color),
    Subtheme(Theme), // A nested theme itself
}

impl fmt::Display for ThemeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThemeValue::Color(color) => write!(f, "{}", color), // Print just the inner Color value
            ThemeValue::Subtheme(theme) => write!(f, "Subtheme({})", theme), // Print the inner theme in display format
        }
    }
}

impl ThemeValue {
    /// Converts a `serde_jsonc2::Value` into a `ThemeValue`.
    ///
    /// # Arguments
    ///
    /// * `value` - A `serde_jsonc2::Value` to convert.
    ///
    /// # Returns
    ///
    /// A `ThemeValue` representing the input value.
    pub fn from_json(value: Value) -> Self {
        match value {
            Value::Object(map) => {
                let nested = map
                    .into_iter()
                    .map(|(key, value)| (key, ThemeValue::from_json(value)))
                    .collect::<HashMap<String, ThemeValue>>();
                ThemeValue::Subtheme(Theme(nested))
            }
            Value::String(s) => ThemeValue::Color(Color::new(s)),
            _ => ThemeValue::Color(Color::new(value.to_string())),
        }
    }

    /// Converts a `serde_yml::Value` into a `ThemeValue`.
    ///
    /// # Arguments
    ///
    /// * `value` - A `serde_yml::Value` to convert.
    ///
    /// # Returns
    ///
    /// A `ThemeValue` representing the input value.
    #[cfg(feature = "theme_yml")]
    pub fn from_yaml(value: YamlValue) -> Self {
        match value {
            YamlValue::Mapping(map) => {
                let nested = map
                    .into_iter()
                    .map(|(key, value): (YamlValue, YamlValue)| {
                        // Convert YamlValue::String into a regular string and handle nested values
                        let key = key.as_str().unwrap_or_default().to_string();
                        (key, ThemeValue::from_yaml(value))
                    })
                    .collect::<HashMap<String, ThemeValue>>();
                ThemeValue::Subtheme(Theme(nested))
            }
            YamlValue::String(s) => ThemeValue::Color(Color::new(s)),
            // Handle other types (like booleans, integers, etc.)
            _ => ThemeValue::Color(Color::new(yml_to_string(&value).unwrap_or_default())),
        }
    }
}

/// Represents a theme, which is a collection of key-value pairs.
#[derive(Debug, Clone)]
pub struct Theme(HashMap<String, ThemeValue>);

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ ")?;
        let mut iter = self.0.iter();
        if let Some((key, value)) = iter.next() {
            write!(f, "{}: {}", key, value)?;
            for (key, value) in iter {
                write!(f, ", {}: {}", key, value)?;
            }
        }
        write!(f, " }}")
    }
}

impl Theme {
    /// Retrieves a value by its key.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice representing the key.
    ///
    /// # Returns
    ///
    /// An `Option` containing the `ThemeValue` associated with the key.
    pub fn get(&self, key: &str) -> Option<ThemeValue> {
        let mut current_map = &self.0;

        let parts: Vec<&str> = key.split('.').collect(); // Split the key into parts

        for (index, part) in parts.iter().enumerate() {
            match current_map.get(*part) {
                Some(ThemeValue::Subtheme(subtheme)) => {
                    // If the current part is a Subtheme, update the current map to the subtheme
                    current_map = &subtheme.0;

                    // If we're at the last part, return the subtheme itself
                    if index == parts.len() - 1 {
                        return Some(ThemeValue::Subtheme(subtheme.clone())); // Return a cloned Subtheme
                    }
                }
                Some(value) => {
                    // If the current part matches a value and it's not a Subtheme, return it
                    if index == parts.len() - 1 {
                        return Some(value.clone()); // Clone the value before returning it
                    }
                }
                None => return None, // If any part is not found, return None
            }
        }

        None // Return None if we couldn't resolve the entire key path
    }

    /// Retrieves a color by its key.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice representing the key.
    ///
    /// # Returns
    ///
    /// An `Option` containing the `Color` associated with the key.
    pub fn get_color(&self, key: &str) -> Option<Color> {
        self.get(key).and_then(|theme_value| {
            if let ThemeValue::Color(color) = theme_value {
                Some(color)
            } else {
                None
            }
        })
    }

    /// Parses a theme configuration from a string.
    ///
    /// # Arguments
    ///
    /// * `contents` - A string slice containing the theme data in either YAML or JSON format.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `Theme` or an error. The function will first attempt to parse the contents as YAML (if the `theme_yml` feature is enabled),
    /// and fallback to JSON parsing if YAML parsing fails or is not supported.
    pub fn parse_theme(contents: &str) -> Result<Theme, Box<dyn std::error::Error>> {
        // Try to parse as YAML if the `theme_yml` feature is enabled
        #[cfg(feature = "theme_yml")]
        if let Ok(yaml_value) = from_yml_str(contents) {
            if let YamlValue::Mapping(_) = &yaml_value {
                let theme = ThemeValue::from_yaml(yaml_value);
                if let ThemeValue::Subtheme(theme_map) = theme {
                    return Ok(theme_map);
                } else {
                    return Err("invalid root type: expected a mapping.".into());
                }
            } else {
                return Err("invalid root type: expected a mapping.".into());
            }
        }

        let value: Value = from_str(contents)?;

        if let Value::Object(_) = &value {
            let theme = ThemeValue::from_json(value); // Convert JSON into ThemeValue
            if let ThemeValue::Subtheme(theme_map) = theme {
                Ok(theme_map) // Return the parsed theme
            } else {
                Err("invalid root type: expected an object.".into()) // Error for unexpected root type
            }
        } else {
            Err("invalid root type: expected an object.".into()) // Error if the root is not an object
        }
    }

    // Recursive function to collect all keys, including nested ones
    pub fn keys(&self) -> Vec<String> {
        let mut keys = Vec::new();
        self.collect_keys_recursive(&self.0, &mut keys, String::new());
        keys
    }

    // Helper function to recursively collect keys
    fn collect_keys_recursive(
        &self,
        current_map: &HashMap<String, ThemeValue>,
        keys: &mut Vec<String>,
        prefix: String,
    ) {
        for (key, value) in current_map {
            let new_key = if prefix.is_empty() {
                key.clone()
            } else {
                format!("{}.{}", prefix, key)
            };
            keys.push(new_key.clone());

            if let ThemeValue::Subtheme(subtheme) = value {
                subtheme.collect_keys_recursive(&subtheme.0, keys, new_key);
            }
        }
    }

    // Method to collect only color keys
    pub fn colors(&self) -> Vec<String> {
        let mut colors = Vec::new();
        self.collect_colors_recursive(&self.0, &mut colors, String::new());
        colors
    }

    // Helper function to recursively collect color keys
    fn collect_colors_recursive(
        &self,
        current_map: &HashMap<String, ThemeValue>,
        colors: &mut Vec<String>,
        prefix: String,
    ) {
        for (key, value) in current_map {
            let new_key = if prefix.is_empty() {
                key.clone()
            } else {
                format!("{}.{}", prefix, key)
            };

            match value {
                // If the value is a color (i.e., string), add it to the colors list
                ThemeValue::Color(_) => colors.push(new_key),
                // If the value is a subtheme, recursively collect from the subtheme
                ThemeValue::Subtheme(subtheme) => {
                    subtheme.collect_colors_recursive(&subtheme.0, colors, new_key);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const JSON_DATA: &str = r###"
    {
        "blue": "#89b4fa",
        "text": {
            "white": "#cdd6f4",
            "dark": {
                "grey": "#313244"
            }
        },
        "red": "#f38ba8"
    }
    "###;

    #[cfg(feature = "theme_yml")]
    const YAML_DATA: &str = r###"
    blue: "#89b4fa"
    text:
        white: "#cdd6f4"
        dark:
            grey: "#313244"
    red: "#f38ba8"
    "###;

    #[test]
    fn test_get_color() {
        let theme = Theme::parse_theme(JSON_DATA).unwrap();

        // Test getting the "blue" color
        if let Some(ThemeValue::Color(color)) = theme.get("blue") {
            assert_eq!(color.as_str(), "#89b4fa");
        } else {
            panic!("Failed to get the blue color");
        }

        // Test getting the "red" color
        if let Some(ThemeValue::Color(color)) = theme.get("red") {
            assert_eq!(color.as_str(), "#f38ba8");
        } else {
            panic!("Failed to get the red color");
        }

        // Test for non-existent key
        assert!(theme.get("yellow").is_none());
    }

    #[test]
    #[cfg(feature = "theme_yml")]
    fn test_get_color_yml() {
        let theme = Theme::parse_theme(YAML_DATA).unwrap();

        println!("{:?}", theme);

        // Test getting the "blue" color
        if let Some(ThemeValue::Color(color)) = theme.get("blue") {
            assert_eq!(color.as_str(), "#89b4fa");
        } else {
            panic!("Failed to get the blue color");
        }

        // Test getting the "red" color
        if let Some(ThemeValue::Color(color)) = theme.get("red") {
            assert_eq!(color.as_str(), "#f38ba8");
        } else {
            panic!("Failed to get the red color");
        }

        // Test for non-existent key
        assert!(theme.get("yellow").is_none());
    }

    #[test]
    fn test_get_subtheme() {
        let theme = Theme::parse_theme(JSON_DATA).unwrap();

        // Test getting the "text" subtheme
        if let Some(ThemeValue::Subtheme(subtheme)) = theme.get("text") {
            assert!(subtheme.get("white").is_some()); // The "white" color should exist within "text"
        } else {
            panic!("Failed to get the text subtheme");
        }

        // Test getting the "text.dark" subtheme
        if let Some(ThemeValue::Subtheme(subtheme)) = theme.get("text.dark") {
            assert!(subtheme.get("grey").is_some()); // The "grey" color should exist within "text.dark"
        } else {
            panic!("Failed to get the text.dark subtheme");
        }

        // Test for non-existent key in subtheme
        assert!(theme.get("text.dark.notfound").is_none());
    }

    #[test]
    fn test_get_nested_subtheme() {
        let theme = Theme::parse_theme(JSON_DATA).unwrap();

        // Test getting the nested subtheme "text.dark"
        if let Some(ThemeValue::Subtheme(subtheme)) = theme.get("text.dark") {
            if let Some(ThemeValue::Color(color)) = subtheme.get("grey") {
                assert_eq!(color.as_str(), "#313244");
            } else {
                panic!("Failed to get grey color from text.dark");
            }
        } else {
            panic!("Failed to get the text.dark subtheme");
        }
    }

    #[test]
    fn test_get_non_existent_key() {
        let theme = Theme::parse_theme(JSON_DATA).unwrap();

        // Test for non-existent key
        assert!(theme.get("green").is_none());
    }
}
