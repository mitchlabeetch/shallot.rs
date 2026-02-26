use maud::{html, Markup};

pub struct Rating {
    pub value_0_5: u8,
}

impl Rating {
    pub fn new(value: u8) -> Self {
        Self {
            value_0_5: value.min(5),
        }
    }

    pub fn render(self) -> Markup {
        let v = self.value_0_5.min(5);
        html! {
            div class="sh-rating" role="img" aria-label={(format!("Rating: {} out of 5", v))} {
                @for i in 1..=5 {
                    span class={(if i <= v {"sh-star sh-star--on"} else {"sh-star"})} aria-hidden="true" { "★" }
                }
                span class="sh-rating__label" { (format!("{}", v)) }
            }
        }
    }
}

/// Generate CSS for rating component
pub fn rating_css() -> String {
    r#"
.sh-rating {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
}

.sh-star {
    font-size: 1.25rem;
    color: var(--sh-text-muted, #d1d5db);
}

.sh-star--on {
    color: var(--sh-warning, #f59e0b);
}

.sh-rating__label {
    font-size: 0.875rem;
    color: var(--sh-text, #1f2937);
    margin-left: 0.5rem;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rating_creation() {
        let rating = Rating::new(4);
        assert_eq!(rating.value_0_5, 4);
    }

    #[test]
    fn test_rating_max_value() {
        let rating = Rating::new(10);
        assert_eq!(rating.value_0_5, 5);
    }

    #[test]
    fn test_rating_css() {
        let css = rating_css();
        assert!(css.contains(".sh-rating"));
        assert!(css.contains(".sh-star"));
    }
}
