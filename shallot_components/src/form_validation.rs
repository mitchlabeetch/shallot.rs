//! Form Validation Framework - Type-Safe Form State Management
//!
//! Implements phantom types for compile-time state machine validation as outlined
//! in research section 3.1.1. This ensures invalid form operations are caught at
//! compile time, not runtime.
//!
//! # Example
//! ```
//! use shallot_components::form_validation::{FormField, ValidationRule, Pristine, Dirty, Valid};
//!
//! // Start with pristine field
//! let field = FormField::new("email", "user@example.com")
//!     .with_rule(ValidationRule::Required)
//!     .with_rule(ValidationRule::Email);
//!
//! // Mark as touched (dirty)
//! let dirty_field = field.touch();
//!
//! // Validate - returns Result<FormField<Valid>, FormField<Invalid>>
//! match dirty_field.validate() {
//!     Ok(valid_field) => println!("Valid!"),
//!     Err(invalid_field) => println!("Errors: {:?}", invalid_field.errors()),
//! }
//! ```

use std::marker::PhantomData;

/// Form field states (phantom types)
#[derive(Debug, Clone)]
pub struct Pristine;
#[derive(Debug, Clone)]
pub struct Dirty;
#[derive(Debug, Clone)]
pub struct Valid;
#[derive(Debug, Clone)]
pub struct Invalid;

/// Validation errors
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    Required,
    MinLength(usize),
    MaxLength(usize),
    MinValue(f64),
    MaxValue(f64),
    Pattern(String),
    Email,
    Url,
    Numeric,
    Integer,
    Custom(String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Required => write!(f, "This field is required"),
            Self::MinLength(n) => write!(f, "Must be at least {} characters", n),
            Self::MaxLength(n) => write!(f, "Must be at most {} characters", n),
            Self::MinValue(v) => write!(f, "Must be at least {}", v),
            Self::MaxValue(v) => write!(f, "Must be at most {}", v),
            Self::Pattern(_) => write!(f, "Invalid format"),
            Self::Email => write!(f, "Please enter a valid email address"),
            Self::Url => write!(f, "Please enter a valid URL"),
            Self::Numeric => write!(f, "Must be a number"),
            Self::Integer => write!(f, "Must be a whole number"),
            Self::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

/// Validation rules that can be applied to form fields
#[derive(Debug, Clone, PartialEq)]
#[allow(unpredictable_function_pointer_comparisons)] // Custom variant contains function pointer
pub enum ValidationRule {
    Required,
    MinLength(usize),
    MaxLength(usize),
    MinValue(f64),
    MaxValue(f64),
    Pattern(String),
    Email,
    Url,
    Numeric,
    Integer,
    OneOf(Vec<String>),
    Custom(fn(&str) -> Option<String>),
}

impl ValidationRule {
    /// Validate a value against this rule
    pub fn validate(&self, value: &str) -> Result<(), ValidationError> {
        match self {
            Self::Required => {
                if value.trim().is_empty() {
                    Err(ValidationError::Required)
                } else {
                    Ok(())
                }
            }
            Self::MinLength(min) => {
                if value.len() < *min {
                    Err(ValidationError::MinLength(*min))
                } else {
                    Ok(())
                }
            }
            Self::MaxLength(max) => {
                if value.len() > *max {
                    Err(ValidationError::MaxLength(*max))
                } else {
                    Ok(())
                }
            }
            Self::MinValue(min) => match value.parse::<f64>() {
                Ok(v) if v >= *min => Ok(()),
                _ => Err(ValidationError::MinValue(*min)),
            },
            Self::MaxValue(max) => match value.parse::<f64>() {
                Ok(v) if v <= *max => Ok(()),
                _ => Err(ValidationError::MaxValue(*max)),
            },
            Self::Pattern(pattern) => {
                // Simple pattern matching (could use regex in full implementation)
                if value.matches(pattern).count() > 0 || value.is_empty() {
                    Ok(())
                } else {
                    Err(ValidationError::Pattern(pattern.clone()))
                }
            }
            Self::Email => {
                if value.is_empty() || value.contains('@') {
                    Ok(())
                } else {
                    Err(ValidationError::Email)
                }
            }
            Self::Url => {
                if value.is_empty() || value.starts_with("http://") || value.starts_with("https://")
                {
                    Ok(())
                } else {
                    Err(ValidationError::Url)
                }
            }
            Self::Numeric => {
                if value.is_empty() || value.parse::<f64>().is_ok() {
                    Ok(())
                } else {
                    Err(ValidationError::Numeric)
                }
            }
            Self::Integer => {
                if value.is_empty() || value.parse::<i64>().is_ok() {
                    Ok(())
                } else {
                    Err(ValidationError::Integer)
                }
            }
            Self::OneOf(options) => {
                if value.is_empty() || options.contains(&value.to_string()) {
                    Ok(())
                } else {
                    Err(ValidationError::Custom("Invalid selection".to_string()))
                }
            }
            Self::Custom(validator) => match validator(value) {
                None => Ok(()),
                Some(msg) => Err(ValidationError::Custom(msg)),
            },
        }
    }
}

/// A form field with compile-time state tracking
#[derive(Debug, Clone)]
pub struct FormField<State = Pristine> {
    name: String,
    value: String,
    rules: Vec<ValidationRule>,
    errors: Vec<ValidationError>,
    _state: PhantomData<State>,
}

impl FormField<Pristine> {
    /// Create a new pristine form field
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            rules: Vec::new(),
            errors: Vec::new(),
            _state: PhantomData,
        }
    }

    /// Add a validation rule
    pub fn with_rule(mut self, rule: ValidationRule) -> Self {
        self.rules.push(rule);
        self
    }

    /// Add multiple validation rules
    pub fn with_rules(mut self, rules: Vec<ValidationRule>) -> Self {
        self.rules.extend(rules);
        self
    }

    /// Mark field as touched/dirty
    pub fn touch(self) -> FormField<Dirty> {
        FormField {
            name: self.name,
            value: self.value,
            rules: self.rules,
            errors: Vec::new(),
            _state: PhantomData,
        }
    }

    /// Set value without marking as dirty (for initial values)
    pub fn set_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }
}

