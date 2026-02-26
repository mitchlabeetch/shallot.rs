//! Glass Card Component - Glassmorphism Effect Card
//!
//! A card component with frosted glass effect using CSS `backdrop-filter: blur()`.
//! Falls back to semi-transparent background on unsupported browsers.
//!
//! Research Reference: Section 5.6.3 - Advanced Visuals
//!
//! # Example
//! ```
//! use shallot_components::glass_card::GlassCard;
//! use maud::html;
//!
//! let card = GlassCard::new(html! {
//!     h3 { "Glass Effect" }
//!     p { "Beautiful frosted glass appearance" }
//! })
//! .blur_amount(20)
//! .tint("rgba(255, 255, 255, 0.1)")
//! .border(true)
//! .border_opacity(0.2);
//! ```

use maud::{html, Markup};

/// Glow position for ambient glow effect
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlowPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
    Top,
    Bottom,
    Custom(&'static str), // CSS position values
}

impl GlowPosition {
    pub fn css_position(&self) -> &'static str {
        match self {
            Self::TopLeft => "top: -50%; left: -50%",
            Self::TopRight => "top: -50%; right: -50%",
            Self::BottomLeft => "bottom: -50%; left: -50%",
            Self::BottomRight => "bottom: -50%; right: -50%",
            Self::Center => "top: 50%; left: 50%; transform: translate(-50%, -50%)",
            Self::Top => "top: -50%; left: 50%; transform: translateX(-50%)",
            Self::Bottom => "bottom: -50%; left: 50%; transform: translateX(-50%)",
            Self::Custom(p) => p,
        }
    }
}

/// Glass effect intensity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlassIntensity {
    Light,   // Subtle effect
    Medium,  // Default
    Heavy,   // Strong blur
    Extreme, // Maximum blur
}

impl GlassIntensity {
    pub fn blur_amount(&self) -> u8 {
        match self {
            Self::Light => 8,
            Self::Medium => 16,
            Self::Heavy => 24,
            Self::Extreme => 40,
        }
    }

    pub fn saturation(&self) -> u8 {
        match self {
            Self::Light => 120,
            Self::Medium => 140,
            Self::Heavy => 160,
            Self::Extreme => 180,
        }
    }
}

/// Glass card with frosted glass effect
#[derive(Debug, Clone)]
pub struct GlassCard {
    content: Markup,
    intensity: GlassIntensity,
    blur_amount: Option<u8>,
    tint: Option<String>,
    border: bool,
    border_opacity: f32,
    border_radius: u16,
    shadow: bool,
    glow: Option<(String, GlowPosition, u16)>, // (color, position, size)
    hover_effect: bool,
    interactive: bool,
    padding: u16,
    class: Option<String>,
    id: Option<String>,
}

impl GlassCard {
    pub fn new(content: Markup) -> Self {
        Self {
            content,
            intensity: GlassIntensity::Medium,
            blur_amount: None,
            tint: None,
            border: true,
            border_opacity: 0.2,
            border_radius: 16,
            shadow: true,
            glow: None,
            hover_effect: true,
            interactive: false,
            padding: 24,
            class: None,
            id: None,
        }
    }

    /// Set glass effect intensity
    pub fn intensity(mut self, intensity: GlassIntensity) -> Self {
        self.intensity = intensity;
        self
    }

    /// Set custom blur amount (overrides intensity)
    pub fn blur_amount(mut self, blur: u8) -> Self {
        self.blur_amount = Some(blur);
        self
    }

    /// Set tint color (CSS color value)
    pub fn tint(mut self, color: impl Into<String>) -> Self {
        self.tint = Some(color.into());
        self
    }

    /// Enable/disable border
    pub fn border(mut self, enable: bool) -> Self {
        self.border = enable;
        self
    }

