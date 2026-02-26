use crate::component::{
    AriaAttrs, Component, ComponentSize,
};
use maud::{html, Markup, Render};
use shallot_foundation::Icon;

/// Enhanced Input component with comprehensive features
pub struct Input<'a> {
    /// Input name/id
    name: &'a str,
    /// Input type
    type_: InputType,
    /// Label text
    label: Option<&'a str>,
    /// Placeholder text
    placeholder: Option<&'a str>,
    /// Current value
    value: Option<&'a str>,
    /// Default value
    default_value: Option<&'a str>,
    /// Helper text/description
    helper_text: Option<&'a str>,
    /// Error message
    error: Option<&'a str>,
    /// Whether the field is required
    required: bool,
    /// Whether the field is disabled
    disabled: bool,
    /// Whether the field is read-only
    readonly: bool,
    /// Whether the field has an error
    has_error: bool,
    /// Size variant
    size: ComponentSize,
    /// Visual variant
    variant: InputVariant,
    /// Left icon
    icon_left: Option<Icon>,
    /// Right icon
    icon_right: Option<Icon>,
    /// ARIA attributes
    aria: AriaAttrs,
    /// Additional CSS classes
    custom_class: Option<&'a str>,
    /// Autocomplete attribute
    autocomplete: Option<&'a str>,
    /// Pattern for validation
    pattern: Option<&'a str>,
    /// Minimum value/length
    min: Option<&'a str>,
    /// Maximum value/length
    max: Option<&'a str>,
    /// Step increment
    step: Option<&'a str>,
    /// Maximum length
    max_length: Option<usize>,
    /// Auto-focus on mount
    autofocus: bool,
}

/// Input type variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputType {
    #[default]
    Text,
    Email,
    Password,
    Number,
    Tel,
    Url,
    Search,
    Date,
    Time,
    DateTimeLocal,
    Month,
    Week,
    Color,
}

impl InputType {
    fn as_str(&self) -> &'static str {
        match self {
            InputType::Text => "text",
            InputType::Email => "email",
            InputType::Password => "password",
            InputType::Number => "number",
            InputType::Tel => "tel",
            InputType::Url => "url",
            InputType::Search => "search",
            InputType::Date => "date",
            InputType::Time => "time",
            InputType::DateTimeLocal => "datetime-local",
            InputType::Month => "month",
            InputType::Week => "week",
            InputType::Color => "color",
        }
    }
}

/// Input visual variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputVariant {
    #[default]
    Default,
    Filled,
    Outlined,
    Flushed,
}

impl InputVariant {
    fn class_suffix(&self) -> &'static str {
        match self {
            InputVariant::Default => "default",
            InputVariant::Filled => "filled",
            InputVariant::Outlined => "outlined",
            InputVariant::Flushed => "flushed",
        }
    }
}

