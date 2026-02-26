//! Toggle Group Component
//!
//! Group of toggleable buttons with single or multiple selection.

use crate::component::ComponentSize;
use maud::{html, Markup, Render};

pub struct ToggleItem<'a> {
    pub value: &'a str,
    pub label: &'a str,
    pub icon: Option<&'a str>,
    pub disabled: bool,
}

pub struct ToggleGroup<'a> {
    name: &'a str,
    items: Vec<ToggleItem<'a>>,
    selected: Vec<&'a str>,
    size: ComponentSize,
    variant: ToggleVariant,
    multiple: bool,
    required: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleVariant {
    #[default]
    Default,
    Outline,
    Filled,
    Pills,
}

impl<'a> ToggleGroup<'a> {
    pub fn new(name: &'a str, items: Vec<ToggleItem<'a>>) -> Self {
        Self {
            name,
            items,
            selected: Vec::new(),
            size: ComponentSize::Md,
            variant: ToggleVariant::Default,
            multiple: false,
            required: false,
        }
    }

    pub fn selected(mut self, values: Vec<&'a str>) -> Self {
        self.selected = values;
        self
    }

    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: ToggleVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    fn variant_class(&self) -> &'static str {
        match self.variant {
            ToggleVariant::Default => "sh-toggle-group--default",
            ToggleVariant::Outline => "sh-toggle-group--outline",
            ToggleVariant::Filled => "sh-toggle-group--filled",
            ToggleVariant::Pills => "sh-toggle-group--pills",
        }
    }

    fn input_type(&self) -> &'static str {
        if self.multiple {
            "checkbox"
        } else {
            "radio"
        }
    }
}

