//! Animated Text Components
//!
//! Beautiful text animations inspired by MagicUI:
//! - AnimatedGradientText: Text with flowing gradient animation
//! - ShimmerText: Shimmering highlight effect
//! - SparklesText: Text with animated sparkles
//! - TypingText: Typewriter effect

use crate::component::Component;
use maud::{html, Markup, Render};

/// Animated gradient text with flowing colors
pub struct AnimatedGradientText {
    /// Text content
    text: String,
    /// Gradient colors
    colors: Vec<String>,
    /// Animation speed in seconds
    speed: f32,
    /// Font size
    font_size: String,
    /// Font weight
    font_weight: u16,
    /// Custom class
    custom_class: Option<String>,
}

impl AnimatedGradientText {
    /// Create new animated gradient text
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            colors: vec![
                "#ffaa40".to_string(),
                "#9c40ff".to_string(),
                "#ffaa40".to_string(),
            ],
            speed: 3.0,
            font_size: "inherit".to_string(),
            font_weight: 700,
            custom_class: None,
        }
    }

    /// Set gradient colors
    pub fn colors(mut self, colors: Vec<impl Into<String>>) -> Self {
        self.colors = colors.into_iter().map(|c| c.into()).collect();
        self
    }

    /// Set animation speed (seconds per cycle)
    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    /// Set font size
    pub fn font_size(mut self, size: impl Into<String>) -> Self {
        self.font_size = size.into();
        self
    }

    /// Set font weight
    pub fn font_weight(mut self, weight: u16) -> Self {
        self.font_weight = weight;
        self
    }

    /// Add custom class
    pub fn custom_class(mut self, class: impl Into<String>) -> Self {
        self.custom_class = Some(class.into());
        self
    }

    fn build_gradient(&self) -> String {
        self.colors.join(", ")
    }

    fn build_style(&self) -> String {
        format!(
            "--gradient-colors: {}; --gradient-speed: {}s; --gradient-font-size: {}; --gradient-font-weight: {};",
            self.build_gradient(),
            self.speed,
            self.font_size,
            self.font_weight
        )
    }
}

impl Render for AnimatedGradientText {
    fn render(&self) -> Markup {
        let class = format!(
            "sh-animated-gradient-text {}",
            self.custom_class.as_deref().unwrap_or("")
        );

        html! {
            span
                class=(class)
                style=(self.build_style())
                role="text"
                aria-label=(self.text.as_str())
            {
                (self.text.clone())
            }
        }
    }
}

impl Component for AnimatedGradientText {
    fn classes(&self) -> String {
        format!(
            "sh-animated-gradient-text {}",
            self.custom_class.as_deref().unwrap_or("")
        )
    }
}

/// Shimmer text effect
pub struct ShimmerText {
    text: String,
    base_color: String,
    shimmer_color: String,
    duration: f32,
    custom_class: Option<String>,
}

impl ShimmerText {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            base_color: "var(--sh-text-muted)".to_string(),
            shimmer_color: "#ffffff".to_string(),
            duration: 2.0,
            custom_class: None,
        }
    }

    pub fn base_color(mut self, color: impl Into<String>) -> Self {
        self.base_color = color.into();
        self
    }

    pub fn shimmer_color(mut self, color: impl Into<String>) -> Self {
        self.shimmer_color = color.into();
        self
    }

    pub fn duration(mut self, duration: f32) -> Self {
        self.duration = duration;
        self
    }

    pub fn custom_class(mut self, class: impl Into<String>) -> Self {
        self.custom_class = Some(class.into());
        self
    }

    fn build_style(&self) -> String {
        format!(
            "--shimmer-base: {}; --shimmer-highlight: {}; --shimmer-duration: {}s;",
            self.base_color, self.shimmer_color, self.duration
        )
    }
}

