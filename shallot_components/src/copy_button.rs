//! Copy Button Component - Button that indicates copy action
//! CSS-only styling with visual feedback states

use maud::{html, Markup, Render};

/// Copy button size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CopyButtonSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Copy button variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CopyButtonVariant {
    #[default]
    Default,
    Ghost,
    Outline,
}

/// Copy button component
#[derive(Debug, Clone)]
pub struct CopyButton<'a> {
    pub text_to_copy: &'a str,
    pub size: CopyButtonSize,
    pub variant: CopyButtonVariant,
    pub label: Option<&'a str>,
    pub success_label: Option<&'a str>,
    pub disabled: bool,
}

impl<'a> CopyButton<'a> {
    /// Create a new copy button
    pub fn new(text_to_copy: &'a str) -> Self {
        Self {
            text_to_copy,
            size: CopyButtonSize::default(),
            variant: CopyButtonVariant::default(),
            label: None,
            success_label: None,
            disabled: false,
        }
    }

    /// Set the size
    pub fn size(mut self, size: CopyButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Set the variant
    pub fn variant(mut self, variant: CopyButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the label text
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    /// Set the success label text
    pub fn success_label(mut self, label: &'a str) -> Self {
        self.success_label = Some(label);
        self
    }

    /// Set the disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-copy-btn".to_string()];

        let size_class = match self.size {
            CopyButtonSize::Sm => "sh-copy-btn--sm",
            CopyButtonSize::Md => "sh-copy-btn--md",
            CopyButtonSize::Lg => "sh-copy-btn--lg",
        };
        classes.push(size_class.to_string());

        let variant_class = match self.variant {
            CopyButtonVariant::Default => "sh-copy-btn--default",
            CopyButtonVariant::Ghost => "sh-copy-btn--ghost",
            CopyButtonVariant::Outline => "sh-copy-btn--outline",
        };
        classes.push(variant_class.to_string());

        if self.disabled {
            classes.push("sh-copy-btn--disabled".to_string());
        }

        classes.join(" ")
    }
}

impl<'a> Render for CopyButton<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let label = self.label.unwrap_or("Copy");
        let success_label = self.success_label.unwrap_or("Copied!");

        html! {
            button
                type="button"
                class=(classes)
                disabled?=(self.disabled)
                data-copy-text=(self.text_to_copy)
                data-label=(label)
                data-success-label=(success_label)
                aria-label={"Copy: " (self.text_to_copy)} {
                span class="sh-copy-btn__icon" {
                    // Copy icon (clipboard)
                    svg
                        class="sh-copy-btn__svg sh-copy-btn__svg--copy"
                        xmlns="http://www.w3.org/2000/svg"
                        width="16"
                        height="16"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round" {
                        rect x="9" y="9" width="13" height="13" rx="2" ry="2" {}
                        path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" {}
                    }
                    // Success icon (checkmark)
                    svg
                        class="sh-copy-btn__svg sh-copy-btn__svg--success"
                        xmlns="http://www.w3.org/2000/svg"
                        width="16"
                        height="16"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round" {
                        polyline points="20 6 9 17 4 12" {}
                    }
                }
                span class="sh-copy-btn__label" {
                    (label)
                }
            }
        }
    }
}

