//! Calendar Component - Date picker calendar widget
//!
//! Provides a monthly calendar view with navigation, day selection,
//! and multiple display variants.

use maud::{html, Markup, Render};

/// Calendar variant for different visual styles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CalendarVariant {
    /// Standard calendar with border
    #[default]
    Default,
    /// Borderless calendar for embedded use
    Borderless,
    /// Elevated calendar with shadow
    Elevated,
    /// Compact calendar for small spaces
    Compact,
}

/// Calendar size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CalendarSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Calendar Component
#[derive(Debug, Clone)]
pub struct Calendar<'a> {
    pub month: &'a str,
    pub year: u32,
    pub days: Vec<Option<u8>>,
    pub selected: Option<u8>,
    pub today: Option<u8>,
    pub variant: CalendarVariant,
    pub size: CalendarSize,
    pub on_select: Option<&'a str>,
}

impl<'a> Default for Calendar<'a> {
    fn default() -> Self {
        Self {
            month: "January",
            year: 2024,
            days: vec![None; 35],
            selected: None,
            today: None,
            variant: CalendarVariant::Default,
            size: CalendarSize::Md,
            on_select: None,
        }
    }
}

impl<'a> Calendar<'a> {
    /// Create a new calendar for a specific month/year
    pub fn new(month: &'a str, year: u32) -> Self {
        Self {
            month,
            year,
            ..Default::default()
        }
    }

    /// Set the days to display
    pub fn days(mut self, days: Vec<Option<u8>>) -> Self {
        self.days = days;
        self
    }

    /// Set the selected day
    pub fn selected(mut self, day: Option<u8>) -> Self {
        self.selected = day;
        self
    }

    /// Set today's date
    pub fn today(mut self, day: u8) -> Self {
        self.today = Some(day);
        self
    }

    /// Set the calendar variant
    pub fn variant(mut self, variant: CalendarVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the calendar size
    pub fn size(mut self, size: CalendarSize) -> Self {
        self.size = size;
        self
    }

    /// Set the onSelect handler (for JavaScript interactivity)
    pub fn on_select(mut self, handler: &'a str) -> Self {
        self.on_select = Some(handler);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-calendar"];

        match self.variant {
            CalendarVariant::Default => classes.push("sh-calendar--default"),
            CalendarVariant::Borderless => classes.push("sh-calendar--borderless"),
            CalendarVariant::Elevated => classes.push("sh-calendar--elevated"),
            CalendarVariant::Compact => classes.push("sh-calendar--compact"),
        }

        match self.size {
            CalendarSize::Sm => classes.push("sh-calendar--sm"),
            CalendarSize::Md => classes.push("sh-calendar--md"),
            CalendarSize::Lg => classes.push("sh-calendar--lg"),
        }

        classes.join(" ")
    }
}

impl<'a> Render for Calendar<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let week_days = vec!["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

        html! {
            div
                class=(classes)
                role="application"
                aria-label=(format!("{} {}", self.month, self.year))
            {
                div class="sh-calendar__header" {
                    span class="sh-calendar__title" { (self.month) " " (self.year) }
                    div class="sh-calendar__nav" {
                        button
                            class="sh-calendar__nav-btn"
                            type="button"
                            aria-label="Previous month"
                        { "←" }
                        button
                            class="sh-calendar__nav-btn"
                            type="button"
                            aria-label="Next month"
                        { "→" }
                    }
                }
                div class="sh-calendar__grid" role="grid" {
                    @for day in &week_days {
                        div class="sh-calendar__weekday" role="columnheader" { (day) }
                    }
                    @for day in &self.days {
                        @if let Some(d) = day {
                            @let is_sel = self.selected == Some(*d);
                            @let is_today = self.today == Some(*d);
                            button
                                class={
                                    "sh-calendar__day"
                                    @if is_sel { " sh-calendar__day--selected" }
                                    @if is_today { " sh-calendar__day--today" }
                                }
                                type="button"
                                disabled
                            { (d) }
                        } @else {
                            div class="sh-calendar__day sh-calendar__day--empty" {}
                        }
                    }
                }
            }
        }
    }
}