    /// Set border opacity (0.0 - 1.0)
    pub fn border_opacity(mut self, opacity: f32) -> Self {
        self.border_opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Set border radius in pixels
    pub fn border_radius(mut self, radius: u16) -> Self {
        self.border_radius = radius;
        self
    }

    /// Enable/disable shadow
    pub fn shadow(mut self, enable: bool) -> Self {
        self.shadow = enable;
        self
    }

    /// Add ambient glow effect
    pub fn with_glow(
        mut self,
        color: impl Into<String>,
        position: GlowPosition,
        size: u16,
    ) -> Self {
        self.glow = Some((color.into(), position, size));
        self
    }

    /// Enable hover lift effect
    pub fn hover_effect(mut self, enable: bool) -> Self {
        self.hover_effect = enable;
        self
    }

    /// Make card interactive (adds cursor: pointer)
    pub fn interactive(mut self) -> Self {
        self.interactive = true;
        self
    }

    /// Set padding in pixels
    pub fn padding(mut self, padding: u16) -> Self {
        self.padding = padding;
        self
    }

    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Get effective blur amount
    fn effective_blur(&self) -> u8 {
        self.blur_amount
            .unwrap_or_else(|| self.intensity.blur_amount())
    }

    /// Get effective tint color
    fn effective_tint(&self) -> String {
        self.tint.clone().unwrap_or_else(|| {
            // Default tint based on current theme
            "var(--sh-glass-tint, rgba(255, 255, 255, 0.1))".to_string()
        })
    }

    fn build_styles(&self) -> String {
        let blur = self.effective_blur();
        let tint = self.effective_tint();
        let _border_opacity = self.border_opacity;

        format!(
            "backdrop-filter:blur({}px) saturate({}%);-webkit-backdrop-filter:blur({}px) saturate({}%);background:{};border-radius:{}px;padding:{}px;",
            blur,
            self.intensity.saturation(),
            blur,
            self.intensity.saturation(),
            tint,
            self.border_radius,
            self.padding
        )
    }

    fn build_border_style(&self) -> String {
        if self.border {
            format!(
                "border:1px solid rgba(255, 255, 255, {});",
                self.border_opacity
            )
        } else {
            String::new()
        }
    }

    fn build_shadow_style(&self) -> String {
        if self.shadow {
            "box-shadow:0 8px 32px 0 rgba(0, 0, 0, 0.37);".to_string()
        } else {
            String::new()
        }
    }

    fn build_hover_styles(&self) -> String {
        if self.hover_effect {
            "transition:transform 0.3s ease,box-shadow 0.3s ease;".to_string()
        } else {
            String::new()
        }
    }

    pub fn render(self) -> Markup {
        let styles = self.build_styles();
        let border = self.build_border_style();
        let shadow = self.build_shadow_style();
        let hover = self.build_hover_styles();
        let cursor = if self.interactive {
            "cursor:pointer;"
        } else {
            ""
        };

        let all_styles = format!("{}{}{}{}{}", styles, border, shadow, hover, cursor);

        let mut classes = vec!["sh-glass-card".to_string()];
        if self.hover_effect {
            classes.push("sh-glass-card--hover".to_string());
        }
        if self.interactive {
            classes.push("sh-glass-card--interactive".to_string());
        }
        if let Some(ref c) = self.class {
            classes.push(c.clone());
        }
        let class = classes.join(" ");

        let glow_element = if let Some((color, position, size)) = self.glow {
            html! {
                div
                    class="sh-glass-glow"
                    style=(format!(
                        "position:absolute;width:{}px;height:{}px;background:{};border-radius:50%;filter:blur({}px);pointer-events:none;z-index:-1;{}",
                        size, size, color, size / 2, position.css_position()
                    ))
                {}
            }
        } else {
            html! {}
        };

        html! {
            div
                class=(class)
                style=(all_styles)
                id=[self.id]
                role="article"
                aria-label="Glass card content"
            {
                (glow_element)
                div class="sh-glass-content" style="position:relative;z-index:1;" {
                    (self.content)
                }
            }
        }
    }
}

/// Glass panel - Full container glass effect
#[derive(Debug, Clone)]
pub struct GlassPanel {
    content: Markup,
    intensity: GlassIntensity,
    tint: Option<String>,
    border_radius: u16,
    full_width: bool,
    full_height: bool,
}

impl GlassPanel {
    pub fn new(content: Markup) -> Self {
        Self {
            content,
            intensity: GlassIntensity::Medium,
            tint: None,
            border_radius: 0,
            full_width: true,
            full_height: true,
        }
    }

    pub fn intensity(mut self, intensity: GlassIntensity) -> Self {
        self.intensity = intensity;
        self
    }

    pub fn tint(mut self, color: impl Into<String>) -> Self {
        self.tint = Some(color.into());
        self
    }

