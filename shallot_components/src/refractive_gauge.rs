//! RefractiveGauge Component - CSS-only Gauge/Speedometer
//!
//! A beautiful gauge/speedometer visualization using pure CSS conic gradients.
//! No JavaScript required for the visual display.

use maud::{html, Markup, Render};

/// RefractiveGauge size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RefractiveGaugeSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl RefractiveGaugeSize {
    fn css_class(&self) -> &'static str {
        match self {
            RefractiveGaugeSize::Small => "sh-gauge--sm",
            RefractiveGaugeSize::Medium => "sh-gauge--md",
            RefractiveGaugeSize::Large => "sh-gauge--lg",
        }
    }
}

/// RefractiveGauge color themes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RefractiveGaugeTheme {
    #[default]
    Blue,
    Green,
    Orange,
    Red,
    Purple,
}

impl RefractiveGaugeTheme {
    fn gradient(&self) -> &'static str {
        match self {
            RefractiveGaugeTheme::Blue => "linear-gradient(135deg, #667eea 0%, #764ba2 100%)",
            RefractiveGaugeTheme::Green => "linear-gradient(135deg, #11998e 0%, #38ef7d 100%)",
            RefractiveGaugeTheme::Orange => "linear-gradient(135deg, #f093fb 0%, #f5576c 100%)",
            RefractiveGaugeTheme::Red => "linear-gradient(135deg, #eb3349 0%, #f45c43 100%)",
            RefractiveGaugeTheme::Purple => "linear-gradient(135deg, #da22ff 0%, #9733ee 100%)",
        }
    }
}

/// RefractiveGauge component
pub struct RefractiveGauge {
    value: f32,
    label: Option<String>,
    size: RefractiveGaugeSize,
    theme: RefractiveGaugeTheme,
    show_value: bool,
    class: Option<String>,
}

impl RefractiveGauge {
    /// Create a new RefractiveGauge (0-100 scale)
    pub fn new(value: f32) -> Self {
        Self {
            value: value.clamp(0.0, 100.0),
            label: None,
            size: RefractiveGaugeSize::default(),
            theme: RefractiveGaugeTheme::default(),
            show_value: true,
            class: None,
        }
    }

    /// Create with custom min/max
    pub fn with_range(value: f32, min: f32, max: f32) -> Self {
        let normalized = ((value - min) / (max - min) * 100.0).clamp(0.0, 100.0);
        Self {
            value: normalized,
            label: None,
            size: RefractiveGaugeSize::default(),
            theme: RefractiveGaugeTheme::default(),
            show_value: true,
            class: None,
        }
    }

    /// Set label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set size
    pub fn size(mut self, size: RefractiveGaugeSize) -> Self {
        self.size = size;
        self
    }

    /// Set theme
    pub fn theme(mut self, theme: RefractiveGaugeTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Show/hide value display
    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    /// Add custom class
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-gauge".to_string()];
        classes.push(self.size.css_class().to_string());
        if let Some(custom) = &self.class {
            classes.push(custom.clone());
        }
        classes.join(" ")
    }

    fn get_percentage(&self) -> f32 {
        self.value
    }

    fn get_conic_gradient(&self) -> String {
        let pct = self.get_percentage();
        let empty_color = "rgba(0, 0, 0, 0.1)";
        format!(
            "conic-gradient({} {}%, {} {}%)",
            self.theme.gradient(),
            pct * 2.7,
            empty_color,
            pct * 2.7
        )
    }
}

impl Render for RefractiveGauge {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let percentage = self.get_percentage();
        let gradient = self.get_conic_gradient();
        let rotation = (percentage / 100.0 * 270.0) - 135.0;

        html! {
            div class=(classes) role="gauge" aria-valuenow=(percentage) aria-valuemin="0" aria-valuemax="100" {
                @if let Some(label) = &self.label {
                    span class="sh-gauge__label" { (label) }
                }
                div class="sh-gauge__circle" style=(format!("background: {}", gradient)) {
                    div class="sh-gauge__inner" {
                        div class="sh-gauge__needle" style=(format!("transform: rotate({}deg)", rotation)) {}
                    }
                }
                @if self.show_value {
                    div class="sh-gauge__value" {
                        (format!("{:.0}%", percentage))
                    }
                }
            }
        }
    }
}

/// Generate CSS for RefractiveGauge component
pub fn refractive_gauge_css() -> String {
    r#"
.sh-gauge {
    position: relative;
    display: inline-flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
}

.sh-gauge__label {
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--sh-text-muted, #6b7280);
}

.sh-gauge__circle {
    position: relative;
    width: 8rem;
    height: 8rem;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
}

.sh-gauge__inner {
    position: absolute;
    inset: 0.5rem;
    background: var(--sh-surface, #fff);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
}

.sh-gauge__needle {
    position: absolute;
    width: 0.25rem;
    height: 3rem;
    background: var(--sh-text, #1f2937);
    border-radius: 9999px;
    transform-origin: bottom center;
    bottom: 50%;
    transition: transform 0.5s cubic-bezier(0.4, 0, 0.2, 1);
}

.sh-gauge__needle::before {
    content: '';
    position: absolute;
    top: -0.5rem;
    left: 50%;
    transform: translateX(-50%);
    width: 0.75rem;
    height: 0.75rem;
    background: var(--sh-text, #1f2937);
    border-radius: 50%;
}

.sh-gauge__value {
    font-size: 1.5rem;
    font-weight: 700;
    color: var(--sh-text, #1f2937);
}

/* Size variants */
.sh-gauge--sm .sh-gauge__circle {
    width: 5rem;
    height: 5rem;
}

.sh-gauge--sm .sh-gauge__inner {
    inset: 0.375rem;
}

.sh-gauge--sm .sh-gauge__needle {
    height: 2rem;
}

.sh-gauge--sm .sh-gauge__value {
    font-size: 1.125rem;
}

.sh-gauge--md .sh-gauge__circle {
    width: 8rem;
    height: 8rem;
}

.sh-gauge--md .sh-gauge__inner {
    inset: 0.5rem;
}

.sh-gauge--md .sh-gauge__needle {
    height: 3rem;
}

.sh-gauge--md .sh-gauge__value {
    font-size: 1.5rem;
}

.sh-gauge--lg .sh-gauge__circle {
    width: 12rem;
    height: 12rem;
}

.sh-gauge--lg .sh-gauge__inner {
    inset: 0.75rem;
}

.sh-gauge--lg .sh-gauge__needle {
    height: 4.5rem;
}

.sh-gauge--lg .sh-gauge__value {
    font-size: 2rem;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gauge_creation() {
        let gauge = RefractiveGauge::new(75.0);
        assert_eq!(gauge.value, 75.0);
    }

    #[test]
    fn test_gauge_with_range() {
        let gauge = RefractiveGauge::with_range(75.0, 0.0, 100.0);
        assert_eq!(gauge.value, 75.0);
    }

    #[test]
    fn test_gauge_label() {
        let gauge = RefractiveGauge::new(50.0).label("Progress");
        assert_eq!(gauge.label, Some("Progress".to_string()));
    }

    #[test]
    fn test_gauge_theme() {
        let gauge = RefractiveGauge::new(50.0).theme(RefractiveGaugeTheme::Green);
        assert_eq!(gauge.theme, RefractiveGaugeTheme::Green);
    }

    #[test]
    fn test_gauge_css() {
        let css = refractive_gauge_css();
        assert!(css.contains(".sh-gauge"));
        assert!(css.contains(".sh-gauge__circle"));
    }
}
