//! Form Utilities - Validation, submission, and form state management
//!
//! This module provides:
//! - Validation rules and validators
//! - Form state management
//! - Field-level and form-level validation
//! - Error message formatting

use std::collections::HashMap;

/// A validation error
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub code: String,
}

impl ValidationError {
    /// Create a new validation error
    pub fn new(
        field: impl Into<String>,
        message: impl Into<String>,
        code: impl Into<String>,
    ) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
            code: code.into(),
        }
    }
}

/// Result of a validation operation
pub type ValidationResult = Result<(), Vec<ValidationError>>;

/// A validation rule that can be applied to a field
pub trait Validator: Send + Sync {
    /// Validate a value and return an error if invalid
    fn validate(&self, value: &str, field_name: &str) -> Option<ValidationError>;
}

/// Required field validator
#[derive(Debug, Clone)]
pub struct RequiredValidator {
    message: String,
}

impl RequiredValidator {
    pub fn new() -> Self {
        Self {
            message: "This field is required".to_string(),
        }
    }

    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }
}

impl Default for RequiredValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl Validator for RequiredValidator {
    fn validate(&self, value: &str, field_name: &str) -> Option<ValidationError> {
        if value.trim().is_empty() {
            Some(ValidationError::new(field_name, &self.message, "required"))
        } else {
            None
        }
    }
}

/// Minimum length validator
#[derive(Debug, Clone)]
pub struct MinLengthValidator {
    min: usize,
    message: String,
}

impl MinLengthValidator {
    pub fn new(min: usize) -> Self {
        Self {
            min,
            message: format!("Must be at least {} characters", min),
        }
    }

    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }
}

impl Validator for MinLengthValidator {
    fn validate(&self, value: &str, field_name: &str) -> Option<ValidationError> {
        if value.chars().count() < self.min {
            Some(ValidationError::new(
                field_name,
                &self.message,
                "min_length",
            ))
        } else {
            None
        }
    }
}

/// Maximum length validator
#[derive(Debug, Clone)]
pub struct MaxLengthValidator {
    max: usize,
    message: String,
}

impl MaxLengthValidator {
    pub fn new(max: usize) -> Self {
        Self {
            max,
            message: format!("Must be at most {} characters", max),
        }
    }

    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }
}

impl Validator for MaxLengthValidator {
    fn validate(&self, value: &str, field_name: &str) -> Option<ValidationError> {
        if value.chars().count() > self.max {
            Some(ValidationError::new(
                field_name,
                &self.message,
                "max_length",
            ))
        } else {
            None
        }
    }
}

/// Pattern validator (regex)
#[derive(Debug, Clone)]
pub struct PatternValidator {
    pattern: regex::Regex,
    message: String,
}

impl PatternValidator {
    pub fn new(pattern: &str) -> Result<Self, regex::Error> {
        let regex = regex::Regex::new(pattern)?;
        Ok(Self {
            pattern: regex,
            message: "Invalid format".to_string(),
        })
    }

    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }
}

impl Validator for PatternValidator {
    fn validate(&self, value: &str, field_name: &str) -> Option<ValidationError> {
        if !self.pattern.is_match(value) {
            Some(ValidationError::new(field_name, &self.message, "pattern"))
        } else {
            None
        }
    }
}

/// Email validator
#[derive(Debug, Clone)]
pub struct EmailValidator {
    message: String,
}

impl EmailValidator {
    pub fn new() -> Self {
        Self {
            message: "Please enter a valid email address".to_string(),
        }
    }

    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }
}

impl Default for EmailValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl Validator for EmailValidator {
    fn validate(&self, value: &str, field_name: &str) -> Option<ValidationError> {
        // Skip validation for empty strings (let required validator handle that)
        if value.is_empty() {
            return None;
        }
        // Simple email validation regex
        let email_regex =
            regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        if !email_regex.is_match(value) {
            Some(ValidationError::new(field_name, &self.message, "email"))
        } else {
            None
        }
    }
}

