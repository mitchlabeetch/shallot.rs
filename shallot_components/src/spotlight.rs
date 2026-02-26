//! Spotlight Component - Spotlight hover effect for cards
//! Pure CSS effect using radial gradient

use maud::{html, Markup, Render};

/// Spotlight size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpotlightSize {
    Sm,
    #[default]
    Md,
    Lg,
    Full,
}

/// Spotlight card component
#[derive(Debug, Clone)]
pub struct Spotlight<'a> {
    pub children: Markup,
    pub size: SpotlightSize,
    pub color: &'a str,
    pub border_color: &'a str,
    pub rounded: bool,
    pub intensity: f32,
}

impl<'a> Spotlight<'a> {
    pub fn new(children: Markup) -> Self {
        Self {
            children,
            size: SpotlightSize::default(),
            color: "#3b82f6",
            border_color: "rgba(59, 130, 246, 0.3)",
            rounded: true,
            intensity: 0.15,
        }
    }

    pub fn size(mut self, size: SpotlightSize) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: &'a str) -> Self {
        self.color = color;
        self
    }

    pub fn border_color(mut self, color: &'a str) -> Self {
        self.border_color = color;
        self
    }

    pub fn rounded(mut self, rounded: bool) -> Self {
        self.rounded = rounded;
        self
    }

    pub fn intensity(mut self, intensity: f32) -> Self {
        self.intensity = intensity.clamp(0.0, 1.0);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-spotlight"];

        match self.size {
            SpotlightSize::Sm => classes.push("sh-spotlight--sm"),
            SpotlightSize::Md => classes.push("sh-spotlight--md"),
            SpotlightSize::Lg => classes.push("sh-spotlight--lg"),
            SpotlightSize::Full => classes.push("sh-spotlight--full"),
        }

        if self.rounded {
            classes.push("sh-spotlight--rounded");
        }

        classes.join(" ")
    }
}

impl<'a> Render for Spotlight<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let style = format!(
            "--sh-spotlight-color: {}; --sh-spotlight-border: {}; --sh-spotlight-intensity: {};",
            self.color, self.border_color, self.intensity
        );

        html! {
            div
                class=(classes)
                style=(style)
                role="region"
                aria-label="Spotlight effect"
            {
                div class="sh-spotlight__content" {
                    (self.children)
                }
                div class="sh-spotlight__overlay" aria-hidden="true" {}
            }
        }
    }
}

pub fn spotlight_css() -> String {
    r#"
.sh-spotlight {
    position: relative;
    overflow: hidden;
    isolation: isolate;
}

.sh-spotlight--sm {
    padding: var(--sh-spacing-sm, 0.5rem);
}

.sh-spotlight--md {
    padding: var(--sh-spacing-md, 1rem);
}

.sh-spotlight--lg {
    padding: var(--sh-spacing-lg, 1.5rem);
}

.sh-spotlight--full {
    width: 100%;
    height: 100%;
}

.sh-spotlight--rounded {
    border-radius: var(--sh-radius-lg, 0.5rem);
}

.sh-spotlight__content {
    position: relative;
    z-index: 1;
}

.sh-spotlight__overlay {
    position: absolute;
    inset: 0;
    z-index: 0;
    pointer-events: none;
    opacity: 0;
    background: radial-gradient(
        400px circle at var(--mouse-x, 50%) var(--mouse-y, 50%),
        var(--sh-spotlight-color, #3b82f6),
        transparent 40%
    );
    transition: opacity 0.3s ease;
}

.sh-spotlight:hover .sh-spotlight__overlay {
    opacity: var(--sh-spotlight-intensity, 0.15);
}

.sh-spotlight::before {
    content: "";
    position: absolute;
    inset: 0;
    border-radius: inherit;
    padding: 1px;
    background: linear-gradient(
        var(--border-angle, 0deg),
        var(--sh-spotlight-border, rgba(59, 130, 246, 0.3)),
        transparent 50%
    );
    -webkit-mask:
        linear-gradient(#fff 0 0) content-box,
        linear-gradient(#fff 0 0);
    mask:
        linear-gradient(#fff 0 0) content-box,
        linear-gradient(#fff 0 0);
    -webkit-mask-composite: xor;
    mask-composite: exclude;
    opacity: 0;
    transition: opacity 0.3s ease;
    pointer-events: none;
}

.sh-spotlight:hover::before {
    opacity: 1;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spotlight_creation() {
        let spotlight = Spotlight::new(html! { "Content" })
            .color("#ff0000")
            .intensity(0.5);

        assert_eq!(spotlight.color, "#ff0000");
        assert!((spotlight.intensity - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_spotlight_render() {
        let spotlight = Spotlight::new(html! { p { "Test" } });
        let rendered = spotlight.render();
        let html = rendered.into_string();

        assert!(html.contains("sh-spotlight"));
        assert!(html.contains("Test"));
    }

    #[test]
    fn test_spotlight_sizes() {
        let sm = Spotlight::new(html! {}).size(SpotlightSize::Sm);
        let lg = Spotlight::new(html! {}).size(SpotlightSize::Lg);

        assert!(sm.build_classes().contains("sh-spotlight--sm"));
        assert!(lg.build_classes().contains("sh-spotlight--lg"));
    }

    #[test]
    fn test_spotlight_rounded() {
        let rounded = Spotlight::new(html! {}).rounded(true);
        let not_rounded = Spotlight::new(html! {}).rounded(false);

        assert!(rounded.build_classes().contains("sh-spotlight--rounded"));
        assert!(!not_rounded
            .build_classes()
            .contains("sh-spotlight--rounded"));
    }

    #[test]
    fn test_intensity_clamping() {
        let spotlight = Spotlight::new(html! {}).intensity(1.5);
        assert!((spotlight.intensity - 1.0).abs() < f32::EPSILON);

        let spotlight2 = Spotlight::new(html! {}).intensity(-0.5);
        assert!((spotlight2.intensity - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_css_generation() {
        let css = spotlight_css();
        assert!(css.contains(".sh-spotlight"));
        assert!(css.contains(".sh-spotlight__overlay"));
    }
}
