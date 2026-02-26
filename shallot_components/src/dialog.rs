//! Dialog Component
//!
//! Modal dialogs using CSS-only interactions (details/summary pattern).

use maud::{html, Markup, Render};

pub struct Dialog<'a> {
    id: &'a str,
    title: Option<&'a str>,
    content: Markup,
    footer: Option<Markup>,
    size: DialogSize,
    variant: DialogVariant,
    open: bool,
    #[allow(dead_code)] // Reserved for CSS-only overlay click handling
    close_on_overlay: bool,
    close_button: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DialogSize {
    #[default]
    Md,
    Sm,
    Lg,
    Xl,
    Full,
}

impl DialogSize {
    fn class(&self) -> &'static str {
        match self {
            DialogSize::Sm => "sh-dialog--sm",
            DialogSize::Md => "sh-dialog--md",
            DialogSize::Lg => "sh-dialog--lg",
            DialogSize::Xl => "sh-dialog--xl",
            DialogSize::Full => "sh-dialog--full",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DialogVariant {
    #[default]
    Default,
    Danger,
    Success,
    Warning,
    Info,
}

impl<'a> Dialog<'a> {
    pub fn new(id: &'a str, content: Markup) -> Self {
        Self {
            id,
            title: None,
            content,
            footer: None,
            size: DialogSize::Md,
            variant: DialogVariant::Default,
            open: false,
            close_on_overlay: true,
            close_button: true,
        }
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    pub fn footer(mut self, footer: Markup) -> Self {
        self.footer = Some(footer);
        self
    }

    pub fn size(mut self, size: DialogSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: DialogVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn close_button(mut self, show: bool) -> Self {
        self.close_button = show;
        self
    }
}

impl<'a> Render for Dialog<'a> {
    fn render(&self) -> Markup {
        let dialog_id = format!("sh-dialog-{}", self.id);
        let variant_class = match self.variant {
            DialogVariant::Default => "",
            DialogVariant::Danger => "sh-dialog--danger",
            DialogVariant::Success => "sh-dialog--success",
            DialogVariant::Warning => "sh-dialog--warning",
            DialogVariant::Info => "sh-dialog--info",
        };

        html! {
            details
                class={(format!("sh-dialog-wrapper {}", variant_class))}
                id=(dialog_id)
                open?[self.open]
            {
                summary class="sh-dialog-trigger" { }

                div class="sh-dialog-overlay" {
                    div class={(format!("sh-dialog {}", self.size.class()))} role="dialog" aria-modal="true" aria-labelledby={(format!("{}-title", dialog_id))} {
                        @if self.close_button {
                            a href="#" class="sh-dialog__close" aria-label="Close dialog" {
                                svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                                    line x1="18" y1="6" x2="6" y2="18";
                                    line x1="6" y1="6" x2="18" y2="18";
                                }
                            }
                        }

                        @if let Some(title) = self.title {
                            header class="sh-dialog__header" {
                                h2 class="sh-dialog__title" id={(format!("{}-title", dialog_id))} { (title) }
                            }
                        }

                        div class="sh-dialog__body" {
                            (self.content.clone())
                        }

                        @if let Some(footer) = &self.footer {
                            footer class="sh-dialog__footer" {
                                (footer.clone())
                            }
                        }
                    }
                }
            }
        }
    }
}

pub struct DialogTrigger<'a> {
    pub dialog_id: &'a str,
    pub label: &'a str,
    pub trigger_class: Option<&'a str>,
}

impl<'a> Render for DialogTrigger<'a> {
    fn render(&self) -> Markup {
        let trigger_id = format!("sh-dialog-trigger-{}", self.dialog_id);
        let cls = self.trigger_class.unwrap_or("sh-btn sh-btn--primary");

        html! {
            a href={(format!("#sh-dialog-{}", self.dialog_id))} class=(cls) id=(trigger_id) {
                (self.label)
            }
        }
    }
}

pub struct ConfirmDialog<'a> {
    id: &'a str,
    title: &'a str,
    message: &'a str,
    confirm_label: &'a str,
    cancel_label: &'a str,
    variant: DialogVariant,
}

impl<'a> ConfirmDialog<'a> {
    pub fn new(id: &'a str, title: &'a str, message: &'a str) -> Self {
        Self {
            id,
            title,
            message,
            confirm_label: "Confirm",
            cancel_label: "Cancel",
            variant: DialogVariant::Default,
        }
    }

    pub fn confirm_label(mut self, label: &'a str) -> Self {
        self.confirm_label = label;
        self
    }

    pub fn cancel_label(mut self, label: &'a str) -> Self {
        self.cancel_label = label;
        self
    }

