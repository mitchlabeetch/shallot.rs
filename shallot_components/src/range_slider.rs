//! Range Slider Component - Dual-thumb slider for selecting a range of values
//! Uses native HTML5 range inputs styled with CSS custom properties
//! Zero JavaScript - CSS-only interaction

use maud::{html, Markup, Render};

/// Size variants for the range slider
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RangeSliderSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Color variants for the range slider track
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RangeSliderVariant {
    #[default]
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
}

/// A dual-thumb range slider component
#[derive(Debug, Clone)]
pub struct RangeSlider<'a> {
    /// Minimum value
    pub min: i32,
    /// Maximum value
    pub max: i32,
    /// Current low value
    pub low: i32,
    /// Current high value
    pub high: i32,
    /// Step increment
    pub step: i32,
    /// Size variant
    pub size: RangeSliderSize,
    /// Color variant
    pub variant: RangeSliderVariant,
    /// Unique ID for the slider
    pub id: &'a str,
    /// Label text
    pub label: Option<&'a str>,
    /// Show value labels
    pub show_values: bool,
    /// Disabled state
    pub disabled: bool,
}

impl<'a> RangeSlider<'a> {
    /// Create a new range slider with default values
    pub fn new(id: &'a str) -> Self {
        Self {
            min: 0,
            max: 100,
            low: 25,
            high: 75,
            step: 1,
            size: RangeSliderSize::default(),
            variant: RangeSliderVariant::default(),
            id,
            label: None,
            show_values: true,
            disabled: false,
        }
    }

    /// Set the minimum value
    pub fn min(mut self, min: i32) -> Self {
        self.min = min;
        self
    }

    /// Set the maximum value
    pub fn max(mut self, max: i32) -> Self {
        self.max = max;
        self
    }

    /// Set the low value
    pub fn low(mut self, low: i32) -> Self {
        self.low = low;
        self
    }

    /// Set the high value
    pub fn high(mut self, high: i32) -> Self {
        self.high = high;
        self
    }

    /// Set the step increment
    pub fn step(mut self, step: i32) -> Self {
        self.step = step;
        self
    }

    /// Set the size variant
    pub fn size(mut self, size: RangeSliderSize) -> Self {
        self.size = size;
        self
    }

    /// Set the color variant
    pub fn variant(mut self, variant: RangeSliderVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the label
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    /// Show or hide value labels
    pub fn show_values(mut self, show: bool) -> Self {
        self.show_values = show;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Build CSS class string
    fn build_classes(&self) -> String {
        let size_class = match self.size {
            RangeSliderSize::Sm => "sh-range-slider--sm",
            RangeSliderSize::Md => "sh-range-slider--md",
            RangeSliderSize::Lg => "sh-range-slider--lg",
        };

        let variant_class = match self.variant {
            RangeSliderVariant::Primary => "sh-range-slider--primary",
            RangeSliderVariant::Secondary => "sh-range-slider--secondary",
            RangeSliderVariant::Success => "sh-range-slider--success",
            RangeSliderVariant::Warning => "sh-range-slider--warning",
            RangeSliderVariant::Danger => "sh-range-slider--danger",
        };

        let mut classes = vec!["sh-range-slider", size_class, variant_class];

        if self.disabled {
            classes.push("sh-range-slider--disabled");
        }

        classes.join(" ")
    }
}

impl<'a> Render for RangeSlider<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let low_id = format!("{}-low", self.id);
        let high_id = format!("{}-high", self.id);

        html! {
            div class=(classes) {
                @if let Some(label) = self.label {
                    label class="sh-range-slider__label" for=(self.id) {
                        (label)
                    }
                }

                @if self.show_values {
                    div class="sh-range-slider__values" {
                        span class="sh-range-slider__value sh-range-slider__value--low" {
                            (self.low)
                        }
                        span class="sh-range-slider__value sh-range-slider__value--high" {
                            (self.high)
                        }
                    }
                }

                div class="sh-range-slider__track" {
                    div class="sh-range-slider__fill" style=(format!("left: {}%; right: {}%",
                        ((self.low - self.min) as f64 / (self.max - self.min) as f64 * 100.0),
                        100.0 - ((self.high - self.min) as f64 / (self.max - self.min) as f64 * 100.0)
                    )) {}

                    input
                        type="range"
                        id=(low_id)
                        name=(low_id)
                        class="sh-range-slider__thumb sh-range-slider__thumb--low"
                        min=(self.min)
                        max=(self.max)
                        value=(self.low)
                        step=(self.step)
                        disabled?[self.disabled]
                        aria-label="Minimum value";

                    input
                        type="range"
                        id=(high_id)
                        name=(high_id)
                        class="sh-range-slider__thumb sh-range-slider__thumb--high"
                        min=(self.min)
                        max=(self.max)
                        value=(self.high)
                        step=(self.step)
                        disabled?[self.disabled]
                        aria-label="Maximum value";
                }
            }
        }
    }
}

