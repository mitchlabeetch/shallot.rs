use maud::{html, Markup, Render};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PatternKind {
    #[default]
    Dot,
    Grid,
    Lines,
    Circles,
    Zigzag,
    Diagonal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PatternSize {
    Sm,
    #[default]
    Md,
    Lg,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PatternOpacity {
    Light,
    #[default]
    Medium,
    Dark,
}

pub struct Pattern {
    pub kind: PatternKind,
    pub size: PatternSize,
    pub opacity: PatternOpacity,
    pub height: &'static str,
    pub color: Option<&'static str>,
    pub children: Option<Markup>,
}

impl Default for Pattern {
    fn default() -> Self {
        Self {
            kind: PatternKind::Dot,
            size: PatternSize::Md,
            opacity: PatternOpacity::Medium,
            height: "100%",
            color: None,
            children: None,
        }
    }
}

impl Pattern {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn kind(mut self, kind: PatternKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn size(mut self, size: PatternSize) -> Self {
        self.size = size;
        self
    }

    pub fn opacity(mut self, opacity: PatternOpacity) -> Self {
        self.opacity = opacity;
        self
    }

    pub fn height(mut self, height: &'static str) -> Self {
        self.height = height;
        self
    }

    pub fn color(mut self, color: &'static str) -> Self {
        self.color = Some(color);
        self
    }

    pub fn children(mut self, children: Markup) -> Self {
        self.children = Some(children);
        self
    }
}

impl Render for Pattern {
    fn render(&self) -> Markup {
        let pattern_class = match self.kind {
            PatternKind::Dot => "sh-pattern sh-pattern--dot",
            PatternKind::Grid => "sh-pattern sh-pattern--grid",
            PatternKind::Lines => "sh-pattern sh-pattern--lines",
            PatternKind::Circles => "sh-pattern sh-pattern--circles",
            PatternKind::Zigzag => "sh-pattern sh-pattern--zigzag",
            PatternKind::Diagonal => "sh-pattern sh-pattern--diagonal",
        };

        let size_class = match self.size {
            PatternSize::Sm => "sh-pattern--sm",
            PatternSize::Md => "sh-pattern--md",
            PatternSize::Lg => "sh-pattern--lg",
        };

        let opacity_class = match self.opacity {
            PatternOpacity::Light => "sh-pattern--light",
            PatternOpacity::Medium => "sh-pattern--medium",
            PatternOpacity::Dark => "sh-pattern--dark",
        };

        let color_style = if let Some(c) = self.color {
            format!("--sh-pattern-color: {};", c)
        } else {
            String::new()
        };

        let style = format!("height: {}; width: 100%; {}", self.height, color_style);

        html! {
            div class=(format!("{} {} {}", pattern_class, size_class, opacity_class)) style=(style) role="presentation" aria-hidden="true" {
                @if let Some(children) = &self.children {
                    div class="sh-pattern__content" {
                        (children)
                    }
                }
            }
        }
    }
}

pub fn pattern_css() -> String {
    r#"
.sh-pattern {
    --sh-pattern-color: var(--sh-border, #e5e7eb);
    --sh-pattern-size: 20px;
    position: relative;
    width: 100%;
    border-radius: var(--sh-radius-lg, 0.75rem);
    overflow: hidden;
    background-color: var(--sh-surface, #ffffff);
}

.sh-pattern--sm {
    --sh-pattern-size: 12px;
}

.sh-pattern--md {
    --sh-pattern-size: 20px;
}

.sh-pattern--lg {
    --sh-pattern-size: 32px;
}

.sh-pattern--light {
    --sh-pattern-opacity: 0.3;
}

.sh-pattern--medium {
    --sh-pattern-opacity: 0.5;
}

.sh-pattern--dark {
    --sh-pattern-opacity: 0.8;
}

.sh-pattern__content {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1;
}

.sh-pattern--dot {
    background-image: radial-gradient(
        circle,
        var(--sh-pattern-color) 1px,
        transparent 1px
    );
    background-size: var(--sh-pattern-size) var(--sh-pattern-size);
    opacity: var(--sh-pattern-opacity, 0.5);
}

.sh-pattern--grid {
    background-image:
        linear-gradient(var(--sh-pattern-color) 1px, transparent 1px),
        linear-gradient(90deg, var(--sh-pattern-color) 1px, transparent 1px);
    background-size: var(--sh-pattern-size) var(--sh-pattern-size);
    opacity: var(--sh-pattern-opacity, 0.5);
}

.sh-pattern--lines {
    background-image: repeating-linear-gradient(
        0deg,
        var(--sh-pattern-color),
        var(--sh-pattern-color) 1px,
        transparent 1px,
        transparent var(--sh-pattern-size)
    );
    opacity: var(--sh-pattern-opacity, 0.5);
}

.sh-pattern--circles {
    background-image: radial-gradient(
        circle at center,
        var(--sh-pattern-color) 0,
        var(--sh-pattern-color) calc(var(--sh-pattern-size) * 0.4),
        transparent calc(var(--sh-pattern-size) * 0.4)
    );
    background-size: var(--sh-pattern-size) var(--sh-pattern-size);
    opacity: var(--sh-pattern-opacity, 0.5);
}

.sh-pattern--zigzag {
    background: linear-gradient(
        135deg,
        var(--sh-pattern-color) 25%,
        transparent 25%
    ),
    linear-gradient(
        225deg,
        var(--sh-pattern-color) 25%,
        transparent 25%
    ),
    linear-gradient(
        45deg,
        var(--sh-pattern-color) 25%,
        transparent 25%
    ),
    linear-gradient(
        315deg,
        var(--sh-pattern-color) 25%,
        transparent 25%
    );
    background-size: calc(var(--sh-pattern-size) * 0.5) calc(var(--sh-pattern-size) * 0.5);
    background-position: 0 0, calc(var(--sh-pattern-size) * 0.25) 0, calc(var(--sh-pattern-size) * 0.25) calc(var(--sh-pattern-size) * -0.25), 0 calc(var(--sh-pattern-size) * 0.25);
    opacity: var(--sh-pattern-opacity, 0.5);
}

.sh-pattern--diagonal {
    background-image: repeating-linear-gradient(
        45deg,
        var(--sh-pattern-color),
        var(--sh-pattern-color) 1px,
        transparent 1px,
        transparent var(--sh-pattern-size)
    );
    opacity: var(--sh-pattern-opacity, 0.5);
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_default() {
        let pattern = Pattern::new();
        let rendered = pattern.render();
        assert!(rendered.0.as_str().contains("sh-pattern"));
    }

    #[test]
    fn test_pattern_kind_dot() {
        let pattern = Pattern::new().kind(PatternKind::Dot);
        let rendered = pattern.render();
        assert!(rendered.0.as_str().contains("sh-pattern--dot"));
    }

    #[test]
    fn test_pattern_kind_grid() {
        let pattern = Pattern::new().kind(PatternKind::Grid);
        let rendered = pattern.render();
        assert!(rendered.0.as_str().contains("sh-pattern--grid"));
    }

    #[test]
    fn test_pattern_kind_lines() {
        let pattern = Pattern::new().kind(PatternKind::Lines);
        let rendered = pattern.render();
        assert!(rendered.0.as_str().contains("sh-pattern--lines"));
    }

    #[test]
    fn test_pattern_kind_circles() {
        let pattern = Pattern::new().kind(PatternKind::Circles);
        let rendered = pattern.render();
        assert!(rendered.0.as_str().contains("sh-pattern--circles"));
    }

    #[test]
    fn test_pattern_kind_zigzag() {
        let pattern = Pattern::new().kind(PatternKind::Zigzag);
        let rendered = pattern.render();
        assert!(rendered.0.as_str().contains("sh-pattern--zigzag"));
    }

    #[test]
    fn test_pattern_kind_diagonal() {
        let pattern = Pattern::new().kind(PatternKind::Diagonal);
        let rendered = pattern.render();
        assert!(rendered.0.as_str().contains("sh-pattern--diagonal"));
    }

    #[test]
    fn test_pattern_size_sm() {
        let pattern = Pattern::new().size(PatternSize::Sm);
        let rendered = pattern.render();
        assert!(rendered.0.as_str().contains("sh-pattern--sm"));
    }

    #[test]
    fn test_pattern_size_lg() {
        let pattern = Pattern::new().size(PatternSize::Lg);
        let rendered = pattern.render();
        assert!(rendered.0.as_str().contains("sh-pattern--lg"));
    }

    #[test]
    fn test_pattern_opacity_light() {
        let pattern = Pattern::new().opacity(PatternOpacity::Light);
        let rendered = pattern.render();
        assert!(rendered.0.as_str().contains("sh-pattern--light"));
    }

    #[test]
    fn test_pattern_opacity_dark() {
        let pattern = Pattern::new().opacity(PatternOpacity::Dark);
        let rendered = pattern.render();
        assert!(rendered.0.as_str().contains("sh-pattern--dark"));
    }

    #[test]
    fn test_pattern_custom_height() {
        let pattern = Pattern::new().height("500px");
        let rendered = pattern.render();
        assert!(rendered.0.as_str().contains("height: 500px"));
    }

    #[test]
    fn test_pattern_custom_color() {
        let pattern = Pattern::new().color("#ff0000");
        let rendered = pattern.render();
        assert!(rendered.0.as_str().contains("--sh-pattern-color: #ff0000"));
    }

    #[test]
    fn test_pattern_a11y() {
        let pattern = Pattern::new();
        let rendered = pattern.render();
        assert!(rendered.0.as_str().contains("role=\"presentation\""));
        assert!(rendered.0.as_str().contains("aria-hidden=\"true\""));
    }

    #[test]
    fn test_pattern_css_function() {
        let css = pattern_css();
        assert!(css.contains(".sh-pattern"));
        assert!(css.contains(".sh-pattern--dot"));
        assert!(css.contains(".sh-pattern--grid"));
    }
}