impl<'a> Input<'a> {
    /// Create a new input with the given name
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            type_: InputType::Text,
            label: None,
            placeholder: None,
            value: None,
            default_value: None,
            helper_text: None,
            error: None,
            required: false,
            disabled: false,
            readonly: false,
            has_error: false,
            size: ComponentSize::Md,
            variant: InputVariant::Default,
            icon_left: None,
            icon_right: None,
            aria: AriaAttrs::new(),
            custom_class: None,
            autocomplete: None,
            pattern: None,
            min: None,
            max: None,
            step: None,
            max_length: None,
            autofocus: false,
        }
    }

    /// Create an email input
    pub fn email(name: &'a str) -> Self {
        Self::new(name).type_(InputType::Email)
    }

    /// Create a password input
    pub fn password(name: &'a str) -> Self {
        Self::new(name).type_(InputType::Password)
    }

    /// Create a number input
    pub fn number(name: &'a str) -> Self {
        Self::new(name).type_(InputType::Number)
    }

    /// Create a search input
    pub fn search(name: &'a str) -> Self {
        Self::new(name).type_(InputType::Search)
    }

    /// Set the input type
    pub fn type_(mut self, type_: InputType) -> Self {
        self.type_ = type_;
        self
    }

    /// Set the label
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    /// Set the placeholder
    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    /// Set the value
    pub fn value(mut self, value: &'a str) -> Self {
        self.value = Some(value);
        self
    }

    /// Set the default value
    pub fn default_value(mut self, value: &'a str) -> Self {
        self.default_value = Some(value);
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

    /// Set readonly state
    pub fn readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    /// Set size variant
    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }

    /// Set visual variant
    pub fn variant(mut self, variant: InputVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set left icon
    pub fn icon_left(mut self, icon: Icon) -> Self {
        self.icon_left = Some(icon);
        self
    }

    /// Set right icon
    pub fn icon_right(mut self, icon: Icon) -> Self {
        self.icon_right = Some(icon);
        self
    }

    /// Set ARIA attributes
    pub fn aria(mut self, aria: AriaAttrs) -> Self {
        self.aria = aria;
        self
    }

    /// Set custom CSS class
    pub fn custom_class(mut self, class: &'a str) -> Self {
        self.custom_class = Some(class);
        self
    }

    /// Set autocomplete attribute
    pub fn autocomplete(mut self, autocomplete: &'a str) -> Self {
        self.autocomplete = Some(autocomplete);
        self
    }

    /// Set pattern for validation
    pub fn pattern(mut self, pattern: &'a str) -> Self {
        self.pattern = Some(pattern);
        self
    }

    /// Set autofocus
    pub fn autofocus(mut self, autofocus: bool) -> Self {
        self.autofocus = autofocus;
        self
    }

    /// Build the CSS classes for the input wrapper
    fn build_wrapper_classes(&self) -> String {
        let mut classes = vec!["sh-input-wrapper".to_string()];

        classes.push(format!("sh-input-wrapper--{}", self.size.class_suffix()));
        classes.push(format!("sh-input-wrapper--{}", self.variant.class_suffix()));

        if self.has_error {
            classes.push("sh-input-wrapper--error".to_string());
        }

        if self.disabled {
            classes.push("sh-input-wrapper--disabled".to_string());
        }

        if self.icon_left.is_some() {
            classes.push("sh-input-wrapper--has-left-icon".to_string());
        }

        if self.icon_right.is_some() {
            classes.push("sh-input-wrapper--has-right-icon".to_string());
        }

        if let Some(custom) = self.custom_class {
            classes.push(custom.to_string());
        }

        classes.join(" ")
    }

    /// Build the CSS classes for the input element
    fn build_input_classes(&self) -> String {
        let mut classes = vec!["sh-input".to_string()];
        classes.push(format!("sh-input--{}", self.size.class_suffix()));
        classes.join(" ")
    }

    /// Render an icon
    fn render_icon(&self, icon: &Icon, position: &str) -> Markup {
        let icon_html = icon.to_svg_string();
        html! {
            span class=(format!("sh-input__icon sh-input__icon--{}", position)) {
                (maud::PreEscaped(icon_html))
            }
        }
    }
}

impl<'a> Render for Input<'a> {
    fn render(&self) -> Markup {
        let wrapper_class = self.build_wrapper_classes();
        let input_class = self.build_input_classes();
        let input_id = format!("sh-input-{}", self.name);

        html! {
            div class=(wrapper_class) {
                @if let Some(label) = self.label {
                    label class="sh-input__label" for=(input_id) {
                        (label)
                        @if self.required {
                            span class="sh-input__required" { " *" }
                        }
                    }
                }

                div class="sh-input__field-wrapper" {
                    @if let Some(icon) = &self.icon_left {
                        (self.render_icon(icon, "left"))
                    }

                    input
                        class=(input_class)
                        type=(self.type_.as_str())
                        name=(self.name)
                        id=(input_id)
                        placeholder=[self.placeholder]
                        value=[self.value]
                        required?[self.required]
                        disabled?[self.disabled]
                        readonly?[self.readonly]
                        autocomplete=[self.autocomplete]
                        pattern=[self.pattern]
                        min=[self.min]
                        max=[self.max]
                        step=[self.step]
                        maxlength=[self.max_length.map(|m| m.to_string())]
                        autofocus?[self.autofocus]
                        aria-invalid=[if self.has_error { Some("true") } else { None }]
                        aria-describedby=[self.error.map(|_| format!("{}-error", input_id))]
                    {};

                    @if let Some(icon) = &self.icon_right {
                        (self.render_icon(icon, "right"))
                    }
                }

                @if let Some(error) = self.error {
                    div class="sh-input__error" id=(format!("{}-error", input_id)) {
                        (error)
                    }
                } @else if let Some(helper) = self.helper_text {
                    div class="sh-input__helper" {
                        (helper)
                    }
                }
            }
        }
    }
}

