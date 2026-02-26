//! Stepper Input Component - Numeric input with increment/decrement buttons
//! CSS-only styling with custom appearance for spinner controls

use maud::{html, Markup, Render};

/// Stepper input size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StepperSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Stepper input component
#[derive(Debug, Clone)]
pub struct StepperInput<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub value: i32,
    pub min: i32,
    pub max: i32,
    pub step: i32,
    pub size: StepperSize,
    pub disabled: bool,
    pub label: Option<&'a str>,
    pub placeholder: Option<&'a str>,
}

impl<'a> StepperInput<'a> {
    /// Create a new stepper input
    pub fn new(id: &'a str, name: &'a str) -> Self {
        Self {
            id,
            name,
            value: 0,
            min: 0,
            max: 100,
            step: 1,
            size: StepperSize::default(),
            disabled: false,
            label: None,
            placeholder: None,
        }
    }

    /// Set the initial value
    pub fn value(mut self, value: i32) -> Self {
        self.value = value;
        self
    }

    /// Set the minimum value
    pub fn min(mut self, min: i32) -> Self {
        self.min = min;
        self
    }

    /// Set the maximum value
    pub fn max(mut self, max: i32) -> Self {
        self.max = max;
        self
    }

    /// Set the step increment
    pub fn step(mut self, step: i32) -> Self {
        self.step = step;
        self
    }

    /// Set the size
    pub fn size(mut self, size: StepperSize) -> Self {
        self.size = size;
        self
    }

    /// Set the disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the label
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    /// Set the placeholder
    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-stepper".to_string()];

        let size_class = match self.size {
            StepperSize::Sm => "sh-stepper--sm",
            StepperSize::Md => "sh-stepper--md",
            StepperSize::Lg => "sh-stepper--lg",
        };
        classes.push(size_class.to_string());

        if self.disabled {
            classes.push("sh-stepper--disabled".to_string());
        }

        classes.join(" ")
    }
}

impl<'a> Render for StepperInput<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();

        html! {
            div class=(classes) {
                @if let Some(label) = self.label {
                    label class="sh-stepper__label" for=(self.id) {
                        (label)
                    }
                }
                div class="sh-stepper__controls" {
                    button
                        type="button"
                        class="sh-stepper__btn sh-stepper__btn--decrement"
                        disabled?=(self.disabled || self.value <= self.min)
                        aria-label="Decrease value"
                        data-action="decrement"
                        data-target=(self.id) {
                        "−"
                    }
                    input
                        type="number"
                        id=(self.id)
                        name=(self.name)
                        value=(self.value)
                        min=(self.min)
                        max=(self.max)
                        step=(self.step)
                        class="sh-stepper__input"
                        disabled?=(self.disabled)
                        placeholder?=(self.placeholder)
                        aria-valuemin=(self.min)
                        aria-valuemax=(self.max)
                        aria-valuenow=(self.value);
                    button
                        type="button"
                        class="sh-stepper__btn sh-stepper__btn--increment"
                        disabled?=(self.disabled || self.value >= self.max)
                        aria-label="Increase value"
                        data-action="increment"
                        data-target=(self.id) {
                        "+"
                    }
                }
            }
        }
    }
}