/// URL validator
#[derive(Debug, Clone)]
pub struct UrlValidator {
    message: String,
}

impl UrlValidator {
    pub fn new() -> Self {
        Self {
            message: "Please enter a valid URL".to_string(),
        }
    }

    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }
}

impl Default for UrlValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl Validator for UrlValidator {
    fn validate(&self, value: &str, field_name: &str) -> Option<ValidationError> {
        let url_regex = regex::Regex::new(r"^https?://.+").unwrap();
        if !url_regex.is_match(value) {
            Some(ValidationError::new(field_name, &self.message, "url"))
        } else {
            None
        }
    }
}

/// Numeric range validator
#[derive(Debug, Clone)]
pub struct RangeValidator<T: PartialOrd + ToString + Clone> {
    min: Option<T>,
    max: Option<T>,
    message: String,
}

impl<T: PartialOrd + ToString + Clone> RangeValidator<T> {
    pub fn new() -> Self {
        Self {
            min: None,
            max: None,
            message: "Value out of range".to_string(),
        }
    }

    pub fn with_min(mut self, min: T) -> Self {
        self.min = Some(min);
        self
    }

    pub fn with_max(mut self, max: T) -> Self {
        self.max = Some(max);
        self
    }

    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }
}

impl Validator for RangeValidator<i32> {
    fn validate(&self, value: &str, field_name: &str) -> Option<ValidationError> {
        let num = value.parse::<i32>().ok()?;

        if let Some(min) = &self.min {
            if num < *min {
                return Some(ValidationError::new(field_name, &self.message, "min"));
            }
        }

        if let Some(max) = &self.max {
            if num > *max {
                return Some(ValidationError::new(field_name, &self.message, "max"));
            }
        }

        None
    }
}

impl Validator for RangeValidator<f64> {
    fn validate(&self, value: &str, field_name: &str) -> Option<ValidationError> {
        let num = value.parse::<f64>().ok()?;

        if let Some(min) = &self.min {
            if num < *min {
                return Some(ValidationError::new(field_name, &self.message, "min"));
            }
        }

        if let Some(max) = &self.max {
            if num > *max {
                return Some(ValidationError::new(field_name, &self.message, "max"));
            }
        }

        None
    }
}

/// Field validation configuration
pub struct FieldValidation {
    pub field_name: String,
    pub validators: Vec<Box<dyn Validator>>,
}

impl Clone for FieldValidation {
    fn clone(&self) -> Self {
        // Clone without validators since Box<dyn Validator> can't be cloned
        Self {
            field_name: self.field_name.clone(),
            validators: Vec::new(),
        }
    }
}

impl std::fmt::Debug for FieldValidation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FieldValidation")
            .field("field_name", &self.field_name)
            .field(
                "validators",
                &format!("[{} validators]", self.validators.len()),
            )
            .finish()
    }
}

impl FieldValidation {
    /// Create a new field validation configuration
    pub fn new(field_name: impl Into<String>) -> Self {
        Self {
            field_name: field_name.into(),
            validators: Vec::new(),
        }
    }

    /// Add a validator
    pub fn add_validator<V: Validator + 'static>(mut self, validator: V) -> Self {
        self.validators.push(Box::new(validator));
        self
    }

    /// Validate a value
    pub fn validate(&self, value: &str) -> Vec<ValidationError> {
        self.validators
            .iter()
            .filter_map(|v| v.validate(value, &self.field_name))
            .collect()
    }
}

/// Form validation schema
#[derive(Debug, Clone, Default)]
pub struct FormSchema {
    fields: HashMap<String, FieldValidation>,
}

impl FormSchema {
    /// Create a new form schema
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    /// Add a field validation
    pub fn field(mut self, validation: FieldValidation) -> Self {
        self.fields
            .insert(validation.field_name.clone(), validation);
        self
    }