impl<'a> Component for Input<'a> {
    fn classes(&self) -> String {
        self.build_wrapper_classes()
    }

    fn is_disabled(&self) -> bool {
        self.disabled
    }
}

/// Textarea component
#[allow(dead_code)] // Fields reserved for future CSS styling
pub struct Textarea<'a> {
    name: &'a str,
    label: Option<&'a str>,
    placeholder: Option<&'a str>,
    value: Option<&'a str>,
    rows: u8,
    cols: Option<u8>,
    required: bool,
    disabled: bool,
    readonly: bool,
    error: Option<&'a str>,
    helper_text: Option<&'a str>,
    size: ComponentSize,
    resize: TextareaResize,
    max_length: Option<usize>,
    custom_class: Option<&'a str>,
}

/// Textarea resize options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextareaResize {
    #[default]
    Both,
    None,
    Horizontal,
    Vertical,
}

impl TextareaResize {
    fn css_value(&self) -> &'static str {
        match self {
            TextareaResize::Both => "both",
            TextareaResize::None => "none",
            TextareaResize::Horizontal => "horizontal",
            TextareaResize::Vertical => "vertical",
        }
    }
}

impl<'a> Textarea<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            label: None,
            placeholder: None,
            value: None,
            rows: 4,
            cols: None,
            required: false,
            disabled: false,
            readonly: false,
            error: None,
            helper_text: None,
            size: ComponentSize::Md,
            resize: TextareaResize::Both,
            max_length: None,
            custom_class: None,
        }
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }
    pub fn placeholder(mut self, p: &'a str) -> Self {
        self.placeholder = Some(p);
        self
    }
    pub fn value(mut self, v: &'a str) -> Self {
        self.value = Some(v);
        self
    }
    pub fn rows(mut self, r: u8) -> Self {
        self.rows = r;
        self
    }
    pub fn required(mut self, r: bool) -> Self {
        self.required = r;
        self
    }
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }
    pub fn error(mut self, e: &'a str) -> Self {
        self.error = Some(e);
        self
    }
    pub fn helper_text(mut self, h: &'a str) -> Self {
        self.helper_text = Some(h);
        self
    }
    pub fn resize(mut self, r: TextareaResize) -> Self {
        self.resize = r;
        self
    }
    pub fn max_length(mut self, m: usize) -> Self {
        self.max_length = Some(m);
        self
    }
}

impl<'a> Render for Textarea<'a> {
    fn render(&self) -> Markup {
        let has_error = self.error.is_some();
        let input_id = format!("sh-textarea-{}", self.name);

        html! {
            div class=(format!("sh-textarea-wrapper {}",
                if has_error { "sh-textarea-wrapper--error" } else { "" }
            )) {
                @if let Some(label) = self.label {
                    label class="sh-input__label" for=(input_id) {
                        (label)
                        @if self.required {
                            span class="sh-input__required" { " *" }
                        }
                    }
                }

                textarea
                    class="sh-textarea"
                    name=(self.name)
                    id=(input_id)
                    rows=(self.rows)
                    placeholder=[self.placeholder]
                    required?[self.required]
                    disabled?[self.disabled]
                    readonly?[self.readonly]
                    maxlength=[self.max_length.map(|m| m.to_string())]
                    style=(format!("resize: {}", self.resize.css_value()))
                {
                    (self.value.unwrap_or(""))
                }

                @if let Some(error) = self.error {
                    div class="sh-input__error" { (error) }
                } @else if let Some(helper) = self.helper_text {
                    div class="sh-input__helper" { (helper) }
                }
            }
        }
    }
}

