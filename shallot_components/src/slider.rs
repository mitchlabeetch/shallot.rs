use maud::{html, Markup, Render};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SliderVariant {
    #[default]
    Default,
    Primary,
    Accent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SliderSize {
    #[default]
    Md,
    Sm,
    Lg,
}

pub struct Slider<'a> {
    pub name: &'a str,
    pub value: u8,
    pub min: u8,
    pub max: u8,
    pub variant: SliderVariant,
    pub size: SliderSize,
    pub disabled: bool,
    pub label: Option<&'a str>,
    pub show_value: bool,
}

impl<'a> Default for Slider<'a> {
    fn default() -> Self {
        Self {
            name: "",
            value: 50,
            min: 0,
            max: 100,
            variant: SliderVariant::Default,
            size: SliderSize::Md,
            disabled: false,
            label: None,
            show_value: false,
        }
    }
}

impl<'a> Slider<'a> {
    pub fn new(name: &'a str) -> Self {
        Self::default().name(name)
    }

    pub fn name(mut self, name: &'a str) -> Self {
        self.name = name;
        self
    }

    pub fn value(mut self, value: u8) -> Self {
        self.value = value.clamp(self.min, self.max);
        self
    }

    pub fn min(mut self, min: u8) -> Self {
        self.min = min;
        self
    }

    pub fn max(mut self, max: u8) -> Self {
        self.max = max;
        self
    }

    pub fn variant(mut self, variant: SliderVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: SliderSize) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }
}

impl<'a> Render for Slider<'a> {
    fn render(&self) -> Markup {
        let variant_class = match self.variant {
            SliderVariant::Default => "sh-slider",
            SliderVariant::Primary => "sh-slider sh-slider--primary",
            SliderVariant::Accent => "sh-slider sh-slider--accent",
        };

        let size_class = match self.size {
            SliderSize::Sm => "sh-slider--sm",
            SliderSize::Md => "sh-slider--md",
            SliderSize::Lg => "sh-slider--lg",
        };

        let value = self.value.clamp(self.min, self.max);
        let percentage = ((value - self.min) as f32 / (self.max - self.min) as f32) * 100.0;
        let aria_label = self.label.unwrap_or(self.name);

        html! {
            div class=(format!("{} {}", variant_class, size_class)) {
                @if let Some(label) = self.label {
                    label class="sh-slider__label" { (label) }
                }
                div class="sh-slider__wrapper" {
                    input
                        type="range"
                        name=(self.name)
                        min=(self.min)
                        max=(self.max)
                        value=(value)
                        disabled?[self.disabled]
                        class="sh-slider__input"
                        role="slider"
                        aria-valuenow=(value)
                        aria-valuemin=(self.min)
                        aria-valuemax=(self.max)
                        aria-label=(aria_label)
                        style=(format!("--sh-slider-value: {}%", percentage))
                    {}
                    @if self.show_value {
                        span class="sh-slider__value" { (value) }
                    }
                }
            }
        }
    }
}

