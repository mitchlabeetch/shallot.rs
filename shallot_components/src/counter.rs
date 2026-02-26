//! Counter Component
//!
//! Animated number counters and statistics display.

use crate::component::ComponentSize;
use maud::{html, Markup, Render};

pub struct Counter<'a> {
    value: f64,
    label: Option<&'a str>,
    prefix: Option<&'a str>,
    suffix: Option<&'a str>,
    size: ComponentSize,
    variant: CounterVariant,
    animated: bool,
    decimals: u8,
    separator: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CounterVariant {
    #[default]
    Default,
    Primary,
    Success,
    Warning,
    Error,
}

impl<'a> Counter<'a> {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            label: None,
            prefix: None,
            suffix: None,
            size: ComponentSize::Md,
            variant: CounterVariant::Default,
            animated: true,
            decimals: 0,
            separator: true,
        }
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn prefix(mut self, prefix: &'a str) -> Self {
        self.prefix = Some(prefix);
        self
    }

    pub fn suffix(mut self, suffix: &'a str) -> Self {
        self.suffix = Some(suffix);
        self
    }

    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: CounterVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    pub fn decimals(mut self, decimals: u8) -> Self {
        self.decimals = decimals;
        self
    }

    pub fn separator(mut self, separator: bool) -> Self {
        self.separator = separator;
        self
    }

    fn variant_class(&self) -> &'static str {
        match self.variant {
            CounterVariant::Default => "",
            CounterVariant::Primary => "sh-counter--primary",
            CounterVariant::Success => "sh-counter--success",
            CounterVariant::Warning => "sh-counter--warning",
            CounterVariant::Error => "sh-counter--error",
        }
    }

    fn format_value(&self) -> String {
        let formatted = format!("{:.1$}", self.value, self.decimals as usize);
        if self.separator {
            let parts: Vec<&str> = formatted.split('.').collect();
            if let Some(int_part) = parts.first() {
                let mut result = String::new();
                let chars: Vec<char> = int_part.chars().collect();
                for (i, c) in chars.iter().enumerate().rev() {
                    if i < chars.len() - 1 && (chars.len() - 1 - i) % 3 == 0 {
                        result.insert(0, ',');
                    }
                    result.insert(0, *c);
                }
                if parts.len() > 1 {
                    result.push('.');
                    result.push_str(parts[1]);
                }
                return result;
            }
        }
        formatted
    }
}

impl<'a> Render for Counter<'a> {
    fn render(&self) -> Markup {
        let size_class = format!("sh-counter--{}", self.size.class_suffix());
        let display_value = self.format_value();

        html! {
            div
                class={(format!("sh-counter {} {}", self.variant_class(), size_class))}
                role="group"
                aria-label=(self.label.unwrap_or("Counter"))
            {
                @if let Some(prefix) = self.prefix {
                    span class="sh-counter__prefix" aria-hidden="true" { (prefix) }
                }

                span
                    class="sh-counter__value"
                    data-value=(self.value)
                    data-animated=[if self.animated { Some("true") } else { None }]
                    aria-live="polite"
                    aria-atomic="true"
                {
                    (display_value)
                }

                @if let Some(suffix) = self.suffix {
                    span class="sh-counter__suffix" aria-hidden="true" { (suffix) }
                }

                @if let Some(label) = self.label {
                    span class="sh-counter__label" { (label) }
                }
            }
        }
    }
}

pub struct CounterGroup<'a> {
    counters: Vec<Counter<'a>>,
    layout: CounterLayout,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CounterLayout {
    #[default]
    Horizontal,
    Vertical,
    Grid,
}

impl<'a> CounterGroup<'a> {
    pub fn new(counters: Vec<Counter<'a>>) -> Self {
        Self {
            counters,
            layout: CounterLayout::Horizontal,
        }
    }

    pub fn layout(mut self, layout: CounterLayout) -> Self {
        self.layout = layout;
        self
    }
}

impl<'a> Render for CounterGroup<'a> {
    fn render(&self) -> Markup {
        let layout_class = match self.layout {
            CounterLayout::Horizontal => "sh-counter-group--horizontal",
            CounterLayout::Vertical => "sh-counter-group--vertical",
            CounterLayout::Grid => "sh-counter-group--grid",
        };

        html! {
            div class={(format!("sh-counter-group {}", layout_class))} {
                @for counter in &self.counters {
                    (counter)
                }
            }
        }
    }
}

pub struct Statistic<'a> {
    value: &'a str,
    label: &'a str,
    trend: Option<TrendData<'a>>,
    icon: Option<&'a str>,
}

pub struct TrendData<'a> {
    pub value: &'a str,
    pub direction: TrendDirection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrendDirection {
    Up,
    Down,
    Neutral,
}

impl<'a> Statistic<'a> {
    pub fn new(value: &'a str, label: &'a str) -> Self {
        Self {
            value,
            label,
            trend: None,
            icon: None,
        }
    }

    pub fn trend(mut self, value: &'a str, direction: TrendDirection) -> Self {
        self.trend = Some(TrendData { value, direction });
        self
    }

    pub fn icon(mut self, icon: &'a str) -> Self {
        self.icon = Some(icon);
        self
    }
}

