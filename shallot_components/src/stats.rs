//! Stats Component
//!
//! Display statistics, metrics, and key performance indicators.
//! Inspired by DaisyUI's stat component with rich variations.

use crate::animated_text::NumberTicker;
use crate::component::{Component, ComponentColor};
use maud::{html, Markup, Render};
use shallot_foundation::Icon;

/// Single stat item
pub struct Stat<'a> {
    /// Stat title/label
    title: &'a str,
    /// Stat value
    value: Markup,
    /// Stat description
    description: Option<&'a str>,
    /// Stat figure/icon
    figure: Option<Markup>,
    /// Trend indicator
    trend: Option<StatTrend>,
    /// Color theme
    color: Option<ComponentColor>,
    /// Custom class
    custom_class: Option<&'a str>,
}

/// Trend indicator for stats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatTrend {
    pub direction: TrendDirection,
    pub value: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrendDirection {
    Up,
    Down,
    Neutral,
}

impl<'a> Stat<'a> {
    /// Create a new stat
    pub fn new(title: &'a str, value: Markup) -> Self {
        Self {
            title,
            value,
            description: None,
            figure: None,
            trend: None,
            color: None,
            custom_class: None,
        }
    }

    /// Create a stat with a numeric value
    pub fn numeric(title: &'a str, value: f64) -> Self {
        let ticker = NumberTicker::new(value).render();
        Self::new(title, ticker)
    }

    /// Set description
    pub fn description(mut self, desc: &'a str) -> Self {
        self.description = Some(desc);
        self
    }

    /// Set figure/icon
    pub fn figure(mut self, figure: Markup) -> Self {
        self.figure = Some(figure);
        self
    }

    /// Set icon as figure
    pub fn icon(mut self, icon: Icon) -> Self {
        let svg = icon.to_svg_string();
        self.figure = Some(html! {
            div class="sh-stat__icon" {
                (maud::PreEscaped(svg))
            }
        });
        self
    }

    /// Set trend indicator
    pub fn trend(mut self, direction: TrendDirection, value: &'static str) -> Self {
        self.trend = Some(StatTrend { direction, value });
        self
    }

    /// Set color
    pub fn color(mut self, color: ComponentColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Add custom class
    pub fn custom_class(mut self, class: &'a str) -> Self {
        self.custom_class = Some(class);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-stat".to_string()];

        if let Some(color) = &self.color {
            classes.push(format!("sh-stat--{}", color.class_suffix()));
        }

        if let Some(custom) = self.custom_class {
            classes.push(custom.to_string());
        }

        classes.join(" ")
    }
}

impl<'a> Render for Stat<'a> {
    fn render(&self) -> Markup {
        let class = self.build_classes();

        html! {
            div
                class=(class)
                role="group"
                aria-label=(format!("Statistic: {}", self.title))
            {
                @if let Some(figure) = &self.figure {
                    div class="sh-stat__figure" aria-hidden="true" { (figure.clone()) }
                }

                div class="sh-stat__title" { (self.title) }
                div class="sh-stat__value" aria-live="polite" { (self.value.clone()) }

                @if let Some(trend) = &self.trend {
                    div class=(format!("sh-stat__trend sh-stat__trend--{}", match trend.direction {
                        TrendDirection::Up => "up",
                        TrendDirection::Down => "down",
                        TrendDirection::Neutral => "neutral",
                    })) {
                        span class="sh-stat__trend-arrow" aria-hidden="true" {
                            @match trend.direction {
                                TrendDirection::Up => "↑",
                                TrendDirection::Down => "↓",
                                TrendDirection::Neutral => "→",
                            }
                        }
                        span { (trend.value) }
                    }
                }

                @if let Some(desc) = self.description {
                    div class="sh-stat__desc" { (desc) }
                }
            }
        }
    }
}

impl<'a> Component for Stat<'a> {
    fn classes(&self) -> String {
        self.build_classes()
    }
}

/// Stats group/container
pub struct Stats<'a> {
    stats: Vec<Stat<'a>>,
    layout: StatsLayout,
    bordered: bool,
    shadow: bool,
    bg_color: Option<String>,
}

/// Stats layout options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StatsLayout {
    #[default]
    Horizontal,
    Vertical,
    Grid,
}

impl<'a> Stats<'a> {
    /// Create a new stats container
    pub fn new(stats: Vec<Stat<'a>>) -> Self {
        Self {
            stats,
            layout: StatsLayout::Horizontal,
            bordered: true,
            shadow: false,
            bg_color: None,
        }
    }

    /// Set layout
    pub fn layout(mut self, layout: StatsLayout) -> Self {
        self.layout = layout;
        self
    }