/// Checkbox component
pub struct Checkbox<'a> {
    label: &'a str,
    name: &'a str,
    checked: bool,
    disabled: bool,
    required: bool,
    indeterminate: bool,
    helper_text: Option<&'a str>,
    size: ComponentSize,
}

impl<'a> Checkbox<'a> {
    pub fn new(label: &'a str, name: &'a str) -> Self {
        Self {
            label,
            name,
            checked: false,
            disabled: false,
            required: false,
            indeterminate: false,
            helper_text: None,
            size: ComponentSize::Md,
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }
    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }
    pub fn helper_text(mut self, text: &'a str) -> Self {
        self.helper_text = Some(text);
        self
    }
    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }
}

impl<'a> Render for Checkbox<'a> {
    fn render(&self) -> Markup {
        let input_id = format!("sh-checkbox-{}", self.name);

        html! {
            label class=(format!("sh-checkbox sh-checkbox--{}", self.size.class_suffix())) {
                input
                    class="sh-checkbox__input"
                    type="checkbox"
                    name=(self.name)
                    id=(input_id)
                    checked?[self.checked]
                    disabled?[self.disabled]
                    required?[self.required]
                    data-indeterminate=[if self.indeterminate { Some("true") } else { None }]
                {};
                span class="sh-checkbox__control" {}
                span class="sh-checkbox__label" {
                    (self.label)
                    @if let Some(helper) = self.helper_text {
                        span class="sh-checkbox__helper" { (helper) }
                    }
                }
            }
        }
    }
}

/// Radio component
pub struct Radio<'a> {
    label: &'a str,
    name: &'a str,
    value: &'a str,
    checked: bool,
    disabled: bool,
    required: bool,
    size: ComponentSize,
}

impl<'a> Radio<'a> {
    pub fn new(label: &'a str, name: &'a str, value: &'a str) -> Self {
        Self {
            label,
            name,
            value,
            checked: false,
            disabled: false,
            required: false,
            size: ComponentSize::Md,
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }
    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }
}

impl<'a> Render for Radio<'a> {
    fn render(&self) -> Markup {
        let input_id = format!("sh-radio-{}-{}", self.name, self.value);

        html! {
            label class=(format!("sh-radio sh-radio--{}", self.size.class_suffix())) {
                input
                    class="sh-radio__input"
                    type="radio"
                    name=(self.name)
                    id=(input_id)
                    value=(self.value)
                    checked?[self.checked]
                    disabled?[self.disabled]
                    required?[self.required]
                {};
                span class="sh-radio__control" {}
                span class="sh-radio__label" { (self.label) }
            }
        }
    }
}

/// Switch/Toggle component
pub struct Switch<'a> {
    label: Option<&'a str>,
    name: &'a str,
    checked: bool,
    disabled: bool,
    required: bool,
    size: ComponentSize,
}

impl<'a> Switch<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            label: None,
            name,
            checked: false,
            disabled: false,
            required: false,
            size: ComponentSize::Md,
        }
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }
}

impl<'a> Render for Switch<'a> {
    fn render(&self) -> Markup {
        let input_id = format!("sh-switch-{}", self.name);

        html! {
            label class=(format!("sh-switch sh-switch--{}", self.size.class_suffix())) {
                input
                    class="sh-switch__input"
                    type="checkbox"
                    name=(self.name)
                    id=(input_id)
                    checked?[self.checked]
                    disabled?[self.disabled]
                    required?[self.required]
                {};
                span class="sh-switch__track" {
                    span class="sh-switch__thumb" {}
                }
                @if let Some(label) = self.label {
                    span class="sh-switch__label" { (label) }
                }
            }
        }
    }
}

/// Select/Option component
pub struct SelectOption<'a> {
    pub label: &'a str,
    pub value: &'a str,
    pub selected: bool,
    pub disabled: bool,
}

pub struct Select<'a> {
    label: Option<&'a str>,
    name: &'a str,
    placeholder: Option<&'a str>,
    options: Vec<SelectOption<'a>>,
    required: bool,
    disabled: bool,
    error: Option<&'a str>,
    helper_text: Option<&'a str>,
    size: ComponentSize,
    multiple: bool,
}