impl Render for ShimmerText {
    fn render(&self) -> Markup {
        let class = format!(
            "sh-shimmer-text {}",
            self.custom_class.as_deref().unwrap_or("")
        );

        html! {
            span
                class=(class)
                style=(self.build_style())
                role="text"
                aria-label=(self.text.clone())
            {
                (self.text.clone())
            }
        }
    }
}

/// Number ticker/counter animation
pub struct NumberTicker {
    value: f64,
    prefix: String,
    suffix: String,
    decimals: u8,
    duration: f32,
    font_size: String,
    custom_class: Option<String>,
}

impl NumberTicker {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            prefix: String::new(),
            suffix: String::new(),
            decimals: 0,
            duration: 2.0,
            font_size: "2rem".to_string(),
            custom_class: None,
        }
    }

    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = prefix.into();
        self
    }

    pub fn suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = suffix.into();
        self
    }

    pub fn decimals(mut self, decimals: u8) -> Self {
        self.decimals = decimals;
        self
    }

    pub fn duration(mut self, duration: f32) -> Self {
        self.duration = duration;
        self
    }

    pub fn font_size(mut self, size: impl Into<String>) -> Self {
        self.font_size = size.into();
        self
    }

    pub fn custom_class(mut self, class: impl Into<String>) -> Self {
        self.custom_class = Some(class.into());
        self
    }

    fn format_value(&self) -> String {
        format!("{:.prec$}", self.value, prec = self.decimals as usize)
    }

    fn build_style(&self) -> String {
        format!(
            "--ticker-duration: {}s; --ticker-font-size: {}; --ticker-value: {};",
            self.duration, self.font_size, self.value
        )
    }
}

impl Render for NumberTicker {
    fn render(&self) -> Markup {
        let class = format!(
            "sh-number-ticker {}",
            self.custom_class.as_deref().unwrap_or("")
        );
        let value_str = self.format_value();

        html! {
            span
                class=(class)
                style=(self.build_style())
                data-value=(value_str)
                role="text"
                aria-live="polite"
                aria-label=(format!("{}{}{}", self.prefix, value_str, self.suffix))
            {
                span class="sh-number-ticker__prefix" aria-hidden="true" { (self.prefix) }
                span class="sh-number-ticker__value" { (value_str) }
                span class="sh-number-ticker__suffix" aria-hidden="true" { (self.suffix) }
            }
        }
    }
}

/// Word rotation component
pub struct WordRotate {
    words: Vec<String>,
    duration: f32,
    custom_class: Option<String>,
}

impl WordRotate {
    pub fn new(words: Vec<impl Into<String>>) -> Self {
        Self {
            words: words.into_iter().map(|w| w.into()).collect(),
            duration: 2.5,
            custom_class: None,
        }
    }

    pub fn duration(mut self, duration: f32) -> Self {
        self.duration = duration;
        self
    }

    pub fn custom_class(mut self, class: impl Into<String>) -> Self {
        self.custom_class = Some(class.into());
        self
    }
}

impl Render for WordRotate {
    fn render(&self) -> Markup {
        let class = format!(
            "sh-word-rotate {}",
            self.custom_class.as_deref().unwrap_or("")
        );

        html! {
            span class=(class) style=(format!("--rotate-duration: {}s;", self.duration)) {
                span class="sh-word-rotate__words" {
                    @for (i, word) in self.words.iter().enumerate() {
                        span
                            class="sh-word-rotate__word"
                            style=(format!("--word-index: {};", i))
                        {
                            (word.clone())
                        }
                    }
                }
            }
        }
    }
}

