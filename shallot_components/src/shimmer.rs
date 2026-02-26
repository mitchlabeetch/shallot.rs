//! Shimmer Component - Shimmer loading effect
//! CSS-only skeleton loading animation

use maud::{html, Markup, Render};

/// Shimmer shape variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ShimmerShape {
    #[default]
    Rect,
    Circle,
    Text,
    Button,
    Avatar,
    Card,
}

/// Shimmer size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ShimmerSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Single shimmer element
#[derive(Debug, Clone)]
pub struct Shimmer {
    pub shape: ShimmerShape,
    pub size: ShimmerSize,
    pub width: Option<String>,
    pub height: Option<String>,
    pub rounded: bool,
}

impl Shimmer {
    pub fn new() -> Self {
        Self {
            shape: ShimmerShape::default(),
            size: ShimmerSize::default(),
            width: None,
            height: None,
            rounded: true,
        }
    }

    pub fn shape(mut self, shape: ShimmerShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn size(mut self, size: ShimmerSize) -> Self {
        self.size = size;
        self
    }

    pub fn width(mut self, width: &str) -> Self {
        self.width = Some(width.to_string());
        self
    }

    pub fn height(mut self, height: &str) -> Self {
        self.height = Some(height.to_string());
        self
    }

    pub fn rounded(mut self, rounded: bool) -> Self {
        self.rounded = rounded;
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-shimmer"];

        match self.shape {
            ShimmerShape::Rect => classes.push("sh-shimmer--rect"),
            ShimmerShape::Circle => classes.push("sh-shimmer--circle"),
            ShimmerShape::Text => classes.push("sh-shimmer--text"),
            ShimmerShape::Button => classes.push("sh-shimmer--button"),
            ShimmerShape::Avatar => classes.push("sh-shimmer--avatar"),
            ShimmerShape::Card => classes.push("sh-shimmer--card"),
        }

        match self.size {
            ShimmerSize::Sm => classes.push("sh-shimmer--sm"),
            ShimmerSize::Md => classes.push("sh-shimmer--md"),
            ShimmerSize::Lg => classes.push("sh-shimmer--lg"),
        }

        if self.rounded {
            classes.push("sh-shimmer--rounded");
        }

        classes.join(" ")
    }

    fn build_style(&self) -> String {
        let mut styles = Vec::new();

        if let Some(ref width) = self.width {
            styles.push(format!("width: {};", width));
        }

        if let Some(ref height) = self.height {
            styles.push(format!("height: {};", height));
        }

        styles.join(" ")
    }
}

impl Default for Shimmer {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for Shimmer {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let style = self.build_style();

        html! {
            div class=(classes) style=(style) aria-hidden="true" {}
        }
    }
}

/// Shimmer group for multiple shimmer elements
#[derive(Debug, Clone)]
pub struct ShimmerGroup {
    pub shimmers: Vec<Shimmer>,
    pub direction: ShimmerDirection,
    pub gap: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ShimmerDirection {
    #[default]
    Vertical,
    Horizontal,
}

impl ShimmerGroup {
    pub fn new() -> Self {
        Self {
            shimmers: Vec::new(),
            direction: ShimmerDirection::default(),
            gap: "0.5rem".to_string(),
        }
    }

    pub fn add(mut self, shimmer: Shimmer) -> Self {
        self.shimmers.push(shimmer);
        self
    }

    pub fn direction(mut self, direction: ShimmerDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn gap(mut self, gap: &str) -> Self {
        self.gap = gap.to_string();
        self
    }

    fn build_classes(&self) -> String {
        match self.direction {
            ShimmerDirection::Vertical => "sh-shimmer-group sh-shimmer-group--vertical",
            ShimmerDirection::Horizontal => "sh-shimmer-group sh-shimmer-group--horizontal",
        }
        .to_string()
    }
}

impl Default for ShimmerGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for ShimmerGroup {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let style = format!("gap: {};", self.gap);

        html! {
            div class=(classes) style=(style) aria-hidden="true" {
                @for shimmer in &self.shimmers {
                    (shimmer.render())
                }
            }
        }
    }
}

pub fn shimmer_css() -> String {
    r#"
.sh-shimmer {
    position: relative;
    overflow: hidden;
    background: var(--sh-color-surface-muted, #e5e5e5);
}

.sh-shimmer::after {
    content: "";
    position: absolute;
    inset: 0;
    background: linear-gradient(
        90deg,
        transparent,
        var(--sh-color-surface, #f5f5f5),
        transparent
    );
    animation: sh-shimmer-wave 1.5s infinite;
}

@keyframes sh-shimmer-wave {
    0% {
        transform: translateX(-100%);
    }
    100% {
        transform: translateX(100%);
    }
}

.sh-shimmer--rounded {
    border-radius: var(--sh-radius-md, 0.375rem);
}

.sh-shimmer--sm {
    height: 1rem;
}

.sh-shimmer--md {
    height: 1.5rem;
}

.sh-shimmer--lg {
    height: 2rem;
}

.sh-shimmer--rect {
    width: 100%;
    height: 100%;
}

.sh-shimmer--circle {
    border-radius: 50%;
}

.sh-shimmer--text {
    width: 100%;
    border-radius: var(--sh-radius-sm, 0.25rem);
}

.sh-shimmer--button {
    width: 80px;
    height: 2.5rem;
    border-radius: var(--sh-radius-md, 0.375rem);
}

.sh-shimmer--avatar {
    width: 40px;
    height: 40px;
    border-radius: 50%;
}

.sh-shimmer--avatar.sh-shimmer--sm {
    width: 24px;
    height: 24px;
}

.sh-shimmer--avatar.sh-shimmer--lg {
    width: 56px;
    height: 56px;
}

.sh-shimmer--card {
    width: 100%;
    height: 200px;
    border-radius: var(--sh-radius-lg, 0.5rem);
}

.sh-shimmer-group {
    display: flex;
}

.sh-shimmer-group--vertical {
    flex-direction: column;
}

.sh-shimmer-group--horizontal {
    flex-direction: row;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shimmer_creation() {
        let shimmer = Shimmer::new()
            .shape(ShimmerShape::Circle)
            .size(ShimmerSize::Lg);

        assert_eq!(shimmer.shape, ShimmerShape::Circle);
        assert_eq!(shimmer.size, ShimmerSize::Lg);
    }

    #[test]
    fn test_shimmer_render() {
        let shimmer = Shimmer::new().width("200px");
        let rendered = shimmer.render();
        let html = rendered.into_string();

        assert!(html.contains("sh-shimmer"));
    }

    #[test]
    fn test_shimmer_group() {
        let group = ShimmerGroup::new()
            .add(Shimmer::new().shape(ShimmerShape::Text))
            .add(Shimmer::new().shape(ShimmerShape::Text))
            .gap("1rem");

        assert_eq!(group.shimmers.len(), 2);
    }

    #[test]
    fn test_shimmer_shapes() {
        let circle = Shimmer::new().shape(ShimmerShape::Circle);
        let avatar = Shimmer::new().shape(ShimmerShape::Avatar);

        assert!(circle.build_classes().contains("sh-shimmer--circle"));
        assert!(avatar.build_classes().contains("sh-shimmer--avatar"));
    }

    #[test]
    fn test_css_generation() {
        let css = shimmer_css();
        assert!(css.contains(".sh-shimmer"));
        assert!(css.contains("@keyframes"));
    }
}