/// Generate CSS for calendar components
pub fn calendar_css() -> String {
    r#"
/* Calendar Component Styles */
.sh-calendar {
    --calendar-bg: var(--sh-surface, #fff);
    --calendar-border: var(--sh-border, #e5e7eb);
    --calendar-text: var(--sh-text, #1f2937);
    --calendar-accent: var(--sh-accent, #3b82f6);
    --calendar-hover: var(--sh-hover, #f3f4f6);
    --calendar-today-bg: var(--sh-accent-light, #dbeafe);
    
    font-family: inherit;
    background: var(--calendar-bg);
    border-radius: 0.5rem;
    padding: 1rem;
    user-select: none;
}

/* Variants */
.sh-calendar--default {
    border: 1px solid var(--calendar-border);
}

.sh-calendar--borderless {
    border: none;
    padding: 0;
}

.sh-calendar--elevated {
    border: 1px solid var(--calendar-border);
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -2px rgba(0, 0, 0, 0.1);
}

.sh-calendar--compact {
    padding: 0.5rem;
}

/* Sizes */
.sh-calendar--sm {
    font-size: 0.75rem;
    padding: 0.5rem;
}

.sh-calendar--sm .sh-calendar__day {
    width: 1.5rem;
    height: 1.5rem;
    font-size: 0.625rem;
}

.sh-calendar--md {
    font-size: 0.875rem;
}

.sh-calendar--md .sh-calendar__day {
    width: 2rem;
    height: 2rem;
}

.sh-calendar--lg {
    font-size: 1rem;
    padding: 1.5rem;
}

.sh-calendar--lg .sh-calendar__day {
    width: 2.5rem;
    height: 2.5rem;
    font-size: 1rem;
}

/* Header */
.sh-calendar__header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
}

.sh-calendar__title {
    font-weight: 600;
    color: var(--calendar-text);
}

.sh-calendar__nav {
    display: flex;
    gap: 0.25rem;
}

.sh-calendar__nav-btn {
    background: transparent;
    border: none;
    cursor: pointer;
    padding: 0.25rem 0.5rem;
    border-radius: 0.25rem;
    color: var(--calendar-text);
    transition: background-color 0.15s ease;
}

.sh-calendar__nav-btn:hover {
    background: var(--calendar-hover);
}

/* Grid */
.sh-calendar__grid {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 0.25rem;
}

/* Weekday headers */
.sh-calendar__weekday {
    text-align: center;
    font-weight: 500;
    color: var(--sh-text-secondary, #6b7280);
    font-size: 0.75rem;
    padding: 0.25rem;
}

/* Day cells */
.sh-calendar__day {
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    border-radius: 0.25rem;
    cursor: pointer;
    color: var(--calendar-text);
    transition: all 0.15s ease;
    aspect-ratio: 1;
    width: 100%;
}

.sh-calendar__day:not(:disabled):hover {
    background: var(--calendar-hover);
}

.sh-calendar__day--empty {
    visibility: hidden;
    pointer-events: none;
}

.sh-calendar__day--today {
    background: var(--calendar-today-bg);
    font-weight: 600;
}

.sh-calendar__day--selected {
    background: var(--calendar-accent);
    color: white;
    font-weight: 600;
}

.sh-calendar__day--selected:hover {
    background: var(--calendar-accent);
}

/* Reduced motion */
@media (prefers-reduced-motion: reduce) {
    .sh-calendar__day,
    .sh-calendar__nav-btn {
        transition: none;
    }
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_calendar_days() -> Vec<Option<u8>> {
        vec![
            None,
            None,
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
            Some(8),
            Some(9),
            Some(10),
            Some(11),
            Some(12),
            Some(13),
            Some(14),
            Some(15),
            Some(16),
            Some(17),
            Some(18),
            Some(19),
            Some(20),
            Some(21),
            Some(22),
            Some(23),
            Some(24),
            Some(25),
            Some(26),
            Some(27),
            Some(28),
            Some(29),
            Some(30),
            None,
            None,
            None,
        ]
    }

    #[test]
    fn test_calendar_creation() {
        let calendar = Calendar::new("January", 2024);
        assert_eq!(calendar.month, "January");
        assert_eq!(calendar.year, 2024);
    }

    #[test]
    fn test_calendar_days() {
        let days = sample_calendar_days();
        let calendar = Calendar::new("January", 2024).days(days.clone());
        assert_eq!(calendar.days.len(), 35);
    }

    #[test]
    fn test_calendar_selected() {
        let calendar = Calendar::new("January", 2024).selected(Some(15));
        assert_eq!(calendar.selected, Some(15));
    }

    #[test]
    fn test_calendar_today() {
        let calendar = Calendar::new("January", 2024).today(20);
        assert_eq!(calendar.today, Some(20));
    }

    #[test]
    fn test_calendar_variants() {
        let default = Calendar::new("January", 2024).variant(CalendarVariant::Default);
        assert_eq!(default.variant, CalendarVariant::Default);

        let borderless = Calendar::new("January", 2024).variant(CalendarVariant::Borderless);
        assert_eq!(borderless.variant, CalendarVariant::Borderless);

        let elevated = Calendar::new("January", 2024).variant(CalendarVariant::Elevated);
        assert_eq!(elevated.variant, CalendarVariant::Elevated);

        let compact = Calendar::new("January", 2024).variant(CalendarVariant::Compact);
        assert_eq!(compact.variant, CalendarVariant::Compact);
    }

    #[test]
    fn test_calendar_sizes() {
        let sm = Calendar::new("January", 2024).size(CalendarSize::Sm);
        assert_eq!(sm.size, CalendarSize::Sm);

        let md = Calendar::new("January", 2024).size(CalendarSize::Md);
        assert_eq!(md.size, CalendarSize::Md);

        let lg = Calendar::new("January", 2024).size(CalendarSize::Lg);
        assert_eq!(lg.size, CalendarSize::Lg);
    }

    #[test]
    fn test_calendar_on_select() {
        let calendar = Calendar::new("January", 2024).on_select("selectDate(day)");
        assert_eq!(calendar.on_select, Some("selectDate(day)"));
    }

    #[test]
    fn test_calendar_render() {
        let calendar = Calendar::new("January", 2024)
            .days(sample_calendar_days())
            .selected(Some(15))
            .today(10);

        let rendered = calendar.render();
        let html_str = rendered.0.as_str();

        assert!(html_str.contains("sh-calendar"));
        assert!(html_str.contains("January"));
        assert!(html_str.contains("2024"));
    }

    #[test]
    fn test_calendar_aria_label() {
        let calendar = Calendar::new("December", 2023);
        let rendered = calendar.render();
        let html_str = rendered.0.as_str();

        assert!(html_str.contains("December 2023"));
    }

    #[test]
    fn test_calendar_css() {
        let css = calendar_css();
        assert!(css.contains(".sh-calendar"));
        assert!(css.contains(".sh-calendar--default"));
        assert!(css.contains(".sh-calendar--borderless"));
        assert!(css.contains(".sh-calendar--elevated"));
        assert!(css.contains(".sh-calendar--compact"));
        assert!(css.contains(".sh-calendar--sm"));
        assert!(css.contains(".sh-calendar--md"));
        assert!(css.contains(".sh-calendar--lg"));
        assert!(css.contains(".sh-calendar__day--selected"));
        assert!(css.contains(".sh-calendar__day--today"));
    }
}
