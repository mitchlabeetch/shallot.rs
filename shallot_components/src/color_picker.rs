//! Color Picker Component
//!
//! Color selection input with swatches and custom color input.

use crate::component::ComponentSize;
use maud::{html, Markup, Render};

pub struct ColorPicker<'a> {
    name: &'a str,
    value: &'a str,
    label: Option<&'a str>,
    swatches: Vec<&'a str>,
    size: ComponentSize,
    show_input: bool,
    disabled: bool,
}

impl<'a> ColorPicker<'a> {
    pub fn new(name: &'a str, value: &'a str) -> Self {
        Self {
            name,
            value,
            label: None,
            swatches: Vec::new(),
            size: ComponentSize::Md,
            show_input: false,
            disabled: false,
        }
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn swatches(mut self, swatches: Vec<&'a str>) -> Self {
        self.swatches = swatches;
        self
    }

    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }

    pub fn show_input(mut self, show: bool) -> Self {
        self.show_input = show;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl<'a> Render for ColorPicker<'a> {
    fn render(&self) -> Markup {
        let input_id = format!("sh-color-{}", self.name);
        let size_class = format!("sh-color-picker--{}", self.size.class_suffix());

        html! {
            div
                class={(format!("sh-color-picker {}", size_class))}
                role="group"
                aria-label=(self.label.unwrap_or("Color picker"))
            {
                @if let Some(label) = self.label {
                    label class="sh-color-picker__label" for=(input_id) {
                        (label)
                    }
                }

                div class="sh-color-picker__preview" {
                    input
                        type="color"
                        class="sh-color-picker__native"
                        id=(input_id)
                        name=(self.name)
                        value=(self.value)
                        disabled?[self.disabled]
                        aria-label="Color value";

                    div class="sh-color-picker__swatch" style=(format!("background-color: {}", self.value)) aria-hidden="true" {}
                }

                @if !self.swatches.is_empty() {
                    div class="sh-color-picker__swatches" role="listbox" aria-label="Color swatches" {
                        @for color in &self.swatches {
                            @let is_selected = *color == self.value;
                            button
                                type="button"
                                class={(format!("sh-color-swatch {}", if is_selected { "sh-color-swatch--selected" } else { "" }))}
                                style=(format!("background-color: {}", color))
                                data-color=(color)
                                disabled?[self.disabled]
                                aria-pressed=(if is_selected { "true" } else { "false" })
                                aria-label=(format!("Select color {}", color))
                            {}
                        }
                    }
                }

                @if self.show_input {
                    input
                        type="text"
                        class="sh-color-picker__input"
                        value=(self.value)
                        placeholder="#000000"
                        disabled?[self.disabled]
                        aria-label="Color hex value";
                }
            }
        }
    }
}

pub struct ColorSwatches<'a> {
    name: &'a str,
    colors: Vec<ColorSwatch<'a>>,
    columns: u8,
    size: ComponentSize,
}

pub struct ColorSwatch<'a> {
    pub value: &'a str,
    pub label: Option<&'a str>,
    pub selected: bool,
}

impl<'a> ColorSwatches<'a> {
    pub fn new(name: &'a str, colors: Vec<ColorSwatch<'a>>) -> Self {
        Self {
            name,
            colors,
            columns: 6,
            size: ComponentSize::Md,
        }
    }

    pub fn columns(mut self, columns: u8) -> Self {
        self.columns = columns;
        self
    }

    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }
}

impl<'a> Render for ColorSwatches<'a> {
    fn render(&self) -> Markup {
        let size_class = format!("sh-color-swatches--{}", self.size.class_suffix());

        html! {
            div class={(format!("sh-color-swatches {}", size_class))} style=(format!("--swatch-columns: {}", self.columns)) {
                @for color in &self.colors {
                    label class={(format!("sh-swatch {}", if color.selected { "sh-swatch--selected" } else { "" }))} {
                        input
                            type="radio"
                            class="sh-swatch__input"
                            name=(self.name)
                            value=(color.value)
                            checked?[color.selected];
                        span
                            class="sh-swatch__color"
                            style=(format!("background-color: {}", color.value))
                            title=[color.label]
                        {}
                        @if let Some(label) = color.label {
                            span class="sh-swatch__label" { (label) }
                        }
                    }
                }
            }
        }
    }
}

