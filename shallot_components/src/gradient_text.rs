//! Gradient Text Component - Animated gradient text effect
//! Pure CSS animation, no JavaScript

use maud::{html, Markup, Render};

/// Gradient direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GradientDirection {
    #[default]
    Horizontal,
    Vertical,
    Diagonal,
    Radial,
}

/// Gradient speed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GradientSpeed {
    Slow,
    #[default]
    Normal,
    Fast,
}

/// Gradient text component
#[derive(Debug, Clone)]
pub struct GradientText<'a> {
    pub text: &'a str,
    pub colors: Vec<&'a str>,
    pub direction: GradientDirection,
    pub speed: GradientSpeed,
    pub animated: bool,
    pub tag: &'a str,
}

impl<'a> GradientText<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            colors: vec!["#3b82f6", "#8b5cf6", "#ec4899"],
            direction: GradientDirection::default(),
            speed: GradientSpeed::default(),
            animated: true,
            tag: "span",
        }
    }

    pub fn colors(mut self, colors: Vec<&'a str>) -> Self {
        self.colors = colors;
        self
    }

    pub fn direction(mut self, direction: GradientDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn speed(mut self, speed: GradientSpeed) -> Self {
        self.speed = speed;
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
        let mut classes = vec!["sh-gradient-text"];

        match self.direction {
            GradientDirection::Horizontal => classes.push("sh-gradient-text--horizontal"),
            GradientDirection::Vertical => classes.push("sh-gradient-text--vertical"),
            GradientDirection::Diagonal => classes.push("sh-gradient-text--diagonal"),
            GradientDirection::Radial => classes.push("sh-gradient-text--radial"),
        }

        match self.speed {
            GradientSpeed::Slow => classes.push("sh-gradient-text--slow"),
            GradientSpeed::Normal => classes.push("sh-gradient-text--normal"),
            GradientSpeed::Fast => classes.push("sh-gradient-text--fast"),
        }

        if !self.animated {
            classes.push("sh-gradient-text--static");
        }

        classes.join(" ")
    }

    fn build_gradient(&self) -> String {
        let colors_str = self.colors.join(", ");

        match self.direction {
            GradientDirection::Horizontal => format!("linear-gradient(90deg, {})", colors_str),
            GradientDirection::Vertical => format!("linear-gradient(180deg, {})", colors_str),
            GradientDirection::Diagonal => format!("linear-gradient(135deg, {})", colors_str),
            GradientDirection::Radial => format!("radial-gradient(circle, {})", colors_str),
        }
    }
}

impl<'a> Render for GradientText<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let gradient = self.build_gradient();
        let tag = self.tag;

        html! {
            @match tag {
                "h1" => h1 class=(classes) style=(format!("background: {}", gradient)) aria-label=(self.text) {
                    (self.text)
                },
                "h2" => h2 class=(classes) style=(format!("background: {}", gradient)) aria-label=(self.text) {
                    (self.text)
                },
                "h3" => h3 class=(classes) style=(format!("background: {}", gradient)) aria-label=(self.text) {
                    (self.text)
                },
                "h4" => h4 class=(classes) style=(format!("background: {}", gradient)) aria-label=(self.text) {
                    (self.text)
                },
                "p" => p class=(classes) style=(format!("background: {}", gradient)) aria-label=(self.text) {
                    (self.text)
                },
                _ => span class=(classes) style=(format!("background: {}", gradient)) role="text" aria-label=(self.text) {
                    (self.text)
                },
            }
        }
    }
}

pub fn gradient_text_css() -> String {
    r#"
.sh-gradient-text {
    display: inline;
    background-clip: text;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-size: 200% 200%;
}

.sh-gradient-text--static {
    background-size: 100% 100%;
}

.sh-gradient-text--slow {
    animation: sh-gradient-shift 8s ease infinite;
}

.sh-gradient-text--normal {
    animation: sh-gradient-shift 4s ease infinite;
}

.sh-gradient-text--fast {
    animation: sh-gradient-shift 2s ease infinite;
}

@keyframes sh-gradient-shift {
    0% {
        background-position: 0% 50%;
    }
    50% {
        background-position: 100% 50%;
    }
    100% {
        background-position: 0% 50%;
    }
}

.sh-gradient-text--horizontal {
    background-size: 200% 100%;
}

.sh-gradient-text--vertical {
    background-size: 100% 200%;
    animation-name: sh-gradient-shift-vertical;
}

@keyframes sh-gradient-shift-vertical {
    0% {
        background-position: 50% 0%;
    }
    50% {
        background-position: 50% 100%;
    }
    100% {
        background-position: 50% 0%;
    }
}

.sh-gradient-text--diagonal {
    background-size: 200% 200%;
}

.sh-gradient-text--radial {
    background-size: 150% 150%;
    animation-name: sh-gradient-shift-radial;
}

@keyframes sh-gradient-shift-radial {
    0% {
        background-position: 0% 0%;
    }
    50% {
        background-position: 100% 100%;
    }
    100% {
        background-position: 0% 0%;
    }
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gradient_text_creation() {
        let text = GradientText::new("Hello World").colors(vec!["#ff0000", "#00ff00", "#0000ff"]);

        assert_eq!(text.text, "Hello World");
        assert_eq!(text.colors.len(), 3);
    }

    #[test]
    fn test_gradient_text_render() {
        let text = GradientText::new("Gradient!");
        let rendered = text.render();
        let html = rendered.into_string();

        assert!(html.contains("sh-gradient-text"));
        assert!(html.contains("Gradient!"));
    }

    #[test]
    fn test_gradient_directions() {
        let horizontal = GradientText::new("H").direction(GradientDirection::Horizontal);
        let diagonal = GradientText::new("D").direction(GradientDirection::Diagonal);

        assert!(horizontal
            .build_classes()
            .contains("sh-gradient-text--horizontal"));
        assert!(diagonal
            .build_classes()
            .contains("sh-gradient-text--diagonal"));
    }

    #[test]
    fn test_gradient_speed() {
        let slow = GradientText::new("Slow").speed(GradientSpeed::Slow);
        let fast = GradientText::new("Fast").speed(GradientSpeed::Fast);

        assert!(slow.build_classes().contains("sh-gradient-text--slow"));
        assert!(fast.build_classes().contains("sh-gradient-text--fast"));
    }

    #[test]
    fn test_static_gradient() {
        let static_text = GradientText::new("Static").animated(false);

        assert!(static_text
            .build_classes()
            .contains("sh-gradient-text--static"));
    }

    #[test]
    fn test_css_generation() {
        let css = gradient_text_css();
        assert!(css.contains(".sh-gradient-text"));
        assert!(css.contains("@keyframes"));
    }
}