impl<'a> Render for Statistic<'a> {
    fn render(&self) -> Markup {
        let trend_class = self.trend.as_ref().map(|t| match t.direction {
            TrendDirection::Up => "sh-statistic--up",
            TrendDirection::Down => "sh-statistic--down",
            TrendDirection::Neutral => "sh-statistic--neutral",
        });

        html! {
            div class={(format!("sh-statistic {}", trend_class.unwrap_or("")))} {
                @if let Some(icon) = self.icon {
                    span class="sh-statistic__icon" {
                        (maud::PreEscaped(icon))
                    }
                }

                div class="sh-statistic__content" {
                    span class="sh-statistic__value" { (self.value) }
                    span class="sh-statistic__label" { (self.label) }

                    @if let Some(trend) = &self.trend {
                        span class="sh-statistic__trend" {
                            @match trend.direction {
                                TrendDirection::Up => {
                                    svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                                        polyline points="18 15 12 9 6 15";
                                    }
                                }
                                TrendDirection::Down => {
                                    svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                                        polyline points="6 9 12 15 18 9";
                                    }
                                }
                                TrendDirection::Neutral => {
                                    svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                                        line x1="5" y1="12" x2="19" y2="12";
                                    }
                                }
                            }
                            (trend.value)
                        }
                    }
                }
            }
        }
    }
}

pub fn counter_css() -> String {
    r#"
.sh-counter {
    display: inline-flex;
    flex-direction: column;
    align-items: center;
    gap: 0.25rem;
}

.sh-counter__value {
    display: flex;
    align-items: baseline;
    font-variant-numeric: tabular-nums;
    font-weight: 700;
    color: var(--sh-text, #1f2937);
    line-height: 1;
}

.sh-counter__prefix,
.sh-counter__suffix {
    font-weight: 500;
    opacity: 0.8;
}

.sh-counter__label {
    font-size: 0.75rem;
    color: var(--sh-text-muted, #6b7280);
    text-transform: uppercase;
    letter-spacing: 0.05em;
}

/* Size variants */
.sh-counter--xs .sh-counter__value { font-size: 1.25rem; }
.sh-counter--sm .sh-counter__value { font-size: 1.5rem; }
.sh-counter--md .sh-counter__value { font-size: 2rem; }
.sh-counter--lg .sh-counter__value { font-size: 2.5rem; }
.sh-counter--xl .sh-counter__value { font-size: 3rem; }

/* Color variants */
.sh-counter--primary .sh-counter__value { color: var(--sh-accent, #3b82f6); }
.sh-counter--success .sh-counter__value { color: var(--sh-success, #10b981); }
.sh-counter--warning .sh-counter__value { color: var(--sh-warning, #f59e0b); }
.sh-counter--error .sh-counter__value { color: var(--sh-error, #ef4444); }

/* Counter group */
.sh-counter-group {
    display: flex;
    gap: 2rem;
}

.sh-counter-group--vertical {
    flex-direction: column;
    gap: 1rem;
}

.sh-counter-group--grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    gap: 1.5rem;
}

/* Statistic */
.sh-statistic {
    display: flex;
    align-items: flex-start;
    gap: 1rem;
    padding: 1rem;
    background: var(--sh-surface, #fff);
    border-radius: var(--sh-radius-md, 0.5rem);
    border: 1px solid var(--sh-border, #e5e7eb);
}

.sh-statistic__icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 2.5rem;
    height: 2.5rem;
    color: var(--sh-accent, #3b82f6);
    background: color-mix(in srgb, var(--sh-accent, #3b82f6) 10%, transparent);
    border-radius: var(--sh-radius-md, 0.5rem);
}

.sh-statistic__content {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
}

.sh-statistic__value {
    font-size: 1.5rem;
    font-weight: 700;
    color: var(--sh-text, #1f2937);
    font-variant-numeric: tabular-nums;
}

.sh-statistic__label {
    font-size: 0.75rem;
    color: var(--sh-text-muted, #6b7280);
}

.sh-statistic__trend {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    font-size: 0.75rem;
    font-weight: 500;
}

.sh-statistic--up .sh-statistic__trend {
    color: var(--sh-success, #10b981);
}

.sh-statistic--down .sh-statistic__trend {
    color: var(--sh-error, #ef4444);
}

.sh-statistic--neutral .sh-statistic__trend {
    color: var(--sh-text-muted, #6b7280);
}

/* Animation */
.sh-counter__value[data-animated="true"] {
    animation: sh-counter-fade-in 0.5s ease-out;
}

@keyframes sh-counter-fade-in {
    from {
        opacity: 0;
        transform: translateY(10px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_creation() {
        let counter = Counter::new(12345.67)
            .label("Total Users")
            .suffix("+")
            .decimals(1);

        assert_eq!(counter.value, 12345.67);
        assert_eq!(counter.suffix, Some("+"));
        assert_eq!(counter.decimals, 1);
    }

    #[test]
    fn test_counter_formatting() {
        let counter = Counter::new(1234567.0).separator(true);
        let formatted = counter.format_value();
        assert!(formatted.contains(","));
    }

    #[test]
    fn test_statistic() {
        let stat = Statistic::new("$12,345", "Revenue").trend("+12%", TrendDirection::Up);

        assert_eq!(stat.value, "$12,345");
        assert!(stat.trend.is_some());
    }
}
