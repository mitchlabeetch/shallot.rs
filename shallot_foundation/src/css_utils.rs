//! CSS Utilities - Helper functions for CSS generation and manipulation
//!
//! This module provides:
//! - CSS variable utilities
//! - Class name utilities
//! - Style merging helpers
//! - CSS validation utilities

use std::collections::HashMap;

/// A utility for building CSS class names conditionally
#[derive(Debug, Clone, Default)]
pub struct ClassBuilder {
    classes: Vec<String>,
}

impl ClassBuilder {
    /// Create a new class builder
    pub fn new() -> Self {
        Self { classes: Vec::new() }
    }

    /// Add a class if the condition is true
    pub fn add_if(mut self, class: impl Into<String>, condition: bool) -> Self {
        if condition {
            self.classes.push(class.into());
        }
        self
    }

    /// Add a class unconditionally
    pub fn add(mut self, class: impl Into<String>) -> Self {
        self.classes.push(class.into());
        self
    }

    /// Add a class with a value (for BEM modifiers)
    pub fn add_with_value(mut self, base: &str, value: Option<impl Into<String>>) -> Self {
        if let Some(v) = value {
            self.classes.push(format!("{}--{}", base, v.into()));
        }
        self
    }

    /// Add multiple classes from a vector
    pub fn add_many(mut self, classes: Vec<impl Into<String>>) -> Self {
        for class in classes {
            self.classes.push(class.into());
        }
        self
    }

    /// Build the final class string
    pub fn build(&self) -> String {
        self.classes.join(" ")
    }

    /// Check if any classes have been added
    pub fn is_empty(&self) -> bool {
        self.classes.is_empty()
    }

    /// Get the number of classes
    pub fn len(&self) -> usize {
        self.classes.len()
    }
}

/// A utility for building inline style attributes
#[derive(Debug, Clone, Default)]
pub struct StyleBuilder {
    styles: HashMap<String, String>,
}

impl StyleBuilder {
    /// Create a new style builder
    pub fn new() -> Self {
        Self {
            styles: HashMap::new(),
        }
    }

    /// Add a style property
    pub fn property(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.styles.insert(name.into(), value.into());
        self
    }

    /// Add a style property conditionally
    pub fn property_if(
        mut self,
        name: impl Into<String>,
        value: impl Into<String>,
        condition: bool,
    ) -> Self {
        if condition {
            self.styles.insert(name.into(), value.into());
        }
        self
    }

    /// Add a CSS variable
    pub fn var(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        let name_str = name.into();
        let var_name = if name_str.starts_with("--") {
            name_str
        } else {
            format!("--{}", name_str)
        };
        self.styles.insert(var_name, value.into());
        self
    }

    /// Add multiple styles from a hashmap
    pub fn extend(mut self, styles: HashMap<String, String>) -> Self {
        self.styles.extend(styles);
        self
    }

    /// Build the final style string
    pub fn build(&self) -> String {
        self.styles
            .iter()
            .map(|(k, v)| format!("{}: {};", k, v))
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Check if any styles have been added
    pub fn is_empty(&self) -> bool {
        self.styles.is_empty()
    }
}

/// CSS variable utilities
pub mod css_vars {
    use super::*;

    /// Create a CSS variable reference
    pub fn var(name: &str) -> String {
        if name.starts_with("--") {
            format!("var({})", name)
        } else {
            format!("var(--{})", name)
        }
    }

    /// Create a CSS variable reference with fallback
    pub fn var_with_fallback(name: &str, fallback: &str) -> String {
        if name.starts_with("--") {
            format!("var({}, {})", name, fallback)
        } else {
            format!("var(--{}, {})", name, fallback)
        }
    }

    /// Generate a CSS custom property definition
    pub fn define(name: &str, value: &str) -> String {
        if name.starts_with("--") {
            format!("{}: {};", name, value)
        } else {
            format!("--{}: {};", name, value)
        }
    }

    /// Generate CSS for a set of variables
    pub fn generate_set(variables: &HashMap<String, String>) -> String {
        variables
            .iter()
            .map(|(k, v)| {
                let key = if k.starts_with("--") {
                    k.clone()
                } else {
                    format!("--{}", k)
                };
                format!("  {}: {};", key, v)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Color manipulation utilities
pub mod color {
    /// Convert hex to RGB
    pub fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
        let hex = hex.trim_start_matches('#');
        
        match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
                Some((r, g, b))
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                Some((r, g, b))
            }
            _ => None,
        }
    }

    /// Convert RGB to hex
    pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }

