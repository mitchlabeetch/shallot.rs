//! Visually Hidden Component - Content hidden visually but accessible to screen readers
//! CSS-only technique using clip and position

use maud::{html, Markup, Render};

/// Visually hidden component for accessibility
#[derive(Debug, Clone)]
pub struct VisuallyHidden<'a> {
    pub content: &'a str,
    pub focusable: bool,
}

impl<'a> VisuallyHidden<'a> {
    /// Create a new visually hidden element
    pub fn new(content: &'a str) -> Self {
        Self {
            content,
            focusable: false,
        }
    }

    /// Make the element visible when focused (for skip links)
    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = focusable;
        self
    }
}

impl<'a> Render for VisuallyHidden<'a> {
    fn render(&self) -> Markup {
        let class = if self.focusable {
            "sh-visually-hidden sh-visually-hidden--focusable"
        } else {
            "sh-visually-hidden"
        };

        html! {
            span
                class=(class)
                role="text"
            {
                (self.content)
            }
        }
    }
}

/// Skip link component for keyboard navigation
#[derive(Debug, Clone)]
pub struct SkipLink<'a> {
    pub target: &'a str,
    pub label: &'a str,
}

impl<'a> SkipLink<'a> {
    /// Create a new skip link
    pub fn new(target: &'a str, label: &'a str) -> Self {
        Self { target, label }
    }
}

impl<'a> Render for SkipLink<'a> {
    fn render(&self) -> Markup {
        html! {
            a
                href=(self.target)
                class="sh-skip-link" {
                (self.label)
            }
        }
    }
}

/// Generate CSS for visually hidden component
pub fn visually_hidden_css() -> String {
    r#"
.sh-visually-hidden {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
}

.sh-visually-hidden--focusable:active,
.sh-visually-hidden--focusable:focus {
    position: static;
    width: auto;
    height: auto;
    padding: inherit;
    margin: inherit;
    overflow: visible;
    clip: auto;
    white-space: normal;
}

.sh-skip-link {
    position: absolute;
    top: var(--sh-spacing-4, 1rem);
    left: var(--sh-spacing-4, 1rem);
    z-index: 9999;
    padding: var(--sh-spacing-3, 0.75rem) var(--sh-spacing-4, 1rem);
    background-color: var(--sh-color-primary, #3b82f6);
    color: var(--sh-color-primary-foreground, #ffffff);
    font-size: var(--sh-font-size-base, 1rem);
    font-weight: var(--sh-font-weight-medium, 500);
    text-decoration: none;
    border-radius: var(--sh-radius-md, 0.375rem);
    transform: translateY(-200%);
    transition: transform 0.2s ease;
}

.sh-skip-link:focus {
    transform: translateY(0);
    outline: 2px solid var(--sh-color-primary-foreground, #ffffff);
    outline-offset: 2px;
}

.sh-skip-link:hover {
    background-color: var(--sh-color-primary-hover, #2563eb);
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visually_hidden_creation() {
        let hidden = VisuallyHidden::new("Screen reader text");

        assert_eq!(hidden.content, "Screen reader text");
        assert!(!hidden.focusable);
    }

    #[test]
    fn test_visually_hidden_render() {
        let hidden = VisuallyHidden::new("Hidden content");
        let html = hidden.render().into_string();

        assert!(html.contains("sh-visually-hidden"));
        assert!(html.contains("Hidden content"));
    }

    #[test]
    fn test_visually_hidden_focusable() {
        let hidden = VisuallyHidden::new("Skip to content").focusable(true);
        let html = hidden.render().into_string();

        assert!(html.contains("sh-visually-hidden--focusable"));
    }

    #[test]
    fn test_skip_link_creation() {
        let link = SkipLink::new("#main-content", "Skip to main content");

        assert_eq!(link.target, "#main-content");
        assert_eq!(link.label, "Skip to main content");
    }

    #[test]
    fn test_skip_link_render() {
        let link = SkipLink::new("#main", "Skip to main");
        let html = link.render().into_string();

        assert!(html.contains("sh-skip-link"));
        assert!(html.contains("href=\"#main\""));
        assert!(html.contains("Skip to main"));
    }

    #[test]
    fn test_visually_hidden_css() {
        let css = visually_hidden_css();
        assert!(css.contains(".sh-visually-hidden"));
        assert!(css.contains(".sh-skip-link"));
        assert!(css.contains("clip: rect"));
    }
}
