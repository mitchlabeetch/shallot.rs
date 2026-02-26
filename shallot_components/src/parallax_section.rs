//! Parallax Section Component - Scroll-based parallax effect

use maud::{html, Markup, Render};

/// Parallax speed multiplier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ParallaxSpeed {
    Slow,
    #[default]
    Medium,
    Fast,
}

/// Parallax direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ParallaxDirection {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

/// Parallax Section Component
#[derive(Debug, Clone)]
pub struct ParallaxSection<'a> {
    pub children: Markup,
    pub speed: ParallaxSpeed,
    pub direction: ParallaxDirection,
    pub height: &'a str,
    pub background: Option<&'a str>,
    pub overlay: bool,
    pub overlay_opacity: f32,
}

impl<'a> ParallaxSection<'a> {
    pub fn new(children: Markup) -> Self {
        Self {
            children,
            speed: ParallaxSpeed::default(),
            direction: ParallaxDirection::default(),
            height: "100vh",
            background: None,
            overlay: false,
            overlay_opacity: 0.5,
        }
    }

    pub fn speed(mut self, speed: ParallaxSpeed) -> Self {
        self.speed = speed;
        self
    }

    pub fn direction(mut self, direction: ParallaxDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn height(mut self, height: &'a str) -> Self {
        self.height = height;
        self
    }

    pub fn background(mut self, bg: &'a str) -> Self {
        self.background = Some(bg);
        self
    }

    pub fn overlay(mut self, overlay: bool) -> Self {
        self.overlay = overlay;
        self
    }

    pub fn overlay_opacity(mut self, opacity: f32) -> Self {
        self.overlay_opacity = opacity.clamp(0.0, 1.0);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-parallax"];

        match self.speed {
            ParallaxSpeed::Slow => classes.push("sh-parallax--slow"),
            ParallaxSpeed::Medium => classes.push("sh-parallax--medium"),
            ParallaxSpeed::Fast => classes.push("sh-parallax--fast"),
        }

        match self.direction {
            ParallaxDirection::Up => classes.push("sh-parallax--up"),
            ParallaxDirection::Down => classes.push("sh-parallax--down"),
            ParallaxDirection::Left => classes.push("sh-parallax--left"),
            ParallaxDirection::Right => classes.push("sh-parallax--right"),
        }

        if self.overlay {
            classes.push("sh-parallax--overlay");
        }

        classes.join(" ")
    }
}

impl<'a> Render for ParallaxSection<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let style = format!(
            "--sh-parallax-height: {}; {}",
            self.height,
            self.background
                .map(|bg| format!("--sh-parallax-bg: {};", bg))
                .unwrap_or_default()
        );
        let overlay_style = format!("--sh-parallax-overlay-opacity: {};", self.overlay_opacity);

        html! {
            section class=(classes) style=(style) {
                @if self.background.is_some() {
                    div class="sh-parallax__bg" {}
                }
                @if self.overlay {
                    div class="sh-parallax__overlay" style=(overlay_style) {}
                }
                div class="sh-parallax__content" {
                    (self.children)
                }
            }
        }
    }
}

/// Parallax Layer for multi-layer parallax
#[derive(Debug, Clone)]
pub struct ParallaxLayer<'a> {
    pub children: Markup,
    pub depth: f32,
    pub offset_y: &'a str,
}

impl<'a> ParallaxLayer<'a> {
    pub fn new(children: Markup) -> Self {
        Self {
            children,
            depth: 0.5,
            offset_y: "0",
        }
    }

    pub fn depth(mut self, depth: f32) -> Self {
        self.depth = depth.clamp(0.0, 1.0);
        self
    }

    pub fn offset_y(mut self, offset: &'a str) -> Self {
        self.offset_y = offset;
        self
    }
}

impl<'a> Render for ParallaxLayer<'a> {
    fn render(&self) -> Markup {
        let style = format!(
            "--sh-parallax-depth: {}; --sh-parallax-offset-y: {};",
            self.depth, self.offset_y
        );

        html! {
            div class="sh-parallax-layer" style=(style) {
                (self.children)
            }
        }
    }
}

