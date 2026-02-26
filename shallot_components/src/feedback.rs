use maud::{html, Markup, Render};

pub struct Skeleton {
    pub width: Option<&'static str>,
    pub height: Option<&'static str>,
}

impl Skeleton {
    pub fn new() -> Self {
        Self {
            width: None,
            height: None,
        }
    }
    pub fn width(mut self, w: &'static str) -> Self {
        self.width = Some(w);
        self
    }
    pub fn height(mut self, h: &'static str) -> Self {
        self.height = Some(h);
        self
    }
}

impl Render for Skeleton {
    fn render(&self) -> Markup {
        let w = self.width.unwrap_or("100%");
        let h = self.height.unwrap_or("20px");
        let style = format!("width: {}; height: {};", w, h);
        html! {
            div class="sh-skeleton" style=(style) aria-hidden="true" {}
        }
    }
}

pub struct Spinner;

impl Spinner {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for Spinner {
    fn render(&self) -> Markup {
        html! {
            div class="sh-spinner" role="status" aria-label="Loading..." {}
        }
    }
}

pub struct Tooltip<'a> {
    pub text: &'a str,
    pub children: Markup,
}

impl<'a> Tooltip<'a> {
    pub fn new(text: &'a str, children: Markup) -> Self {
        Self { text, children }
    }
}

impl<'a> Render for Tooltip<'a> {
    fn render(&self) -> Markup {
        html! {
            div class="sh-tooltip-wrapper" {
                (self.children)
                div class="sh-tooltip" role="tooltip" { (self.text) }
            }
        }
    }
}

// Simple dialog using HTML <dialog> element.
// Note: <dialog> often needs JS to open (showModal()), but open attribute works for static display.
pub struct Dialog<'a> {
    pub open: bool,
    pub title: &'a str,
    pub children: Markup,
    pub close_href: Option<&'a str>,
}

impl<'a> Dialog<'a> {
    pub fn new(title: &'a str, children: Markup) -> Self {
        Self {
            open: false,
            title,
            children,
            close_href: None,
        }
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn close_href(mut self, href: &'a str) -> Self {
        self.close_href = Some(href);
        self
    }
}

impl<'a> Render for Dialog<'a> {
    fn render(&self) -> Markup {
        if !self.open {
            return html! {};
        }

        let close_href = self.close_href.unwrap_or("/");

        html! {
            a class="sh-dialog__overlay" href=(close_href) aria-label="Close dialog" {}
            div class="sh-dialog" role="dialog" aria-modal="true" aria-label=(self.title) {
                div class="sh-dialog__header" {
                    h3 class="sh-h3" { (self.title) }
                    a class="sh-dialog__close" href=(close_href) aria-label="Close" {
                        "×"
                    }
                }
                div class="sh-dialog__body" {
                    (self.children)
                }
            }
        }
    }
}

/// Generate CSS for feedback components
pub fn feedback_css() -> String {
    r#"
/* Skeleton */
.sh-skeleton {
    background: linear-gradient(90deg, var(--sh-surface-2, #f3f4f6) 25%, var(--sh-surface-hover, #e5e7eb) 50%, var(--sh-surface-2, #f3f4f6) 75%);
    background-size: 200% 100%;
    animation: sh-skeleton-loading 1.5s ease-in-out infinite;
    border-radius: var(--sh-radius-sm, 0.25rem);
}

@keyframes sh-skeleton-loading {
    0% { background-position: 200% 0; }
    100% { background-position: -200% 0; }
}

/* Spinner */
.sh-spinner {
    width: 2rem;
    height: 2rem;
    border: 3px solid var(--sh-surface-2, #f3f4f6);
    border-top-color: var(--sh-primary, #3b82f6);
    border-radius: 50%;
    animation: sh-spinner-spin 0.8s linear infinite;
}

@keyframes sh-spinner-spin {
    to { transform: rotate(360deg); }
}

/* Tooltip */
.sh-tooltip-wrapper {
    position: relative;
    display: inline-block;
}

.sh-tooltip {
    position: absolute;
    bottom: 100%;
    left: 50%;
    transform: translateX(-50%);
    padding: 0.5rem 0.75rem;
    background: var(--sh-surface-inverse, #1f2937);
    color: white;
    font-size: 0.75rem;
    border-radius: var(--sh-radius-md, 0.375rem);
    white-space: nowrap;
    opacity: 0;
    visibility: hidden;
    transition: opacity 0.2s ease, visibility 0.2s ease;
    z-index: 1000;
}

.sh-tooltip-wrapper:hover .sh-tooltip {
    opacity: 1;
    visibility: visible;
}

/* Dialog */
.sh-dialog__overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    z-index: 999;
}

.sh-dialog {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    background: var(--sh-surface, #fff);
    border-radius: var(--sh-radius-lg, 0.5rem);
    box-shadow: var(--sh-shadow-xl, 0 20px 40px rgba(0, 0, 0, 0.15));
    z-index: 1000;
    min-width: 20rem;
    max-width: 90vw;
    max-height: 90vh;
    overflow: auto;
}

.sh-dialog__header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 1.5rem;
    border-bottom: 1px solid var(--sh-border, #e5e7eb);
}

.sh-dialog__close {
    font-size: 1.5rem;
    line-height: 1;
    color: var(--sh-text-muted, #6b7280);
    text-decoration: none;
    transition: color 0.2s ease;
}

.sh-dialog__close:hover {
    color: var(--sh-text, #1f2937);
}

.sh-dialog__body {
    padding: 1.5rem;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_creation() {
        let skeleton = Skeleton::new();
        assert_eq!(skeleton.width, None);
        assert_eq!(skeleton.height, None);
    }

    #[test]
    fn test_skeleton_size() {
        let skeleton = Skeleton::new().width("100px").height("50px");
        assert_eq!(skeleton.width, Some("100px"));
        assert_eq!(skeleton.height, Some("50px"));
    }

    #[test]
    fn test_spinner_creation() {
        let spinner = Spinner::new();
        let html = spinner.render().into_string();
        assert!(html.contains("sh-spinner"));
        assert!(html.contains("role=\"status\""));
    }

    #[test]
    fn test_tooltip_creation() {
        let tooltip = Tooltip::new("Help text", html! { "Content" });
        assert_eq!(tooltip.text, "Help text");
    }

    #[test]
    fn test_dialog_creation() {
        let dialog = Dialog::new("Title", html! { "Content" });
        assert!(!dialog.open);
        assert_eq!(dialog.title, "Title");
    }

    #[test]
    fn test_feedback_css() {
        let css = feedback_css();
        assert!(css.contains(".sh-skeleton"));
        assert!(css.contains(".sh-spinner"));
        assert!(css.contains(".sh-tooltip"));
    }
}