    /// Set bordered style
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// Set shadow
    pub fn shadow(mut self, shadow: bool) -> Self {
        self.shadow = shadow;
        self
    }

    /// Set background color
    pub fn bg_color(mut self, color: impl Into<String>) -> Self {
        self.bg_color = Some(color.into());
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-stats".to_string()];

        classes.push(format!(
            "sh-stats--{}",
            match self.layout {
                StatsLayout::Horizontal => "horizontal",
                StatsLayout::Vertical => "vertical",
                StatsLayout::Grid => "grid",
            }
        ));

        if self.bordered {
            classes.push("sh-stats--bordered".to_string());
        }

        if self.shadow {
            classes.push("sh-stats--shadow".to_string());
        }

        classes.join(" ")
    }

    fn build_style(&self) -> String {
        if let Some(bg) = &self.bg_color {
            format!("background: {};", bg)
        } else {
            String::new()
        }
    }
}

impl<'a> Render for Stats<'a> {
    fn render(&self) -> Markup {
        let class = self.build_classes();
        let style = self.build_style();

        html! {
            div class=(class) style=(style) {
                @for stat in &self.stats {
                    (stat.render())
                }
            }
        }
    }
}

/// Simple metric card for dashboards
pub struct MetricCard<'a> {
    title: &'a str,
    value: Markup,
    change: Option<&'a str>,
    change_positive: Option<bool>,
    icon: Option<Icon>,
    color: ComponentColor,
}

impl<'a> MetricCard<'a> {
    pub fn new(title: &'a str, value: Markup) -> Self {
        Self {
            title,
            value,
            change: None,
            change_positive: None,
            icon: None,
            color: ComponentColor::Primary,
        }
    }

    pub fn change(mut self, change: &'a str, positive: bool) -> Self {
        self.change = Some(change);
        self.change_positive = Some(positive);
        self
    }

    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn color(mut self, color: ComponentColor) -> Self {
        self.color = color;
        self
    }
}

impl<'a> Render for MetricCard<'a> {
    fn render(&self) -> Markup {
        let class = format!(
            "sh-metric-card sh-metric-card--{}",
            self.color.class_suffix()
        );

        html! {
            div class=(class) {
                div class="sh-metric-card__header" {
                    span class="sh-metric-card__title" { (self.title) }
                    @if let Some(icon) = &self.icon {
                        div class="sh-metric-card__icon" {
                            (maud::PreEscaped(icon.to_svg_string()))
                        }
                    }
                }
                div class="sh-metric-card__value" { (self.value.clone()) }
                @if let Some(change) = self.change {
                    @if let Some(positive) = self.change_positive {
                        div class=(format!("sh-metric-card__change {}",
                            if positive { "sh-metric-card__change--positive" } else { "sh-metric-card__change--negative" }
                        )) {
                            span { (if positive { "↑" } else { "↓" }) " " (change) }
                        }
                    }
                }
            }
        }
    }
}

/// Generate CSS for stats components
pub fn stats_css() -> String {
    r#"
/* Stat Base */
.sh-stat {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  padding: 1.5rem;
}

.sh-stat__figure {
  margin-bottom: 0.5rem;
}

.sh-stat__icon {
  display: inline-flex;
  padding: 0.75rem;
  background: color-mix(in srgb, var(--sh-accent) 10%, transparent);
  border-radius: var(--sh-radius-md);
  color: var(--sh-accent);
}

.sh-stat__icon svg {
  width: 1.5rem;
  height: 1.5rem;
}

.sh-stat__title {
  font-size: 0.875rem;
  color: var(--sh-text-muted);
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.sh-stat__value {
  font-size: 2.5rem;
  font-weight: 700;
  color: var(--sh-text);
  line-height: 1;
}

.sh-stat__trend {
  display: inline-flex;
  align-items: center;
  gap: 0.25rem;
  font-size: 0.875rem;
  font-weight: 500;
}

.sh-stat__trend--up {
  color: var(--sh-success);
}

.sh-stat__trend--down {
  color: var(--sh-error);
}

.sh-stat__trend--neutral {
  color: var(--sh-text-muted);
}

.sh-stat__trend-arrow {
  font-size: 1rem;
}

.sh-stat__desc {
  font-size: 0.875rem;
  color: var(--sh-text-muted);
}

/* Color variants */
.sh-stat--primary .sh-stat__value { color: var(--sh-accent); }
.sh-stat--success .sh-stat__value { color: var(--sh-success); }
.sh-stat--warning .sh-stat__value { color: var(--sh-warning); }
.sh-stat--error .sh-stat__value { color: var(--sh-error); }

/* Stats Container */
.sh-stats {
  display: flex;
  background: var(--sh-surface);
  border-radius: var(--sh-radius-lg);
  overflow: hidden;
}

.sh-stats--horizontal {
  flex-direction: row;
}

.sh-stats--horizontal .sh-stat {
  flex: 1;
  border-right: 1px solid var(--sh-border);
}

.sh-stats--horizontal .sh-stat:last-child {
  border-right: none;
}

.sh-stats--vertical {
  flex-direction: column;
}

.sh-stats--vertical .sh-stat {
  border-bottom: 1px solid var(--sh-border);
}

.sh-stats--vertical .sh-stat:last-child {
  border-bottom: none;
}

.sh-stats--grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
}

.sh-stats--grid .sh-stat {
  border: 1px solid var(--sh-border);
}

.sh-stats--bordered {
  border: 1px solid var(--sh-border);
}

.sh-stats--shadow {
  box-shadow: var(--sh-shadow-md);
}

/* Metric Card */
.sh-metric-card {
  display: flex;
  flex-direction: column;
  padding: 1.5rem;
  background: var(--sh-surface);
  border-radius: var(--sh-radius-lg);
  border: 1px solid var(--sh-border);
  transition: all 0.2s ease;
}

.sh-metric-card:hover {
  transform: translateY(-2px);
  box-shadow: var(--sh-shadow-md);
}

.sh-metric-card__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 1rem;
}