    /// Validate all fields in a form data map
    pub fn validate(&self, data: &HashMap<String, String>) -> ValidationResult {
        let mut errors = Vec::new();

        for (field_name, field_validation) in &self.fields {
            let value = data.get(field_name).map(|s| s.as_str()).unwrap_or("");
            let field_errors = field_validation.validate(value);
            errors.extend(field_errors);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Validate a single field
    pub fn validate_field(&self, field_name: &str, value: &str) -> Vec<ValidationError> {
        self.fields
            .get(field_name)
            .map(|f| f.validate(value))
            .unwrap_or_default()
    }
}

/// Helper functions for common validations
pub mod validators {
    use super::*;

    /// Create a required validator
    pub fn required() -> RequiredValidator {
        RequiredValidator::new()
    }

    /// Create a minimum length validator
    pub fn min_length(min: usize) -> MinLengthValidator {
        MinLengthValidator::new(min)
    }

    /// Create a maximum length validator
    pub fn max_length(max: usize) -> MaxLengthValidator {
        MaxLengthValidator::new(max)
    }

    /// Create an email validator
    pub fn email() -> EmailValidator {
        EmailValidator::new()
    }

    /// Create a URL validator
    pub fn url() -> UrlValidator {
        UrlValidator::new()
    }

    /// Create a range validator for integers
    pub fn range_i32() -> RangeValidator<i32> {
        RangeValidator::new()
    }

    /// Create a range validator for floats
    pub fn range_f64() -> RangeValidator<f64> {
        RangeValidator::new()
    }

    /// Create a pattern validator
    pub fn pattern(pattern: &str) -> Result<PatternValidator, regex::Error> {
        PatternValidator::new(pattern)
    }
}

/// Form state management
#[derive(Debug, Clone, Default)]
pub struct FormState {
    pub values: HashMap<String, String>,
    pub errors: HashMap<String, Vec<String>>,
    pub touched: HashMap<String, bool>,
    pub is_valid: bool,
    pub is_dirty: bool,
}

impl FormState {
    /// Create a new form state
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a field value
    pub fn set_value(&mut self, field: impl Into<String>, value: impl Into<String>) {
        self.values.insert(field.into(), value.into());
        self.is_dirty = true;
    }

    /// Get a field value
    pub fn get_value(&self, field: &str) -> Option<&String> {
        self.values.get(field)
    }

    /// Mark a field as touched
    pub fn touch(&mut self, field: impl Into<String>) {
        self.touched.insert(field.into(), true);
    }

    /// Check if a field is touched
    pub fn is_touched(&self, field: &str) -> bool {
        self.touched.get(field).copied().unwrap_or(false)
    }

    /// Set field errors
    pub fn set_errors(&mut self, field: impl Into<String>, errors: Vec<String>) {
        self.errors.insert(field.into(), errors);
        self.update_validity();
    }

    /// Get field errors
    pub fn get_errors(&self, field: &str) -> Option<&Vec<String>> {
        self.errors.get(field)
    }

    /// Clear all errors
    pub fn clear_errors(&mut self) {
        self.errors.clear();
        self.is_valid = true;
    }

    /// Update form validity based on errors
    fn update_validity(&mut self) {
        self.is_valid = self.errors.values().all(|e| e.is_empty());
    }

    /// Validate the form with a schema
    pub fn validate(&mut self, schema: &FormSchema) -> bool {
        self.errors.clear();

        for (field_name, field_validation) in &schema.fields {
            let value = self
                .values
                .get(field_name)
                .map(|s| s.as_str())
                .unwrap_or("");
            let errors: Vec<String> = field_validation
                .validate(value)
                .into_iter()
                .map(|e| e.message)
                .collect();

            if !errors.is_empty() {
                self.errors.insert(field_name.clone(), errors);
            }
        }

        self.update_validity();
        self.is_valid
    }
}

#[cfg(test)]
mod tests {
    use super::validators::*;
    use super::*;

    #[test]
    fn test_required_validator() {
        let validator = RequiredValidator::new();
        assert!(validator.validate("hello", "field").is_none());
        assert!(validator.validate("", "field").is_some());
        assert!(validator.validate("   ", "field").is_some());
    }

    #[test]
    fn test_min_length_validator() {
        let validator = MinLengthValidator::new(5);
        assert!(validator.validate("hello", "field").is_none());
        assert!(validator.validate("hi", "field").is_some());
    }

    #[test]
    fn test_max_length_validator() {
        let validator = MaxLengthValidator::new(5);
        assert!(validator.validate("hello", "field").is_none());
        assert!(validator.validate("hello world", "field").is_some());
    }

    #[test]
    fn test_email_validator() {
        let validator = EmailValidator::new();
        assert!(validator.validate("test@example.com", "field").is_none());
        assert!(validator.validate("invalid", "field").is_some());
        assert!(validator.validate("@example.com", "field").is_some());
    }

    #[test]
    fn test_range_validator() {
        let validator = RangeValidator::new().with_min(0).with_max(100);

        assert!(validator.validate("50", "field").is_none());
        assert!(validator.validate("-1", "field").is_some());
        assert!(validator.validate("101", "field").is_some());
    }

    #[test]
    fn test_field_validation() {
        let field = FieldValidation::new("email")
            .add_validator(required())
            .add_validator(email());

        assert!(field.validate("test@example.com").is_empty());
        assert_eq!(field.validate("").len(), 1); // Only required fails
        assert_eq!(field.validate("invalid").len(), 1); // Only email fails
    }

    #[test]
    fn test_form_schema() {
        let schema = FormSchema::new()
            .field(FieldValidation::new("name").add_validator(required()))
            .field(
                FieldValidation::new("email")
                    .add_validator(required())
                    .add_validator(email()),
            );

        let mut data = HashMap::new();
        data.insert("name".to_string(), "John".to_string());
        data.insert("email".to_string(), "test@example.com".to_string());

        assert!(schema.validate(&data).is_ok());

        data.insert("email".to_string(), "invalid".to_string());
        assert!(schema.validate(&data).is_err());
    }

    #[test]
    fn test_form_state() {
        let mut state = FormState::new();
        state.set_value("name", "John");
        state.set_value("email", "test@example.com");

        assert_eq!(state.get_value("name"), Some(&"John".to_string()));
        assert!(state.is_dirty);

        state.touch("name");
        assert!(state.is_touched("name"));
        assert!(!state.is_touched("email"));
    }

    #[test]
    fn test_helper_functions() {
        let required = validators::required();
        assert!(required.validate("value", "field").is_none());

        let min = validators::min_length(5);
        assert!(min.validate("test", "field").is_some());

        let max = validators::max_length(5);
        assert!(max.validate("testing", "field").is_some());
    }
}

// =============================================================================
// UI Components for Form Rendering
// =============================================================================

use maud::{html, Markup};

/// Form variant for visual styling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FormVariant {
    /// Default form styling
    #[default]
    Default,
    /// Card-styled form with border
    Card,
    /// Compact form with minimal spacing
    Compact,
    /// Inline form (horizontal layout)
    Inline,
}

/// Form size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FormSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// Form component with validation support
pub struct Form<'a> {
    /// Form identifier
    pub id: Option<&'a str>,
    /// Form action URL
    pub action: Option<&'a str>,
    /// HTTP method
    pub method: Option<&'a str>,
    /// Visual variant
    pub variant: FormVariant,
    /// Size variant
    pub size: FormSize,
    /// Whether form is disabled
    pub disabled: bool,
    /// Form fields content
    pub fields: Markup,
    /// Submit button content
    pub submit_button: Option<Markup>,
    /// Additional CSS classes
    pub class: Option<&'a str>,
    /// Aria label
    pub aria_label: Option<&'a str>,
    /// Whether to show validation errors inline
    pub show_inline_errors: bool,
}

