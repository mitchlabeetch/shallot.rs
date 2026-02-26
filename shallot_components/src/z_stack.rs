//! ZStack Component - Layered/Overlay Layout
//!
//! A stack that layers children on top of each other using z-index.
//! Perfect for overlays, badges, and composite UI elements.

use maud::{html, Markup, Render};

/// ZStack alignment options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ZStackAlignment {
    #[default]
    Center,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Top,
    Bottom,
    Left,
    Right,
}

impl ZStackAlignment {
    fn css_class(&self) -> &'static str {
        match self {
            ZStackAlignment::Center => "sh-zstack--center",
            ZStackAlignment::TopLeft => "sh-zstack--top-left",
            ZStackAlignment::TopRight => "sh-zstack--top-right",
            ZStackAlignment::BottomLeft => "sh-zstack--bottom-left",
            ZStackAlignment::BottomRight => "sh-zstack--bottom-right",
            ZStackAlignment::Top => "sh-zstack--top",
            ZStackAlignment::Bottom => "sh-zstack--bottom",
            ZStackAlignment::Left => "sh-zstack--left",
            ZStackAlignment::Right => "sh-zstack--right",
        }
    }
}

/// ZStack component for layered layouts
pub struct ZStack<'a> {
    children: Vec<Markup>,
    alignment: ZStackAlignment,
    fill: bool,
    class: Option<&'a str>,
}

impl<'a> ZStack<'a> {
    /// Create a new ZStack with children
    pub fn new(children: Vec<Markup>) -> Self {
        Self {
            children,
            alignment: ZStackAlignment::default(),
            fill: false,
            class: None,
        }
    }

    /// Set the alignment for all children
    pub fn alignment(mut self, alignment: ZStackAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Make children fill the stack
    pub fn fill(mut self, fill: bool) -> Self {
        self.fill = fill;
        self
    }

    /// Add custom class
    pub fn class(mut self, class: &'a str) -> Self {
        self.class = Some(class);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-zstack".to_string()];
        classes.push(self.alignment.css_class().to_string());
        if self.fill {
            classes.push("sh-zstack--fill".to_string());
        }
        if let Some(custom) = self.class {
            classes.push(custom.to_string());
        }
        classes.join(" ")
    }
}

impl<'a> Render for ZStack<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();

        html! {
            div
                class=(classes)
                role="group"
                aria-label="Layered content"
            {
                @for (i, child) in self.children.iter().enumerate() {
                    div class="sh-zstack__layer" style=(format!("z-index: {}", i)) {
                        (child)
                    }
                }
            }
        }
    }
}

/// Generate CSS for ZStack component
pub fn z_stack_css() -> String {
    r#"
.sh-zstack {
    position: relative;
    display: inline-block;
    width: 100%;
    height: 100%;
}

.sh-zstack--fill {
    width: 100%;
    height: 100%;
}

.sh-zstack__layer {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
}

/* Alignment variants */
.sh-zstack--center .sh-zstack__layer {
    align-items: center;
    justify-content: center;
}

.sh-zstack--top-left .sh-zstack__layer {
    align-items: flex-start;
    justify-content: flex-start;
}

.sh-zstack--top-right .sh-zstack__layer {
    align-items: flex-start;
    justify-content: flex-end;
}

.sh-zstack--bottom-left .sh-zstack__layer {
    align-items: flex-end;
    justify-content: flex-start;
}

.sh-zstack--bottom-right .sh-zstack__layer {
    align-items: flex-end;
    justify-content: flex-end;
}

.sh-zstack--top .sh-zstack__layer {
    align-items: flex-start;
    justify-content: center;
}

.sh-zstack--bottom .sh-zstack__layer {
    align-items: flex-end;
    justify-content: center;
}

.sh-zstack--left .sh-zstack__layer {
    align-items: center;
    justify-content: flex-start;
}

.sh-zstack--right .sh-zstack__layer {
    align-items: center;
    justify-content: flex-end;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zstack_creation() {
        let stack = ZStack::new(vec![html! { "Layer 1" }, html! { "Layer 2" }]);
        assert_eq!(stack.alignment, ZStackAlignment::Center);
        assert!(!stack.fill);
    }

    #[test]
    fn test_zstack_alignment() {
        let stack = ZStack::new(vec![]).alignment(ZStackAlignment::TopRight);
        assert_eq!(stack.alignment, ZStackAlignment::TopRight);
    }

    #[test]
    fn test_zstack_fill() {
        let stack = ZStack::new(vec![]).fill(true);
        assert!(stack.fill);
    }

    #[test]
    fn test_zstack_css() {
        let css = z_stack_css();
        assert!(css.contains(".sh-zstack"));
        assert!(css.contains(".sh-zstack__layer"));
    }
}