/// Generate CSS for the range slider component
pub fn range_slider_css() -> String {
    r#"
.sh-range-slider {
    --sh-range-track-height: 8px;
    --sh-range-thumb-size: 20px;
    display: flex;
    flex-direction: column;
    gap: var(--sh-spacing-sm, 0.5rem);
    width: 100%;
}

.sh-range-slider--sm {
    --sh-range-track-height: 4px;
    --sh-range-thumb-size: 16px;
}

.sh-range-slider--lg {
    --sh-range-track-height: 12px;
    --sh-range-thumb-size: 28px;
}

.sh-range-slider__label {
    font-size: var(--sh-font-size-sm, 0.875rem);
    font-weight: var(--sh-font-weight-medium, 500);
    color: var(--sh-color-text, #1a1a1a);
}

.sh-range-slider__values {
    display: flex;
    justify-content: space-between;
    font-size: var(--sh-font-size-sm, 0.875rem);
    color: var(--sh-color-text-muted, #666);
}

.sh-range-slider__track {
    position: relative;
    width: 100%;
    height: var(--sh-range-track-height);
    background: var(--sh-color-surface-muted, #e5e5e5);
    border-radius: var(--sh-radius-full, 9999px);
}

.sh-range-slider__fill {
    position: absolute;
    top: 0;
    height: 100%;
    border-radius: var(--sh-radius-full, 9999px);
    transition: background-color 0.2s ease;
}

.sh-range-slider--primary .sh-range-slider__fill {
    background: var(--sh-color-primary, #3b82f6);
}

.sh-range-slider--secondary .sh-range-slider__fill {
    background: var(--sh-color-secondary, #6b7280);
}

.sh-range-slider--success .sh-range-slider__fill {
    background: var(--sh-color-success, #22c55e);
}

.sh-range-slider--warning .sh-range-slider__fill {
    background: var(--sh-color-warning, #f59e0b);
}

.sh-range-slider--danger .sh-range-slider__fill {
    background: var(--sh-color-danger, #ef4444);
}

.sh-range-slider__thumb {
    position: absolute;
    top: 50%;
    width: var(--sh-range-thumb-size);
    height: var(--sh-range-thumb-size);
    transform: translate(-50%, -50%);
    appearance: none;
    background: white;
    border: 2px solid var(--sh-color-primary, #3b82f6);
    border-radius: 50%;
    cursor: pointer;
    pointer-events: auto;
    z-index: 1;
}

.sh-range-slider__thumb::-webkit-slider-thumb {
    appearance: none;
    width: var(--sh-range-thumb-size);
    height: var(--sh-range-thumb-size);
    background: white;
    border: 2px solid var(--sh-color-primary, #3b82f6);
    border-radius: 50%;
    cursor: pointer;
}

.sh-range-slider__thumb::-moz-range-thumb {
    width: var(--sh-range-thumb-size);
    height: var(--sh-range-thumb-size);
    background: white;
    border: 2px solid var(--sh-color-primary, #3b82f6);
    border-radius: 50%;
    cursor: pointer;
}

.sh-range-slider__thumb:focus {
    outline: none;
    box-shadow: 0 0 0 3px var(--sh-color-primary-alpha, rgba(59, 130, 246, 0.2));
}

.sh-range-slider--disabled {
    opacity: 0.5;
    pointer-events: none;
}

.sh-range-slider--disabled .sh-range-slider__thumb {
    cursor: not-allowed;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_slider_creation() {
        let slider = RangeSlider::new("price-range")
            .min(0)
            .max(1000)
            .low(100)
            .high(500)
            .label("Price Range");

        assert_eq!(slider.min, 0);
        assert_eq!(slider.max, 1000);
        assert_eq!(slider.low, 100);
        assert_eq!(slider.high, 500);
        assert_eq!(slider.label, Some("Price Range"));
    }

    #[test]
    fn test_range_slider_render() {
        let slider = RangeSlider::new("test-slider").low(20).high(80);

        let rendered = slider.render();
        let html = rendered.into_string();

        assert!(html.contains("sh-range-slider"));
        assert!(html.contains("type=\"range\""));
    }

    #[test]
    fn test_range_slider_disabled() {
        let slider = RangeSlider::new("disabled-slider").disabled(true);

        let classes = slider.build_classes();
        assert!(classes.contains("sh-range-slider--disabled"));
    }

    #[test]
    fn test_range_slider_sizes() {
        let sm = RangeSlider::new("sm").size(RangeSliderSize::Sm);
        let lg = RangeSlider::new("lg").size(RangeSliderSize::Lg);

        assert!(sm.build_classes().contains("sh-range-slider--sm"));
        assert!(lg.build_classes().contains("sh-range-slider--lg"));
    }

    #[test]
    fn test_range_slider_variants() {
        let success = RangeSlider::new("success").variant(RangeSliderVariant::Success);
        let danger = RangeSlider::new("danger").variant(RangeSliderVariant::Danger);

        assert!(success.build_classes().contains("sh-range-slider--success"));
        assert!(danger.build_classes().contains("sh-range-slider--danger"));
    }

    #[test]
    fn test_css_generation() {
        let css = range_slider_css();
        assert!(css.contains(".sh-range-slider"));
        assert!(css.contains(".sh-range-slider__thumb"));
    }
}
