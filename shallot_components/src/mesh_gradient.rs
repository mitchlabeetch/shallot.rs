
//! MeshGradientBackground Component - Procedural Animated Gradient
//!
//! A beautiful animated mesh gradient background using CSS only.
//! Creates a flowing, organic gradient effect.

use maud::{html, Markup, Render};

/// MeshGradient size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MeshGradientSize {
    Small,
    #[default]
    Medium,
    Large,
    Full,
}

impl MeshGradientSize {
    fn css_class(&self) -> &'static str {
        match self {
            MeshGradientSize::Small => "sh-meshgradient--sm",
            MeshGradientSize::Medium => "sh-meshgradient--md",
            MeshGradientSize::Large => "sh-meshgradient--lg",
            MeshGradientSize::Full => "sh-meshgradient--full",
        }
    }
}

/// MeshGradientBackground component
pub struct MeshGradientBackground<'a> {
    colors: Vec<&'a str>,
    size: MeshGradientSize,
    animated: bool,
    speed: MeshGradientSpeed,
    class: Option<&'a str>,
}

impl<'a> MeshGradientBackground<'a> {
    /// Create a new MeshGradientBackground with default colors
    pub fn new() -> Self {
        Self {
            colors: vec!["#667eea", "#764ba2", "#f093fb"],
            size: MeshGradientSize::default(),
            animated: true,
            speed: MeshGradientSpeed::default(),
            class: None,
        }
    }

    /// Set custom colors
    pub fn colors(mut self, colors: Vec<&'a str>) -> Self {
        self.colors = colors;
        self
    }

    /// Set size
    pub fn size(mut self, size: MeshGradientSize) -> Self {
        self.size = size;
        self
    }

    /// Enable/disable animation
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Set animation speed
    pub fn speed(mut self, speed: MeshGradientSpeed) -> Self {
        self.speed = speed;
        self
    }

    /// Add custom class
    pub fn class(mut self, class: &'a str) -> Self {
        self.class = Some(class);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-meshgradient".to_string()];
        classes.push(self.size.css_class().to_string());
        if !self.animated {
            classes.push("sh-meshgradient--static".to_string());
        }
        classes.push(format!("sh-meshgradient--{}", self.speed.css_class()));
        if let Some(custom) = self.class {
            classes.push(custom.to_string());
        }
        classes.join(" ")
    }
}

impl<'a> Default for MeshGradientBackground<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Render for MeshGradientBackground<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let gradient_id = "mesh-gradient";

        html! {
            div class=(classes) aria-hidden="true" {
                svg class="sh-meshgradient__svg" viewBox="0 0 100 100" preserveAspectRatio="none" {
                    defs {
                        radialGradient id=(format!("{}-1", gradient_id)) cx="50%" cy="50%" r="60%" {
                            @for (i, color) in self.colors.iter().enumerate() {
                                stop offset=(format!("{}%", i * 25)) style=(format!("stop-color: {}", color));
                            }
                        }
                        radialGradient id=(format!("{}-2", gradient_id)) cx="80%" cy="20%" r="50%" {
                            @for (i, color) in self.colors.iter().enumerate() {
                                stop offset=(format!("{}%", i * 25)) style=(format!("stop-color: {}", color));
                            }
                        }
                        radialGradient id=(format!("{}-3", gradient_id)) cx="20%" cy="80%" r="50%" {
                            @for (i, color) in self.colors.iter().enumerate() {
                                stop offset=(format!("{}%", i * 25)) style=(format!("stop-color: {}", color));
                            }
                        }
                    }
                    rect width="100" height="100" fill=(format!("url(#{}-1)", gradient_id));
                    rect width="100" height="100" fill=(format!("url(#{}-2)", gradient_id)) opacity="0.5";
                    rect width="100" height="100" fill=(format!("url(#{}-3)", gradient_id)) opacity="0.5";
                }
            }
        }
    }
}

/// Animation speed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MeshGradientSpeed {
    Slow,
    #[default]
    Normal,
    Fast,
}

impl MeshGradientSpeed {
    fn css_class(&self) -> &'static str {
        match self {
            MeshGradientSpeed::Slow => "slow",
            MeshGradientSpeed::Normal => "normal",
            MeshGradientSpeed::Fast => "fast",
        }
    }
}

/// Generate CSS for MeshGradientBackground component
pub fn mesh_gradient_css() -> String {
    r#"
.sh-meshgradient {
    position: relative;
    width: 100%;
    height: 100%;
    overflow: hidden;
}

.sh-meshgradient__svg {
    width: 100%;
    height: 100%;
    filter: blur(40px) contrast(1.2);
}

/* Size variants */
.sh-meshgradient--sm {
    height: 12rem;
}

.sh-meshgradient--md {
    height: 20rem;
}

.sh-meshgradient--lg {
    height: 32rem;
}

.sh-meshgradient--full {
    position: fixed;
    inset: 0;
    z-index: -1;
}

/* Animation */
.sh-meshgradient:not(.sh-meshgradient--static) .sh-meshgradient__svg {
    animation: sh-mesh-flow var(--sh-mesh-speed, 15s) ease-in-out infinite;
}

.sh-meshgradient--slow {
    --sh-mesh-speed: 25s;
}

.sh-meshgradient--normal {
    --sh-mesh-speed: 15s;
}

.sh-meshgradient--fast {
    --sh-mesh-speed: 8s;
}

@keyframes sh-mesh-flow {
    0%, 100% {
        transform: scale(1) rotate(0deg);
    }
    25% {
        transform: scale(1.1) rotate(2deg);
    }
    50% {
        transform: scale(0.95) rotate(-1deg);
    }
    75% {
        transform: scale(1.05) rotate(1deg);
    }
}

/* Static variant */
.sh-meshgradient--static .sh-meshgradient__svg {
    animation: none;
}

/* Reduced motion */
@media (prefers-reduced-motion: reduce) {
    .sh-meshgradient__svg {
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
    fn test_meshgradient_creation() {
        let gradient = MeshGradientBackground::new();
        assert_eq!(gradient.colors.len(), 3);
        assert!(gradient.animated);
    }

    #[test]
    fn test_meshgradient_colors() {
        let gradient = MeshGradientBackground::new().colors(vec!["#ff0", "#0f0", "#00f"]);
        assert_eq!(gradient.colors.len(), 3);
    }

    #[test]
    fn test_meshgradient_size() {
        let gradient = MeshGradientBackground::new().size(MeshGradientSize::Large);
        assert_eq!(gradient.size, MeshGradientSize::Large);
    }

    #[test]
    fn test_meshgradient_animated() {
        let gradient = MeshGradientBackground::new().animated(false);
        assert!(!gradient.animated);
    }

    #[test]
    fn test_meshgradient_css() {
        let css = mesh_gradient_css();
        assert!(css.contains(".sh-meshgradient"));
        assert!(css.contains("@keyframes sh-mesh-flow"));
    }
}
