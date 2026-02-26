//! MultiSelect Component - CSS-only Tag Selection Interface
//!
//! A multi-select interface using checkbox hacks and CSS peer selectors.
//! Selected items appear as removable tags.

use maud::{html, Markup, Render};

/// Option for MultiSelect
pub struct MultiSelectOption<'a> {
    pub value: &'a str,
    pub label: &'a str,
}

/// MultiSelect component
pub struct MultiSelect<'a> {
    name: &'a str,
    options: Vec<MultiSelectOption<'a>>,
    selected: Vec<&'a str>,
    placeholder: Option<&'a str>,
    disabled: bool,
    class: Option<&'a str>,
}

impl<'a> MultiSelect<'a> {
    /// Create a new MultiSelect
    pub fn new(name: &'a str, options: Vec<MultiSelectOption<'a>>) -> Self {
        Self {
            name,
            options,
            selected: Vec::new(),
            placeholder: None,
            disabled: false,
            class: None,
        }
    }

    /// Set pre-selected values
    pub fn selected(mut self, values: Vec<&'a str>) -> Self {
        self.selected = values;
        self
    }

    /// Set placeholder
    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Add custom class
    pub fn class(mut self, class: &'a str) -> Self {
        self.class = Some(class);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-multiselect".to_string()];
        if self.disabled {
            classes.push("sh-multiselect--disabled".to_string());
        }
        if let Some(custom) = self.class {
            classes.push(custom.to_string());
        }
        classes.join(" ")
    }

    fn is_selected(&self, value: &str) -> bool {
        self.selected.contains(&value)
    }
}

impl<'a> Render for MultiSelect<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();

        html! {
            div
                class=(classes)
                role="group"
                aria-label="Multi-select options"
            {
                @if let Some(placeholder) = self.placeholder {
                    span class="sh-multiselect__placeholder" { (placeholder) }
                }

                div class="sh-multiselect__options" {
                    @for option in &self.options {
                        label class="sh-multiselect__option" {
                            input
                                type="checkbox"
                                name=(self.name)
                                value=(option.value)
                                checked=[if self.is_selected(option.value) { Some("") } else { None }]
                                disabled?[self.disabled]
                                class="sh-multiselect__input"
                            ;
                            span class="sh-multiselect__label" { (option.label) }
                            span class="sh-multiselect__check" aria-hidden="true" {
                                svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" {
                                    polyline points="20 6 9 17 4 12";
                                }
                            }
                        }
                    }
                }

                @if !self.selected.is_empty() {
                    div class="sh-multiselect__tags" aria-label="Selected items" {
                        @for value in &self.selected {
                            @if let Some(option) = self.options.iter().find(|o| o.value == *value) {
                                span class="sh-multiselect__tag" {
                                    (option.label)
                                    button
                                        type="button"
                                        class="sh-multiselect__tag-remove"
                                        aria-label=(format!("Remove {}", option.label))
                                        tabindex="0"
                                    {
                                        svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" {
                                            line x1="18" y1="6" x2="6" y2="18";
                                            line x1="6" y1="6" x2="18" y2="18";
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Generate CSS for MultiSelect component
pub fn multi_select_css() -> String {
    r#"
.sh-multiselect {
    position: relative;
    border: 1px solid var(--sh-border, #e5e7eb);
    border-radius: var(--sh-radius-lg, 0.5rem);
    padding: 0.75rem;
    background: var(--sh-surface, #fff);
    min-height: 3rem;
}

.sh-multiselect__placeholder {
    color: var(--sh-text-muted, #9ca3af);
    font-size: 0.875rem;
    pointer-events: none;
}

.sh-multiselect__options {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    margin-top: 0.5rem;
}

.sh-multiselect__option {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0.75rem;
    background: var(--sh-surface-2, #f3f4f6);
    border-radius: var(--sh-radius-md, 0.375rem);
    cursor: pointer;
    transition: all 0.2s ease;
    font-size: 0.875rem;
}

.sh-multiselect__option:hover {
    background: var(--sh-surface-hover, #e5e7eb);
}

.sh-multiselect__input {
    width: 1rem;
    height: 1rem;
    accent-color: var(--sh-primary, #3b82f6);
    cursor: pointer;
}

.sh-multiselect__check {
    display: none;
    color: var(--sh-primary, #3b82f6);
}

.sh-multiselect__input:checked + .sh-multiselect__label {
    font-weight: 600;
    color: var(--sh-primary, #3b82f6);
}

.sh-multiselect__input:checked ~ .sh-multiselect__check {
    display: inline-block;
}

.sh-multiselect__tags {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    margin-top: 0.75rem;
    padding-top: 0.75rem;
    border-top: 1px solid var(--sh-border, #e5e7eb);
}

.sh-multiselect__tag {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.25rem 0.5rem;
    background: var(--sh-primary, #3b82f6);
    color: white;
    border-radius: var(--sh-radius-sm, 0.25rem);
    font-size: 0.75rem;
    font-weight: 500;
}

.sh-multiselect__tag-remove {
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    color: white;
    cursor: pointer;
    padding: 0;
    opacity: 0.8;
    transition: opacity 0.2s ease;
}

.sh-multiselect__tag-remove:hover {
    opacity: 1;
}

/* Disabled state */
.sh-multiselect--disabled {
    opacity: 0.5;
    pointer-events: none;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiselect_creation() {
        let options = vec![
            MultiSelectOption {
                value: "1",
                label: "One",
            },
            MultiSelectOption {
                value: "2",
                label: "Two",
            },
        ];
        let ms = MultiSelect::new("items", options);
        assert_eq!(ms.name, "items");
        assert!(ms.selected.is_empty());
    }

    #[test]
    fn test_multiselect_selected() {
        let options = vec![
            MultiSelectOption {
                value: "1",
                label: "One",
            },
            MultiSelectOption {
                value: "2",
                label: "Two",
            },
        ];
        let ms = MultiSelect::new("items", options).selected(vec!["1"]);
        assert_eq!(ms.selected.len(), 1);
    }

    #[test]
    fn test_multiselect_is_selected() {
        let options = vec![MultiSelectOption {
            value: "1",
            label: "One",
        }];
        let ms = MultiSelect::new("items", options).selected(vec!["1"]);
        assert!(ms.is_selected("1"));
        assert!(!ms.is_selected("2"));
    }

    #[test]
    fn test_multiselect_css() {
        let css = multi_select_css();
        assert!(css.contains(".sh-multiselect"));
        assert!(css.contains(".sh-multiselect__tag"));
    }
}
