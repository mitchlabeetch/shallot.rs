//! RichText Component - Pure CSS Text Formatting Toolbar
//!
//! A visually-styled rich text editor interface using CSS only.
//! Actual formatting happens server-side; this provides the UI shell.

use maud::{html, Markup, Render};

/// RichText toolbar style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RichTextStyle {
    #[default]
    Minimal,
    Compact,
    Full,
}

impl RichTextStyle {
    fn css_class(&self) -> &'static str {
        match self {
            RichTextStyle::Minimal => "sh-richtext--minimal",
            RichTextStyle::Compact => "sh-richtext--compact",
            RichTextStyle::Full => "sh-richtext--full",
        }
    }
}

/// RichText component with formatting toolbar
pub struct RichText<'a> {
    name: &'a str,
    value: &'a str,
    placeholder: Option<&'a str>,
    style: RichTextStyle,
    disabled: bool,
    readonly: bool,
    class: Option<&'a str>,
}

impl<'a> RichText<'a> {
    /// Create a new RichText editor
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            value: "",
            placeholder: None,
            style: RichTextStyle::default(),
            disabled: false,
            readonly: false,
            class: None,
        }
    }

    /// Set the initial value
    pub fn value(mut self, value: &'a str) -> Self {
        self.value = value;
        self
    }

    /// Set placeholder text
    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    /// Set the toolbar style
    pub fn style(mut self, style: RichTextStyle) -> Self {
        self.style = style;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set readonly state
    pub fn readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    /// Add custom class
    pub fn class(mut self, class: &'a str) -> Self {
        self.class = Some(class);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-richtext".to_string()];
        classes.push(self.style.css_class().to_string());
        if self.disabled {
            classes.push("sh-richtext--disabled".to_string());
        }
        if self.readonly {
            classes.push("sh-richtext--readonly".to_string());
        }
        if let Some(custom) = self.class {
            classes.push(custom.to_string());
        }
        classes.join(" ")
    }
}

impl<'a> Render for RichText<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let editor_id = format!("sh-richtext-{}", self.name);

        html! {
            div
                class=(classes)
                role="application"
                aria-label="Rich text editor"
            {
                div class="sh-richtext__toolbar" role="toolbar" aria-label="Formatting options" {
                    button type="button" class="sh-richtext__btn" aria-label="Bold" tabindex="-1" {
                        svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" {
                            path d="M6 4h8a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z";
                            path d="M6 12h9a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z";
                        }
                    }
                    button type="button" class="sh-richtext__btn" aria-label="Italic" tabindex="-1" {
                        svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" {
                            line x1="19" y1="4" x2="10" y2="4";
                            line x1="14" y1="20" x2="5" y2="20";
                            line x1="15" y1="4" x2="9" y2="20";
                        }
                    }
                    button type="button" class="sh-richtext__btn" aria-label="Underline" tabindex="-1" {
                        svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" {
                            path d="M6 3v7a6 6 0 0 0 6 6 6 6 0 0 0 6-6V3";
                            line x1="4" y1="21" x2="20" y2="21";
                        }
                    }
                    @if self.style == RichTextStyle::Full {
                        button type="button" class="sh-richtext__btn" aria-label="Strikethrough" tabindex="-1" {
                            svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" {
                                path d="M16 4H9a3 3 0 0 0-2.83 4";
                                path d="M14 12a4 4 0 0 1 0 8H6";
                                line x1="4" y1="12" x2="20" y2="12";
                            }
                        }
                        button type="button" class="sh-richtext__btn" aria-label="Link" tabindex="-1" {
                            svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" {
                                path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71";
                                path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71";
                            }
                        }
                    }
                }
                div
                    class="sh-richtext__editor"
                    id=(editor_id)
                    contenteditable=[if !self.readonly { Some("true") } else { None }]
                    aria-multiline="true"
                    disabled=[if self.disabled { Some("") } else { None }]
                {
                    (self.value)
                }
                @if let Some(placeholder) = self.placeholder {
                    span class="sh-richtext__placeholder" aria-hidden="true" { (placeholder) }
                }
            }
        }
    }
}

/// Generate CSS for RichText component
pub fn rich_text_css() -> String {
    r#"
.sh-richtext {
    border: 1px solid var(--sh-border, #e5e7eb);
    border-radius: var(--sh-radius-lg, 0.5rem);
    overflow: hidden;
    background: var(--sh-surface, #fff);
}

.sh-richtext__toolbar {
    display: flex;
    gap: 0.25rem;
    padding: 0.5rem;
    background: var(--sh-surface-2, #f9fafb);
    border-bottom: 1px solid var(--sh-border, #e5e7eb);
}

.sh-richtext__btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 2rem;
    height: 2rem;
    border: none;
    background: transparent;
    border-radius: var(--sh-radius-md, 0.375rem);
    color: var(--sh-text-muted, #6b7280);
    cursor: pointer;
    transition: all 0.2s ease;
}

.sh-richtext__btn:hover {
    background: var(--sh-surface, #fff);
    color: var(--sh-text, #1f2937);
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

.sh-richtext__editor {
    min-height: 8rem;
    padding: 1rem;
    font-size: 1rem;
    line-height: 1.6;
    color: var(--sh-text, #1f2937);
    outline: none;
}

.sh-richtext__editor:focus {
    background: var(--sh-surface-focus, #fff);
}

.sh-richtext__placeholder {
    position: absolute;
    color: var(--sh-text-muted, #9ca3af);
    pointer-events: none;
}

/* Style variants */
.sh-richtext--minimal .sh-richtext__toolbar {
    display: none;
}

.sh-richtext--compact .sh-richtext__editor {
    min-height: 4rem;
}

.sh-richtext--full .sh-richtext__toolbar {
    flex-wrap: wrap;
}

/* States */
.sh-richtext--disabled {
    opacity: 0.5;
    pointer-events: none;
}

.sh-richtext--readonly .sh-richtext__toolbar {
    display: none;
}

.sh-richtext--readonly .sh-richtext__editor {
    background: var(--sh-surface-2, #f3f4f6);
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_richtext_creation() {
        let rt = RichText::new("content");
        assert_eq!(rt.name, "content");
        assert_eq!(rt.style, RichTextStyle::Minimal);
    }

    #[test]
    fn test_richtext_value() {
        let rt = RichText::new("editor").value("Initial content");
        assert_eq!(rt.value, "Initial content");
    }

    #[test]
    fn test_richtext_placeholder() {
        let rt = RichText::new("editor").placeholder("Enter text...");
        assert_eq!(rt.placeholder, Some("Enter text..."));
    }

    #[test]
    fn test_richtext_style() {
        let rt = RichText::new("editor").style(RichTextStyle::Full);
        assert_eq!(rt.style, RichTextStyle::Full);
    }

    #[test]
    fn test_richtext_disabled() {
        let rt = RichText::new("editor").disabled(true);
        assert!(rt.disabled);
    }

    #[test]
    fn test_richtext_css() {
        let css = rich_text_css();
        assert!(css.contains(".sh-richtext"));
        assert!(css.contains(".sh-richtext__toolbar"));
    }
}
