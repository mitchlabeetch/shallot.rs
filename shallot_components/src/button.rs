use maud::{html, Markup};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Ghost,
    Danger,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Sm,
    Md,
    Lg,
}

pub struct Button<'a> {
    pub label: &'a str,
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub disabled: bool,
    pub href: Option<&'a str>,
}

impl<'a> Button<'a> {
    /// Create a new button with the given label
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            variant: ButtonVariant::Primary,
            size: ButtonSize::Md,
            disabled: false,
            href: None,
        }
    }

    /// Set the button variant
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the button size
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Set the button disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the button href (renders as link if set)
    pub fn href(mut self, href: &'a str) -> Self {
        self.href = Some(href);
        self
    }

    pub fn render(self) -> Markup {
        let variant_class = match self.variant {
            ButtonVariant::Primary => "sh-btn sh-btn--primary",
            ButtonVariant::Secondary => "sh-btn sh-btn--secondary",
            ButtonVariant::Ghost => "sh-btn sh-btn--ghost",
            ButtonVariant::Danger => "sh-btn sh-btn--danger",
        };

        let size_class = match self.size {
            ButtonSize::Sm => "sh-btn--sm",
            ButtonSize::Md => "sh-btn--md",
            ButtonSize::Lg => "sh-btn--lg",
        };

        let class = format!("{} {}", variant_class, size_class);

        if let Some(href) = self.href {
            let disabled = self.disabled;
            html! {
                a
                    class=(class)
                    href=(if disabled { "#" } else { href })
                    role="button"
                    aria-disabled?[disabled]
                    tabindex=(if disabled { "-1" } else { "0" })
                {
                    (self.label)
                }
            }
        } else {
            html! {
                button
                    type="button"
                    class=(class)
                    disabled?[self.disabled]
                {
                    (self.label)
                }
            }
        }
    }
}

/// Generate CSS for button component
pub fn button_css() -> String {
    r#"
.sh-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0.5rem 1rem;
    font-size: 0.875rem;
    font-weight: 500;
    border-radius: var(--sh-radius-md, 0.375rem);
    transition: all 0.2s ease;
    cursor: pointer;
    border: none;
    text-decoration: none;
}

.sh-btn--primary {
    background: var(--sh-primary, #3b82f6);
    color: white;
}

.sh-btn--primary:hover {
    background: var(--sh-primary-hover, #2563eb);
}

.sh-btn--secondary {
    background: var(--sh-surface-2, #f3f4f6);
    color: var(--sh-text, #1f2937);
}

.sh-btn--secondary:hover {
    background: var(--sh-surface-hover, #e5e7eb);
}

.sh-btn--ghost {
    background: transparent;
    color: var(--sh-primary, #3b82f6);
}

.sh-btn--ghost:hover {
    background: var(--sh-surface-2, #f3f4f6);
}

.sh-btn--danger {
    background: var(--sh-error, #ef4444);
    color: white;
}

.sh-btn--danger:hover {
    background: var(--sh-error-hover, #dc2626);
}

.sh-btn--sm {
    padding: 0.375rem 0.75rem;
    font-size: 0.75rem;
}

.sh-btn--md {
    padding: 0.5rem 1rem;
    font-size: 0.875rem;
}

.sh-btn--lg {
    padding: 0.75rem 1.5rem;
    font-size: 1rem;
}

.sh-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_creation() {
        let button = Button::new("Click me");
        assert_eq!(button.label, "Click me");
        assert_eq!(button.variant, ButtonVariant::Primary);
        assert_eq!(button.size, ButtonSize::Md);
    }

    #[test]
    fn test_button_variant() {
        let button = Button::new("Test").variant(ButtonVariant::Danger);
        assert_eq!(button.variant, ButtonVariant::Danger);
    }

    #[test]
    fn test_button_size() {
        let button = Button::new("Test").size(ButtonSize::Lg);
        assert_eq!(button.size, ButtonSize::Lg);
    }

    #[test]
    fn test_button_disabled() {
        let button = Button::new("Test").disabled(true);
        assert!(button.disabled);
    }

    #[test]
    fn test_button_css() {
        let css = button_css();
        assert!(css.contains(".sh-btn"));
        assert!(css.contains(".sh-btn--primary"));
    }
}