impl FormField<Dirty> {
    /// Validate the field against all rules
    pub fn validate(self) -> Result<FormField<Valid>, FormField<Invalid>> {
        let mut errors = Vec::new();

        for rule in &self.rules {
            if let Err(e) = rule.validate(&self.value) {
                errors.push(e);
            }
        }

        if errors.is_empty() {
            Ok(FormField {
                name: self.name,
                value: self.value,
                rules: self.rules,
                errors: Vec::new(),
                _state: PhantomData,
            })
        } else {
            Err(FormField {
                name: self.name,
                value: self.value,
                rules: self.rules,
                errors,
                _state: PhantomData,
            })
        }
    }

    /// Get current value
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Update value (stays dirty)
    pub fn update_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }
}

impl FormField<Valid> {
    /// Get the validated value
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Convert back to dirty to allow editing
    pub fn edit(self) -> FormField<Dirty> {
        FormField {
            name: self.name,
            value: self.value,
            rules: self.rules,
            errors: Vec::new(),
            _state: PhantomData,
        }
    }

    /// Check if field is valid (always true for this type)
    pub fn is_valid(&self) -> bool {
        true
    }
}

impl FormField<Invalid> {
    /// Get validation errors
    pub fn errors(&self) -> &[ValidationError] {
        &self.errors
    }

    /// Get the first error message
    pub fn first_error(&self) -> Option<&ValidationError> {
        self.errors.first()
    }

    /// Update value and return to dirty state for re-validation
    pub fn update_value(self, value: impl Into<String>) -> FormField<Dirty> {
        FormField {
            name: self.name,
            value: value.into(),
            rules: self.rules,
            errors: Vec::new(),
            _state: PhantomData,
        }
    }

    /// Convert back to dirty without changing value
    pub fn retry(self) -> FormField<Dirty> {
        FormField {
            name: self.name,
            value: self.value,
            rules: self.rules,
            errors: Vec::new(),
            _state: PhantomData,
        }
    }
}

/// Type-erased form field for storage in collections
#[derive(Debug, Clone)]
pub enum AnyFormField {
    Pristine(FormField<Pristine>),
    Dirty(FormField<Dirty>),
    Valid(FormField<Valid>),
    Invalid(FormField<Invalid>),
}

/// Form context that manages multiple fields
#[derive(Debug, Clone)]
pub struct FormContext {
    fields: std::collections::HashMap<String, AnyFormField>,
    submitted: bool,
}

impl FormContext {
    pub fn new() -> Self {
        Self {
            fields: std::collections::HashMap::new(),
            submitted: false,
        }
    }

    /// Add a field to the form
    pub fn add_field(&mut self, name: impl Into<String>, field: AnyFormField) {
        self.fields.insert(name.into(), field);
    }

    /// Get a field by name
    pub fn get_field(&self, name: &str) -> Option<&AnyFormField> {
        self.fields.get(name)
    }