/// Generate CSS for copy button component
pub fn copy_button_css() -> String {
    r#"
.sh-copy-btn {
    display: inline-flex;
    align-items: center;
    gap: var(--sh-spacing-2, 0.5rem);
    border: 1px solid transparent;
    border-radius: var(--sh-radius-md, 0.375rem);
    font-weight: var(--sh-font-weight-medium, 500);
    cursor: pointer;
    transition: all 0.15s ease;
}

.sh-copy-btn--sm {
    padding: var(--sh-spacing-1, 0.25rem) var(--sh-spacing-2, 0.5rem);
    font-size: var(--sh-font-size-sm, 0.875rem);
}

.sh-copy-btn--md {
    padding: var(--sh-spacing-2, 0.5rem) var(--sh-spacing-3, 0.75rem);
    font-size: var(--sh-font-size-sm, 0.875rem);
}

.sh-copy-btn--lg {
    padding: var(--sh-spacing-3, 0.75rem) var(--sh-spacing-4, 1rem);
    font-size: var(--sh-font-size-base, 1rem);
}

.sh-copy-btn--default {
    background-color: var(--sh-color-muted, #f3f4f6);
    color: var(--sh-color-foreground, #1f2937);
    border-color: var(--sh-color-border, #e5e7eb);
}

.sh-copy-btn--default:hover:not(:disabled) {
    background-color: var(--sh-color-primary, #3b82f6);
    color: var(--sh-color-primary-foreground, #ffffff);
    border-color: var(--sh-color-primary, #3b82f6);
}

.sh-copy-btn--ghost {
    background-color: transparent;
    color: var(--sh-color-foreground, #1f2937);
}

.sh-copy-btn--ghost:hover:not(:disabled) {
    background-color: var(--sh-color-muted, #f3f4f6);
}

.sh-copy-btn--outline {
    background-color: transparent;
    color: var(--sh-color-foreground, #1f2937);
    border-color: var(--sh-color-border, #e5e7eb);
}

.sh-copy-btn--outline:hover:not(:disabled) {
    background-color: var(--sh-color-muted, #f3f4f6);
    border-color: var(--sh-color-primary, #3b82f6);
}

.sh-copy-btn:focus-visible {
    outline: 2px solid var(--sh-color-primary, #3b82f6);
    outline-offset: 2px;
}

.sh-copy-btn--disabled,
.sh-copy-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

.sh-copy-btn__icon {
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    width: 1em;
    height: 1em;
}

.sh-copy-btn__svg {
    width: 100%;
    height: 100%;
}

.sh-copy-btn__svg--success {
    display: none;
    color: var(--sh-color-success, #10b981);
}

.sh-copy-btn--copied .sh-copy-btn__svg--copy {
    display: none;
}

.sh-copy-btn--copied .sh-copy-btn__svg--success {
    display: block;
}

.sh-copy-btn--copied {
    background-color: var(--sh-color-success-muted, #d1fae5);
    color: var(--sh-color-success, #10b981);
    border-color: var(--sh-color-success, #10b981);
}

.sh-copy-btn__label {
    white-space: nowrap;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_button_creation() {
        let btn = CopyButton::new("Hello, World!");

        assert_eq!(btn.text_to_copy, "Hello, World!");
        assert_eq!(btn.size, CopyButtonSize::Md);
        assert_eq!(btn.variant, CopyButtonVariant::Default);
    }

    #[test]
    fn test_copy_button_render() {
        let btn = CopyButton::new("Copy this text").label("Copy");

        let html = btn.render().into_string();
        assert!(html.contains("sh-copy-btn"));
        assert!(html.contains("data-copy-text"));
        assert!(html.contains("Copy this text"));
        assert!(html.contains("Copy"));
    }

    #[test]
    fn test_copy_button_sizes() {
        let sm = CopyButton::new("test").size(CopyButtonSize::Sm);
        let md = CopyButton::new("test").size(CopyButtonSize::Md);
        let lg = CopyButton::new("test").size(CopyButtonSize::Lg);

        assert!(sm.render().into_string().contains("sh-copy-btn--sm"));
        assert!(md.render().into_string().contains("sh-copy-btn--md"));
        assert!(lg.render().into_string().contains("sh-copy-btn--lg"));
    }

    #[test]
    fn test_copy_button_variants() {
        let default = CopyButton::new("test").variant(CopyButtonVariant::Default);
        let ghost = CopyButton::new("test").variant(CopyButtonVariant::Ghost);
        let outline = CopyButton::new("test").variant(CopyButtonVariant::Outline);

        assert!(default
            .render()
            .into_string()
            .contains("sh-copy-btn--default"));
        assert!(ghost.render().into_string().contains("sh-copy-btn--ghost"));
        assert!(outline
            .render()
            .into_string()
            .contains("sh-copy-btn--outline"));
    }

    #[test]
    fn test_copy_button_disabled() {
        let btn = CopyButton::new("test").disabled(true);
        let html = btn.render().into_string();

        assert!(html.contains("sh-copy-btn--disabled"));
        assert!(html.contains("disabled"));
    }

    #[test]
    fn test_copy_button_css() {
        let css = copy_button_css();
        assert!(css.contains(".sh-copy-btn"));
        assert!(css.contains(".sh-copy-btn__icon"));
        assert!(css.contains(".sh-copy-btn--copied"));
    }
}