impl<'a> Form<'a> {
    /// Create a new form
    pub fn new() -> Self {
        Self {
            id: None,
            action: None,
            method: Some("POST"),
            variant: FormVariant::Default,
            size: FormSize::Medium,
            disabled: false,
            fields: html! {},
            submit_button: None,
            class: None,
            aria_label: None,
            show_inline_errors: true,
        }
    }

    /// Set form id
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }

    /// Set form action
    pub fn action(mut self, action: &'a str) -> Self {
        self.action = Some(action);
        self
    }

    /// Set form method
    pub fn method(mut self, method: &'a str) -> Self {
        self.method = Some(method);
        self
    }

    /// Set form variant
    pub fn variant(mut self, variant: FormVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set form size
    pub fn size(mut self, size: FormSize) -> Self {
        self.size = size;
        self
    }

    /// Set form disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set form fields content
    pub fn fields(mut self, fields: Markup) -> Self {
        self.fields = fields;
        self
    }

    /// Set submit button
    pub fn submit_button(mut self, button: Markup) -> Self {
        self.submit_button = Some(button);
        self
    }

    /// Set additional CSS classes
    pub fn class(mut self, class: &'a str) -> Self {
        self.class = Some(class);
        self
    }

    /// Set aria label
    pub fn aria_label(mut self, label: &'a str) -> Self {
        self.aria_label = Some(label);
        self
    }

    /// Render the form
    pub fn render(self) -> Markup {
        let variant_class = match self.variant {
            FormVariant::Default => "sh-form",
            FormVariant::Card => "sh-form sh-form--card",
            FormVariant::Compact => "sh-form sh-form--compact",
            FormVariant::Inline => "sh-form sh-form--inline",
        };

        let size_class = match self.size {
            FormSize::Small => "sh-form--sm",
            FormSize::Medium => "sh-form--md",
            FormSize::Large => "sh-form--lg",
        };

        html! {
            form
                class={(variant_class) " " (size_class) " " (self.class.unwrap_or(""))}
                id=[self.id]
                action=[self.action]
                method=[self.method]
                disabled=[if self.disabled { Some("") } else { None }]
                aria-label=[self.aria_label]
                role="form"
                novalidate
            {
                (self.fields)
                @if let Some(button) = self.submit_button {
                    div class="sh-form__actions" {
                        (button)
                    }
                }
            }
        }
    }
}

