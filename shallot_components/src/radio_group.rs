//! Radio Group Component - Grouped radio buttons with legend
//! CSS-only styling using fieldset/legend

use maud::{html, Markup, Render};

/// Radio item
#[derive(Debug, Clone)]
pub struct RadioItem<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub label: &'a str,
    pub value: &'a str,
    pub checked: bool,
    pub disabled: bool,
}

impl<'a> RadioItem<'a> {
    pub fn new(name: &'a str, id: &'a str, label: &'a str) -> Self {
        Self {
            id,
            name,
            label,
            value: id,
            checked: false,
            disabled: false,
        }
    }

    pub fn value(mut self, value: &'a str) -> Self {
        self.value = value;
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn render(&self) -> Markup {
        html! {
            label class="sh-radio-group__item" {
                input
                    type="radio"
                    id=(self.id)
                    name=(self.name)
                    value=(self.value)
                    checked[self.checked]
                    disabled[self.disabled]
                    class="sh-radio-group__input";
                span class="sh-radio-group__circle" {}
                span class="sh-radio-group__label" {
                    (self.label)
                }
            }
        }
    }
}

/// Radio group orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RadioOrientation {
    #[default]
    Vertical,
    Horizontal,
}

/// Radio group component
#[derive(Debug, Clone)]
pub struct RadioGroup<'a> {
    pub legend: &'a str,
    pub name: &'a str,
    pub items: Vec<RadioItem<'a>>,
    pub orientation: RadioOrientation,
    pub disabled: bool,
    pub required: bool,
    pub error: Option<&'a str>,
}

impl<'a> RadioGroup<'a> {
    pub fn new(name: &'a str, legend: &'a str) -> Self {
        Self {
            legend,
            name,
            items: Vec::new(),
            orientation: RadioOrientation::default(),
            disabled: false,
            required: false,
            error: None,
        }
    }

    pub fn add(mut self, item: RadioItem<'a>) -> Self {
        self.items.push(item);
        self
    }

    pub fn orientation(mut self, orientation: RadioOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn error(mut self, error: &'a str) -> Self {
        self.error = Some(error);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-radio-group"];

        match self.orientation {
            RadioOrientation::Vertical => classes.push("sh-radio-group--vertical"),
            RadioOrientation::Horizontal => classes.push("sh-radio-group--horizontal"),
        }

        if self.disabled {
            classes.push("sh-radio-group--disabled");
        }

        if self.error.is_some() {
            classes.push("sh-radio-group--error");
        }

        classes.join(" ")
    }
}

impl<'a> Render for RadioGroup<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();

        html! {
            fieldset class=(classes) disabled[self.disabled] role="radiogroup" {
                legend class="sh-radio-group__legend" {
                    (self.legend)
                    @if self.required {
                        span class="sh-radio-group__required" aria-label="required" {
                            "*"
                        }
                    }
                }

                div class="sh-radio-group__items" {
                    @for item in &self.items {
                        (item.render())
                    }
                }

                @if let Some(error) = self.error {
                    p class="sh-radio-group__error" role="alert" {
                        (error)
                    }
                }
            }
        }
    }
}

pub fn radio_group_css() -> String {
    r#"
.sh-radio-group {
    border: none;
    padding: 0;
    margin: 0;
}

.sh-radio-group__legend {
    font-size: var(--sh-font-size-sm, 0.875rem);
    font-weight: var(--sh-font-weight-medium, 500);
    color: var(--sh-color-text, #1a1a1a);
    margin-bottom: var(--sh-spacing-sm, 0.5rem);
    padding: 0;
}

.sh-radio-group__required {
    color: var(--sh-color-danger, #ef4444);
    margin-left: var(--sh-spacing-xs, 0.25rem);
}

.sh-radio-group__items {
    display: flex;
    gap: var(--sh-spacing-sm, 0.5rem);
}

.sh-radio-group--vertical .sh-radio-group__items {
    flex-direction: column;
}

.sh-radio-group--horizontal .sh-radio-group__items {
    flex-direction: row;
    flex-wrap: wrap;
}

.sh-radio-group__item {
    display: flex;
    align-items: center;
    gap: var(--sh-spacing-sm, 0.5rem);
    cursor: pointer;
    user-select: none;
}

.sh-radio-group__input {
    position: absolute;
    opacity: 0;
    width: 0;
    height: 0;
}

.sh-radio-group__circle {
    width: 1.125rem;
    height: 1.125rem;
    border: 2px solid var(--sh-color-border, #e5e5e5);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.15s ease;
    flex-shrink: 0;
}

.sh-radio-group__circle::after {
    content: "";
    width: 0.5rem;
    height: 0.5rem;
    border-radius: 50%;
    background: white;
    transform: scale(0);
    transition: transform 0.15s ease;
}

.sh-radio-group__input:checked + .sh-radio-group__circle {
    background: var(--sh-color-primary, #3b82f6);
    border-color: var(--sh-color-primary, #3b82f6);
}

.sh-radio-group__input:checked + .sh-radio-group__circle::after {
    transform: scale(1);
}

.sh-radio-group__input:focus-visible + .sh-radio-group__circle {
    box-shadow: 0 0 0 3px var(--sh-color-primary-alpha, rgba(59, 130, 246, 0.2));
}

.sh-radio-group__input:disabled + .sh-radio-group__circle {
    opacity: 0.5;
    cursor: not-allowed;
}

.sh-radio-group--disabled .sh-radio-group__item {
    opacity: 0.5;
    cursor: not-allowed;
}

.sh-radio-group--error .sh-radio-group__circle {
    border-color: var(--sh-color-danger, #ef4444);
}

.sh-radio-group__label {
    font-size: var(--sh-font-size-md, 1rem);
    color: var(--sh-color-text, #1a1a1a);
}

.sh-radio-group__error {
    font-size: var(--sh-font-size-sm, 0.875rem);
    color: var(--sh-color-danger, #ef4444);
    margin: var(--sh-spacing-sm, 0.5rem) 0 0 0;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radio_item_creation() {
        let item = RadioItem::new("size", "lg", "Large")
            .value("large")
            .checked(true);

        assert_eq!(item.name, "size");
        assert!(item.checked);
    }

    #[test]
    fn test_radio_group_creation() {
        let group = RadioGroup::new("size", "Select size")
            .add(RadioItem::new("size", "sm", "Small"))
            .add(RadioItem::new("size", "lg", "Large"));

        assert_eq!(group.items.len(), 2);
    }

    #[test]
    fn test_radio_group_render() {
        let group = RadioGroup::new("test", "Test").orientation(RadioOrientation::Horizontal);

        let html = group.render().into_string();
        assert!(html.contains("sh-radio-group--horizontal"));
        assert!(html.contains("role=\"radiogroup\""));
    }

    #[test]
    fn test_radio_group_required() {
        let group = RadioGroup::new("test", "Test").required(true);
        let html = group.render().into_string();

        assert!(html.contains("sh-radio-group__required"));
    }

    #[test]
    fn test_css_generation() {
        let css = radio_group_css();
        assert!(css.contains(".sh-radio-group"));
    }
}