.sh-metric-card__title {
  font-size: 0.875rem;
  color: var(--sh-text-muted);
  font-weight: 500;
}

.sh-metric-card__icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 2.5rem;
  height: 2.5rem;
  border-radius: var(--sh-radius-md);
  background: color-mix(in srgb, var(--sh-accent) 10%, transparent);
  color: var(--sh-accent);
}

.sh-metric-card__icon svg {
  width: 1.25rem;
  height: 1.25rem;
}

.sh-metric-card__value {
  font-size: 2rem;
  font-weight: 700;
  color: var(--sh-text);
  margin-bottom: 0.5rem;
}

.sh-metric-card__change {
  font-size: 0.875rem;
  font-weight: 500;
}

.sh-metric-card__change--positive {
  color: var(--sh-success);
}

.sh-metric-card__change--negative {
  color: var(--sh-error);
}

/* Color variants for metric cards */
.sh-metric-card--primary .sh-metric-card__icon {
  background: color-mix(in srgb, var(--sh-accent) 10%, transparent);
  color: var(--sh-accent);
}

.sh-metric-card--success .sh-metric-card__icon {
  background: color-mix(in srgb, var(--sh-success) 10%, transparent);
  color: var(--sh-success);
}

.sh-metric-card--warning .sh-metric-card__icon {
  background: color-mix(in srgb, var(--sh-warning) 10%, transparent);
  color: var(--sh-warning);
}

.sh-metric-card--error .sh-metric-card__icon {
  background: color-mix(in srgb, var(--sh-error) 10%, transparent);
  color: var(--sh-error);
}

/* Responsive */
@media (max-width: 640px) {
  .sh-stats--horizontal {
    flex-direction: column;
  }

  .sh-stats--horizontal .sh-stat {
    border-right: none;
    border-bottom: 1px solid var(--sh-border);
  }

  .sh-stats--horizontal .sh-stat:last-child {
    border-bottom: none;
  }

  .sh-stat__value {
    font-size: 2rem;
  }
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stat_creation() {
        let stat = Stat::new("Users", html! { "1,234" })
            .description("Total active users")
            .trend(TrendDirection::Up, "12%");

        assert_eq!(stat.title, "Users");
        assert!(stat.description.is_some());
        assert!(stat.trend.is_some());
    }

    #[test]
    fn test_stat_numeric() {
        let stat = Stat::numeric("Revenue", 12345.67).color(ComponentColor::Success);

        assert_eq!(stat.title, "Revenue");
        assert_eq!(stat.color, Some(ComponentColor::Success));
    }

    #[test]
    fn test_stats_container() {
        let stats = Stats::new(vec![
            Stat::new("A", html! { "100" }),
            Stat::new("B", html! { "200" }),
        ])
        .layout(StatsLayout::Grid)
        .shadow(true);

        assert!(stats.shadow);
        assert!(matches!(stats.layout, StatsLayout::Grid));
    }

    #[test]
    fn test_metric_card() {
        let card = MetricCard::new("Sales", html! { "$10K" })
            .change("+20%", true)
            .color(ComponentColor::Success);

        assert_eq!(card.title, "Sales");
        assert_eq!(card.change, Some("+20%"));
        assert_eq!(card.change_positive, Some(true));
    }
}