impl<'a> Select<'a> {
    pub fn new(name: &'a str, options: Vec<SelectOption<'a>>) -> Self {
        Self {
            label: None,
            name,
            placeholder: None,
            options,
            required: false,
            disabled: false,
            error: None,
            helper_text: None,
            size: ComponentSize::Md,
            multiple: false,
        }
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }
    pub fn placeholder(mut self, p: &'a str) -> Self {
        self.placeholder = Some(p);
        self
    }
    pub fn required(mut self, r: bool) -> Self {
        self.required = r;
        self
    }
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }
    pub fn error(mut self, e: &'a str) -> Self {
        self.error = Some(e);
        self
    }
    pub fn helper_text(mut self, h: &'a str) -> Self {
        self.helper_text = Some(h);
        self
    }
    pub fn multiple(mut self, m: bool) -> Self {
        self.multiple = m;
        self
    }
}

impl<'a> Render for Select<'a> {
    fn render(&self) -> Markup {
        let has_error = self.error.is_some();
        let input_id = format!("sh-select-{}", self.name);

        html! {
            div class=(format!("sh-select-wrapper {}",
                if has_error { "sh-select-wrapper--error" } else { "" }
            )) {
                @if let Some(label) = self.label {
                    label class="sh-input__label" for=(input_id) {
                        (label)
                        @if self.required {
                            span class="sh-input__required" { " *" }
                        }
                    }
                }

                select
                    class=(format!("sh-select sh-select--{}", self.size.class_suffix()))
                    name=(self.name)
                    id=(input_id)
                    required?[self.required]
                    disabled?[self.disabled]
                    multiple?[self.multiple]
                {
                    @if let Some(placeholder) = self.placeholder {
                        option value="" disabled selected=(self.options.iter().all(|o| !o.selected)) {
                            (placeholder)
                        }
                    }
                    @for opt in &self.options {
                        option
                            value=(opt.value)
                            selected?[opt.selected]
                            disabled?[opt.disabled]
                        {
                            (opt.label)
                        }
                    }
                }

                @if let Some(error) = self.error {
                    div class="sh-input__error" { (error) }
                } @else if let Some(helper) = self.helper_text {
                    div class="sh-input__helper" { (helper) }
                }
            }
        }
    }
}

/// Generate CSS for input components
pub fn input_css() -> String {
    r#"
/* Input Base */
.sh-input-wrapper {
  display: flex;
  flex-direction: column;
  gap: 0.375rem;
  width: 100%;
}

.sh-input__label {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--sh-text);
}

.sh-input__required {
  color: var(--sh-error);
}

.sh-input__field-wrapper {
  position: relative;
  display: flex;
  align-items: center;
}

/* Input Element */
.sh-input {
  width: 100%;
  font-family: inherit;
  background: var(--sh-surface);
  border: 1px solid var(--sh-border);
  border-radius: var(--sh-radius-md);
  color: var(--sh-text);
  transition: all 0.2s ease;
}

.sh-input:focus {
  outline: none;
  border-color: var(--sh-accent);
  box-shadow: 0 0 0 3px color-mix(in srgb, var(--sh-accent) 15%, transparent);
}

.sh-input::placeholder {
  color: var(--sh-text-muted);
}

.sh-input:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  background: var(--sh-surface-2);
}

/* Size variants */
.sh-input--xs { padding: 0.375rem 0.5rem; font-size: 0.75rem; }
.sh-input--sm { padding: 0.5rem 0.625rem; font-size: 0.875rem; }
.sh-input--md { padding: 0.625rem 0.875rem; font-size: 1rem; }
.sh-input--lg { padding: 0.75rem 1rem; font-size: 1.125rem; }
.sh-input--xl { padding: 0.875rem 1.25rem; font-size: 1.25rem; }

/* Variant styles */
.sh-input-wrapper--filled .sh-input {
  background: var(--sh-surface-2);
  border-color: transparent;
}

.sh-input-wrapper--outlined .sh-input {
  background: transparent;
  border-width: 2px;
}

