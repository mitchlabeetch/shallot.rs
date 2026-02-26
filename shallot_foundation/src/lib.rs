//! Shallot Foundation - Core Design System Library
//!
//! This crate provides the foundational building blocks for the Shallot UI library:
//! - Design tokens (colors, typography, spacing, shadows)
//! - Animation and transition systems
//! - Responsive design utilities
//! - Icon system
//! - CSS utilities

mod theme;
mod design_tokens;
mod responsive;
mod animations;
mod icon;
mod css_utils;
mod transitions;

// Re-export core types
pub use theme::{ColorMode, Theme};
pub use design_tokens::{
    HSLColor, ColorPalette, ColorScheme, 
    TypographyScale, SpacingScale, BorderRadiusScale, ShadowScale, DesignTokens
};
pub use responsive::{
    Breakpoint, ResponsiveValue, ResponsiveProperty,
    ContainerConfig, GridConfig, FlexConfig,
    FlexDirection, FlexWrap, JustifyContent, AlignItems,
    generate_responsive_css, generate_container_css, generate_grid_css
};
pub use animations::{
    EasingFunction, AnimationTiming, Keyframe, KeyframeAnimation, 
    AnimationIterations, AnimationDirection, AnimationFillMode, AnimationPlayState,
    fade_in, fade_out, slide_in_up, slide_in_down, scale_in, bounce_in, 
    shake, pulse, glow, float,
    generate_all_animations, animation_classes
};

// Re-export icon system
pub use icon::{
    Icon, IconSize, IconStyle, IconCategory,
    available_icons, icons_by_category, icon_css
};

// Re-export CSS utilities
pub use css_utils::{
    ClassBuilder, StyleBuilder,
    css_vars, color, units,
    utility_classes, css_escape, is_valid_css_identifier
};

// Re-export transitions
pub use transitions::{
    Transition, TransitionSet, TransitionProperty, TimingFunction,
    transform, filter, presets as transition_presets,
    transition_css
};

/// Get all foundational CSS in one string
pub fn all_css() -> String {
    let mut css = String::new();
    
    // Base theme CSS
    css.push_str(&Theme::default().css());
    css.push('\n');
    
    // Animations
    css.push_str(&generate_all_animations());
    css.push('\n');
    css.push_str(&animation_classes());
    css.push('\n');
    
    // Transitions
    css.push_str(&transition_css());
    css.push('\n');
    
    // Responsive
    css.push_str(&generate_responsive_css());
    css.push('\n');
    
    // Icons
    css.push_str(&icon_css());
    css.push('\n');
    
    // Utilities
    css.push_str(&utility_classes());
    css.push('\n');
    
    css
}

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Library metadata
pub fn library_info() -> &'static str {
    "Shallot Foundation - Zero-JS Rust UI Design System"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_library_info() {
        assert!(!library_info().is_empty());
    }

    #[test]
    fn test_all_css_generation() {
        let css = all_css();
        assert!(!css.is_empty());
        assert!(css.contains(":root"));
        assert!(css.contains("@keyframes"));
    }
}