pub fn color_picker_css() -> String {
    r#"
.sh-color-picker {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
}

.sh-color-picker__label {
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--sh-text, #1f2937);
}

.sh-color-picker__preview {
    position: relative;
    width: 3rem;
    height: 3rem;
    cursor: pointer;
}

.sh-color-picker__native {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    opacity: 0;
    cursor: pointer;
}

.sh-color-picker__swatch {
    width: 100%;
    height: 100%;
    border-radius: var(--sh-radius-md, 0.5rem);
    border: 2px solid var(--sh-border, #e5e7eb);
    pointer-events: none;
    transition: all 0.15s ease;
}

.sh-color-picker__preview:hover .sh-color-picker__swatch {
    border-color: var(--sh-accent, #3b82f6);
}

.sh-color-picker__swatches {
    display: flex;
    flex-wrap: wrap;
    gap: 0.375rem;
}

.sh-color-swatch {
    width: 1.5rem;
    height: 1.5rem;
    padding: 0;
    border: 2px solid transparent;
    border-radius: var(--sh-radius-sm, 0.25rem);
    cursor: pointer;
    transition: all 0.15s ease;
}

.sh-color-swatch:hover {
    transform: scale(1.1);
}

.sh-color-swatch--selected {
    border-color: var(--sh-text, #1f2937);
    box-shadow: 0 0 0 2px var(--sh-surface, #fff);
}

.sh-color-picker__input {
    width: 100%;
    padding: 0.5rem 0.75rem;
    font-size: 0.875rem;
    font-family: monospace;
    color: var(--sh-text, #1f2937);
    background: var(--sh-surface, #fff);
    border: 1px solid var(--sh-border, #e5e7eb);
    border-radius: var(--sh-radius-md, 0.5rem);
}

.sh-color-picker__input:focus {
    outline: none;
    border-color: var(--sh-accent, #3b82f6);
}

/* Size variants */
.sh-color-picker--sm .sh-color-picker__preview {
    width: 2.5rem;
    height: 2.5rem;
}

.sh-color-picker--lg .sh-color-picker__preview {
    width: 4rem;
    height: 4rem;
}

/* Color swatches grid */
.sh-color-swatches {
    display: grid;
    grid-template-columns: repeat(var(--swatch-columns, 6), 1fr);
    gap: 0.5rem;
}

.sh-swatch {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.25rem;
    cursor: pointer;
}

.sh-swatch__input {
    position: absolute;
    opacity: 0;
    width: 0;
    height: 0;
}

.sh-swatch__color {
    width: 2rem;
    height: 2rem;
    border-radius: var(--sh-radius-md, 0.5rem);
    border: 2px solid var(--sh-border, #e5e7eb);
    transition: all 0.15s ease;
}

.sh-swatch__input:checked + .sh-swatch__color {
    border-color: var(--sh-accent, #3b82f6);
    box-shadow: 0 0 0 2px color-mix(in srgb, var(--sh-accent, #3b82f6) 30%, transparent);
}

.sh-swatch__input:hover + .sh-swatch__color {
    border-color: var(--sh-text-muted, #6b7280);
}

.sh-swatch__label {
    font-size: 0.6875rem;
    color: var(--sh-text-muted, #6b7280);
}

/* Size variants for swatches */
.sh-color-swatches--sm .sh-swatch__color {
    width: 1.5rem;
    height: 1.5rem;
}

.sh-color-swatches--lg .sh-swatch__color {
    width: 3rem;
    height: 3rem;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_picker_creation() {
        let picker = ColorPicker::new("color", "#3b82f6")
            .label("Background Color")
            .swatches(vec!["#3b82f6", "#10b981", "#ef4444"]);

        assert_eq!(picker.name, "color");
        assert_eq!(picker.value, "#3b82f6");
        assert_eq!(picker.swatches.len(), 3);
    }

    #[test]
    fn test_color_swatches() {
        let colors = vec![
            ColorSwatch {
                value: "#000",
                label: Some("Black"),
                selected: true,
            },
            ColorSwatch {
                value: "#fff",
                label: Some("White"),
                selected: false,
            },
        ];

        let swatches = ColorSwatches::new("theme", colors).columns(4);

        assert_eq!(swatches.columns, 4);
        assert_eq!(swatches.colors.len(), 2);
    }
}