pub fn slider_css() -> String {
    r#"
.sh-slider {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    width: 100%;
}

.sh-slider__label {
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--sh-text-primary, #111827);
}

.sh-slider__wrapper {
    display: flex;
    align-items: center;
    gap: 0.75rem;
}

.sh-slider__input {
    -webkit-appearance: none;
    appearance: none;
    width: 100%;
    height: 4px;
    background: var(--sh-border-color, #e5e7eb);
    border-radius: 9999px;
    outline: none;
    cursor: pointer;
    --sh-slider-value: 50%;
}

.sh-slider__input::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 16px;
    height: 16px;
    background: var(--sh-bg-primary, #ffffff);
    border: 2px solid var(--sh-accent, #6366f1);
    border-radius: 50%;
    cursor: pointer;
    transition: transform 0.15s ease, box-shadow 0.15s ease;
}

.sh-slider__input::-webkit-slider-thumb:hover {
    transform: scale(1.1);
    box-shadow: 0 0 0 4px rgba(99, 102, 241, 0.15);
}

.sh-slider__input::-moz-range-thumb {
    width: 16px;
    height: 16px;
    background: var(--sh-bg-primary, #ffffff);
    border: 2px solid var(--sh-accent, #6366f1);
    border-radius: 50%;
    cursor: pointer;
    transition: transform 0.15s ease, box-shadow 0.15s ease;
}

.sh-slider__input::-moz-range-thumb:hover {
    transform: scale(1.1);
    box-shadow: 0 0 0 4px rgba(99, 102, 241, 0.15);
}

.sh-slider__input:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

.sh-slider__input:disabled::-webkit-slider-thumb {
    cursor: not-allowed;
}

.sh-slider--sm .sh-slider__input {
    height: 2px;
}

.sh-slider--sm .sh-slider__input::-webkit-slider-thumb {
    width: 12px;
    height: 12px;
}

.sh-slider--sm .sh-slider__input::-moz-range-thumb {
    width: 12px;
    height: 12px;
}

.sh-slider--lg .sh-slider__input {
    height: 6px;
}

.sh-slider--lg .sh-slider__input::-webkit-slider-thumb {
    width: 20px;
    height: 20px;
}

.sh-slider--lg .sh-slider__input::-moz-range-thumb {
    width: 20px;
    height: 20px;
}

.sh-slider--primary .sh-slider__input::-webkit-slider-thumb {
    border-color: var(--sh-primary, #3b82f6);
}

.sh-slider--primary .sh-slider__input::-moz-range-thumb {
    border-color: var(--sh-primary, #3b82f6);
}

.sh-slider--accent .sh-slider__input::-webkit-slider-thumb {
    border-color: var(--sh-accent, #6366f1);
}

.sh-slider--accent .sh-slider__input::-moz-range-thumb {
    border-color: var(--sh-accent, #6366f1);
}

.sh-slider__value {
    min-width: 2.5rem;
    text-align: center;
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--sh-text-secondary, #6b7280);
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slider_default() {
        let slider = Slider::new("test");
        let rendered = slider.render();
        assert!(rendered.0.as_str().contains("sh-slider"));
    }

    #[test]
    fn test_slider_with_value() {
        let slider = Slider::new("test").value(75);
        let rendered = slider.render();
        assert!(rendered.0.as_str().contains("value=\"75\""));
    }

    #[test]
    fn test_slider_with_label() {
        let slider = Slider::new("test").label("Volume");
        let rendered = slider.render();
        assert!(rendered.0.as_str().contains("Volume"));
        assert!(rendered.0.as_str().contains("sh-slider__label"));
    }

    #[test]
    fn test_slider_variant_primary() {
        let slider = Slider::new("test").variant(SliderVariant::Primary);
        let rendered = slider.render();
        assert!(rendered.0.as_str().contains("sh-slider--primary"));
    }

    #[test]
    fn test_slider_size_sm() {
        let slider = Slider::new("test").size(SliderSize::Sm);
        let rendered = slider.render();
        assert!(rendered.0.as_str().contains("sh-slider--sm"));
    }

    #[test]
    fn test_slider_size_lg() {
        let slider = Slider::new("test").size(SliderSize::Lg);
        let rendered = slider.render();
        assert!(rendered.0.as_str().contains("sh-slider--lg"));
    }

    #[test]
    fn test_slider_disabled() {
        let slider = Slider::new("test").disabled(true);
        let rendered = slider.render();
        assert!(rendered.0.as_str().contains("disabled"));
    }

    #[test]
    fn test_slider_show_value() {
        let slider = Slider::new("test").value(42).show_value(true);
        let rendered = slider.render();
        assert!(rendered.0.as_str().contains("sh-slider__value"));
        assert!(rendered.0.as_str().contains("42"));
    }

    #[test]
    fn test_slider_min_max() {
        let slider = Slider::new("test").min(10).max(50);
        let rendered = slider.render();
        assert!(rendered.0.as_str().contains("min=\"10\""));
        assert!(rendered.0.as_str().contains("max=\"50\""));
    }

    #[test]
    fn test_slider_a11y() {
        let slider = Slider::new("test").value(60);
        let rendered = slider.render();
        assert!(rendered.0.as_str().contains("role=\"slider\""));
        assert!(rendered.0.as_str().contains("aria-valuenow"));
        assert!(rendered.0.as_str().contains("aria-valuemin"));
        assert!(rendered.0.as_str().contains("aria-valuemax"));
    }

    #[test]
    fn test_slider_css() {
        let css = slider_css();
        assert!(css.contains(".sh-slider"));
        assert!(css.contains(".sh-slider__input"));
    }
}