impl<'a> Default for Form<'a> {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Form Field Component
// =============================================================================

/// Form field wrapper with label, input, and error display
pub struct FormField<'a> {
    /// Field name/id
    pub name: &'a str,
    /// Label text
    pub label: Option<&'a str>,
    /// Helper text/description
    pub helper_text: Option<&'a str>,
    /// Error message
    pub error: Option<&'a str>,
    /// Whether the field is required
    pub required: bool,
    /// Whether the field has an error
    pub has_error: bool,
    /// Whether the field is disabled
    pub disabled: bool,
    /// Input content
    pub input: Markup,
    /// Additional CSS classes
    pub class: Option<&'a str>,
}

impl<'a> FormField<'a> {
    /// Create a new form field
    pub fn new(name: &'a str, input: Markup) -> Self {
        Self {
            name,
            label: None,
            helper_text: None,
            error: None,
            required: false,
            has_error: false,
            disabled: false,
            input,
            class: None,
        }
    }

    /// Set label
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    /// Set helper text
    pub fn helper_text(mut self, text: &'a str) -> Self {
        self.helper_text = Some(text);
        self
    }

    /// Set error message
    pub fn error(mut self, error: &'a str) -> Self {
        self.error = Some(error);
        self.has_error = true;
        self
    }

    /// Set required state
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set additional CSS classes
    pub fn class(mut self, class: &'a str) -> Self {
        self.class = Some(class);
        self
    }