    /// Calculate relative luminance for contrast ratio
    pub fn luminance(r: u8, g: u8, b: u8) -> f32 {
        let rs_rgb = [r, g, b].map(|c| {
            let c = c as f32 / 255.0;
            if c <= 0.03928 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        });
        0.2126 * rs_rgb[0] + 0.7152 * rs_rgb[1] + 0.0722 * rs_rgb[2]
    }

    /// Calculate contrast ratio between two colors
    pub fn contrast_ratio(color1: &str, color2: &str) -> Option<f32> {
        let (r1, g1, b1) = hex_to_rgb(color1)?;
        let (r2, g2, b2) = hex_to_rgb(color2)?;

        let lum1 = luminance(r1, g1, b1);
        let lum2 = luminance(r2, g2, b2);

        let brightest = lum1.max(lum2);
        let darkest = lum1.min(lum2);

        Some((brightest + 0.05) / (darkest + 0.05))
    }

    /// Check if color contrast meets WCAG AA standard (4.5:1 for normal text)
    pub fn meets_wcag_aa(color1: &str, color2: &str) -> bool {
        match contrast_ratio(color1, color2) {
            Some(ratio) => ratio >= 4.5,
            None => false,
        }
    }

    /// Check if color contrast meets WCAG AAA standard (7:1 for normal text)
    pub fn meets_wcag_aaa(color1: &str, color2: &str) -> bool {
        match contrast_ratio(color1, color2) {
            Some(ratio) => ratio >= 7.0,
            None => false,
        }
    }
}

/// CSS unit utilities
pub mod units {
    /// Represent a CSS value with unit
    #[derive(Debug, Clone, PartialEq)]
    pub enum Unit {
        Px(f32),
        Rem(f32),
        Em(f32),
        Percent(f32),
        Vw(f32),
        Vh(f32),
        Auto,
        None,
    }

    impl Unit {
        /// Convert to CSS string
        pub fn to_css(&self) -> String {
            match self {
                Unit::Px(v) => format!("{}px", v),
                Unit::Rem(v) => format!("{}rem", v),
                Unit::Em(v) => format!("{}em", v),
                Unit::Percent(v) => format!("{}%", v),
                Unit::Vw(v) => format!("{}vw", v),
                Unit::Vh(v) => format!("{}vh", v),
                Unit::Auto => "auto".to_string(),
                Unit::None => "".to_string(),
            }
        }
    }

    /// Helper to create pixel values
    pub fn px(value: f32) -> Unit {
        Unit::Px(value)
    }

    /// Helper to create rem values
    pub fn rem(value: f32) -> Unit {
        Unit::Rem(value)
    }

    /// Helper to create percentage values
    pub fn percent(value: f32) -> Unit {
        Unit::Percent(value)
    }
}

/// Generate common CSS utility classes
pub fn utility_classes() -> String {
    r#"
/* Layout utilities */
.sh-sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
}

.sh-visually-hidden {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  border: 0;
}

/* Focus utilities */
.sh-focus-ring:focus-visible {
  outline: 2px solid var(--sh-accent);
  outline-offset: 2px;
}

.sh-focus-ring:focus:not(:focus-visible) {
  outline: none;
}

/* Transition utilities */
.sh-transition {
  transition: all var(--sh-dur-med) var(--sh-ease-out);
}

.sh-transition-fast {
  transition: all var(--sh-dur-fast) var(--sh-ease-out);
}

.sh-transition-slow {
  transition: all var(--sh-dur-slow) var(--sh-ease-out);
}

/* Cursor utilities */
.sh-cursor-pointer {
  cursor: pointer;
}

.sh-cursor-not-allowed {
  cursor: not-allowed;
}

/* Pointer events */
.sh-pointer-events-none {
  pointer-events: none;
}

.sh-pointer-events-auto {
  pointer-events: auto;
}

/* User select */
.sh-select-none {
  user-select: none;
}

.sh-select-text {
  user-select: text;
}

/* Overflow utilities */
.sh-overflow-hidden {
  overflow: hidden;
}

.sh-overflow-auto {
  overflow: auto;
}

.sh-overflow-scroll {
  overflow: scroll;
}

/* Position utilities */
.sh-relative {
  position: relative;
}

.sh-absolute {
  position: absolute;
}

.sh-fixed {
  position: fixed;
}

.sh-sticky {
  position: sticky;
}

/* Z-index utilities */
.sh-z-dropdown {
  z-index: 1000;
}

.sh-z-sticky {
  z-index: 1020;
}

.sh-z-modal {
  z-index: 1050;
}

.sh-z-popover {
  z-index: 1060;
}

.sh-z-tooltip {
  z-index: 1070;
}

/* Visibility utilities */
.sh-invisible {
  visibility: hidden;
}

.sh-visible {
  visibility: visible;
}

/* Opacity utilities */
.sh-opacity-0 {
  opacity: 0;
}

.sh-opacity-50 {
  opacity: 0.5;
}

.sh-opacity-100 {
  opacity: 1;
}
"#.to_string()
}

