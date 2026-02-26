//! GlassSelect Component - Custom Styled Select with CSS-only Dropdown
//!
//! A beautifully styled select dropdown using pure CSS peer selectors.
//! No JavaScript required for the dropdown interaction.

use maud::{html, Markup, Render};

/// GlassSelect size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GlassSelectSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl GlassSelectSize {
    fn css_class(&self) -> &'static str {
        match self {
            GlassSelectSize::Small => "sh-glassselect--sm",
            GlassSelectSize::Medium => "sh-glassselect--md",
            GlassSelectSize::Large => "sh-glassselect--lg",
        }
    }
}

/// Option for GlassSelect
pub struct GlassSelectOption<'a> {
    pub value: &'a str,
    pub label: &'a str,
    pub disabled: bool,
}

/// GlassSelect component
pub struct GlassSelect<'a> {
    name: &'a str,
    options: Vec<GlassSelectOption<'a>>,
    selected: Option<&'a str>,
    placeholder: Option<&'a str>,
    size: GlassSelectSize,
    disabled: bool,
    required: bool,
    class: Option<&'a str>,
}

impl<'a> GlassSelect<'a> {
    /// Create a new GlassSelect
    pub fn new(name: &'a str, options: Vec<GlassSelectOption<'a>>) -> Self {
        Self {
            name,
            options,
            selected: None,
            placeholder: None,
            size: GlassSelectSize::default(),
            disabled: false,
            required: false,
            class: None,
        }
    }

    /// Set selected value
    pub fn selected(mut self, value: &'a str) -> Self {
        self.selected = Some(value);
        self
    }

    /// Set placeholder
    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    /// Set size
    pub fn size(mut self, size: GlassSelectSize) -> Self {
        self.size = size;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set required state
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Add custom class
    pub fn class(mut self, class: &'a str) -> Self {
        self.class = Some(class);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-glassselect".to_string()];
        classes.push(self.size.css_class().to_string());
        if self.disabled {
            classes.push("sh-glassselect--disabled".to_string());
        }
        if let Some(custom) = self.class {
            classes.push(custom.to_string());
        }
        classes.join(" ")
    }
}

impl<'a> Render for GlassSelect<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let select_id = format!("sh-glassselect-{}", self.name);

        html! {
            div class=(classes) role="combobox" aria-haspopup="listbox" aria-expanded="false" {
                label class="sh-glassselect__label" for=(select_id) {
                    @if let Some(placeholder) = self.placeholder {
                        (placeholder)
                    } @else {
                        "Select an option"
                    }
                }
                select
                    id=(select_id)
                    name=(self.name)
                    disabled?[self.disabled]
                    required?[self.required]
                    aria-label=(self.placeholder.unwrap_or("Select an option"))
                {
                    @for option in &self.options {
                        option
                            value=(option.value)
                            selected=[if self.selected == Some(option.value) { Some("") } else { None }]
                            disabled?[option.disabled]
                        {
                            (option.label)
                        }
                    }
                }
                span class="sh-glassselect__arrow" aria-hidden="true" {
                    svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" {
                        polyline points="6 9 12 15 18 9";
                    }
                }
            }
        }
    }
}

/// Generate CSS for GlassSelect component
pub fn glass_select_css() -> String {
    r#"
.sh-glassselect {
    position: relative;
    display: inline-block;
    min-width: 12rem;
}

.sh-glassselect__label {
    display: block;
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--sh-text, #1f2937);
    margin-bottom: 0.5rem;
}

.sh-glassselect select {
    appearance: none;
    -webkit-appearance: none;
    -moz-appearance: none;
    width: 100%;
    padding: 0.75rem 2.5rem 0.75rem 1rem;
    font-size: 1rem;
    font-family: inherit;
    color: var(--sh-text, #1f2937);
    background: var(--sh-surface, #fff);
    border: 1px solid var(--sh-border, #e5e7eb);
    border-radius: var(--sh-radius-md, 0.375rem);
    cursor: pointer;
    transition: all 0.2s ease;
}

.sh-glassselect select:hover {
    border-color: var(--sh-primary, #3b82f6);
}

.sh-glassselect select:focus {
    outline: none;
    border-color: var(--sh-primary, #3b82f6);
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.sh-glassselect__arrow {
    position: absolute;
    right: 1rem;
    top: 50%;
    transform: translateY(-50%);
    pointer-events: none;
    color: var(--sh-text-muted, #6b7280);
}

/* Size variants */
.sh-glassselect--sm select {
    padding: 0.5rem 2.5rem 0.5rem 0.75rem;
    font-size: 0.875rem;
}

.sh-glassselect--md select {
    padding: 0.75rem 2.5rem 0.75rem 1rem;
    font-size: 1rem;
}

.sh-glassselect--lg select {
    padding: 1rem 2.5rem 1rem 1.25rem;
    font-size: 1.125rem;
}

/* Disabled state */
.sh-glassselect--disabled {
    opacity: 0.5;
    pointer-events: none;
}

.sh-glassselect--disabled select {
    background: var(--sh-surface-2, #f3f4f6);
    cursor: not-allowed;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glassselect_creation() {
        let options = vec![
            GlassSelectOption {
                value: "1",
                label: "Option 1",
                disabled: false,
            },
            GlassSelectOption {
                value: "2",
                label: "Option 2",
                disabled: false,
            },
        ];
        let select = GlassSelect::new("test", options);
        assert_eq!(select.name, "test");
        assert_eq!(select.size, GlassSelectSize::Medium);
    }

    #[test]
    fn test_glassselect_selected() {
        let options = vec![GlassSelectOption {
            value: "1",
            label: "One",
            disabled: false,
        }];
        let select = GlassSelect::new("test", options).selected("1");
        assert_eq!(select.selected, Some("1"));
    }

    #[test]
    fn test_glassselect_size() {
        let options = vec![];
        let select = GlassSelect::new("test", options).size(GlassSelectSize::Large);
        assert_eq!(select.size, GlassSelectSize::Large);
    }

    #[test]
    fn test_glassselect_css() {
        let css = glass_select_css();
        assert!(css.contains(".sh-glassselect"));
        assert!(css.contains(".sh-glassselect__arrow"));
    }
}
