//! Checkbox Group Component - Grouped checkboxes with legend
//! CSS-only styling using fieldset/legend

use maud::{html, Markup, Render};

/// Checkbox item
#[derive(Debug, Clone)]
pub struct CheckboxItem<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub label: &'a str,
    pub value: &'a str,
    pub checked: bool,
    pub disabled: bool,
}

impl<'a> CheckboxItem<'a> {
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
            label class="sh-checkbox-group__item" {
                input
                    type="checkbox"
                    id=(self.id)
                    name=(self.name)
                    value=(self.value)
                    checked[self.checked]
                    disabled[self.disabled]
                    class="sh-checkbox-group__input";
                span class="sh-checkbox-group__box" {}
                span class="sh-checkbox-group__label" {
                    (self.label)
                }
            }
        }
    }
}

/// Checkbox group orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CheckboxOrientation {
    #[default]
    Vertical,
    Horizontal,
}

/// Checkbox group component
#[derive(Debug, Clone)]
pub struct CheckboxGroup<'a> {
    pub legend: &'a str,
    pub name: &'a str,
    pub items: Vec<CheckboxItem<'a>>,
    pub orientation: CheckboxOrientation,
    pub disabled: bool,
    pub required: bool,
    pub error: Option<&'a str>,
}

impl<'a> CheckboxGroup<'a> {
    pub fn new(name: &'a str, legend: &'a str) -> Self {
        Self {
            legend,
            name,
            items: Vec::new(),
            orientation: CheckboxOrientation::default(),
            disabled: false,
            required: false,
            error: None,
        }
    }

    pub fn add(mut self, item: CheckboxItem<'a>) -> Self {
        self.items.push(item);
        self
    }

    pub fn orientation(mut self, orientation: CheckboxOrientation) -> Self {
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
        let mut classes = vec!["sh-checkbox-group"];

        match self.orientation {
            CheckboxOrientation::Vertical => classes.push("sh-checkbox-group--vertical"),
            CheckboxOrientation::Horizontal => classes.push("sh-checkbox-group--horizontal"),
        }

        if self.disabled {
            classes.push("sh-checkbox-group--disabled");
        }

        if self.error.is_some() {
            classes.push("sh-checkbox-group--error");
        }

        classes.join(" ")
    }
}

impl<'a> Render for CheckboxGroup<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();

        html! {
            fieldset class=(classes) disabled[self.disabled] {
                legend class="sh-checkbox-group__legend" {
                    (self.legend)
                    @if self.required {
                        span class="sh-checkbox-group__required" aria-label="required" {
                            "*"
                        }
                    }
                }

                div class="sh-checkbox-group__items" {
                    @for item in &self.items {
                        (item.render())
                    }
                }

                @if let Some(error) = self.error {
                    p class="sh-checkbox-group__error" role="alert" {
                        (error)
                    }
                }
            }
        }
    }
}

pub fn checkbox_group_css() -> String {
    r#"
.sh-checkbox-group {
    border: none;
    padding: 0;
    margin: 0;
}

.sh-checkbox-group__legend {
    font-size: var(--sh-font-size-sm, 0.875rem);
    font-weight: var(--sh-font-weight-medium, 500);
    color: var(--sh-color-text, #1a1a1a);
    margin-bottom: var(--sh-spacing-sm, 0.5rem);
    padding: 0;
}

.sh-checkbox-group__required {
    color: var(--sh-color-danger, #ef4444);
    margin-left: var(--sh-spacing-xs, 0.25rem);
}

.sh-checkbox-group__items {
    display: flex;
    gap: var(--sh-spacing-sm, 0.5rem);
}

.sh-checkbox-group--vertical .sh-checkbox-group__items {
    flex-direction: column;
}

.sh-checkbox-group--horizontal .sh-checkbox-group__items {
    flex-direction: row;
    flex-wrap: wrap;
}

.sh-checkbox-group__item {
    display: flex;
    align-items: center;
    gap: var(--sh-spacing-sm, 0.5rem);
    cursor: pointer;
    user-select: none;
}

.sh-checkbox-group__input {
    position: absolute;
    opacity: 0;
    width: 0;
    height: 0;
}

.sh-checkbox-group__box {
    width: 1.125rem;
    height: 1.125rem;
    border: 2px solid var(--sh-color-border, #e5e5e5);
    border-radius: var(--sh-radius-sm, 0.25rem);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.15s ease;
    flex-shrink: 0;
}

.sh-checkbox-group__box::after {
    content: "";
    width: 0.5rem;
    height: 0.25rem;
    border: 2px solid white;
    border-top: none;
    border-right: none;
    transform: rotate(-45deg) scale(0);
    transition: transform 0.15s ease;
    margin-bottom: 2px;
}

.sh-checkbox-group__input:checked + .sh-checkbox-group__box {
    background: var(--sh-color-primary, #3b82f6);
    border-color: var(--sh-color-primary, #3b82f6);
}

.sh-checkbox-group__input:checked + .sh-checkbox-group__box::after {
    transform: rotate(-45deg) scale(1);
}

.sh-checkbox-group__input:focus-visible + .sh-checkbox-group__box {
    box-shadow: 0 0 0 3px var(--sh-color-primary-alpha, rgba(59, 130, 246, 0.2));
}

.sh-checkbox-group__input:disabled + .sh-checkbox-group__box {
    opacity: 0.5;
    cursor: not-allowed;
}

.sh-checkbox-group--disabled .sh-checkbox-group__item {
    opacity: 0.5;
    cursor: not-allowed;
}

.sh-checkbox-group--error .sh-checkbox-group__box {
    border-color: var(--sh-color-danger, #ef4444);
}

.sh-checkbox-group__label {
    font-size: var(--sh-font-size-md, 1rem);
    color: var(--sh-color-text, #1a1a1a);
}

.sh-checkbox-group__error {
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
    fn test_checkbox_item_creation() {
        let item = CheckboxItem::new("colors", "red", "Red")
            .checked(true)
            .value("#ff0000");

        assert_eq!(item.name, "colors");
        assert_eq!(item.id, "red");
        assert!(item.checked);
    }

    #[test]
    fn test_checkbox_group_creation() {
        let group = CheckboxGroup::new("colors", "Choose colors")
            .add(CheckboxItem::new("colors", "red", "Red"))
            .add(CheckboxItem::new("colors", "blue", "Blue"));

        assert_eq!(group.items.len(), 2);
    }

    #[test]
    fn test_checkbox_group_render() {
        let group =
            CheckboxGroup::new("options", "Options").orientation(CheckboxOrientation::Horizontal);

        let html = group.render().into_string();
        assert!(html.contains("sh-checkbox-group--horizontal"));
    }

    #[test]
    fn test_checkbox_group_error() {
        let group = CheckboxGroup::new("test", "Test").error("Select at least one");

        let html = group.render().into_string();
        assert!(html.contains("sh-checkbox-group--error"));
        assert!(html.contains("Select at least one"));
    }

    #[test]
    fn test_css_generation() {
        let css = checkbox_group_css();
        assert!(css.contains(".sh-checkbox-group"));
    }
}