    /// Render the form field
    pub fn render(self) -> Markup {
        let error_class = if self.has_error {
            "sh-form-field--error"
        } else {
            ""
        };

        let disabled_class = if self.disabled {
            "sh-form-field--disabled"
        } else {
            ""
        };

        let required_marker = if self.required {
            html! { span class="sh-form-field__required" aria-hidden="true" { "*" } }
        } else {
            html! {}
        };

        html! {
            div class={"sh-form-field " (error_class) " " (disabled_class) " " (self.class.unwrap_or(""))} {
                @if let Some(label_text) = self.label {
                    label
                        for=(self.name)
                        class="sh-form-field__label"
                    {
                        (label_text)
                        (required_marker)
                    }
                }
                div class="sh-form-field__input" {
                    (self.input)
                }
                @if let Some(error_msg) = self.error {
                    span
                        class="sh-form-field__error"
                        role="alert"
                        aria-live="polite"
                    {
                        (error_msg)
                    }
                } @else if let Some(helper) = self.helper_text {
                    span class="sh-form-field__helper" {
                        (helper)
                    }
                }
            }
        }
    }
}

// =============================================================================
// Form Group Component
// =============================================================================

/// Layout for grouping form fields
pub enum FormGroupLayout {
    /// Stack fields vertically
    Vertical,
    /// Arrange fields horizontally
    Horizontal,
    /// Inline layout
    Inline,
}

/// Group of form fields with consistent layout
pub struct FormGroup<'a> {
    /// Group label/legend
    pub legend: Option<&'a str>,
    /// Layout direction
    pub layout: FormGroupLayout,
    /// Field content
    pub fields: Markup,
    /// Additional CSS classes
    pub class: Option<&'a str>,
}

impl<'a> FormGroup<'a> {
    /// Create a new form group
    pub fn new(fields: Markup) -> Self {
        Self {
            legend: None,
            layout: FormGroupLayout::Vertical,
            fields,
            class: None,
        }
    }

    /// Set legend
    pub fn legend(mut self, legend: &'a str) -> Self {
        self.legend = Some(legend);
        self
    }

    /// Set layout
    pub fn layout(mut self, layout: FormGroupLayout) -> Self {
        self.layout = layout;
        self
    }

    /// Set additional CSS classes
    pub fn class(mut self, class: &'a str) -> Self {
        self.class = Some(class);
        self
    }

    /// Render the form group
    pub fn render(self) -> Markup {
        let layout_class = match self.layout {
            FormGroupLayout::Vertical => "sh-form-group--vertical",
            FormGroupLayout::Horizontal => "sh-form-group--horizontal",
            FormGroupLayout::Inline => "sh-form-group--inline",
        };

        html! {
            fieldset class={"sh-form-group " (layout_class) " " (self.class.unwrap_or(""))} {
                @if let Some(legend_text) = self.legend {
                    legend class="sh-form-group__legend" {
                        (legend_text)
                    }
                }
                (self.fields)
            }
        }
    }
}

// =============================================================================
// CSS Generation
// =============================================================================

/// Generate CSS for form components
pub fn form_css() -> String {
    r#"
.sh-form {
    display: flex;
    flex-direction: column;
    gap: var(--sh-form-gap, 1rem);
    width: 100%;
    font-family: inherit;
}

.sh-form--card {
    padding: var(--sh-form-padding, 1.5rem);
    background: var(--sh-form-bg, var(--sh-surface));
    border: 1px solid var(--sh-form-border-color, var(--sh-border));
    border-radius: var(--sh-form-radius, var(--sh-radius-lg));
    box-shadow: var(--sh-form-shadow, var(--sh-shadow-sm));
}

.sh-form--compact {
    gap: var(--sh-form-gap-compact, 0.5rem);
}

.sh-form--inline {
    flex-direction: row;
    align-items: flex-end;
    gap: var(--sh-form-gap-inline, 0.75rem);
}

.sh-form--sm {
    font-size: var(--sh-text-sm);
}

.sh-form--md {
    font-size: var(--sh-text-base);
}

.sh-form--lg {
    font-size: var(--sh-text-lg);
}

.sh-form__actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
    margin-top: 0.5rem;
    padding-top: 1rem;
    border-top: 1px solid var(--sh-border);
}