.sh-input-wrapper--flushed .sh-input {
  background: transparent;
  border-width: 0 0 2px 0;
  border-radius: 0;
  padding-left: 0;
  padding-right: 0;
}

/* Error state */
.sh-input-wrapper--error .sh-input {
  border-color: var(--sh-error);
}

.sh-input-wrapper--error .sh-input:focus {
  box-shadow: 0 0 0 3px color-mix(in srgb, var(--sh-error) 15%, transparent);
}

.sh-input__error {
  font-size: 0.75rem;
  color: var(--sh-error);
  margin-top: 0.25rem;
}

.sh-input__helper {
  font-size: 0.75rem;
  color: var(--sh-text-muted);
  margin-top: 0.25rem;
}

/* Icons */
.sh-input__icon {
  position: absolute;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--sh-text-muted);
  pointer-events: none;
}

.sh-input__icon--left {
  left: 0.75rem;
}

.sh-input__icon--right {
  right: 0.75rem;
}

.sh-input-wrapper--has-left-icon .sh-input {
  padding-left: 2.5rem;
}

.sh-input-wrapper--has-right-icon .sh-input {
  padding-right: 2.5rem;
}

/* Textarea */
.sh-textarea-wrapper {
  display: flex;
  flex-direction: column;
  gap: 0.375rem;
}

.sh-textarea {
  width: 100%;
  min-height: 80px;
  padding: 0.625rem 0.875rem;
  font-family: inherit;
  font-size: 1rem;
  line-height: 1.5;
  background: var(--sh-surface);
  border: 1px solid var(--sh-border);
  border-radius: var(--sh-radius-md);
  color: var(--sh-text);
  resize: vertical;
  transition: all 0.2s ease;
}

.sh-textarea:focus {
  outline: none;
  border-color: var(--sh-accent);
  box-shadow: 0 0 0 3px color-mix(in srgb, var(--sh-accent) 15%, transparent);
}

/* Checkbox */
.sh-checkbox {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
  user-select: none;
}

.sh-checkbox__input {
  position: absolute;
  opacity: 0;
  width: 0;
  height: 0;
}

.sh-checkbox__control {
  width: 1.125rem;
  height: 1.125rem;
  border: 2px solid var(--sh-border);
  border-radius: var(--sh-radius-sm);
  background: var(--sh-surface);
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  justify-content: center;
}

.sh-checkbox__input:checked + .sh-checkbox__control {
  background: var(--sh-accent);
  border-color: var(--sh-accent);
}

.sh-checkbox__input:checked + .sh-checkbox__control::after {
  content: "";
  width: 6px;
  height: 10px;
  border: solid white;
  border-width: 0 2px 2px 0;
  transform: rotate(45deg) translateY(-1px);
}

.sh-checkbox__input:focus + .sh-checkbox__control {
  box-shadow: 0 0 0 3px color-mix(in srgb, var(--sh-accent) 15%, transparent);
}

.sh-checkbox__label {
  font-size: 0.875rem;
  color: var(--sh-text);
}

/* Radio */
.sh-radio {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
  user-select: none;
}

.sh-radio__input {
  position: absolute;
  opacity: 0;
  width: 0;
  height: 0;
}

.sh-radio__control {
  width: 1.125rem;
  height: 1.125rem;
  border: 2px solid var(--sh-border);
  border-radius: 50%;
  background: var(--sh-surface);
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  justify-content: center;
}

.sh-radio__input:checked + .sh-radio__control {
  border-color: var(--sh-accent);
}

.sh-radio__input:checked + .sh-radio__control::after {
  content: "";
  width: 0.5rem;
  height: 0.5rem;
  background: var(--sh-accent);
  border-radius: 50%;
}

/* Switch */
.sh-switch {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
  user-select: none;
}

.sh-switch__input {
  position: absolute;
  opacity: 0;
  width: 0;
  height: 0;
}

.sh-switch__track {
  width: 2.75rem;
  height: 1.5rem;
  background: var(--sh-surface-2);
  border-radius: 9999px;
  position: relative;
  transition: background 0.2s ease;
  border: 1px solid var(--sh-border);
}

