//! Time Picker Component - CSS-only time selection
//!
//! A zero-JavaScript time picker using native input[type="time"] with enhanced styling.

use maud::{html, Markup, Render};

/// Time picker size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TimePickerSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Time format variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TimeFormat {
    #[default]
    Hour24,
    Hour12,
}

/// Time Picker component
#[derive(Debug, Clone)]
pub struct TimePicker<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub value: Option<&'a str>,
    pub min_time: Option<&'a str>,
    pub max_time: Option<&'a str>,
    pub step: Option<u32>,
    pub size: TimePickerSize,
    pub format: TimeFormat,
    pub disabled: bool,
    pub required: bool,
    pub label: Option<&'a str>,
}

impl<'a> TimePicker<'a> {
    /// Create a new time picker
    pub fn new(id: &'a str, name: &'a str) -> Self {
        Self {
            id,
            name,
            value: None,
            min_time: None,
            max_time: None,
            step: None,
            size: TimePickerSize::Md,
            format: TimeFormat::Hour24,
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

    /// Set minimum time
    pub fn min_time(mut self, min: &'a str) -> Self {
        self.min_time = Some(min);
        self
    }

    /// Set maximum time
    pub fn max_time(mut self, max: &'a str) -> Self {
        self.max_time = Some(max);
        self
    }

    /// Set step (in seconds)
    pub fn step(mut self, step: u32) -> Self {
        self.step = Some(step);
        self
    }

    /// Set size variant
    pub fn size(mut self, size: TimePickerSize) -> Self {
        self.size = size;
        self
    }

    /// Set time format
    pub fn format(mut self, format: TimeFormat) -> Self {
        self.format = format;
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
        let mut classes = vec!["sh-time-picker".to_string()];

        classes.push(match self.size {
            TimePickerSize::Sm => "sh-time-picker--sm".to_string(),
            TimePickerSize::Md => "sh-time-picker--md".to_string(),
            TimePickerSize::Lg => "sh-time-picker--lg".to_string(),
        });

        if self.disabled {
            classes.push("sh-time-picker--disabled".to_string());
        }

        classes.join(" ")
    }
}

impl<'a> Render for TimePicker<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let step_value = self.step.map(|s| s.to_string());

        html! {
            div class=(classes) {
                @if let Some(label) = self.label {
                    label class="sh-time-picker__label" for=(self.id) {
                        (label)
                        @if self.required {
                            span class="sh-time-picker__required" { "*" }
                        }
                    }
                }
                input
                    type="time"
                    id=(self.id)
                    name=(self.name)
                    class="sh-time-picker__input"
                    value=[self.value]
                    min=[self.min_time]
                    max=[self.max_time]
                    step=[step_value.as_deref()]
                    disabled?[self.disabled]
                    required?[self.required]
                    aria-label=[self.label.or(Some("Time picker"))];
            }
        }
    }
}

/// Generate time picker CSS
pub fn time_picker_css() -> String {
    r#"
/* Time Picker Component */
.sh-time-picker {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    font-family: var(--sh-font-sans);
}

.sh-time-picker__label {
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--sh-text);
}

.sh-time-picker__required {
    color: var(--sh-error);
    margin-left: 0.25rem;
}

.sh-time-picker__input {
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

.sh-time-picker__input:hover:not(:disabled) {
    border-color: var(--sh-accent);
}

.sh-time-picker__input:focus {
    outline: none;
    border-color: var(--sh-accent);
    box-shadow: 0 0 0 3px color-mix(in srgb, var(--sh-accent) 20%, transparent);
}

/* Size variants */
.sh-time-picker--sm .sh-time-picker__input {
    padding: 0.5rem 0.75rem;
    font-size: 0.875rem;
}

.sh-time-picker--lg .sh-time-picker__input {
    padding: 1rem 1.25rem;
    font-size: 1.125rem;
}

/* Disabled state */
.sh-time-picker--disabled .sh-time-picker__input {
    opacity: 0.5;
    cursor: not-allowed;
    background: var(--sh-surface-2);
}

/* Native time picker styling */
.sh-time-picker__input::-webkit-calendar-picker-indicator {
    cursor: pointer;
    opacity: 0.6;
    transition: opacity var(--sh-dur-fast);
}

.sh-time-picker__input:hover::-webkit-calendar-picker-indicator {
    opacity: 1;
}

/* Dark mode adjustments */
@media (prefers-color-scheme: dark) {
    .sh-time-picker__input {
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
    fn test_time_picker_creation() {
        let picker = TimePicker::new("time", "time")
            .value("14:30")
            .min_time("09:00")
            .max_time("18:00");

        assert_eq!(picker.id, "time");
        assert_eq!(picker.value, Some("14:30"));
    }

    #[test]
    fn test_time_picker_step() {
        let picker = TimePicker::new("time", "time").step(900); // 15 minutes

        assert_eq!(picker.step, Some(900));
    }
}
