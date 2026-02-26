//! Icon System - Type-safe icon library with SVG support
//! 
//! This module provides a comprehensive icon system with:
//! - Common UI icons (navigation, actions, status)
//! - Size variants and styling options
//! - Accessibility attributes
//! - CSS generation for icon styling

#[allow(unused_imports)]
use std::collections::HashMap;

/// Standard icon sizes following the design system
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconSize {
    Xs,  // 12px
    Sm,  // 16px
    Md,  // 20px
    Lg,  // 24px
    Xl,  // 32px
    Xxl, // 48px
}

impl IconSize {
    /// Get the pixel size for this icon variant
    pub fn px(&self) -> u8 {
        match self {
            IconSize::Xs => 12,
            IconSize::Sm => 16,
            IconSize::Md => 20,
            IconSize::Lg => 24,
            IconSize::Xl => 32,
            IconSize::Xxl => 48,
        }
    }

    /// Get the CSS class suffix
    pub fn class_suffix(&self) -> &'static str {
        match self {
            IconSize::Xs => "xs",
            IconSize::Sm => "sm",
            IconSize::Md => "md",
            IconSize::Lg => "lg",
            IconSize::Xl => "xl",
            IconSize::Xxl => "xxl",
        }
    }
}

/// Icon style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconStyle {
    Solid,
    Outline,
    Thin,
}

/// Complete icon definition
#[derive(Debug, Clone)]
pub struct Icon {
    pub name: &'static str,
    pub size: IconSize,
    pub style: IconStyle,
    pub color: Option<String>,
    pub aria_label: Option<String>,
}

