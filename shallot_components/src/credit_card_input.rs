//! CreditCardInput Component - Formatted Credit Card Input
//!
//! A credit card input with visual card preview and formatted input.
//! Server-side validation handles actual card verification.

use maud::{html, Markup, Render};

/// Credit card type detection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CardType {
    #[default]
    Unknown,
    Visa,
    Mastercard,
    Amex,
    Discover,
}

impl CardType {
    fn css_class(&self) -> &'static str {
        match self {
            CardType::Visa => "sh-ccinput--visa",
            CardType::Mastercard => "sh-ccinput--mastercard",
            CardType::Amex => "sh-ccinput--amex",
            CardType::Discover => "sh-ccinput--discover",
            _ => "",
        }
    }
}

/// CreditCardInput component
pub struct CreditCardInput<'a> {
    name: &'a str,
    value: Option<&'a str>,
    placeholder: Option<&'a str>,
    card_type: CardType,
    disabled: bool,
    required: bool,
    class: Option<&'a str>,
}

impl<'a> CreditCardInput<'a> {
    /// Create a new CreditCardInput
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            value: None,
            placeholder: Some("1234 5678 9012 3456"),
            card_type: CardType::default(),
            disabled: false,
            required: false,
            class: None,
        }
    }

    /// Set the value
    pub fn value(mut self, value: &'a str) -> Self {
        self.value = Some(value);
        self
    }

    /// Set placeholder
    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    /// Set card type (detected server-side)
    pub fn card_type(mut self, card_type: CardType) -> Self {
        self.card_type = card_type;
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

    /// Add custom class
    pub fn class(mut self, class: &'a str) -> Self {
        self.class = Some(class);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-ccinput".to_string()];
        if self.card_type != CardType::Unknown {
            classes.push(self.card_type.css_class().to_string());
        }
        if self.disabled {
            classes.push("sh-ccinput--disabled".to_string());
        }
        if let Some(custom) = self.class {
            classes.push(custom.to_string());
        }
        classes.join(" ")
    }
}

impl<'a> Render for CreditCardInput<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let input_id = format!("sh-ccinput-{}", self.name);

        html! {
            div class=(classes) {
                div class="sh-ccinput__card-preview" aria-hidden="true" {
                    div class="sh-ccinput__chip" {}
                    div class="sh-ccinput__logo" {
                        @match self.card_type {
                            CardType::Visa => "VISA",
                            CardType::Mastercard => "Mastercard",
                            CardType::Amex => "AMEX",
                            CardType::Discover => "Discover",
                            _ => "CARD",
                        }
                    }
                    div class="sh-ccinput__number-display" {
                        @if let Some(value) = self.value {
                            (value)
                        } @else {
                            "•••• •••• •••• ••••"
                        }
                    }
                }
                div class="sh-ccinput__field" {
                    label class="sh-ccinput__label" for=(input_id) {
                        "Card Number"
                    }
                    input
                        type="text"
                        id=(input_id)
                        name=(self.name)
                        value=[self.value]
                        placeholder=[self.placeholder]
                        inputmode="numeric"
                        pattern="[0-9]*"
                        autocomplete="cc-number"
                        disabled?[self.disabled]
                        required?[self.required]
                        class="sh-ccinput__input"
                        aria-label="Credit card number"
                    ;
                    span class="sh-ccinput__icon" aria-hidden="true" {
                        svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" {
                            rect x="1" y="4" width="22" height="16" rx="2" ry="2";
                            line x1="1" y1="10" x2="23" y2="10";
                        }
                    }
                }
            }
        }
    }
}

/// Generate CSS for CreditCardInput component
pub fn credit_card_input_css() -> String {
    r#"
.sh-ccinput {
    width: 100%;
    max-width: 24rem;
}

.sh-ccinput__card-preview {
    position: relative;
    width: 100%;
    height: 10rem;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    border-radius: var(--sh-radius-xl, 0.75rem);
    padding: 1.25rem;
    margin-bottom: 1rem;
    color: white;
    box-shadow: 0 10px 40px rgba(102, 126, 234, 0.4);
}

.sh-ccinput__chip {
    width: 3rem;
    height: 2.25rem;
    background: linear-gradient(135deg, #fbbf24 0%, #d97706 100%);
    border-radius: var(--sh-radius-md, 0.375rem);
    margin-bottom: 1rem;
}

.sh-ccinput__logo {
    position: absolute;
    top: 1.25rem;
    right: 1.25rem;
    font-weight: 700;
    font-size: 0.875rem;
    opacity: 0.9;
}

.sh-ccinput__number-display {
    font-size: 1.25rem;
    font-family: monospace;
    letter-spacing: 0.125em;
    margin-top: 2rem;
}

.sh-ccinput__field {
    position: relative;
}

.sh-ccinput__label {
    display: block;
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--sh-text, #1f2937);
    margin-bottom: 0.5rem;
}

.sh-ccinput__input {
    width: 100%;
    padding: 0.875rem 3rem 0.875rem 1rem;
    font-size: 1rem;
    font-family: monospace;
    letter-spacing: 0.05em;
    border: 1px solid var(--sh-border, #e5e7eb);
    border-radius: var(--sh-radius-md, 0.375rem);
    background: var(--sh-surface, #fff);
    transition: all 0.2s ease;
}

.sh-ccinput__input:focus {
    outline: none;
    border-color: var(--sh-primary, #3b82f6);
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.sh-ccinput__icon {
    position: absolute;
    right: 1rem;
    top: 2.5rem;
    color: var(--sh-text-muted, #6b7280);
    pointer-events: none;
}

/* Card type colors */
.sh-ccinput--visa .sh-ccinput__card-preview {
    background: linear-gradient(135deg, #1a1f71 0%, #3d5afe 100%);
}

.sh-ccinput--mastercard .sh-ccinput__card-preview {
    background: linear-gradient(135deg, #eb001b 0%, #f79e1b 100%);
}

.sh-ccinput--amex .sh-ccinput__card-preview {
    background: linear-gradient(135deg, #006fcf 0%, #00c6f7 100%);
}

.sh-ccinput--discover .sh-ccinput__card-preview {
    background: linear-gradient(135deg, #f60 0%, #ff5f00 100%);
}

/* Disabled state */
.sh-ccinput--disabled {
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
    fn test_ccinput_creation() {
        let input = CreditCardInput::new("card");
        assert_eq!(input.name, "card");
        assert_eq!(input.card_type, CardType::Unknown);
    }

    #[test]
    fn test_ccinput_value() {
        let input = CreditCardInput::new("card").value("4111111111111111");
        assert_eq!(input.value, Some("4111111111111111"));
    }

    #[test]
    fn test_ccinput_card_type() {
        let input = CreditCardInput::new("card").card_type(CardType::Visa);
        assert_eq!(input.card_type, CardType::Visa);
    }

    #[test]
    fn test_ccinput_disabled() {
        let input = CreditCardInput::new("card").disabled(true);
        assert!(input.disabled);
    }

    #[test]
    fn test_ccinput_css() {
        let css = credit_card_input_css();
        assert!(css.contains(".sh-ccinput"));
        assert!(css.contains(".sh-ccinput__card-preview"));
    }
}
