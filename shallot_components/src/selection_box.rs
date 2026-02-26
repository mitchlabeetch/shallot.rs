//! Selection Box Component - Toggleable selection card
//! CSS-only styling with checkbox/radio integration

use maud::{html, Markup, Render};

/// Selection box variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectionBoxVariant {
    #[default]
    Default,
    Card,
    Compact,
}

/// Selection box component
#[derive(Debug, Clone)]
pub struct SelectionBox<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub label: &'a str,
    pub value: &'a str,
    pub description: Option<&'a str>,
    pub variant: SelectionBoxVariant,
    pub selected: bool,
    pub disabled: bool,
    pub multiple: bool,
}

impl<'a> SelectionBox<'a> {
    /// Create a new selection box
    pub fn new(id: &'a str, name: &'a str, label: &'a str) -> Self {
        Self {
            id,
            name,
            label,
            value: id,
            description: None,
            variant: SelectionBoxVariant::default(),
            selected: false,
            disabled: false,
            multiple: false,
        }
    }

    /// Set the value
    pub fn value(mut self, value: &'a str) -> Self {
        self.value = value;
        self
    }

    /// Set the description
    pub fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    /// Set the variant
    pub fn variant(mut self, variant: SelectionBoxVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the selected state
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Set the disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Allow multiple selections (checkbox mode)
    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-selection-box".to_string()];

        let variant_class = match self.variant {
            SelectionBoxVariant::Default => "sh-selection-box--default",
            SelectionBoxVariant::Card => "sh-selection-box--card",
            SelectionBoxVariant::Compact => "sh-selection-box--compact",
        };
        classes.push(variant_class.to_string());

        if self.selected {
            classes.push("sh-selection-box--selected".to_string());
        }

        if self.disabled {
            classes.push("sh-selection-box--disabled".to_string());
        }

        classes.join(" ")
    }
}

impl<'a> Render for SelectionBox<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let input_type = if self.multiple { "checkbox" } else { "radio" };

        html! {
            label class=(classes) for=(self.id) {
                input
                    type=(input_type)
                    id=(self.id)
                    name=(self.name)
                    value=(self.value)
                    class="sh-selection-box__input"
                    checked?=(self.selected)
                    disabled?=(self.disabled)
                    aria-checked=(if self.selected { "true" } else { "false" });
                span class="sh-selection-box__indicator" aria-hidden="true" {
                    @if self.multiple {
                        svg
                            class="sh-selection-box__check"
                            xmlns="http://www.w3.org/2000/svg"
                            width="16"
                            height="16"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="3"
                            stroke-linecap="round"
                            stroke-linejoin="round" {
                            polyline points="20 6 9 17 4 12" {}
                        }
                    }
                }
                div class="sh-selection-box__content" {
                    span class="sh-selection-box__label" {
                        (self.label)
                    }
                    @if let Some(desc) = self.description {
                        span class="sh-selection-box__description" {
                            (desc)
                        }
                    }
                }
            }
        }
    }
}