impl Icon {
    /// Create a new icon with default settings
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            size: IconSize::Md,
            style: IconStyle::Outline,
            color: None,
            aria_label: None,
        }
    }

    /// Set the icon size
    pub fn size(mut self, size: IconSize) -> Self {
        self.size = size;
        self
    }

    /// Set the icon style
    pub fn style(mut self, style: IconStyle) -> Self {
        self.style = style;
        self
    }

    /// Set the icon color (CSS color value)
    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Set the aria-label for accessibility
    pub fn aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    /// Get the SVG path data for this icon
    pub fn svg_path(&self) -> &'static str {
        get_icon_svg(self.name, self.style)
    }

    /// Get the complete SVG markup as a string
    pub fn to_svg_string(&self) -> String {
        let size = self.size.px();
        let color_attr = self.color.as_ref()
            .map(|c| format!(r#" color="{}""#, c))
            .unwrap_or_default();
        let aria_attr = self.aria_label.as_ref()
            .map(|l| format!(r#" aria-label="{}" role="img""#, l))
            .unwrap_or_else(|| r#" aria-hidden="true""#.to_string());

        format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"{}{}>{}</svg>"#,
            size, size, color_attr, aria_attr, self.svg_path()
        )
    }

    /// Get the CSS class for this icon
    pub fn class(&self) -> String {
        format!("sh-icon sh-icon--{}", self.size.class_suffix())
    }
}

impl Default for Icon {
    fn default() -> Self {
        Self::new("circle")
    }
}

/// Icon categories for organization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconCategory {
    Navigation,
    Action,
    Status,
    Communication,
    Media,
    File,
    Social,
    System,
}

/// Get SVG path data for a named icon
fn get_icon_svg(name: &str, style: IconStyle) -> &'static str {
    // Icon paths organized by category for better maintainability
    let path = match name {
        // Navigation icons
        "arrow-left" => "<path d=\"m15 18-6-6 6-6\"/>",
        "arrow-right" => "<path d=\"m9 18 6-6-6-6\"/>",
        "arrow-up" => "<path d=\"m18 15-6-6-6 6\"/>",
        "arrow-down" => "<path d=\"m6 9 6 6 6-6\"/>",
        "chevron-left" => "<path d=\"m15 18-6-6 6-6\"/>",
        "chevron-right" => "<path d=\"m9 18 6-6-6-6\"/>",
        "chevron-up" => "<path d=\"m18 15-6-6-6 6\"/>",
        "chevron-down" => "<path d=\"m6 9 6 6 6-6\"/>",
        "menu" => "<line x1=\"4\" x2=\"20\" y1=\"12\" y2=\"12\"/><line x1=\"4\" x2=\"20\" y1=\"6\" y2=\"6\"/><line x1=\"4\" x2=\"20\" y1=\"18\" y2=\"18\"/>",
        "home" => "<path d=\"m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z\"/><polyline points=\"9 22 9 12 15 12 15 22\"/>",
        "search" => "<circle cx=\"11\" cy=\"11\" r=\"8\"/><path d=\"m21 21-4.3-4.3\"/>",
        
        // Action icons
        "plus" => "<path d=\"M5 12h14\"/><path d=\"M12 5v14\"/>",
        "minus" => "<path d=\"M5 12h14\"/>",
        "x" | "close" => "<path d=\"M18 6 6 18\"/><path d=\"m6 6 12 12\"/>",
        "check" => "<path d=\"M20 6 9 17l-5-5\"/>",
        "edit" => "<path d=\"M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7\"/><path d=\"M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z\"/>",
        "trash" => "<path d=\"M3 6h18\"/><path d=\"M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6\"/><path d=\"M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2\"/>",
        "copy" => "<rect width=\"14\" height=\"14\" x=\"8\" y=\"8\" rx=\"2\" ry=\"2\"/><path d=\"M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2\"/>",
        "download" => "<path d=\"M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4\"/><polyline points=\"7 10 12 15 17 10\"/><line x1=\"12\" x2=\"12\" y1=\"15\" y2=\"3\"/>",
        "upload" => "<path d=\"M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4\"/><polyline points=\"17 8 12 3 7 8\"/><line x1=\"12\" x2=\"12\" y1=\"3\" y2=\"15\"/>",
        "refresh" => "<path d=\"M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8\"/><path d=\"M21 3v5h-5\"/><path d=\"M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16\"/><path d=\"M8 16H3v5\"/>",
        "settings" => "<path d=\"M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.1a2 2 0 0 1-1-1.72v-.51a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z\"/><circle cx=\"12\" cy=\"12\" r=\"3\"/>",
        
        // Status icons
        "info" => "<circle cx=\"12\" cy=\"12\" r=\"10\"/><path d=\"M12 16v-4\"/><path d=\"M12 8h.01\"/>",
        "warning" => "<path d=\"m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z\"/><line x1=\"12\" x2=\"12\" y1=\"9\" y2=\"13\"/><line x1=\"12\" x2=\"12.01\" y1=\"17\" y2=\"17\"/>",
        "error" | "alert-circle" => "<circle cx=\"12\" cy=\"12\" r=\"10\"/><line x1=\"12\" x2=\"12\" y1=\"8\" y2=\"12\"/><line x1=\"12\" x2=\"12.01\" y1=\"16\" y2=\"16\"/>",
        "success" | "check-circle" => "<circle cx=\"12\" cy=\"12\" r=\"10\"/><path d=\"m9 12 2 2 4-4\"/>",
        "help-circle" => "<circle cx=\"12\" cy=\"12\" r=\"10\"/><path d=\"M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3\"/><path d=\"M12 17h.01\"/>",
        "bell" => "<path d=\"M6 8a6 6 0 0 1 12 0c0 7 3 9 3 9H3s3-2 3-9\"/><path d=\"M10.3 21a1.94 1.94 0 0 0 3.4 0\"/>",
        "bell-off" => "<path d=\"M13.73 21a2 2 0 0 1-3.46 0\"/><path d=\"M18.63 13A17.888 17.888 0 0 1 18 8\"/><path d=\"M6.26 6.26A5.86 5.86 0 0 0 6 8c0 7-3 9-3 9h14\"/><path d=\"M18 8a6 6 0 0 0-9.33-5\"/><path d=\"m2 2 20 20\"/>",
        
        // Communication icons
        "mail" => "<rect width=\"20\" height=\"16\" x=\"2\" y=\"4\" rx=\"2\"/><path d=\"m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7\"/>",
        "message-circle" => "<path d=\"m3 21 1.9-5.7a8.5 8.5 0 1 1 3.8 3.8z\"/>",
        "phone" => "<path d=\"M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z\"/>",
        "share" => "<path d=\"M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8\"/><polyline points=\"16 6 12 2 8 6\"/><line x1=\"12\" x2=\"12\" y1=\"2\" y2=\"15\"/>",
        "heart" => "<path d=\"M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z\"/>",
        "star" => "<polygon points=\"12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2\"/>",
        "bookmark" => "<path d=\"m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v16z\"/>",
        
        // Media icons
        "play" => "<polygon points=\"6 3 20 12 6 21 6 3\"/>",
        "pause" => "<rect width=\"4\" height=\"16\" x=\"6\" y=\"4\"/><rect width=\"4\" height=\"16\" x=\"14\" y=\"4\"/>",
        "skip-forward" => "<polygon points=\"5 4 15 12 5 20 5 4\"/><line x1=\"19\" x2=\"19\" y1=\"5\" y2=\"19\"/>",
        "skip-back" => "<polygon points=\"19 20 9 12 19 4 19 20\"/><line x1=\"5\" x2=\"5\" y1=\"19\" y2=\"5\"/>",
        "volume" | "volume-high" => "<polygon points=\"11 5 6 9 2 9 2 15 6 15 11 19 11 5\"/><path d=\"M15.54 8.46a5 5 0 0 1 0 7.07\"/><path d=\"M19.07 4.93a10 10 0 0 1 0 14.14\"/>",
        "volume-low" => "<polygon points=\"11 5 6 9 2 9 2 15 6 15 11 19 11 5\"/><path d=\"M15.54 8.46a5 5 0 0 1 0 7.07\"/>",
        "volume-x" | "mute" => "<polygon points=\"11 5 6 9 2 9 2 15 6 15 11 19 11 5\"/><line x1=\"23\" x2=\"17\" y1=\"9\" y2=\"15\"/><line x1=\"17\" x2=\"23\" y1=\"9\" y2=\"15\"/>",
        "image" => "<rect width=\"18\" height=\"18\" x=\"3\" y=\"3\" rx=\"2\" ry=\"2\"/><circle cx=\"9\" cy=\"9\" r=\"2\"/><path d=\"m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21\"/>",
        "video" => "<path d=\"m22 8-6 4 6 4V8Z\"/><rect width=\"14\" height=\"12\" x=\"2\" y=\"6\" rx=\"2\"/>",
        
        // File icons
        "file" => "<path d=\"M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z\"/><polyline points=\"14 2 14 8 20 8\"/>",
        "folder" => "<path d=\"M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z\"/>",
        "file-text" => "<path d=\"M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z\"/><polyline points=\"14 2 14 8 20 8\"/><line x1=\"16\" x2=\"8\" y1=\"13\" y2=\"13\"/><line x1=\"16\" x2=\"8\" y1=\"17\" y2=\"17\"/><line x1=\"10\" x2=\"8\" y1=\"9\" y2=\"9\"/>",
        "paperclip" => "<path d=\"m21.44 11.05-9.19 9.19a6 6 0 0 1-8.49-8.49l8.57-8.57A4 4 0 1 1 18 8.84l-8.59 8.57a2 2 0 0 1-2.83-2.83l8.49-8.48\"/>",
        
        // System icons
        "user" => "<path d=\"M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2\"/><circle cx=\"12\" cy=\"7\" r=\"4\"/>",
        "users" => "<path d=\"M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2\"/><circle cx=\"9\" cy=\"7\" r=\"4\"/><path d=\"M22 21v-2a4 4 0 0 0-3-3.87\"/><path d=\"M16 3.13a4 4 0 0 1 0 7.75\"/>",
        "lock" => "<rect width=\"18\" height=\"11\" x=\"3\" y=\"11\" rx=\"2\" ry=\"2\"/><path d=\"M7 11V7a5 5 0 0 1 10 0v4\"/>",
        "unlock" => "<rect width=\"18\" height=\"11\" x=\"3\" y=\"11\" rx=\"2\" ry=\"2\"/><path d=\"M7 11V7a5 5 0 0 1 9.9-1\"/>",
        "eye" => "<path d=\"M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z\"/><circle cx=\"12\" cy=\"12\" r=\"3\"/>",
        "eye-off" => "<path d=\"M9.88 9.88a3 3 0 1 0 4.24 4.24\"/><path d=\"M10.73 5.08A10.43 10.43 0 0 1 12 5c7 0 10 7 10 7a13.16 13.16 0 0 1-1.67 2.68\"/><path d=\"M6.61 6.61A13.526 13.526 0 0 0 2 12s3 7 10 7c.78 0 1.53-.09 2.24-.26\"/><path d=\"M2 2l20 20\"/>",
        "calendar" => "<rect width=\"18\" height=\"18\" x=\"3\" y=\"4\" rx=\"2\" ry=\"2\"/><line x1=\"16\" x2=\"16\" y1=\"2\" y2=\"6\"/><line x1=\"8\" x2=\"8\" y1=\"2\" y2=\"6\"/><line x1=\"3\" x2=\"21\" y1=\"10\" y2=\"10\"/>",
        "clock" => "<circle cx=\"12\" cy=\"12\" r=\"10\"/><polyline points=\"12 6 12 12 16 14\"/>",
        "circle" => "<circle cx=\"12\" cy=\"12\" r=\"10\"/>",
        "square" => "<rect width=\"18\" height=\"18\" x=\"3\" y=\"3\" rx=\"2\"/>",
        "more-horizontal" => "<circle cx=\"12\" cy=\"12\" r=\"1\"/><circle cx=\"19\" cy=\"12\" r=\"1\"/><circle cx=\"5\" cy=\"12\" r=\"1\"/>",
        "more-vertical" => "<circle cx=\"12\" cy=\"12\" r=\"1\"/><circle cx=\"12\" cy=\"5\" r=\"1\"/><circle cx=\"12\" cy=\"19\" r=\"1\"/>",
        
        // Loading/spinner
        "loader" | "loading" | "spinner" => "<line x1=\"12\" y1=\"2\" x2=\"12\" y2=\"6\"/><line x1=\"12\" y1=\"18\" x2=\"12\" y2=\"22\"/><line x1=\"4.93\" y1=\"4.93\" x2=\"7.76\" y2=\"7.76\"/><line x1=\"16.24\" y1=\"16.24\" x2=\"19.07\" y2=\"19.07\"/><line x1=\"2\" y1=\"12\" x2=\"6\" y2=\"12\"/><line x1=\"18\" y1=\"12\" x2=\"22\" y2=\"12\"/><line x1=\"4.93\" y1=\"19.07\" x2=\"7.76\" y2=\"16.24\"/><line x1=\"16.24\" y1=\"7.76\" x2=\"19.07\" y2=\"4.93\"/>",
        
        // External links
        "external-link" => "<path d=\"M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6\"/><polyline points=\"15 3 21 3 21 9\"/><line x1=\"10\" x2=\"21\" y1=\"14\" y2=\"3\"/>",
        "link" => "<path d=\"M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71\"/><path d=\"M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71\"/>",
        
        // Filters/Sort
        "filter" => "<polygon points=\"22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3\"/>",
        "sliders" => "<line x1=\"4\" x2=\"4\" y1=\"21\" y2=\"14\"/><line x1=\"4\" x2=\"4\" y1=\"10\" y2=\"3\"/><line x1=\"12\" x2=\"12\" y1=\"21\" y2=\"12\"/><line x1=\"12\" x2=\"12\" y1=\"8\" y2=\"3\"/><line x1=\"20\" x2=\"20\" y1=\"21\" y2=\"16\"/><line x1=\"20\" x2=\"20\" y1=\"12\" y2=\"3\"/><line x1=\"1\" x2=\"7\" y1=\"14\" y2=\"14\"/><line x1=\"9\" x2=\"15\" y1=\"8\" y2=\"8\"/><line x1=\"17\" x2=\"23\" y1=\"16\" y2=\"16\"/>",
        "sort-asc" => "<path d=\"m3 8 4-4 4 4\"/><path d=\"M7 4v16\"/>",
        "sort-desc" => "<path d=\"m3 16 4 4 4-4\"/><path d=\"M7 20V4\"/>",
        
        // Default fallback
        _ => "<circle cx=\"12\" cy=\"12\" r=\"10\"/>",
    };

    // For solid style, we could modify the paths, but for now we return the same
    // In a full implementation, this would return filled versions
    match style {
        IconStyle::Solid => path,
        IconStyle::Outline => path,
        IconStyle::Thin => path,
    }
}

/// Get all available icon names
pub fn available_icons() -> Vec<&'static str> {
    vec![
        // Navigation
        "arrow-left", "arrow-right", "arrow-up", "arrow-down",
        "chevron-left", "chevron-right", "chevron-up", "chevron-down",
        "menu", "home", "search",
        // Actions
        "plus", "minus", "x", "close", "check", "edit", "trash", "copy",
        "download", "upload", "refresh", "settings",
        // Status
        "info", "warning", "error", "alert-circle", "success", "check-circle",
        "help-circle", "bell", "bell-off",
        // Communication
        "mail", "message-circle", "phone", "share", "heart", "star", "bookmark",
        // Media
        "play", "pause", "skip-forward", "skip-back", "volume", "volume-high",
        "volume-low", "volume-x", "mute", "image", "video",
        // File
        "file", "folder", "file-text", "paperclip",
        // System
        "user", "users", "lock", "unlock", "eye", "eye-off", "calendar",
        "clock", "circle", "square", "more-horizontal", "more-vertical",
        // Loading
        "loader", "loading", "spinner",
        // External
        "external-link", "link",
        // Filters
        "filter", "sliders", "sort-asc", "sort-desc",
    ]
}