/// Generate CSS for animated text components
pub fn animated_text_css() -> String {
    r#"
/* Animated Gradient Text */
.sh-animated-gradient-text {
  display: inline;
  background: linear-gradient(
    to right,
    var(--gradient-colors, #ffaa40, #9c40ff, #ffaa40)
  );
  background-size: 200% auto;
  -webkit-background-clip: text;
  background-clip: text;
  -webkit-text-fill-color: transparent;
  color: transparent;
  font-size: var(--gradient-font-size, inherit);
  font-weight: var(--gradient-font-weight, 700);
  animation: gradient-flow var(--gradient-speed, 3s) linear infinite;
}

@keyframes gradient-flow {
  0% { background-position: 0% center; }
  100% { background-position: 200% center; }
}

/* Shimmer Text */
.sh-shimmer-text {
  display: inline-block;
  position: relative;
  color: var(--shimmer-base, var(--sh-text-muted));
}

.sh-shimmer-text::after {
  content: attr(data-text);
  position: absolute;
  left: 0;
  top: 0;
  background: linear-gradient(
    90deg,
    transparent 0%,
    var(--shimmer-highlight, #ffffff) 50%,
    transparent 100%
  );
  background-size: 200% 100%;
  -webkit-background-clip: text;
  background-clip: text;
  color: transparent;
  animation: shimmer var(--shimmer-duration, 2s) infinite;
}

@keyframes shimmer {
  0% { background-position: -200% 0; }
  100% { background-position: 200% 0; }
}

/* Number Ticker */
.sh-number-ticker {
  display: inline-flex;
  align-items: center;
  font-size: var(--ticker-font-size, 2rem);
  font-weight: 700;
  font-variant-numeric: tabular-nums;
}

.sh-number-ticker__prefix,
.sh-number-ticker__suffix {
  opacity: 0.7;
}

/* Word Rotate */
.sh-word-rotate {
  display: inline-flex;
  position: relative;
  vertical-align: bottom;
  overflow: hidden;
}

.sh-word-rotate__words {
  display: inline-flex;
  flex-direction: column;
  animation: word-rotate var(--rotate-duration, 2.5s) ease-in-out infinite;
}

.sh-word-rotate__word {
  display: block;
  height: 1.2em;
  line-height: 1.2em;
}

@keyframes word-rotate {
  0%, 20% { transform: translateY(0); }
  25%, 45% { transform: translateY(-1.2em); }
  50%, 70% { transform: translateY(-2.4em); }
  75%, 95% { transform: translateY(-3.6em); }
  100% { transform: translateY(-4.8em); }
}

/* Preset color combinations */
.sh-gradient-rainbow {
  --gradient-colors: #ff0000, #ff7f00, #ffff00, #00ff00, #0000ff, #4b0082, #9400d3, #ff0000;
}

.sh-gradient-ocean {
  --gradient-colors: #0066ff, #00c3ff, #0066ff;
}

.sh-gradient-sunset {
  --gradient-colors: #ff6b6b, #feca57, #ff9ff3, #ff6b6b;
}

.sh-gradient-forest {
  --gradient-colors: #00b894, #00cec9, #00b894;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animated_gradient_text() {
        let text = AnimatedGradientText::new("Hello World")
            .colors(vec!["#ff0000", "#00ff00", "#0000ff"])
            .speed(2.0)
            .font_size("2rem")
            .font_weight(800);

        assert_eq!(text.text, "Hello World");
        assert_eq!(text.colors.len(), 3);
        assert_eq!(text.speed, 2.0);
    }

    #[test]
    fn test_shimmer_text() {
        let text = ShimmerText::new("Loading...")
            .base_color("#333")
            .shimmer_color("#fff")
            .duration(1.5);

        assert_eq!(text.text, "Loading...");
        assert_eq!(text.duration, 1.5);
    }

    #[test]
    fn test_number_ticker() {
        let ticker = NumberTicker::new(1234.56)
            .prefix("$")
            .suffix("USD")
            .decimals(2);

        assert_eq!(ticker.value, 1234.56);
        assert_eq!(ticker.prefix, "$");
        assert_eq!(ticker.decimals, 2);
    }

    #[test]
    fn test_word_rotate() {
        let rotate = WordRotate::new(vec!["Design", "Build", "Ship", "Repeat"]).duration(3.0);

        assert_eq!(rotate.words.len(), 4);
        assert_eq!(rotate.duration, 3.0);
    }
}