/// Generate CSS for selection box component
pub fn selection_box_css() -> String {
    r#"
.sh-selection-box {
    display: flex;
    align-items: flex-start;
    gap: var(--sh-spacing-3, 0.75rem);
    padding: var(--sh-spacing-3, 0.75rem);
    border: 2px solid var(--sh-color-border, #e5e7eb);
    border-radius: var(--sh-radius-md, 0.375rem);
    background-color: var(--sh-color-background, #ffffff);
    cursor: pointer;
    transition: all 0.15s ease;
}

.sh-selection-box:hover:not(.sh-selection-box--disabled) {
    border-color: var(--sh-color-primary, #3b82f6);
}

.sh-selection-box--card {
    padding: var(--sh-spacing-4, 1rem);
    border-radius: var(--sh-radius-lg, 0.5rem);
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.sh-selection-box--compact {
    padding: var(--sh-spacing-2, 0.5rem);
    gap: var(--sh-spacing-2, 0.5rem);
}

.sh-selection-box--selected {
    border-color: var(--sh-color-primary, #3b82f6);
    background-color: var(--sh-color-primary-muted, #eff6ff);
}

.sh-selection-box--disabled {
    opacity: 0.5;
    cursor: not-allowed;
    background-color: var(--sh-color-muted, #f3f4f6);
}

.sh-selection-box__input {
    position: absolute;
    opacity: 0;
    width: 0;
    height: 0;
}

.sh-selection-box__indicator {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 20px;
    height: 20px;
    border: 2px solid var(--sh-color-border, #e5e7eb);
    border-radius: 50%;
    flex-shrink: 0;
    margin-top: 2px;
    transition: all 0.15s ease;
}

.sh-selection-box__input:checked + .sh-selection-box__indicator {
    border-color: var(--sh-color-primary, #3b82f6);
    background-color: var(--sh-color-primary, #3b82f6);
}

.sh-selection-box__check {
    width: 12px;
    height: 12px;
    color: var(--sh-color-primary-foreground, #ffffff);
    opacity: 0;
    transform: scale(0.5);
    transition: all 0.15s ease;
}

.sh-selection-box__input:checked + .sh-selection-box__indicator .sh-selection-box__check {
    opacity: 1;
    transform: scale(1);
}

/* Checkbox style for multiple selections */
.sh-selection-box:has(.sh-selection-box__input[type="checkbox"]) .sh-selection-box__indicator {
    border-radius: var(--sh-radius-sm, 0.25rem);
}

.sh-selection-box:focus-within {
    outline: 2px solid var(--sh-color-primary, #3b82f6);
    outline-offset: 2px;
}

.sh-selection-box__content {
    display: flex;
    flex-direction: column;
    gap: var(--sh-spacing-1, 0.25rem);
    flex: 1;
    min-width: 0;
}

.sh-selection-box__label {
    font-size: var(--sh-font-size-sm, 0.875rem);
    font-weight: var(--sh-font-weight-medium, 500);
    color: var(--sh-color-foreground, #1f2937);
}

.sh-selection-box--compact .sh-selection-box__label {
    font-size: var(--sh-font-size-xs, 0.75rem);
}

.sh-selection-box__description {
    font-size: var(--sh-font-size-xs, 0.75rem);
    color: var(--sh-color-muted-foreground, #6b7280);
    line-height: 1.4;
}

.sh-selection-box--compact .sh-selection-box__description {
    display: none;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_box_creation() {
        let box_ = SelectionBox::new("opt1", "options", "Option 1");

        assert_eq!(box_.id, "opt1");
        assert_eq!(box_.name, "options");
        assert_eq!(box_.label, "Option 1");
        assert!(!box_.multiple);
    }

    #[test]
    fn test_selection_box_render() {
        let box_ = SelectionBox::new("opt1", "options", "Option 1").description("This is option 1");

        let html = box_.render().into_string();

        assert!(html.contains("sh-selection-box"));
        assert!(html.contains("Option 1"));
        assert!(html.contains("This is option 1"));
        assert!(html.contains("type=\"radio\""));
    }

    #[test]
    fn test_selection_box_variants() {
        let default = SelectionBox::new("1", "opts", "Opt").variant(SelectionBoxVariant::Default);
        let card = SelectionBox::new("1", "opts", "Opt").variant(SelectionBoxVariant::Card);
        let compact = SelectionBox::new("1", "opts", "Opt").variant(SelectionBoxVariant::Compact);

        assert!(default
            .render()
            .into_string()
            .contains("sh-selection-box--default"));
        assert!(card
            .render()
            .into_string()
            .contains("sh-selection-box--card"));
        assert!(compact
            .render()
            .into_string()
            .contains("sh-selection-box--compact"));
    }

    #[test]
    fn test_selection_box_selected() {
        let box_ = SelectionBox::new("1", "opts", "Opt").selected(true);
        let html = box_.render().into_string();

        assert!(html.contains("sh-selection-box--selected"));
        assert!(html.contains("checked"));
    }

    #[test]
    fn test_selection_box_disabled() {
        let box_ = SelectionBox::new("1", "opts", "Opt").disabled(true);
        let html = box_.render().into_string();

        assert!(html.contains("sh-selection-box--disabled"));
        assert!(html.contains("disabled"));
    }

    #[test]
    fn test_selection_box_multiple() {
        let box_ = SelectionBox::new("1", "opts", "Opt").multiple(true);
        let html = box_.render().into_string();

        assert!(html.contains("type=\"checkbox\""));
    }

    #[test]
    fn test_selection_box_css() {
        let css = selection_box_css();
        assert!(css.contains(".sh-selection-box"));
        assert!(css.contains(".sh-selection-box__indicator"));
    }
}