/// Generate CSS for icons
pub fn icon_css() -> String {
    format!(
        r#"
/* Icon System */
.sh-icon {{
  display: inline-flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  vertical-align: middle;
}}

.sh-icon svg {{
  width: 100%;
  height: 100%;
}}

/* Size variants */
.sh-icon--xs {{ width: 12px; height: 12px; }}
.sh-icon--sm {{ width: 16px; height: 16px; }}
.sh-icon--md {{ width: 20px; height: 20px; }}
.sh-icon--lg {{ width: 24px; height: 24px; }}
.sh-icon--xl {{ width: 32px; height: 32px; }}
.sh-icon--xxl {{ width: 48px; height: 48px; }}

/* Spinning animation for loader */
@keyframes sh-icon-spin {{
  from {{ transform: rotate(0deg); }}
  to {{ transform: rotate(360deg); }}
}}

.sh-icon--spin {{
  animation: sh-icon-spin 1s linear infinite;
}}

/* Icon button base */
.sh-icon-button {{
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 0.5rem;
  border-radius: var(--sh-radius-md);
  background: transparent;
  border: none;
  cursor: pointer;
  transition: all var(--sh-dur-fast) var(--sh-ease-out);
}}

.sh-icon-button:hover {{
  background: var(--sh-surface-2);
}}

.sh-icon-button:focus-visible {{
  outline: 2px solid var(--sh-accent);
  outline-offset: 2px;
}}

/* Icon with text */
.sh-icon-text {{
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
}}
"#
    )
}