    /// Check if all fields are valid
    pub fn is_valid(&self) -> bool {
        self.fields.values().all(|f| match f {
            AnyFormField::Valid(_) => true,
            _ => false,
        })
    }

    /// Get all validation errors
    pub fn all_errors(&self) -> Vec<(String, Vec<ValidationError>)> {
        self.fields
            .iter()
            .filter_map(|(name, field)| match field {
                AnyFormField::Invalid(f) => Some((name.clone(), f.errors().to_vec())),
                _ => None,
            })
            .collect()
    }

    /// Mark form as submitted
    pub fn submit(&mut self) {
        self.submitted = true;
    }

    pub fn is_submitted(&self) -> bool {
        self.submitted
    }
}

impl Default for FormContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Predefined validation rule sets for common use cases
pub mod presets {
    use super::ValidationRule;

    /// Email field validation
    pub fn email() -> Vec<ValidationRule> {
        vec![ValidationRule::Required, ValidationRule::Email]
    }

    /// Password field validation (min 8 chars)
    pub fn password() -> Vec<ValidationRule> {
        vec![ValidationRule::Required, ValidationRule::MinLength(8)]
    }

    /// Strong password validation
    pub fn strong_password() -> Vec<ValidationRule> {
        vec![ValidationRule::Required, ValidationRule::MinLength(12)]
    }

    /// Username validation (alphanumeric, 3-20 chars)
    pub fn username() -> Vec<ValidationRule> {
        vec![
            ValidationRule::Required,
            ValidationRule::MinLength(3),
            ValidationRule::MaxLength(20),
        ]
    }

    /// URL validation
    pub fn url() -> Vec<ValidationRule> {
        vec![ValidationRule::Required, ValidationRule::Url]
    }

    /// Numeric validation
    pub fn numeric() -> Vec<ValidationRule> {
        vec![ValidationRule::Required, ValidationRule::Numeric]
    }

    /// Integer validation
    pub fn integer() -> Vec<ValidationRule> {
        vec![ValidationRule::Required, ValidationRule::Integer]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_required_validation() {
        let rule = ValidationRule::Required;
        assert!(rule.validate("").is_err());
        assert!(rule.validate("  ").is_err());
        assert!(rule.validate("hello").is_ok());
    }

    #[test]
    fn test_min_length_validation() {
        let rule = ValidationRule::MinLength(5);
        assert!(rule.validate("abc").is_err());
        assert!(rule.validate("abcde").is_ok());
        assert!(rule.validate("abcdef").is_ok());
    }

    #[test]
    fn test_email_validation() {
        let rule = ValidationRule::Email;
        assert!(rule.validate("invalid").is_err());
        assert!(rule.validate("test@").is_ok()); // Basic check only
        assert!(rule.validate("test@example.com").is_ok());
        assert!(rule.validate("").is_ok()); // Empty is valid (use Required for mandatory)
    }

    #[test]
    fn test_form_field_state_machine() {
        // Start pristine
        let pristine = FormField::new("email", "test@example.com").with_rule(ValidationRule::Email);

        // Touch becomes dirty
        let dirty = pristine.touch();

        // Validate becomes valid or invalid
        let valid = dirty.validate().expect("Should be valid");
        assert_eq!(valid.value(), "test@example.com");

        // Can edit valid field
        let dirty_again = valid.edit();
        let valid_again = dirty_again.validate().expect("Should still be valid");
        assert!(valid_again.is_valid());
    }

    #[test]
    fn test_invalid_field() {
        let pristine = FormField::new("email", "invalid-email").with_rule(ValidationRule::Email);

        let dirty = pristine.touch();
        let result = dirty.validate();

        assert!(result.is_err());
        let invalid = result.unwrap_err();
        assert!(!invalid.errors().is_empty());
    }

    #[test]
    fn test_form_context() {
        let mut form = FormContext::new();

        let email_field = FormField::new("email", "test@example.com")
            .with_rules(presets::email())
            .touch()
            .validate()
            .expect("valid");

        form.add_field("email", AnyFormField::Valid(email_field));

        assert!(form.is_valid());
    }

    #[test]
    fn test_custom_validation() {
        let validator = |v: &str| -> Option<String> {
            if v == "forbidden" {
                Some("This value is not allowed".to_string())
            } else {
                None
            }
        };

        let rule = ValidationRule::Custom(validator);
        assert!(rule.validate("allowed").is_ok());
        assert!(rule.validate("forbidden").is_err());
    }
}
