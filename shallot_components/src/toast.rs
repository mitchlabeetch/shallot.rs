use maud::{html, Markup, Render};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastVariant {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct Toast<'a> {
    pub title: &'a str,
    pub message: &'a str,
    pub variant: ToastVariant,
    pub close_href: Option<&'a str>,
}

impl<'a> Toast<'a> {
    pub fn new(title: &'a str, message: &'a str, variant: ToastVariant) -> Self {
        Self {
            title,
            message,
            variant,
            close_href: None,
        }
    }

    pub fn close_href(mut self, href: &'a str) -> Self {
        self.close_href = Some(href);
        self
    }
}

impl<'a> Render for Toast<'a> {
    fn render(&self) -> Markup {
        let icon = match self.variant {
            ToastVariant::Info => "information-circle",
            ToastVariant::Success => "checkmark-circle",
            ToastVariant::Warning => "alert-circle",
            ToastVariant::Error => "close-circle",
        };

        let color = match self.variant {
            ToastVariant::Info => "var(--sh-accent)",
            ToastVariant::Success => "var(--sh-success)",
            ToastVariant::Warning => "var(--sh-warning)",
            ToastVariant::Error => "var(--sh-error)",
        };

        html! {
            div class="sh-toast" role="alert" aria-live="polite" {
                div style=(format!("color: {}; display: flex", color)) {
                    img class="sh-icon" src=(format!("/icons/{}.svg", icon)) alt="" loading="lazy";
                }
                div {
                    div style="font-weight: 600; font-size: 13px" { (self.title) }
                    div style="color: var(--sh-text-muted); font-size: 12px" { (self.message) }
                }
                @if let Some(href) = self.close_href {
                    a href=(href) aria-label="Close" style="margin-left: auto; color: var(--sh-text-muted); display: flex" {
                        img class="sh-icon" style="width: 14px; height: 14px" src="/icons/close.svg" alt="" loading="lazy";
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ToastContainer<'a> {
    pub toasts: Vec<Toast<'a>>,
}

impl<'a> ToastContainer<'a> {
    pub fn new(toasts: Vec<Toast<'a>>) -> Self {
        Self { toasts }
    }
}

impl<'a> Render for ToastContainer<'a> {
    fn render(&self) -> Markup {
        if self.toasts.is_empty() {
            return html! {};
        }
        html! {
            div class="sh-toast-container" role="region" aria-label="Notifications" {
                @for toast in &self.toasts {
                    (toast.render())
                }
            }
        }
    }
}

/// Generate CSS for toast component
pub fn toast_css() -> String {
    r#"
.sh-toast-container {
    position: fixed;
    top: 1rem;
    right: 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    z-index: 1000;
}

.sh-toast {
    display: flex;
    align-items: flex-start;
    gap: 0.75rem;
    padding: 1rem;
    background: var(--sh-surface, #fff);
    border: 1px solid var(--sh-border, #e5e7eb);
    border-radius: var(--sh-radius-lg, 0.5rem);
    box-shadow: var(--sh-shadow-lg, 0 10px 40px rgba(0, 0, 0, 0.1));
    min-width: 20rem;
    max-width: 24rem;
    animation: sh-toast-slide 0.3s ease;
}

@keyframes sh-toast-slide {
    from {
        transform: translateX(100%);
        opacity: 0;
    }
    to {
        transform: translateX(0);
        opacity: 1;
    }
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toast_creation() {
        let toast = Toast::new("Title", "Message", ToastVariant::Info);
        assert_eq!(toast.title, "Title");
        assert_eq!(toast.variant, ToastVariant::Info);
    }

    #[test]
    fn test_toast_variants() {
        assert_eq!(ToastVariant::Success as u8, 1);
        assert_eq!(ToastVariant::Warning as u8, 2);
        assert_eq!(ToastVariant::Error as u8, 3);
    }

    #[test]
    fn test_toast_container_creation() {
        let toasts = vec![Toast::new("Test", "Msg", ToastVariant::Success)];
        let container = ToastContainer::new(toasts);
        assert_eq!(container.toasts.len(), 1);
    }

    #[test]
    fn test_toast_css() {
        let css = toast_css();
        assert!(css.contains(".sh-toast"));
        assert!(css.contains(".sh-toast-container"));
    }
}