    pub fn border_radius(mut self, radius: u16) -> Self {
        self.border_radius = radius;
        self
    }

    pub fn render(self) -> Markup {
        let blur = self.intensity.blur_amount();
        let tint = self
            .tint
            .unwrap_or_else(|| "var(--sh-glass-tint, rgba(255, 255, 255, 0.1))".to_string());

        let mut styles = format!(
            "backdrop-filter:blur({}px) saturate({}%);-webkit-backdrop-filter:blur({}px) saturate({}%);background:{};",
            blur,
            self.intensity.saturation(),
            blur,
            self.intensity.saturation(),
            tint
        );

        if self.full_width {
            styles.push_str("width:100%;");
        }
        if self.full_height {
            styles.push_str("height:100%;");
        }
        if self.border_radius > 0 {
            styles.push_str(&format!("border-radius:{}px;", self.border_radius));
        }

        html! {
            div class="sh-glass-panel" style=(styles) {
                (self.content)
            }
        }
    }
}

/// Generate CSS for glass components
pub fn glass_css() -> String {
    r#"
/* Glass Card & Panel Styles */

.sh-glass-card {
    position: relative;
    overflow: hidden;
}

.sh-glass-card--hover {
    transition: transform 0.3s cubic-bezier(0.34, 1.56, 0.64, 1),
                box-shadow 0.3s ease;
}

.sh-glass-card--hover:hover {
    transform: translateY(-4px);
    box-shadow: 0 12px 40px 0 rgba(0, 0, 0, 0.4);
}

.sh-glass-card--interactive {
    cursor: pointer;
}

.sh-glass-card--interactive:active {
    transform: translateY(-2px) scale(0.98);
}

.sh-glass-content {
    position: relative;
    z-index: 1;
}

.sh-glass-glow {
    pointer-events: none;
    opacity: 0.6;
    animation: glass-glow-pulse 4s ease-in-out infinite;
}

@keyframes glass-glow-pulse {
    0%, 100% {
        opacity: 0.4;
        transform: scale(1);
    }
    50% {
        opacity: 0.7;
        transform: scale(1.1);
    }
}

.sh-glass-panel {
    box-sizing: border-box;
}

/* Fallback for browsers without backdrop-filter support */
@supports not (backdrop-filter: blur(10px)) {
    .sh-glass-card,
    .sh-glass-panel {
        background: var(--sh-glass-fallback, rgba(255, 255, 255, 0.95)) !important;
    }
}

/* Dark mode adjustments */
@media (prefers-color-scheme: dark) {
    .sh-glass-card,
    .sh-glass-panel {
        border-color: rgba(255, 255, 255, 0.1);
    }
}

/* Reduced motion support */
@media (prefers-reduced-motion: reduce) {
    .sh-glass-card--hover {
        transition: none;
    }

    .sh-glass-card--hover:hover {
        transform: none;
    }

    .sh-glass-glow {
        animation: none;
    }
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glass_card_basic() {
        let card = GlassCard::new(html! { "Content" })
            .intensity(GlassIntensity::Heavy)
            .border(true)
            .border_radius(20);

        assert_eq!(card.effective_blur(), 24);
        assert!(card.border);
        assert_eq!(card.border_radius, 20);
    }

    #[test]
    fn test_glass_card_custom_blur() {
        let card = GlassCard::new(html! {})
            .blur_amount(30)
            .intensity(GlassIntensity::Light);

        // Custom blur should override intensity
        assert_eq!(card.effective_blur(), 30);
    }

    #[test]
    fn test_glass_card_with_glow() {
        let card = GlassCard::new(html! {}).with_glow("#ff00ff", GlowPosition::TopRight, 200);

        assert!(card.glow.is_some());
    }

    #[test]
    fn test_glass_intensity_values() {
        assert_eq!(GlassIntensity::Light.blur_amount(), 8);
        assert_eq!(GlassIntensity::Extreme.blur_amount(), 40);
        assert_eq!(GlassIntensity::Medium.saturation(), 140);
    }

    #[test]
    fn test_border_opacity_clamping() {
        let card = GlassCard::new(html! {}).border_opacity(1.5); // Should clamp to 1.0

        assert_eq!(card.border_opacity, 1.0);
    }
}
