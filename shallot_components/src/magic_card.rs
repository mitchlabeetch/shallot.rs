//! Magic Card Component
//!
//! A card with a spotlight effect that follows the mouse cursor on hover.
//! Inspired by MagicUI's MagicCard component.
//! Pure CSS implementation using radial gradients and custom properties.

use crate::component::{AriaAttrs, Component};
use maud::{html, Markup, Render};

/// Magic Card with spotlight hover effect
pub struct MagicCard {
    /// Card content
    content: Markup,
    /// Gradient size (in pixels)
    gradient_size: u16,
    /// Gradient color
    gradient_color: String,
    /// Gradient opacity (0.0 - 1.0)
    gradient_opacity: f32,
    /// Border gradient from color
    border_from: String,
    /// Border gradient to color
    border_to: String,
    /// Border width
    border_width: u8,
    /// Border radius
    border_radius: String,
    /// Background color
    background: String,
    /// Padding
    padding: String,
    /// Custom CSS class
    custom_class: Option<String>,
    /// ARIA attributes
    aria: AriaAttrs,
}

impl MagicCard {
    /// Create a new MagicCard with content
    pub fn new(content: Markup) -> Self {
        Self {
            content,
            gradient_size: 200,
            gradient_color: "#262626".to_string(),
            gradient_opacity: 0.8,
            border_from: "#9E7AFF".to_string(),
            border_to: "#FE8BBB".to_string(),
            border_width: 1,
            border_radius: "var(--sh-radius-lg)".to_string(),
            background: "var(--sh-surface)".to_string(),
            padding: "1.5rem".to_string(),
            custom_class: None,
            aria: AriaAttrs::new(),
        }
    }

    /// Set the spotlight gradient size
    pub fn gradient_size(mut self, size: u16) -> Self {
        self.gradient_size = size;
        self
    }

    /// Set the spotlight gradient color
    pub fn gradient_color(mut self, color: impl Into<String>) -> Self {
        self.gradient_color = color.into();
        self
    }