/// Get icons by category
pub fn icons_by_category(category: IconCategory) -> Vec<&'static str> {
    match category {
        IconCategory::Navigation => vec![
            "arrow-left", "arrow-right", "arrow-up", "arrow-down",
            "chevron-left", "chevron-right", "chevron-up", "chevron-down",
            "menu", "home", "search",
        ],
        IconCategory::Action => vec![
            "plus", "minus", "x", "close", "check", "edit", "trash", "copy",
            "download", "upload", "refresh", "settings",
        ],
        IconCategory::Status => vec![
            "info", "warning", "error", "alert-circle", "success", "check-circle",
            "help-circle", "bell", "bell-off",
        ],
        IconCategory::Communication => vec![
            "mail", "message-circle", "phone", "share", "heart", "star", "bookmark",
        ],
        IconCategory::Media => vec![
            "play", "pause", "skip-forward", "skip-back", "volume", "volume-high",
            "volume-low", "volume-x", "mute", "image", "video",
        ],
        IconCategory::File => vec![
            "file", "folder", "file-text", "paperclip",
        ],
        IconCategory::System => vec![
            "user", "users", "lock", "unlock", "eye", "eye-off", "calendar",
            "clock", "circle", "square", "more-horizontal", "more-vertical",
        ],
        IconCategory::Social => vec![
            "share", "heart", "star", "bookmark",
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_creation() {
        let icon = Icon::new("home")
            .size(IconSize::Lg)
            .style(IconStyle::Solid)
            .color("red")
            .aria_label("Home button");

        assert_eq!(icon.name, "home");
        assert_eq!(icon.size, IconSize::Lg);
        assert_eq!(icon.style, IconStyle::Solid);
        assert_eq!(icon.color, Some("red".to_string()));
        assert_eq!(icon.aria_label, Some("Home button".to_string()));
    }

    #[test]
    fn test_icon_size_px() {
        assert_eq!(IconSize::Xs.px(), 12);
        assert_eq!(IconSize::Sm.px(), 16);
        assert_eq!(IconSize::Md.px(), 20);
        assert_eq!(IconSize::Lg.px(), 24);
        assert_eq!(IconSize::Xl.px(), 32);
        assert_eq!(IconSize::Xxl.px(), 48);
    }

    #[test]
    fn test_svg_generation() {
        let icon = Icon::new("circle").size(IconSize::Md);
        let svg = icon.to_svg_string();
        
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
        assert!(svg.contains(r#"width="20""#));
        assert!(svg.contains(r#"height="20""#));
        assert!(svg.contains("aria-hidden"));
    }

    #[test]
    fn test_svg_with_aria_label() {
        let icon = Icon::new("home").aria_label("Go to homepage");
        let svg = icon.to_svg_string();
        
        assert!(svg.contains("aria-label"));
        assert!(svg.contains("Go to homepage"));
        assert!(svg.contains(r#"role="img""#));
    }

    #[test]
    fn test_icon_categories() {
        let nav_icons = icons_by_category(IconCategory::Navigation);
        assert!(!nav_icons.is_empty());
        assert!(nav_icons.contains(&"home"));
        assert!(nav_icons.contains(&"search"));

        let action_icons = icons_by_category(IconCategory::Action);
        assert!(!action_icons.is_empty());
        assert!(action_icons.contains(&"edit"));
    }

    #[test]
    fn test_available_icons() {
        let icons = available_icons();
        assert!(!icons.is_empty());
        assert!(icons.contains(&"home"));
        assert!(icons.contains(&"user"));
    }

    #[test]
    fn test_icon_css_generation() {
        let css = icon_css();
        assert!(css.contains(".sh-icon"));
        assert!(css.contains(".sh-icon--spin"));
        assert!(css.contains("@keyframes sh-icon-spin"));
    }
}