/* Form Field */
.sh-form-field {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
    width: 100%;
}

.sh-form-field--error .sh-form-field__input {
    outline: 2px solid var(--sh-error);
    outline-offset: 1px;
    border-radius: var(--sh-radius);
}

.sh-form-field--disabled {
    opacity: 0.6;
    pointer-events: none;
}

.sh-form-field__label {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    font-weight: 500;
    color: var(--sh-form-label-color, var(--sh-text-primary));
    cursor: pointer;
}

.sh-form-field__required {
    color: var(--sh-error);
    margin-left: 0.125rem;
}

.sh-form-field__input {
    position: relative;
    display: flex;
    width: 100%;
}

.sh-form-field__error {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    color: var(--sh-error);
    font-size: var(--sh-text-sm);
}

.sh-form-field__helper {
    color: var(--sh-text-secondary);
    font-size: var(--sh-text-sm);
}

/* Form Group */
.sh-form-group {
    border: none;
    padding: 0;
    margin: 0;
}

.sh-form-group__legend {
    font-weight: 600;
    font-size: var(--sh-text-base);
    color: var(--sh-text-primary);
    margin-bottom: 0.75rem;
    padding: 0;
}

.sh-form-group--vertical {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
}

.sh-form-group--horizontal {
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
    gap: 1rem;
}

.sh-form-group--horizontal .sh-form-field {
    flex: 1;
    min-width: 200px;
}

.sh-form-group--inline {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 0.5rem;
}

/* Focus states */
.sh-form:focus-within {
    outline: none;
}

.sh-form-field:focus-within .sh-form-field__label {
    color: var(--sh-accent);
}

/* Reduced motion */
@media (prefers-reduced-motion: reduce) {
    .sh-form * {
        transition: none !important;
        animation: none !important;
    }
}
"#
    .to_string()
}

#[cfg(test)]
mod ui_tests {
    use super::*;
    use maud::html;

    #[test]
    fn test_form_default() {
        let form = Form::new();
        let rendered = form.render();
        let html = rendered.into_string();
        assert!(html.contains("sh-form"));
    }

    #[test]
    fn test_form_card_variant() {
        let form = Form::new().variant(FormVariant::Card).render();
        let html = form.into_string();
        assert!(html.contains("sh-form--card"));
    }

    #[test]
    fn test_form_sizes() {
        let form_sm = Form::new().size(FormSize::Small).render();
        let html_sm = form_sm.into_string();
        assert!(html_sm.contains("sh-form--sm"));

        let form_lg = Form::new().size(FormSize::Large).render();
        let html_lg = form_lg.into_string();
        assert!(html_lg.contains("sh-form--lg"));
    }

    #[test]
    fn test_form_field() {
        let field = FormField::new("email", html! { input type="email" name="email"; })
            .label("Email Address")
            .required(true)
            .helper_text("We'll never share your email")
            .render();
        let html = field.into_string();
        assert!(html.contains("sh-form-field"));
        assert!(html.contains("Email Address"));
    }

    #[test]
    fn test_form_field_error() {
        let field = FormField::new("email", html! { input type="email" name="email"; })
            .error("Invalid email format")
            .render();
        let html = field.into_string();
        assert!(html.contains("sh-form-field--error"));
        assert!(html.contains("Invalid email format"));
    }

    #[test]
    fn test_form_group() {
        let group = FormGroup::new(html! {
            div { "Field 1" }
            div { "Field 2" }
        })
        .legend("Personal Information")
        .layout(FormGroupLayout::Horizontal)
        .render();
        let html = group.into_string();
        assert!(html.contains("sh-form-group"));
        assert!(html.contains("Personal Information"));
    }

    #[test]
    fn test_form_css_generation() {
        let css = form_css();
        assert!(css.contains(".sh-form"));
        assert!(css.contains(".sh-form-field"));
        assert!(css.contains(".sh-form-group"));
    }
}