    /// Set the spotlight gradient opacity
    pub fn gradient_opacity(mut self, opacity: f32) -> Self {
        self.gradient_opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Set the border gradient colors
    pub fn border_gradient(mut self, from: impl Into<String>, to: impl Into<String>) -> Self {
        self.border_from = from.into();
        self.border_to = to.into();
        self
    }

    /// Set the border width
    pub fn border_width(mut self, width: u8) -> Self {
        self.border_width = width;
        self
    }

    /// Set the border radius
    pub fn border_radius(mut self, radius: impl Into<String>) -> Self {
        self.border_radius = radius.into();
        self
    }

    /// Set the background color
    pub fn background(mut self, bg: impl Into<String>) -> Self {
        self.background = bg.into();
        self
    }

    /// Set the padding
    pub fn padding(mut self, padding: impl Into<String>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Add a custom CSS class
    pub fn custom_class(mut self, class: impl Into<String>) -> Self {
        self.custom_class = Some(class.into());
        self
    }

    /// Set ARIA attributes
    pub fn aria(mut self, aria: AriaAttrs) -> Self {
        self.aria = aria;
        self
    }

    /// Build CSS custom properties style
    fn build_style(&self) -> String {
        format!(
            "--magic-gradient-size: {}px; --magic-gradient-color: {}; --magic-gradient-opacity: {}; \
             --magic-border-from: {}; --magic-border-to: {}; --magic-border-width: {}px; \
             --magic-border-radius: {}; --magic-bg: {}; --magic-padding: {};",
            self.gradient_size,
            self.gradient_color,
            self.gradient_opacity,
            self.border_from,
            self.border_to,
            self.border_width,
            self.border_radius,
            self.background,
            self.padding
        )
    }
}

impl Render for MagicCard {
    fn render(&self) -> Markup {
        let class = format!(
            "sh-magic-card {}",
            self.custom_class.as_deref().unwrap_or("")
        );

        html! {
            div
                class=(class)
                style=(self.build_style())
                aria-label=[self.aria.label.clone()]
            {
                div class="sh-magic-card__border" {}
                div class="sh-magic-card__spotlight" {}
                div class="sh-magic-card__content" {
                    (self.content.clone())
                }
            }
        }
    }
}

impl Component for MagicCard {
    fn classes(&self) -> String {
        format!(
            "sh-magic-card {}",
            self.custom_class.as_deref().unwrap_or("")
        )
    }
}

/// Generate CSS for MagicCard
pub fn magic_card_css() -> String {
    r#"
/* Magic Card Component */
.sh-magic-card {
  position: relative;
  border-radius: var(--magic-border-radius, var(--sh-radius-lg));
  background: var(--magic-bg, var(--sh-surface));
  overflow: hidden;
  isolation: isolate;
}

.sh-magic-card__border {
  position: absolute;
  inset: 0;
  border-radius: inherit;
  padding: var(--magic-border-width, 1px);
  background: linear-gradient(
    135deg,
    var(--magic-border-from, #9E7AFF),
    var(--magic-border-to, #FE8BBB)
  );
  -webkit-mask:
    linear-gradient(#fff 0 0) content-box,
    linear-gradient(#fff 0 0);
  -webkit-mask-composite: xor;
  mask-composite: exclude;
  opacity: 0.5;
  transition: opacity 0.3s ease;
}

.sh-magic-card:hover .sh-magic-card__border {
  opacity: 1;
}

.sh-magic-card__spotlight {
  position: absolute;
  inset: 0;
  border-radius: inherit;
  background: radial-gradient(
    var(--magic-gradient-size, 200px) circle at var(--mouse-x, 50%) var(--mouse-y, 50%),
    color-mix(in srgb, var(--magic-gradient-color, #262626) calc(var(--magic-gradient-opacity, 0.8) * 100%), transparent),
    transparent 100%
  );
  opacity: 0;
  transition: opacity 0.3s ease;
  pointer-events: none;
}

.sh-magic-card:hover .sh-magic-card__spotlight {
  opacity: 1;
}

.sh-magic-card__content {
  position: relative;
  z-index: 1;
  padding: var(--magic-padding, 1.5rem);
  border-radius: inherit;
  background: var(--magic-bg, var(--sh-surface));
}

/* Size variants */
.sh-magic-card--sm {
  --magic-padding: 1rem;
}

.sh-magic-card--lg {
  --magic-padding: 2rem;
}

/* Preset color combinations */
.sh-magic-card--purple {
  --magic-border-from: #9E7AFF;
  --magic-border-to: #FE8BBB;
}

.sh-magic-card--blue {
  --magic-border-from: #00D9FF;
  --magic-border-to: #0066FF;
}

.sh-magic-card--green {
  --magic-border-from: #00FF94;
  --magic-border-to: #00CC66;
}

.sh-magic-card--orange {
  --magic-border-from: #FFB800;
  --magic-border-to: #FF6600;
}

/* Animation variant */
.sh-magic-card--animated .sh-magic-card__border {
  animation: magic-border-rotate 4s linear infinite;
  background: conic-gradient(
    from 0deg,
    var(--magic-border-from, #9E7AFF),
    var(--magic-border-to, #FE8BBB),
    var(--magic-border-from, #9E7AFF)
  );
}

@keyframes magic-border-rotate {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}
"#.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magic_card_creation() {
        let card = MagicCard::new(html! { "Content" })
            .gradient_size(300)
            .gradient_color("#333333")
            .gradient_opacity(0.5)
            .border_gradient("#FF0000", "#00FF00");

        assert_eq!(card.gradient_size, 300);
        assert_eq!(card.gradient_color, "#333333");
        assert_eq!(card.gradient_opacity, 0.5);
        assert_eq!(card.border_from, "#FF0000");
        assert_eq!(card.border_to, "#00FF00");
    }

    #[test]
    fn test_magic_card_css_generation() {
        let css = magic_card_css();
        assert!(css.contains(".sh-magic-card"));
        assert!(css.contains(".sh-magic-card__border"));
        assert!(css.contains(".sh-magic-card__spotlight"));
    }
}