    pub fn variant(mut self, variant: DialogVariant) -> Self {
        self.variant = variant;
        self
    }
}

impl<'a> Render for ConfirmDialog<'a> {
    fn render(&self) -> Markup {
        let content = html! { p { (self.message) } };
        let footer = html! {
            div class="sh-dialog__actions" {
                a href="#" class="sh-btn sh-btn--ghost" { (self.cancel_label) }
                a href="#" class="sh-btn sh-btn--primary" { (self.confirm_label) }
            }
        };

        Dialog::new(self.id, content)
            .title(self.title)
            .footer(footer)
            .variant(self.variant)
            .size(DialogSize::Sm)
            .render()
    }
}

pub fn dialog_css() -> String {
    r#"
.sh-dialog-wrapper {
    display: contents;
}

.sh-dialog-trigger {
    display: none;
}

.sh-dialog-wrapper[open] .sh-dialog-overlay {
    display: flex;
}

.sh-dialog-overlay {
    display: none;
    position: fixed;
    inset: 0;
    z-index: 1000;
    align-items: center;
    justify-content: center;
    padding: 1rem;
    background: rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(4px);
    animation: sh-dialog-fade-in 0.2s ease;
}

@keyframes sh-dialog-fade-in {
    from { opacity: 0; }
    to { opacity: 1; }
}

.sh-dialog {
    position: relative;
    width: 100%;
    max-height: calc(100vh - 2rem);
    background: var(--sh-surface, #fff);
    border-radius: var(--sh-radius-lg, 0.75rem);
    box-shadow: var(--sh-shadow-xl, 0 25px 50px -12px rgba(0, 0, 0, 0.25));
    overflow: hidden;
    animation: sh-dialog-scale-in 0.2s ease;
}

@keyframes sh-dialog-scale-in {
    from {
        opacity: 0;
        transform: scale(0.95) translateY(-10px);
    }
    to {
        opacity: 1;
        transform: scale(1) translateY(0);
    }
}

.sh-dialog--sm { max-width: 400px; }
.sh-dialog--md { max-width: 500px; }
.sh-dialog--lg { max-width: 700px; }
.sh-dialog--xl { max-width: 900px; }
.sh-dialog--full {
    max-width: none;
    max-height: none;
    width: calc(100vw - 2rem);
    height: calc(100vh - 2rem);
}

.sh-dialog__close {
    position: absolute;
    top: 1rem;
    right: 1rem;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 2rem;
    height: 2rem;
    color: var(--sh-text-muted, #6b7280);
    border-radius: var(--sh-radius-sm, 0.25rem);
    transition: all 0.2s ease;
    text-decoration: none;
}

.sh-dialog__close:hover {
    background: var(--sh-surface-2, #f3f4f6);
    color: var(--sh-text, #1f2937);
}

.sh-dialog__header {
    padding: 1.5rem 1.5rem 0;
}

.sh-dialog__title {
    margin: 0;
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--sh-text, #1f2937);
}

.sh-dialog__body {
    padding: 1.5rem;
    font-size: 0.9375rem;
    line-height: 1.6;
    color: var(--sh-text-secondary, #4b5563);
    overflow-y: auto;
    max-height: calc(100vh - 16rem);
}

.sh-dialog__footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
    padding: 1rem 1.5rem;
    background: var(--sh-surface-2, #f9fafb);
    border-top: 1px solid var(--sh-border, #e5e7eb);
}

.sh-dialog__actions {
    display: flex;
    gap: 0.75rem;
}

/* Variants */
.sh-dialog-wrapper--danger .sh-dialog__title {
    color: var(--sh-error, #ef4444);
}

.sh-dialog-wrapper--success .sh-dialog__title {
    color: var(--sh-success, #10b981);
}

.sh-dialog-wrapper--warning .sh-dialog__title {
    color: var(--sh-warning, #f59e0b);
}

.sh-dialog-wrapper--info .sh-dialog__title {
    color: var(--sh-info, #3b82f6);
}

/* Dark mode support */
@media (prefers-color-scheme: dark) {
    .sh-dialog {
        background: var(--sh-surface, #1f2937);
    }

    .sh-dialog__footer {
        background: var(--sh-surface-2, #111827);
        border-color: var(--sh-border, #374151);
    }
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dialog_creation() {
        let content = html! { p { "Hello" } };
        let dialog = Dialog::new("test", content)
            .title("Test Dialog")
            .size(DialogSize::Lg);

        assert_eq!(dialog.id, "test");
        assert_eq!(dialog.title, Some("Test Dialog"));
        assert_eq!(dialog.size, DialogSize::Lg);
    }

    #[test]
    fn test_confirm_dialog() {
        let confirm = ConfirmDialog::new("delete", "Delete Item?", "Are you sure?")
            .variant(DialogVariant::Danger);

        assert_eq!(confirm.title, "Delete Item?");
        assert_eq!(confirm.variant, DialogVariant::Danger);
    }
}