.sh-switch__thumb {
  position: absolute;
  top: 2px;
  left: 2px;
  width: 1.125rem;
  height: 1.125rem;
  background: white;
  border-radius: 50%;
  transition: transform 0.2s ease;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.sh-switch__input:checked + .sh-switch__track {
  background: var(--sh-accent);
  border-color: var(--sh-accent);
}

.sh-switch__input:checked + .sh-switch__track .sh-switch__thumb {
  transform: translateX(1.25rem);
}

.sh-switch__input:focus + .sh-switch__track {
  box-shadow: 0 0 0 3px color-mix(in srgb, var(--sh-accent) 15%, transparent);
}

.sh-switch__label {
  font-size: 0.875rem;
  color: var(--sh-text);
}

/* Select */
.sh-select-wrapper {
  display: flex;
  flex-direction: column;
  gap: 0.375rem;
}

.sh-select {
  width: 100%;
  padding: 0.625rem 2rem 0.625rem 0.875rem;
  font-family: inherit;
  font-size: 1rem;
  background: var(--sh-surface);
  border: 1px solid var(--sh-border);
  border-radius: var(--sh-radius-md);
  color: var(--sh-text);
  cursor: pointer;
  appearance: none;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='24' height='24' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpath d='m6 9 6 6 6-6'/%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 0.5rem center;
  background-size: 1.25rem;
  transition: all 0.2s ease;
}

.sh-select:focus {
  outline: none;
  border-color: var(--sh-accent);
  box-shadow: 0 0 0 3px color-mix(in srgb, var(--sh-accent) 15%, transparent);
}

.sh-select:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  background-color: var(--sh-surface-2);
}
"#.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_creation() {
        let input = Input::new("email")
            .type_(InputType::Email)
            .label("Email Address")
            .placeholder("Enter your email")
            .required(true);

        assert_eq!(input.name, "email");
        assert_eq!(input.type_, InputType::Email);
        assert!(input.required);
    }

    #[test]
    fn test_input_factory_methods() {
        let email = Input::email("contact");
        assert_eq!(email.type_, InputType::Email);

        let password = Input::password("pass");
        assert_eq!(password.type_, InputType::Password);

        let number = Input::number("age");
        assert_eq!(number.type_, InputType::Number);
    }

    #[test]
    fn test_input_classes() {
        let input = Input::new("test")
            .size(ComponentSize::Lg)
            .variant(InputVariant::Filled);

        let classes = input.build_wrapper_classes();
        assert!(classes.contains("sh-input-wrapper--lg"));
        assert!(classes.contains("sh-input-wrapper--filled"));
    }

    #[test]
    fn test_textarea() {
        let textarea = Textarea::new("description")
            .label("Description")
            .rows(6)
            .resize(TextareaResize::Vertical);

        assert_eq!(textarea.rows, 6);
        assert_eq!(textarea.resize, TextareaResize::Vertical);
    }

    #[test]
    fn test_checkbox() {
        let checkbox = Checkbox::new("Accept terms", "terms")
            .checked(true)
            .required(true);

        assert!(checkbox.checked);
        assert!(checkbox.required);
    }

    #[test]
    fn test_radio() {
        let radio = Radio::new("Option 1", "option", "1").checked(true);

        assert!(radio.checked);
        assert_eq!(radio.value, "1");
    }

    #[test]
    fn test_switch() {
        let switch = Switch::new("notifications")
            .label("Enable notifications")
            .checked(true);

        assert!(switch.checked);
        assert_eq!(switch.label, Some("Enable notifications"));
    }

    #[test]
    fn test_select() {
        let options = vec![
            SelectOption {
                label: "Option 1",
                value: "1",
                selected: true,
                disabled: false,
            },
            SelectOption {
                label: "Option 2",
                value: "2",
                selected: false,
                disabled: false,
            },
        ];

        let select = Select::new("choice", options)
            .label("Choose an option")
            .placeholder("Select...");

        assert_eq!(select.options.len(), 2);
        assert!(select.placeholder.is_some());
    }
}