/// Escape a string for use in CSS
pub fn css_escape(input: &str) -> String {
    input
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

/// Validate a CSS identifier
pub fn is_valid_css_identifier(ident: &str) -> bool {
    if ident.is_empty() {
        return false;
    }

    let first = ident.chars().next().unwrap();
    if !(first.is_ascii_alphabetic() || first == '_' || first == '-') {
        return false;
    }

    ident.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_builder() {
        let classes = ClassBuilder::new()
            .add("btn")
            .add("btn-primary")
            .add_if("active", true)
            .add_if("disabled", false)
            .build();

        assert_eq!(classes, "btn btn-primary active");
    }

    #[test]
    fn test_class_builder_with_value() {
        let classes = ClassBuilder::new()
            .add("btn")
            .add_with_value("btn", Some("lg"))
            .add_with_value("btn", Option::<String>::None)
            .build();

        assert_eq!(classes, "btn btn--lg");
    }

    #[test]
    fn test_style_builder() {
        let style = StyleBuilder::new()
            .property("color", "red")
            .property("font-size", "16px")
            .build();

        assert!(style.contains("color: red;"));
        assert!(style.contains("font-size: 16px;"));
    }

    #[test]
    fn test_css_var_utils() {
        assert_eq!(css_vars::var("color-primary"), "var(--color-primary)");
        assert_eq!(css_vars::var("--color-primary"), "var(--color-primary)");
        assert_eq!(
            css_vars::var_with_fallback("color", "red"),
            "var(--color, red)"
        );
    }

    #[test]
    fn test_color_hex_to_rgb() {
        assert_eq!(color::hex_to_rgb("#ff0000"), Some((255, 0, 0)));
        assert_eq!(color::hex_to_rgb("#00ff00"), Some((0, 255, 0)));
        assert_eq!(color::hex_to_rgb("#0000ff"), Some((0, 0, 255)));
        assert_eq!(color::hex_to_rgb("#f00"), Some((255, 0, 0)));
        assert_eq!(color::hex_to_rgb("invalid"), None);
    }

    #[test]
    fn test_contrast_ratio() {
        // Black and white should have high contrast
        let ratio = color::contrast_ratio("#000000", "#ffffff");
        assert!(ratio.unwrap() > 20.0);

        // Similar colors should have low contrast
        let ratio = color::contrast_ratio("#777777", "#888888");
        assert!(ratio.unwrap() < 2.0);
    }

    #[test]
    fn test_wcag_compliance() {
        assert!(color::meets_wcag_aa("#000000", "#ffffff"));
        assert!(color::meets_wcag_aaa("#000000", "#ffffff"));
        assert!(!color::meets_wcag_aa("#777777", "#888888"));
    }

    #[test]
    fn test_units() {
        assert_eq!(units::px(16.0).to_css(), "16px");
        assert_eq!(units::rem(1.5).to_css(), "1.5rem");
        assert_eq!(units::percent(50.0).to_css(), "50%");
    }

    #[test]
    fn test_css_validation() {
        assert!(is_valid_css_identifier("valid-name"));
        assert!(is_valid_css_identifier("_private"));
        assert!(is_valid_css_identifier("camelCase"));
        assert!(!is_valid_css_identifier("123number"));
        assert!(!is_valid_css_identifier(""));
        assert!(!is_valid_css_identifier("has space"));
        assert!(!is_valid_css_identifier("has@symbol"));
    }
}
