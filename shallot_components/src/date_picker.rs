//! Date Picker Component - CSS-only date selection
//!
//! A zero-JavaScript date picker using native input[type="date"] with enhanced styling.

use maud::{html, Markup, Render};

/// Date picker size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DatePickerSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Date picker style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DatePickerVariant {
    #[default]
    Default,
    Inline,
    Range,
}

/// Date Picker component
#[derive(Debug, Clone)]
pub struct DatePicker<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub value: Option<&'a str>,
    pub min_date: Option<&'a str>,
    pub max_date: Option<&'a str>,
    pub placeholder: Option<&'a str>,
    pub size: DatePickerSize,
    pub variant: DatePickerVariant,
    pub disabled: bool,
    pub required: bool,
    pub label: Option<&'a str>,
}

impl<'a> DatePicker<'a> {
    /// Create a new date picker
    pub fn new(id: &'a str, name: &'a str) -> Self {
        Self {
            id,
            name,
            value: None,
            min_date: None,
            max_date: None,
            placeholder: None,
            size: DatePickerSize::Md,
            variant: DatePickerVariant::Default,
            disabled: false,
            required: false,
            label: None,
        }
    }

    /// Set the initial value
    pub fn value(mut self, value: &'a str) -> Self {
        self.value = Some(value);
        self
    }

    /// Set minimum date
    pub fn min_date(mut self, min: &'a str) -> Self {
        self.min_date = Some(min);
        self
    }

    /// Set maximum date
    pub fn max_date(mut self, max: &'a str) -> Self {
        self.max_date = Some(max);
        self
    }

    /// Set placeholder text
    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    /// Set size variant
    pub fn size(mut self, size: DatePickerSize) -> Self {
        self.size = size;
        self
    }

    /// Set style variant
    pub fn variant(mut self, variant: DatePickerVariant) -> Self {
        self.variant = variant;
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

    /// Set label
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-date-picker".to_string()];

        classes.push(match self.size {
            DatePickerSize::Sm => "sh-date-picker--sm".to_string(),
            DatePickerSize::Md => "sh-date-picker--md".to_string(),
            DatePickerSize::Lg => "sh-date-picker--lg".to_string(),
        });

        classes.push(match self.variant {
            DatePickerVariant::Default => "sh-date-picker--default".to_string(),
            DatePickerVariant::Inline => "sh-date-picker--inline".to_string(),
            DatePickerVariant::Range => "sh-date-picker--range".to_string(),
        });

        if self.disabled {
            classes.push("sh-date-picker--disabled".to_string());
        }

        classes.join(" ")
    }
}

impl<'a> Render for DatePicker<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();

        html! {
            div class=(classes) {
                @if let Some(label) = self.label {
                    label class="sh-date-picker__label" for=(self.id) {
                        (label)
                        @if self.required {
                            span class="sh-date-picker__required" { "*" }
                        }
                    }
                }
                input
                    type="date"
                    id=(self.id)
                    name=(self.name)
                    class="sh-date-picker__input"
                    value=[self.value]
                    min=[self.min_date]
                    max=[self.max_date]
                    placeholder=[self.placeholder]
                    disabled?[self.disabled]
                    required?[self.required]
                    aria-label=[self.label.or(Some("Date picker"))];
            }
        }
    }
}

/// Generate date picker CSS
pub fn date_picker_css() -> String {
    r#"
/* Date Picker Component */
.sh-date-picker {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    font-family: var(--sh-font-sans);
}

.sh-date-picker__label {
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--sh-text);
}

.sh-date-picker__required {
    color: var(--sh-error);
    margin-left: 0.25rem;
}

.sh-date-picker__input {
    width: 100%;
    padding: 0.75rem 1rem;
    font-size: 1rem;
    font-family: inherit;
    color: var(--sh-text);
    background: var(--sh-surface);
    border: 1px solid var(--sh-border);
    border-radius: var(--sh-radius-md);
    cursor: pointer;
    transition: all var(--sh-dur-fast) var(--sh-ease-out);
}

.sh-date-picker__input:hover:not(:disabled) {
    border-color: var(--sh-accent);
}

.sh-date-picker__input:focus {
    outline: none;
    border-color: var(--sh-accent);
    box-shadow: 0 0 0 3px color-mix(in srgb, var(--sh-accent) 20%, transparent);
}

/* Size variants */
.sh-date-picker--sm .sh-date-picker__input {
    padding: 0.5rem 0.75rem;
    font-size: 0.875rem;
}

.sh-date-picker--lg .sh-date-picker__input {
    padding: 1rem 1.25rem;
    font-size: 1.125rem;
}

/* Inline variant */
.sh-date-picker--inline {
    flex-direction: row;
    align-items: center;
    gap: 0.75rem;
}

.sh-date-picker--inline .sh-date-picker__input {
    width: auto;
}

/* Disabled state */
.sh-date-picker--disabled .sh-date-picker__input {
    opacity: 0.5;
    cursor: not-allowed;
    background: var(--sh-surface-2);
}

/* Native date picker styling */
.sh-date-picker__input::-webkit-calendar-picker-indicator {
    cursor: pointer;
    opacity: 0.6;
    transition: opacity var(--sh-dur-fast);
}

.sh-date-picker__input:hover::-webkit-calendar-picker-indicator {
    opacity: 1;
}

/* Dark mode adjustments */
@media (prefers-color-scheme: dark) {
    .sh-date-picker__input {
        color-scheme: dark;
    }
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_picker_creation() {
        let picker = DatePicker::new("date", "date")
            .value("2026-02-22")
            .min_date("2026-01-01")
            .max_date("2026-12-31");

        assert_eq!(picker.id, "date");
        assert_eq!(picker.name, "date");
        assert_eq!(picker.value, Some("2026-02-22"));
    }

    #[test]
    fn test_date_picker_size() {
        let picker = DatePicker::new("date", "date").size(DatePickerSize::Lg);

        assert_eq!(picker.size, DatePickerSize::Lg);
    }
}
