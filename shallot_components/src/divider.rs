//! Divider Component - Horizontal and vertical separators
//! CSS-only styling with variants

use maud::{html, Markup, Render};

/// Divider orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DividerOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Divider style variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DividerStyle {
    #[default]
    Solid,
    Dashed,
    Dotted,
    Gradient,
}

/// Divider component
#[derive(Debug, Clone)]
pub struct Divider {
    pub orientation: DividerOrientation,
    pub style: DividerStyle,
    pub label: Option<String>,
    pub inset: bool,
}

impl Divider {
    pub fn new() -> Self {
        Self {
            orientation: DividerOrientation::default(),
            style: DividerStyle::default(),
            label: None,
            inset: false,
        }
    }

    pub fn orientation(mut self, orientation: DividerOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn style(mut self, style: DividerStyle) -> Self {
        self.style = style;
        self
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn inset(mut self, inset: bool) -> Self {
        self.inset = inset;
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-divider"];

        match self.orientation {
            DividerOrientation::Horizontal => classes.push("sh-divider--horizontal"),
            DividerOrientation::Vertical => classes.push("sh-divider--vertical"),
        }

        match self.style {
            DividerStyle::Solid => classes.push("sh-divider--solid"),
            DividerStyle::Dashed => classes.push("sh-divider--dashed"),
            DividerStyle::Dotted => classes.push("sh-divider--dotted"),
            DividerStyle::Gradient => classes.push("sh-divider--gradient"),
        }

        if self.inset {
            classes.push("sh-divider--inset");
        }

        if self.label.is_some() {
            classes.push("sh-divider--labeled");
        }

        classes.join(" ")
    }
}

impl Default for Divider {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for Divider {
    fn render(&self) -> Markup {
        let classes = self.build_classes();

        html! {
            div class=(classes) role="separator" aria-orientation=(match self.orientation {
                DividerOrientation::Horizontal => "horizontal",
                DividerOrientation::Vertical => "vertical",
            }) {
                @if let Some(ref label) = self.label {
                    span class="sh-divider__label" {
                        (label)
                    }
                }
            }
        }
    }
}

pub fn divider_css() -> String {
    r#"
.sh-divider {
    border: none;
    margin: 0;
}

.sh-divider--horizontal {
    display: flex;
    align-items: center;
    width: 100%;
    min-height: 1px;
}

.sh-divider--vertical {
    display: inline-flex;
    flex-direction: column;
    align-items: center;
    height: 100%;
    min-width: 1px;
    vertical-align: middle;
}

.sh-divider--solid.sh-divider--horizontal {
    border-top: 1px solid var(--sh-color-border, #e5e5e5);
}

.sh-divider--solid.sh-divider--vertical {
    border-left: 1px solid var(--sh-color-border, #e5e5e5);
}

.sh-divider--dashed.sh-divider--horizontal {
    border-top: 1px dashed var(--sh-color-border, #e5e5e5);
}

.sh-divider--dashed.sh-divider--vertical {
    border-left: 1px dashed var(--sh-color-border, #e5e5e5);
}

.sh-divider--dotted.sh-divider--horizontal {
    border-top: 1px dotted var(--sh-color-border, #e5e5e5);
}

.sh-divider--dotted.sh-divider--vertical {
    border-left: 1px dotted var(--sh-color-border, #e5e5e5);
}

.sh-divider--gradient.sh-divider--horizontal {
    background: linear-gradient(90deg, transparent, var(--sh-color-border, #e5e5e5), transparent);
    height: 1px;
}

.sh-divider--gradient.sh-divider--vertical {
    background: linear-gradient(180deg, transparent, var(--sh-color-border, #e5e5e5), transparent);
    width: 1px;
}

.sh-divider--inset.sh-divider--horizontal {
    margin-left: var(--sh-spacing-lg, 1.5rem);
    margin-right: var(--sh-spacing-lg, 1.5rem);
    width: calc(100% - 3rem);
}

.sh-divider--labeled {
    gap: var(--sh-spacing-md, 1rem);
}

.sh-divider--labeled.sh-divider--horizontal::before,
.sh-divider--labeled.sh-divider--horizontal::after {
    content: "";
    flex: 1;
    border-top: 1px solid var(--sh-color-border, #e5e5e5);
}

.sh-divider__label {
    font-size: var(--sh-font-size-sm, 0.875rem);
    color: var(--sh-color-text-muted, #666);
    white-space: nowrap;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divider_creation() {
        let divider = Divider::new()
            .orientation(DividerOrientation::Vertical)
            .style(DividerStyle::Dashed);

        assert_eq!(divider.orientation, DividerOrientation::Vertical);
        assert_eq!(divider.style, DividerStyle::Dashed);
    }

    #[test]
    fn test_divider_render() {
        let divider = Divider::new().label("Section");

        let html = divider.render().into_string();
        assert!(html.contains("sh-divider"));
        assert!(html.contains("Section"));
    }

    #[test]
    fn test_divider_inset() {
        let divider = Divider::new().inset(true);
        assert!(divider.build_classes().contains("sh-divider--inset"));
    }

    #[test]
    fn test_divider_styles() {
        let gradient = Divider::new().style(DividerStyle::Gradient);
        assert!(gradient.build_classes().contains("sh-divider--gradient"));
    }

    #[test]
    fn test_css_generation() {
        let css = divider_css();
        assert!(css.contains(".sh-divider"));
    }
}