impl<'a> Render for ToggleGroup<'a> {
    fn render(&self) -> Markup {
        let size_class = format!("sh-toggle-group--{}", self.size.class_suffix());

        html! {
            div class={(format!("sh-toggle-group {} {}", self.variant_class(), size_class))} role="group" {
                @for item in &self.items {
                    @let input_id = format!("sh-toggle-{}-{}", self.name, item.value);
                    @let is_selected = self.selected.contains(&item.value);
                    @let item_classes = {
                        let mut cls = vec!["sh-toggle-item"];
                        if is_selected { cls.push("sh-toggle-item--active"); }
                        if item.disabled { cls.push("sh-toggle-item--disabled"); }
                        cls.join(" ")
                    };

                    label class=(item_classes) {
                        input
                            class="sh-toggle-item__input"
                            type=(self.input_type())
                            name=(self.name)
                            id=(input_id)
                            value=(item.value)
                            checked?[is_selected]
                            disabled?[item.disabled]
                            required?[self.required && self.items.first().map(|i| i.value) == Some(item.value)];

                        span class="sh-toggle-item__content" {
                            @if let Some(icon) = item.icon {
                                span class="sh-toggle-item__icon" {
                                    (maud::PreEscaped(icon))
                                }
                            }
                            @if !item.label.is_empty() {
                                span class="sh-toggle-item__label" { (item.label) }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub struct IconButtonGroup<'a> {
    name: &'a str,
    buttons: Vec<IconButton<'a>>,
    selected: Option<&'a str>,
    size: ComponentSize,
}

pub struct IconButton<'a> {
    pub value: &'a str,
    pub icon: &'a str,
    pub label: &'a str,
    pub disabled: bool,
}

impl<'a> IconButtonGroup<'a> {
    pub fn new(name: &'a str, buttons: Vec<IconButton<'a>>) -> Self {
        Self {
            name,
            buttons,
            selected: None,
            size: ComponentSize::Md,
        }
    }

    pub fn selected(mut self, value: &'a str) -> Self {
        self.selected = Some(value);
        self
    }

    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }
}

impl<'a> Render for IconButtonGroup<'a> {
    fn render(&self) -> Markup {
        let size_class = format!("sh-toggle-group--{}", self.size.class_suffix());

        html! {
            div class={(format!("sh-toggle-group sh-toggle-group--icons {}", size_class))} role="group" {
                @for btn in &self.buttons {
                    @let input_id = format!("sh-toggle-{}-{}", self.name, btn.value);
                    @let is_selected = self.selected == Some(btn.value);
                    @let item_classes = {
                        let mut cls = vec!["sh-toggle-item"];
                        if is_selected { cls.push("sh-toggle-item--active"); }
                        if btn.disabled { cls.push("sh-toggle-item--disabled"); }
                        cls.join(" ")
                    };

                    label class=(item_classes) {
                        input
                            class="sh-toggle-item__input"
                            type="radio"
                            name=(self.name)
                            id=(input_id)
                            value=(btn.value)
                            checked?[is_selected]
                            disabled?[btn.disabled];

                        span class="sh-toggle-item__icon" {
                            (maud::PreEscaped(btn.icon))
                        }
                    }
                }
            }
        }
    }
}

pub fn toggle_group_css() -> String {
    r#"
.sh-toggle-group {
    display: inline-flex;
    border-radius: var(--sh-radius-md, 0.5rem);
    overflow: hidden;
}

.sh-toggle-item {
    position: relative;
    display: inline-flex;
}

.sh-toggle-item__input {
    position: absolute;
    opacity: 0;
    width: 0;
    height: 0;
}

.sh-toggle-item__content {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--sh-text, #1f2937);
    background: var(--sh-surface, #fff);
    border: 1px solid var(--sh-border, #e5e7eb);
    cursor: pointer;
    transition: all 0.15s ease;
    margin: -1px 0 0 -1px;
}

.sh-toggle-item:first-child .sh-toggle-item__content {
    border-top-left-radius: var(--sh-radius-md, 0.5rem);
    border-bottom-left-radius: var(--sh-radius-md, 0.5rem);
}

.sh-toggle-item:last-child .sh-toggle-item__content {
    border-top-right-radius: var(--sh-radius-md, 0.5rem);
    border-bottom-right-radius: var(--sh-radius-md, 0.5rem);
}

.sh-toggle-item__input:checked + .sh-toggle-item__content {
    background: var(--sh-accent, #3b82f6);
    border-color: var(--sh-accent, #3b82f6);
    color: #fff;
    z-index: 1;
}

.sh-toggle-item__input:focus + .sh-toggle-item__content {
    z-index: 2;
    box-shadow: 0 0 0 2px color-mix(in srgb, var(--sh-accent, #3b82f6) 30%, transparent);
}

.sh-toggle-item__input:hover:not(:disabled) + .sh-toggle-item__content {
    background: var(--sh-surface-hover, #f3f4f6);
}

.sh-toggle-item__input:checked:hover + .sh-toggle-item__content {
    background: var(--sh-accent-dark, #2563eb);
}

.sh-toggle-item--disabled .sh-toggle-item__content {
    opacity: 0.5;
    cursor: not-allowed;
    pointer-events: none;
}

.sh-toggle-item__icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 1rem;
    height: 1rem;
}

/* Variant: Outline */
.sh-toggle-group--outline .sh-toggle-item__content {
    background: transparent;
}

.sh-toggle-group--outline .sh-toggle-item__input:checked + .sh-toggle-item__content {
    background: transparent;
    color: var(--sh-accent, #3b82f6);
    border-color: var(--sh-accent, #3b82f6);
}

/* Variant: Filled */
.sh-toggle-group--filled .sh-toggle-item__content {
    background: var(--sh-surface-2, #f3f4f6);
    border: none;
}

.sh-toggle-group--filled .sh-toggle-item:first-child .sh-toggle-item__content {
    border-radius: 0;
}

.sh-toggle-group--filled .sh-toggle-item:last-child .sh-toggle-item__content {
    border-radius: 0;
}

.sh-toggle-group--filled {
    background: var(--sh-surface-2, #f3f4f6);
    border-radius: var(--sh-radius-md, 0.5rem);
    padding: 0.25rem;
}

.sh-toggle-group--filled .sh-toggle-item__input:checked + .sh-toggle-item__content {
    background: var(--sh-surface, #fff);
    color: var(--sh-text, #1f2937);
    border-radius: var(--sh-radius-sm, 0.25rem);
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

/* Variant: Pills */
.sh-toggle-group--pills {
    border-radius: 9999px;
    padding: 0.25rem;
    background: var(--sh-surface-2, #f3f4f6);
}

.sh-toggle-group--pills .sh-toggle-item__content {
    border: none;
    background: transparent;
    border-radius: 9999px;
}

.sh-toggle-group--pills .sh-toggle-item:first-child .sh-toggle-item__content,
.sh-toggle-group--pills .sh-toggle-item:last-child .sh-toggle-item__content {
    border-radius: 9999px;
}

.sh-toggle-group--pills .sh-toggle-item__input:checked + .sh-toggle-item__content {
    background: var(--sh-accent, #3b82f6);
    color: #fff;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

/* Size variants */
.sh-toggle-group--sm .sh-toggle-item__content {
    padding: 0.375rem 0.75rem;
    font-size: 0.8125rem;
}

.sh-toggle-group--lg .sh-toggle-item__content {
    padding: 0.625rem 1.25rem;
    font-size: 1rem;
}

/* Icon-only variant */
.sh-toggle-group--icons .sh-toggle-item__content {
    padding: 0.5rem;
}

.sh-toggle-group--icons.sh-toggle-group--sm .sh-toggle-item__content {
    padding: 0.375rem;
}

.sh-toggle-group--icons.sh-toggle-group--lg .sh-toggle-item__content {
    padding: 0.75rem;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_group_creation() {
        let items = vec![
            ToggleItem {
                value: "list",
                label: "List",
                icon: None,
                disabled: false,
            },
            ToggleItem {
                value: "grid",
                label: "Grid",
                icon: None,
                disabled: false,
            },
        ];

        let group = ToggleGroup::new("view", items)
            .selected(vec!["list"])
            .variant(ToggleVariant::Pills);

        assert_eq!(group.name, "view");
        assert_eq!(group.selected, vec!["list"]);
    }

    #[test]
    fn test_icon_button_group() {
        let buttons = vec![IconButton {
            value: "bold",
            icon: "<b>B</b>",
            label: "Bold",
            disabled: false,
        }];

        let group = IconButtonGroup::new("format", buttons).selected("bold");

        assert_eq!(group.selected, Some("bold"));
    }
}
