//! OTP Input Component - One-time password input
//!
//! A zero-JavaScript OTP input with multiple single-character fields.

use maud::{html, Markup, Render};

/// OTP input size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OtpSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// OTP input style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OtpVariant {
    #[default]
    Default,
    Filled,
    Underline,
}

/// OTP Input component
#[derive(Debug, Clone)]
pub struct OtpInput<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub length: usize,
    pub value: Option<&'a str>,
    pub size: OtpSize,
    pub variant: OtpVariant,
    pub disabled: bool,
    pub required: bool,
    pub label: Option<&'a str>,
    pub autocomplete: bool,
}

impl<'a> OtpInput<'a> {
    /// Create a new OTP input with default 6 digits
    pub fn new(id: &'a str, name: &'a str) -> Self {
        Self {
            id,
            name,
            length: 6,
            value: None,
            size: OtpSize::Md,
            variant: OtpVariant::Default,
            disabled: false,
            required: false,
            label: None,
            autocomplete: true,
        }
    }

    /// Set number of digits
    pub fn length(mut self, length: usize) -> Self {
        self.length = length.max(1).min(12);
        self
    }

    /// Set initial value
    pub fn value(mut self, value: &'a str) -> Self {
        self.value = Some(value);
        self
    }

    /// Set size variant
    pub fn size(mut self, size: OtpSize) -> Self {
        self.size = size;
        self
    }

    /// Set style variant
    pub fn variant(mut self, variant: OtpVariant) -> Self {
        self.variant = variant;
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

    /// Set label
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    /// Enable/disable autocomplete
    pub fn autocomplete(mut self, autocomplete: bool) -> Self {
        self.autocomplete = autocomplete;
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-otp-input".to_string()];

        classes.push(match self.size {
            OtpSize::Sm => "sh-otp-input--sm".to_string(),
            OtpSize::Md => "sh-otp-input--md".to_string(),
            OtpSize::Lg => "sh-otp-input--lg".to_string(),
        });

        classes.push(match self.variant {
            OtpVariant::Default => "sh-otp-input--default".to_string(),
            OtpVariant::Filled => "sh-otp-input--filled".to_string(),
            OtpVariant::Underline => "sh-otp-input--underline".to_string(),
        });

        if self.disabled {
            classes.push("sh-otp-input--disabled".to_string());
        }

        classes.join(" ")
    }
}

impl<'a> Render for OtpInput<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let chars: Vec<char> = self.value.unwrap_or("").chars().collect();

        html! {
            div class=(classes) role="group" aria-label=[self.label.or(Some("OTP code"))] {
                @if let Some(label) = self.label {
                    label class="sh-otp-input__label" {
                        (label)
                        @if self.required {
                            span class="sh-otp-input__required" { "*" }
                        }
                    }
                }
                div class="sh-otp-input__fields" {
                    @for i in 0..self.length {
                        @let char_value = chars.get(i).map(|c| c.to_string()).unwrap_or_default();
                        @let field_id = format!("{}-{}", self.id, i);
                        input
                            type="text"
                            inputmode="numeric"
                            pattern="[0-9]"
                            maxlength="1"
                            id=(field_id)
                            name=(format!("{}[{}]", self.name, i))
                            class="sh-otp-input__field"
                            value=(char_value)
                            disabled?[self.disabled]
                            required?[self.required]
                            autocomplete=[if self.autocomplete { Some("one-time-code") } else { None }]
                            aria-label=(format!("Digit {}", i + 1));
                    }
                }
            }
        }
    }
}

/// Generate OTP input CSS
pub fn otp_input_css() -> String {
    r#"
/* OTP Input Component */
.sh-otp-input {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    font-family: var(--sh-font-sans);
}

.sh-otp-input__label {
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--sh-text);
}

.sh-otp-input__required {
    color: var(--sh-error);
    margin-left: 0.25rem;
}

.sh-otp-input__fields {
    display: flex;
    gap: 0.5rem;
    justify-content: center;
}

.sh-otp-input__field {
    width: 3rem;
    height: 3.5rem;
    text-align: center;
    font-size: 1.5rem;
    font-weight: 600;
    font-family: var(--sh-font-mono);
    color: var(--sh-text);
    background: var(--sh-surface);
    border: 2px solid var(--sh-border);
    border-radius: var(--sh-radius-md);
    transition: all var(--sh-dur-fast) var(--sh-ease-out);
    caret-color: var(--sh-accent);
}

.sh-otp-input__field:hover:not(:disabled) {
    border-color: color-mix(in srgb, var(--sh-accent) 50%, var(--sh-border));
}

.sh-otp-input__field:focus {
    outline: none;
    border-color: var(--sh-accent);
    box-shadow: 0 0 0 3px color-mix(in srgb, var(--sh-accent) 20%, transparent);
    background: var(--sh-surface);
}

/* Size variants */
.sh-otp-input--sm .sh-otp-input__field {
    width: 2.5rem;
    height: 3rem;
    font-size: 1.25rem;
}

.sh-otp-input--lg .sh-otp-input__field {
    width: 3.5rem;
    height: 4rem;
    font-size: 1.75rem;
}

/* Filled variant */
.sh-otp-input--filled .sh-otp-input__field {
    background: var(--sh-surface-2);
    border-color: transparent;
}

.sh-otp-input--filled .sh-otp-input__field:focus {
    background: var(--sh-surface);
    border-color: var(--sh-accent);
}

/* Underline variant */
.sh-otp-input--underline .sh-otp-input__field {
    background: transparent;
    border: none;
    border-bottom: 2px solid var(--sh-border);
    border-radius: 0;
}

.sh-otp-input--underline .sh-otp-input__field:focus {
    border-bottom-color: var(--sh-accent);
    box-shadow: none;
}

/* Disabled state */
.sh-otp-input--disabled .sh-otp-input__field {
    opacity: 0.5;
    cursor: not-allowed;
    background: var(--sh-surface-2);
}

/* Animation for filled state */
.sh-otp-input__field:not(:placeholder-shown) {
    border-color: var(--sh-success);
}

/* Responsive */
@media (max-width: 480px) {
    .sh-otp-input__field {
        width: 2.5rem;
        height: 3rem;
        font-size: 1.25rem;
    }
    
    .sh-otp-input__fields {
        gap: 0.375rem;
    }
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_otp_input_creation() {
        let otp = OtpInput::new("otp", "otp").length(6).value("123456");

        assert_eq!(otp.id, "otp");
        assert_eq!(otp.length, 6);
    }

    #[test]
    fn test_otp_input_length_bounds() {
        let otp = OtpInput::new("otp", "otp").length(20); // Should be clamped to 12

        assert_eq!(otp.length, 12);
    }

    #[test]
    fn test_otp_input_size() {
        let otp = OtpInput::new("otp", "otp").size(OtpSize::Lg);

        assert_eq!(otp.size, OtpSize::Lg);
    }
}
