//! Date Range Picker Component - Select a range of dates
//! CSS-only styling with native date inputs

use maud::{html, Markup, Render};

/// Date range picker size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DateRangeSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Date range picker variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DateRangeVariant {
    #[default]
    Default,
    Inline,
    Split,
}

/// Date range picker component
#[derive(Debug, Clone)]
pub struct DateRangePicker<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub start_date: Option<&'a str>,
    pub end_date: Option<&'a str>,
    pub min_date: Option<&'a str>,
    pub max_date: Option<&'a str>,
    pub size: DateRangeSize,
    pub variant: DateRangeVariant,
    pub label: Option<&'a str>,
    pub start_label: Option<&'a str>,
    pub end_label: Option<&'a str>,
    pub disabled: bool,
    pub required: bool,
}

impl<'a> DateRangePicker<'a> {
    /// Create a new date range picker
    pub fn new(id: &'a str, name: &'a str) -> Self {
        Self {
            id,
            name,
            start_date: None,
            end_date: None,
            min_date: None,
            max_date: None,
            size: DateRangeSize::default(),
            variant: DateRangeVariant::default(),
            label: None,
            start_label: None,
            end_label: None,
            disabled: false,
            required: false,
        }
    }

    /// Set the start date
    pub fn start_date(mut self, date: &'a str) -> Self {
        self.start_date = Some(date);
        self
    }

    /// Set the end date
    pub fn end_date(mut self, date: &'a str) -> Self {
        self.end_date = Some(date);
        self
    }

    /// Set the minimum allowed date
    pub fn min_date(mut self, date: &'a str) -> Self {
        self.min_date = Some(date);
        self
    }

    /// Set the maximum allowed date
    pub fn max_date(mut self, date: &'a str) -> Self {
        self.max_date = Some(date);
        self
    }

    /// Set the size
    pub fn size(mut self, size: DateRangeSize) -> Self {
        self.size = size;
        self
    }

    /// Set the variant
    pub fn variant(mut self, variant: DateRangeVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the label
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    /// Set the start date input label
    pub fn start_label(mut self, label: &'a str) -> Self {
        self.start_label = Some(label);
        self
    }

    /// Set the end date input label
    pub fn end_label(mut self, label: &'a str) -> Self {
        self.end_label = Some(label);
        self
    }

    /// Set the disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the required state
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-date-range".to_string()];

        let size_class = match self.size {
            DateRangeSize::Sm => "sh-date-range--sm",
            DateRangeSize::Md => "sh-date-range--md",
            DateRangeSize::Lg => "sh-date-range--lg",
        };
        classes.push(size_class.to_string());

        let variant_class = match self.variant {
            DateRangeVariant::Default => "sh-date-range--default",
            DateRangeVariant::Inline => "sh-date-range--inline",
            DateRangeVariant::Split => "sh-date-range--split",
        };
        classes.push(variant_class.to_string());

        if self.disabled {
            classes.push("sh-date-range--disabled".to_string());
        }

        classes.join(" ")
    }
}

impl<'a> Render for DateRangePicker<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let start_label = self.start_label.unwrap_or("Start date");
        let end_label = self.end_label.unwrap_or("End date");
        let start_id = format!("{}-start", self.id);
        let end_id = format!("{}-end", self.id);

        html! {
            fieldset class=(classes) {
                @if let Some(label) = self.label {
                    legend class="sh-date-range__label" {
                        (label)
                        @if self.required {
                            span class="sh-date-range__required" { "*" }
                        }
                    }
                }
                div class="sh-date-range__inputs" {
                    div class="sh-date-range__field" {
                        label class="sh-date-range__field-label" for=(start_id) {
                            (start_label)
                        }
                        input
                            type="date"
                            id=(start_id)
                            name={ (self.name) "-start" }
                            class="sh-date-range__input"
                            value?=(self.start_date)
                            min?=(self.min_date)
                            max?=(self.max_date)
                            disabled?=(self.disabled)
                            required?=(self.required)
                            aria-label=(start_label);
                    }
                    span class="sh-date-range__separator" {
                        "to"
                    }
                    div class="sh-date-range__field" {
                        label class="sh-date-range__field-label" for=(end_id) {
                            (end_label)
                        }
                        input
                            type="date"
                            id=(end_id)
                            name={ (self.name) "-end" }
                            class="sh-date-range__input"
                            value?=(self.end_date)
                            min?=(self.min_date)
                            max?=(self.max_date)
                            disabled?=(self.disabled)
                            required?=(self.required)
                            aria-label=(end_label);
                    }
                }
            }
        }
    }
}

