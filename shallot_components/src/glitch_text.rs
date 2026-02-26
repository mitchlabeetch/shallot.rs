//! Glitch Text Component - Glitch animation effect

use maud::{html, Markup, Render};

/// Glitch effect intensity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GlitchIntensity {
    Subtle,
    #[default]
    Medium,
    Intense,
}

/// Glitch color scheme
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GlitchColor {
    #[default]
    CyanRed,
    GreenMagenta,
    Custom(&'static str, &'static str),
}

/// Glitch Text Component
#[derive(Debug, Clone)]
pub struct GlitchText<'a> {
    pub text: &'a str,
    pub intensity: GlitchIntensity,
    pub color: GlitchColor,
    pub animated: bool,
    pub tag: &'a str,
}

impl<'a> GlitchText<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            intensity: GlitchIntensity::default(),
            color: GlitchColor::default(),
            animated: true,
            tag: "span",
        }
    }

    pub fn intensity(mut self, intensity: GlitchIntensity) -> Self {
        self.intensity = intensity;
        self
    }

    pub fn color(mut self, color: GlitchColor) -> Self {
        self.color = color;
        self
    }

    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    pub fn tag(mut self, tag: &'a str) -> Self {
        self.tag = tag;
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-glitch-text"];

        match self.intensity {
            GlitchIntensity::Subtle => classes.push("sh-glitch-text--subtle"),
            GlitchIntensity::Medium => classes.push("sh-glitch-text--medium"),
            GlitchIntensity::Intense => classes.push("sh-glitch-text--intense"),
        }

        if !self.animated {
            classes.push("sh-glitch-text--static");
        }

        classes.join(" ")
    }

    fn color_values(&self) -> (&'static str, &'static str) {
        match self.color {
            GlitchColor::CyanRed => ("#00ffff", "#ff00ff"),
            GlitchColor::GreenMagenta => ("#00ff00", "#ff0080"),
            GlitchColor::Custom(c1, c2) => (c1, c2),
        }
    }
}

impl<'a> Render for GlitchText<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let (color1, color2) = self.color_values();
        let style = format!(
            "--sh-glitch-color-1: {}; --sh-glitch-color-2: {};",
            color1, color2
        );
        let tag = self.tag;

        html! {
            (maud::PreEscaped(format!(
                r#"<{tag} class="{classes}" style="{style}" data-text="{text}">{text}</{tag}>"#,
                tag = tag,
                classes = classes,
                style = style,
                text = self.text
            )))
        }
    }
}

/// Generate CSS for glitch text components
pub fn glitch_text_css() -> String {
    r#"
/* Glitch Text Styles */
.sh-glitch-text {
    position: relative;
    display: inline-block;
}

.sh-glitch-text::before,
.sh-glitch-text::after {
    content: attr(data-text);
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    opacity: 0.8;
}

.sh-glitch-text::before {
    color: var(--sh-glitch-color-1);
    animation: glitch-1 0.3s infinite linear alternate-reverse;
    clip-path: polygon(0 0, 100% 0, 100% 45%, 0 45%);
}

.sh-glitch-text::after {
    color: var(--sh-glitch-color-2);
    animation: glitch-2 0.3s infinite linear alternate-reverse;
    clip-path: polygon(0 55%, 100% 55%, 100% 100%, 0 100%);
}

/* Intensity variants */
.sh-glitch-text--subtle::before,
.sh-glitch-text--subtle::after {
    animation-duration: 0.5s;
}

.sh-glitch-text--medium::before,
.sh-glitch-text--medium::after {
    animation-duration: 0.3s;
}

.sh-glitch-text--intense::before,
.sh-glitch-text--intense::after {
    animation-duration: 0.15s;
}

/* Static (hover-only) variant */
.sh-glitch-text--static::before,
.sh-glitch-text--static::after {
    animation: none;
    opacity: 0;
}

.sh-glitch-text--static:hover::before,
.sh-glitch-text--static:hover::after {
    opacity: 0.8;
    animation: glitch-1 0.3s infinite linear alternate-reverse;
}

/* Glitch animations */
@keyframes glitch-1 {
    0% {
        transform: translateX(0);
    }
    20% {
        transform: translateX(-2px);
    }
    40% {
        transform: translateX(2px);
    }
    60% {
        transform: translateX(-1px);
    }
    80% {
        transform: translateX(1px);
    }
    100% {
        transform: translateX(0);
    }
}

@keyframes glitch-2 {
    0% {
        transform: translateX(0);
    }
    20% {
        transform: translateX(2px);
    }
    40% {
        transform: translateX(-2px);
    }
    60% {
        transform: translateX(1px);
    }
    80% {
        transform: translateX(-1px);
    }
    100% {
        transform: translateX(0);
    }
}

/* Reduced motion */
@media (prefers-reduced-motion: reduce) {
    .sh-glitch-text::before,
    .sh-glitch-text::after {
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
    fn test_glitch_text_creation() {
        let glitch = GlitchText::new("ERROR").intensity(GlitchIntensity::Intense);

        assert_eq!(glitch.text, "ERROR");
        assert_eq!(glitch.intensity, GlitchIntensity::Intense);
    }

    #[test]
    fn test_glitch_colors() {
        let glitch = GlitchText::new("Test").color(GlitchColor::GreenMagenta);

        let (c1, c2) = glitch.color_values();
        assert_eq!(c1, "#00ff00");
        assert_eq!(c2, "#ff0080");
    }

    #[test]
    fn test_glitch_static() {
        let glitch = GlitchText::new("Static").animated(false);

        assert!(!glitch.animated);
    }
}
