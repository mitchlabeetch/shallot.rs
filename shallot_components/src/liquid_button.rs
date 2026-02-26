//! LiquidButton Component - SVG Filter Warp Animation
//!
//! A button with a liquid/warping effect using SVG filters.
//! Creates a mesmerizing fluid animation on hover.

use maud::{html, Markup, Render};

/// LiquidButton size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LiquidButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl LiquidButtonSize {
    fn css_class(&self) -> &'static str {
        match self {
            LiquidButtonSize::Small => "sh-liquidbtn--sm",
            LiquidButtonSize::Medium => "sh-liquidbtn--md",
            LiquidButtonSize::Large => "sh-liquidbtn--lg",
        }
    }
}

/// LiquidButton component
pub struct LiquidButton<'a> {
    label: &'a str,
    href: Option<&'a str>,
    size: LiquidButtonSize,
    disabled: bool,
    class: Option<&'a str>,
}

impl<'a> LiquidButton<'a> {
    /// Create a new LiquidButton
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            href: None,
            size: LiquidButtonSize::default(),
            disabled: false,
            class: None,
        }
    }

    /// Set as link
    pub fn href(mut self, href: &'a str) -> Self {
        self.href = Some(href);
        self
    }

    /// Set size
    pub fn size(mut self, size: LiquidButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Set disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Add custom class
    pub fn class(mut self, class: &'a str) -> Self {
        self.class = Some(class);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-liquidbtn".to_string()];
        classes.push(self.size.css_class().to_string());
        if self.disabled {
            classes.push("sh-liquidbtn--disabled".to_string());
        }
        if let Some(custom) = self.class {
            classes.push(custom.to_string());
        }
        classes.join(" ")
    }
}

impl<'a> Render for LiquidButton<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let filter_id = "liquid-filter";

        html! {
            @if let Some(href) = self.href {
                a
                    class=(classes)
                    href=(if self.disabled { "#" } else { href })
                    aria-disabled=[if self.disabled { Some("true") } else { None }]
                    tabindex=[if self.disabled { Some("-1") } else { None }]
                {
                    (self.render_content(&filter_id))
                }
            } @else {
                button
                    class=(classes)
                    type="button"
                    disabled?[self.disabled]
                {
                    (self.render_content(&filter_id))
                }
            }
        }
    }
}

impl<'a> LiquidButton<'a> {
    fn render_content(&self, filter_id: &str) -> Markup {
        html! {
            span class="sh-liquidbtn__content" {
                (self.label)
            }
            svg class="sh-liquidbtn__svg" aria-hidden="true" {
                defs {
                    filter id=(filter_id) {
                        feTurbulence
                            type="fractalNoise"
                            baseFrequency="0.015"
                            numOctaves="3"
                            result="noise"
                        {}
                        feDisplacementMap
                            in="SourceGraphic"
                            in2="noise"
                            scale="10"
                            xChannelSelector="R"
                            yChannelSelector="G"
                        {}
                    }
                }
            }
        }
    }
}

/// Generate CSS for LiquidButton component
pub fn liquid_button_css() -> String {
    r#"
.sh-liquidbtn {
    position: relative;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0.875rem 2rem;
    font-size: 1rem;
    font-weight: 600;
    color: white;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    border: none;
    border-radius: var(--sh-radius-full, 9999px);
    cursor: pointer;
    text-decoration: none;
    overflow: visible;
    transition: transform 0.3s ease, box-shadow 0.3s ease;
    isolation: isolate;
}

.sh-liquidbtn:hover:not(.sh-liquidbtn--disabled) {
    transform: translateY(-2px);
    box-shadow: 0 10px 40px rgba(102, 126, 234, 0.4);
}

.sh-liquidbtn:active:not(.sh-liquidbtn--disabled) {
    transform: translateY(0);
}

.sh-liquidbtn__content {
    position: relative;
    z-index: 2;
    pointer-events: none;
}

.sh-liquidbtn__svg {
    position: absolute;
    inset: -20%;
    width: 140%;
    height: 140%;
    opacity: 0;
    transition: opacity 0.3s ease;
    z-index: -1;
    pointer-events: none;
}

.sh-liquidbtn__svg filter {
    filter: url(#liquid-filter);
}

/* Liquid effect on hover */
.sh-liquidbtn:hover .sh-liquidbtn__svg {
    opacity: 1;
    animation: sh-liquid-warp 2s ease-in-out infinite;
}

@keyframes sh-liquid-warp {
    0%, 100% {
        transform: scale(1) rotate(0deg);
    }
    50% {
        transform: scale(1.05) rotate(2deg);
    }
}

/* Size variants */
.sh-liquidbtn--sm {
    padding: 0.625rem 1.5rem;
    font-size: 0.875rem;
}

.sh-liquidbtn--md {
    padding: 0.875rem 2rem;
    font-size: 1rem;
}

.sh-liquidbtn--lg {
    padding: 1.125rem 2.5rem;
    font-size: 1.125rem;
}

/* Disabled state */
.sh-liquidbtn--disabled {
    opacity: 0.5;
    cursor: not-allowed;
    pointer-events: none;
}

/* Focus state */
.sh-liquidbtn:focus-visible {
    outline: 2px solid var(--sh-primary, #667eea);
    outline-offset: 2px;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_liquidbutton_creation() {
        let btn = LiquidButton::new("Click me");
        assert_eq!(btn.label, "Click me");
        assert_eq!(btn.size, LiquidButtonSize::Medium);
    }

    #[test]
    fn test_liquidbutton_href() {
        let btn = LiquidButton::new("Link").href("/page");
        assert_eq!(btn.href, Some("/page"));
    }

    #[test]
    fn test_liquidbutton_size() {
        let btn = LiquidButton::new("Big").size(LiquidButtonSize::Large);
        assert_eq!(btn.size, LiquidButtonSize::Large);
    }

    #[test]
    fn test_liquidbutton_disabled() {
        let btn = LiquidButton::new("Disabled").disabled(true);
        assert!(btn.disabled);
    }

    #[test]
    fn test_liquidbutton_css() {
        let css = liquid_button_css();
        assert!(css.contains(".sh-liquidbtn"));
        assert!(css.contains("@keyframes sh-liquid-warp"));
    }
}