/// Generate CSS for stepper input component
pub fn stepper_input_css() -> String {
    r#"
.sh-stepper {
    display: inline-flex;
    flex-direction: column;
    gap: var(--sh-spacing-1, 0.25rem);
}

.sh-stepper__label {
    font-size: var(--sh-font-size-sm, 0.875rem);
    font-weight: var(--sh-font-weight-medium, 500);
    color: var(--sh-color-foreground, #1f2937);
}

.sh-stepper__controls {
    display: inline-flex;
    align-items: center;
    border: 1px solid var(--sh-color-border, #e5e7eb);
    border-radius: var(--sh-radius-md, 0.375rem);
    background-color: var(--sh-color-background, #ffffff);
    overflow: hidden;
}

.sh-stepper--sm .sh-stepper__controls {
    height: var(--sh-spacing-8, 2rem);
}

.sh-stepper--md .sh-stepper__controls {
    height: var(--sh-spacing-10, 2.5rem);
}

.sh-stepper--lg .sh-stepper__controls {
    height: var(--sh-spacing-12, 3rem);
}

.sh-stepper__btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: var(--sh-spacing-8, 2rem);
    height: 100%;
    border: none;
    background-color: var(--sh-color-muted, #f3f4f6);
    color: var(--sh-color-foreground, #1f2937);
    font-size: var(--sh-font-size-lg, 1.125rem);
    font-weight: var(--sh-font-weight-medium, 500);
    cursor: pointer;
    transition: background-color 0.15s ease;
}

.sh-stepper__btn:hover:not(:disabled) {
    background-color: var(--sh-color-primary-hover, #2563eb);
    color: var(--sh-color-primary-foreground, #ffffff);
}

.sh-stepper__btn:focus-visible {
    outline: 2px solid var(--sh-color-primary, #3b82f6);
    outline-offset: -2px;
}

.sh-stepper__btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

.sh-stepper--sm .sh-stepper__btn {
    width: var(--sh-spacing-6, 1.5rem);
    font-size: var(--sh-font-size-sm, 0.875rem);
}

.sh-stepper--lg .sh-stepper__btn {
    width: var(--sh-spacing-10, 2.5rem);
    font-size: var(--sh-font-size-xl, 1.25rem);
}

.sh-stepper__input {
    width: var(--sh-spacing-16, 4rem);
    height: 100%;
    border: none;
    border-left: 1px solid var(--sh-color-border, #e5e7eb);
    border-right: 1px solid var(--sh-color-border, #e5e7eb);
    text-align: center;
    font-size: var(--sh-font-size-base, 1rem);
    color: var(--sh-color-foreground, #1f2937);
    background-color: transparent;
    padding: 0 var(--sh-spacing-2, 0.5rem);
}

.sh-stepper--sm .sh-stepper__input {
    width: var(--sh-spacing-12, 3rem);
    font-size: var(--sh-font-size-sm, 0.875rem);
}

.sh-stepper--lg .sh-stepper__input {
    width: var(--sh-spacing-20, 5rem);
    font-size: var(--sh-font-size-lg, 1.125rem);
}

.sh-stepper__input:focus {
    outline: none;
}

.sh-stepper__input::-webkit-outer-spin-button,
.sh-stepper__input::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
}

.sh-stepper__input[type=number] {
    -moz-appearance: textfield;
}

.sh-stepper--disabled .sh-stepper__controls {
    opacity: 0.5;
    cursor: not-allowed;
    background-color: var(--sh-color-muted, #f3f4f6);
}

.sh-stepper--disabled .sh-stepper__input {
    background-color: var(--sh-color-muted, #f3f4f6);
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stepper_input_creation() {
        let stepper = StepperInput::new("quantity", "quantity")
            .value(5)
            .min(0)
            .max(10)
            .step(1);

        assert_eq!(stepper.id, "quantity");
        assert_eq!(stepper.value, 5);
        assert_eq!(stepper.min, 0);
        assert_eq!(stepper.max, 10);
        assert_eq!(stepper.step, 1);
    }

    #[test]
    fn test_stepper_input_render() {
        let stepper = StepperInput::new("qty", "qty").label("Quantity").value(1);

        let html = stepper.render().into_string();
        assert!(html.contains("sh-stepper"));
        assert!(html.contains("sh-stepper__label"));
        assert!(html.contains("Quantity"));
        assert!(html.contains("type=\"number\""));
        assert!(html.contains("value=\"1\""));
    }

    #[test]
    fn test_stepper_input_sizes() {
        let sm = StepperInput::new("qty", "qty").size(StepperSize::Sm);
        let md = StepperInput::new("qty", "qty").size(StepperSize::Md);
        let lg = StepperInput::new("qty", "qty").size(StepperSize::Lg);

        assert!(sm.render().into_string().contains("sh-stepper--sm"));
        assert!(md.render().into_string().contains("sh-stepper--md"));
        assert!(lg.render().into_string().contains("sh-stepper--lg"));
    }

    #[test]
    fn test_stepper_input_disabled() {
        let stepper = StepperInput::new("qty", "qty").disabled(true);
        let html = stepper.render().into_string();

        assert!(html.contains("sh-stepper--disabled"));
        assert!(html.contains("disabled"));
    }

    #[test]
    fn test_stepper_css() {
        let css = stepper_input_css();
        assert!(css.contains(".sh-stepper"));
        assert!(css.contains(".sh-stepper__controls"));
        assert!(css.contains(".sh-stepper__input"));
        assert!(css.contains(".sh-stepper__btn"));
    }
}
