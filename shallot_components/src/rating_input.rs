//! Rating Input Component - Star/number rating selector
//! CSS-only using radio buttons styled as stars

use maud::{html, Markup, Render};

/// Rating size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RatingSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Rating input component
#[derive(Debug, Clone)]
pub struct RatingInput<'a> {
    pub name: &'a str,
    pub max: u8,
    pub value: u8,
    pub size: RatingSize,
    pub disabled: bool,
    pub readonly: bool,
    pub label: Option<&'a str>,
}

impl<'a> RatingInput<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            max: 5,
            value: 0,
            size: RatingSize::default(),
            disabled: false,
            readonly: false,
            label: None,
        }
    }

    pub fn max(mut self, max: u8) -> Self {
        self.max = max.max(1);
        self
    }

    pub fn value(mut self, value: u8) -> Self {
        self.value = value.min(self.max);
        self
    }

    pub fn size(mut self, size: RatingSize) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-rating"];

        match self.size {
            RatingSize::Sm => classes.push("sh-rating--sm"),
            RatingSize::Md => classes.push("sh-rating--md"),
            RatingSize::Lg => classes.push("sh-rating--lg"),
        }

        if self.disabled {
            classes.push("sh-rating--disabled");
        }

        if self.readonly {
            classes.push("sh-rating--readonly");
        }

        classes.join(" ")
    }
}

impl<'a> Render for RatingInput<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();

        html! {
            div class=(classes) role="radiogroup" {
                @if let Some(label) = self.label {
                    span class="sh-rating__label" {
                        (label)
                    }
                }

                div class="sh-rating__stars" {
                    @for i in (1..=self.max).rev() {
                        label class="sh-rating__star" {
                            input
                                type="radio"
                                name=(self.name)
                                value=(i)
                                checked[i == self.value]
                                disabled[self.disabled || self.readonly]
                                class="sh-rating__input";
                            span class="sh-rating__icon" {
                                "★"
                            }
                        }
                    }
                }

                span class="sh-rating__value" {
                    (self.value) " / " (self.max)
                }
            }
        }
    }
}

pub fn rating_input_css() -> String {
    r#"
.sh-rating {
    display: inline-flex;
    align-items: center;
    gap: var(--sh-spacing-sm, 0.5rem);
}

.sh-rating__label {
    font-size: var(--sh-font-size-sm, 0.875rem);
    font-weight: var(--sh-font-weight-medium, 500);
    color: var(--sh-color-text, #1a1a1a);
}

.sh-rating__stars {
    display: inline-flex;
    flex-direction: row-reverse;
    gap: 0.125rem;
}

.sh-rating__star {
    cursor: pointer;
    display: flex;
}

.sh-rating__input {
    position: absolute;
    opacity: 0;
    width: 0;
    height: 0;
}

.sh-rating__icon {
    color: var(--sh-color-border, #e5e5e5);
    transition: color 0.15s ease;
}

.sh-rating--md .sh-rating__icon {
    font-size: 1.25rem;
}

.sh-rating--sm .sh-rating__icon {
    font-size: 1rem;
}

.sh-rating--lg .sh-rating__icon {
    font-size: 1.5rem;
}

.sh-rating__input:checked ~ .sh-rating__icon,
.sh-rating__input:checked ~ .sh-rating__star .sh-rating__icon {
    color: var(--sh-color-warning, #f59e0b);
}

.sh-rating__star:hover .sh-rating__icon,
.sh-rating__star:hover ~ .sh-rating__star .sh-rating__icon {
    color: var(--sh-color-warning, #f59e0b);
}

.sh-rating--disabled .sh-rating__star,
.sh-rating--readonly .sh-rating__star {
    cursor: default;
    pointer-events: none;
}

.sh-rating--disabled {
    opacity: 0.5;
}

.sh-rating__value {
    font-size: var(--sh-font-size-sm, 0.875rem);
    color: var(--sh-color-text-muted, #666);
    min-width: 3rem;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rating_creation() {
        let rating = RatingInput::new("rating").max(10).value(7);

        assert_eq!(rating.max, 10);
        assert_eq!(rating.value, 7);
    }

    #[test]
    fn test_rating_clamping() {
        let rating = RatingInput::new("test").max(5).value(10);

        assert_eq!(rating.value, 5);
    }

    #[test]
    fn test_rating_max_clamping() {
        let rating = RatingInput::new("test").max(0);
        assert_eq!(rating.max, 1);
    }

    #[test]
    fn test_rating_render() {
        let rating = RatingInput::new("rating").label("Rate this").value(3);

        let html = rating.render().into_string();
        assert!(html.contains("sh-rating"));
        assert!(html.contains("Rate this"));
    }

    #[test]
    fn test_rating_readonly() {
        let rating = RatingInput::new("test").readonly(true);
        let html = rating.render().into_string();

        assert!(html.contains("sh-rating--readonly"));
    }

    #[test]
    fn test_css_generation() {
        let css = rating_input_css();
        assert!(css.contains(".sh-rating"));
    }
}