/// Generate CSS for parallax components
pub fn parallax_section_css() -> String {
    r#"
/* Parallax Section Styles */
.sh-parallax {
    position: relative;
    overflow: hidden;
    height: var(--sh-parallax-height, 100vh);
}

.sh-parallax__bg {
    position: absolute;
    inset: -50%;
    width: 200%;
    height: 200%;
    background-image: var(--sh-parallax-bg);
    background-size: cover;
    background-position: center;
    will-change: transform;
}

.sh-parallax--slow .sh-parallax__bg {
    animation: parallax-scroll-slow 1s linear infinite;
    animation-play-state: paused;
    animation-delay: calc(var(--scroll-position, 0) * -0.1s);
}

.sh-parallax--medium .sh-parallax__bg {
    animation: parallax-scroll-medium 1s linear infinite;
    animation-play-state: paused;
    animation-delay: calc(var(--scroll-position, 0) * -0.2s);
}

.sh-parallax--fast .sh-parallax__bg {
    animation: parallax-scroll-fast 1s linear infinite;
    animation-play-state: paused;
    animation-delay: calc(var(--scroll-position, 0) * -0.3s);
}

.sh-parallax__overlay {
    position: absolute;
    inset: 0;
    background: rgba(0, 0, 0, var(--sh-parallax-overlay-opacity, 0.5));
    pointer-events: none;
}

.sh-parallax__content {
    position: relative;
    z-index: 1;
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
}

/* Direction variants for content */
.sh-parallax--up .sh-parallax__content {
    animation: content-float-up 2s ease-in-out infinite;
}

.sh-parallax--down .sh-parallax__content {
    animation: content-float-down 2s ease-in-out infinite;
}

/* Parallax Layer */
.sh-parallax-layer {
    position: absolute;
    will-change: transform;
    transform: translateY(calc(var(--sh-parallax-offset-y, 0) * var(--sh-parallax-depth, 0.5)));
}

/* Parallax keyframes */
@keyframes parallax-scroll-slow {
    from { transform: translateY(0); }
    to { transform: translateY(-10%); }
}

@keyframes parallax-scroll-medium {
    from { transform: translateY(0); }
    to { transform: translateY(-20%); }
}

@keyframes parallax-scroll-fast {
    from { transform: translateY(0); }
    to { transform: translateY(-30%); }
}

@keyframes content-float-up {
    0%, 100% { transform: translateY(0); }
    50% { transform: translateY(-10px); }
}

@keyframes content-float-down {
    0%, 100% { transform: translateY(0); }
    50% { transform: translateY(10px); }
}

/* Reduced motion */
@media (prefers-reduced-motion: reduce) {
    .sh-parallax__bg,
    .sh-parallax__content,
    .sh-parallax-layer {
        animation: none !important;
        transform: none !important;
    }
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use maud::html;

    #[test]
    fn test_parallax_section_creation() {
        let section = ParallaxSection::new(html! { "Content" })
            .speed(ParallaxSpeed::Fast)
            .height("80vh");

        assert_eq!(section.height, "80vh");
        assert_eq!(section.speed, ParallaxSpeed::Fast);
    }

    #[test]
    fn test_parallax_direction() {
        let section = ParallaxSection::new(html! { "Content" }).direction(ParallaxDirection::Left);

        assert_eq!(section.direction, ParallaxDirection::Left);
    }

    #[test]
    fn test_parallax_overlay() {
        let section = ParallaxSection::new(html! { "Content" })
            .overlay(true)
            .overlay_opacity(0.7);

        assert!(section.overlay);
        assert_eq!(section.overlay_opacity, 0.7);
    }

    #[test]
    fn test_parallax_layer() {
        let layer = ParallaxLayer::new(html! { "Layer" })
            .depth(0.8)
            .offset_y("50px");

        assert_eq!(layer.depth, 0.8);
        assert_eq!(layer.offset_y, "50px");
    }
}