/// Generate CSS for date range picker component
pub fn date_range_picker_css() -> String {
    r#"
.sh-date-range {
    display: flex;
    flex-direction: column;
    gap: var(--sh-spacing-2, 0.5rem);
    border: none;
    padding: 0;
    margin: 0;
}

.sh-date-range__label {
    font-size: var(--sh-font-size-sm, 0.875rem);
    font-weight: var(--sh-font-weight-medium, 500);
    color: var(--sh-color-foreground, #1f2937);
    padding: 0;
}

.sh-date-range__required {
    color: var(--sh-color-danger, #ef4444);
    margin-left: var(--sh-spacing-1, 0.25rem);
}

.sh-date-range__inputs {
    display: flex;
    align-items: flex-end;
    gap: var(--sh-spacing-2, 0.5rem);
}

.sh-date-range--inline .sh-date-range__inputs {
    flex-wrap: nowrap;
}

.sh-date-range--split .sh-date-range__inputs {
    flex-direction: column;
    align-items: stretch;
}

.sh-date-range--split .sh-date-range__separator {
    align-self: center;
}

.sh-date-range__field {
    display: flex;
    flex-direction: column;
    gap: var(--sh-spacing-1, 0.25rem);
    flex: 1;
    min-width: 0;
}

.sh-date-range__field-label {
    font-size: var(--sh-font-size-xs, 0.75rem);
    color: var(--sh-color-muted-foreground, #6b7280);
}

.sh-date-range__separator {
    color: var(--sh-color-muted-foreground, #6b7280);
    font-size: var(--sh-font-size-sm, 0.875rem);
    padding-bottom: var(--sh-spacing-2, 0.5rem);
}

.sh-date-range__input {
    width: 100%;
    padding: var(--sh-spacing-2, 0.5rem) var(--sh-spacing-3, 0.75rem);
    border: 1px solid var(--sh-color-border, #e5e7eb);
    border-radius: var(--sh-radius-md, 0.375rem);
    background-color: var(--sh-color-background, #ffffff);
    color: var(--sh-color-foreground, #1f2937);
    font-size: var(--sh-font-size-sm, 0.875rem);
    transition: border-color 0.15s ease, box-shadow 0.15s ease;
}

.sh-date-range--sm .sh-date-range__input {
    padding: var(--sh-spacing-1, 0.25rem) var(--sh-spacing-2, 0.5rem);
    font-size: var(--sh-font-size-xs, 0.75rem);
}

.sh-date-range--lg .sh-date-range__input {
    padding: var(--sh-spacing-3, 0.75rem) var(--sh-spacing-4, 1rem);
    font-size: var(--sh-font-size-base, 1rem);
}

.sh-date-range__input:hover:not(:disabled) {
    border-color: var(--sh-color-primary, #3b82f6);
}

.sh-date-range__input:focus {
    outline: none;
    border-color: var(--sh-color-primary, #3b82f6);
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.sh-date-range--disabled .sh-date-range__input,
.sh-date-range__input:disabled {
    background-color: var(--sh-color-muted, #f3f4f6);
    cursor: not-allowed;
    opacity: 0.7;
}

.sh-date-range__input::-webkit-calendar-picker-indicator {
    cursor: pointer;
    opacity: 0.6;
    transition: opacity 0.15s ease;
}

.sh-date-range__input::-webkit-calendar-picker-indicator:hover {
    opacity: 1;
}

.sh-date-range--disabled .sh-date-range__input::-webkit-calendar-picker-indicator {
    cursor: not-allowed;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_range_picker_creation() {
        let picker = DateRangePicker::new("dates", "date-range");

        assert_eq!(picker.id, "dates");
        assert_eq!(picker.name, "date-range");
        assert_eq!(picker.size, DateRangeSize::Md);
    }

    #[test]
    fn test_date_range_picker_render() {
        let picker = DateRangePicker::new("dates", "date-range")
            .label("Select date range")
            .start_date("2024-01-01")
            .end_date("2024-01-31");

        let html = picker.render().into_string();

        assert!(html.contains("sh-date-range"));
        assert!(html.contains("Select date range"));
        assert!(html.contains("2024-01-01"));
        assert!(html.contains("2024-01-31"));
    }

    #[test]
    fn test_date_range_picker_sizes() {
        let sm = DateRangePicker::new("dates", "range").size(DateRangeSize::Sm);
        let md = DateRangePicker::new("dates", "range").size(DateRangeSize::Md);
        let lg = DateRangePicker::new("dates", "range").size(DateRangeSize::Lg);

        assert!(sm.render().into_string().contains("sh-date-range--sm"));
        assert!(md.render().into_string().contains("sh-date-range--md"));
        assert!(lg.render().into_string().contains("sh-date-range--lg"));
    }

    #[test]
    fn test_date_range_picker_variants() {
        let default = DateRangePicker::new("dates", "range").variant(DateRangeVariant::Default);
        let inline = DateRangePicker::new("dates", "range").variant(DateRangeVariant::Inline);
        let split = DateRangePicker::new("dates", "range").variant(DateRangeVariant::Split);

        assert!(default
            .render()
            .into_string()
            .contains("sh-date-range--default"));
        assert!(inline
            .render()
            .into_string()
            .contains("sh-date-range--inline"));
        assert!(split
            .render()
            .into_string()
            .contains("sh-date-range--split"));
    }

    #[test]
    fn test_date_range_picker_disabled() {
        let picker = DateRangePicker::new("dates", "range").disabled(true);
        let html = picker.render().into_string();

        assert!(html.contains("sh-date-range--disabled"));
        assert!(html.contains("disabled"));
    }

    #[test]
    fn test_date_range_picker_required() {
        let picker = DateRangePicker::new("dates", "range")
            .label("Dates")
            .required(true);
        let html = picker.render().into_string();

        assert!(html.contains("required"));
        assert!(html.contains("*"));
    }

    #[test]
    fn test_date_range_picker_css() {
        let css = date_range_picker_css();
        assert!(css.contains(".sh-date-range"));
        assert!(css.contains(".sh-date-range__input"));
    }
}
