//! Transition System - CSS Transitions and Effects
//!
//! This module provides:
//! - Predefined transition presets
//! - Transform utilities
//! - Filter effects
//! - Backdrop effects
//! - CSS generation for transitions

use std::time::Duration;

/// A transition property definition
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionProperty {
    All,
    None,
    Opacity,
    Transform,
    Color,
    BackgroundColor,
    BorderColor,
    BoxShadow,
    Width,
    Height,
    MaxWidth,
    MaxHeight,
    Margin,
    Padding,
    Filter,
    Custom(&'static str),
}

impl TransitionProperty {
    /// Convert to CSS property name
    pub fn to_css(&self) -> String {
        match self {
            TransitionProperty::All => "all".to_string(),
            TransitionProperty::None => "none".to_string(),
            TransitionProperty::Opacity => "opacity".to_string(),
            TransitionProperty::Transform => "transform".to_string(),
            TransitionProperty::Color => "color".to_string(),
            TransitionProperty::BackgroundColor => "background-color".to_string(),
            TransitionProperty::BorderColor => "border-color".to_string(),
            TransitionProperty::BoxShadow => "box-shadow".to_string(),
            TransitionProperty::Width => "width".to_string(),
            TransitionProperty::Height => "height".to_string(),
            TransitionProperty::MaxWidth => "max-width".to_string(),
            TransitionProperty::MaxHeight => "max-height".to_string(),
            TransitionProperty::Margin => "margin".to_string(),
            TransitionProperty::Padding => "padding".to_string(),
            TransitionProperty::Filter => "filter".to_string(),
            TransitionProperty::Custom(s) => s.to_string(),
        }
    }
}

/// Transition timing functions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimingFunction {
    Linear,
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    StepStart,
    StepEnd,
    CubicBezier(f32, f32, f32, f32),
}

impl TimingFunction {
    /// Convert to CSS timing function
    pub fn to_css(&self) -> String {
        match self {
            TimingFunction::Linear => "linear".to_string(),
            TimingFunction::Ease => "ease".to_string(),
            TimingFunction::EaseIn => "ease-in".to_string(),
            TimingFunction::EaseOut => "ease-out".to_string(),
            TimingFunction::EaseInOut => "ease-in-out".to_string(),
            TimingFunction::StepStart => "step-start".to_string(),
            TimingFunction::StepEnd => "step-end".to_string(),
            TimingFunction::CubicBezier(x1, y1, x2, y2) => {
                format!("cubic-bezier({:.3}, {:.3}, {:.3}, {:.3})", x1, y1, x2, y2)
            }
        }
    }

    /// Standard ease-out-expo curve
    pub fn ease_out_expo() -> Self {
        TimingFunction::CubicBezier(0.19, 1.0, 0.22, 1.0)
    }

    /// Standard ease-in-out-cubic curve
    pub fn ease_in_out_cubic() -> Self {
        TimingFunction::CubicBezier(0.645, 0.045, 0.355, 1.0)
    }

    /// Standard ease-out-back curve (slight overshoot)
    pub fn ease_out_back() -> Self {
        TimingFunction::CubicBezier(0.175, 0.885, 0.32, 1.275)
    }

    /// Standard ease-out-quint curve
    pub fn ease_out_quint() -> Self {
        TimingFunction::CubicBezier(0.22, 1.0, 0.36, 1.0)
    }

    /// Standard ease-out-quart curve
    pub fn ease_out_quart() -> Self {
        TimingFunction::CubicBezier(0.25, 1.0, 0.5, 1.0)
    }
}

/// A complete transition definition
#[derive(Debug, Clone)]
pub struct Transition {
    pub property: TransitionProperty,
    pub duration: Duration,
    pub timing_function: TimingFunction,
    pub delay: Duration,
}

impl Default for Transition {
    fn default() -> Self {
        Self {
            property: TransitionProperty::All,
            duration: Duration::from_millis(200),
            timing_function: TimingFunction::EaseOut,
            delay: Duration::ZERO,
        }
    }
}

impl Transition {
    /// Create a new transition with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the transition property
    pub fn property(mut self, property: TransitionProperty) -> Self {
        self.property = property;
        self
    }

    /// Set the transition duration
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Set the transition duration in milliseconds
    pub fn duration_ms(self, millis: u64) -> Self {
        self.duration(Duration::from_millis(millis))
    }

    /// Set the timing function
    pub fn timing_function(mut self, timing: TimingFunction) -> Self {
        self.timing_function = timing;
        self
    }

    /// Set the delay
    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    /// Set the delay in milliseconds
    pub fn delay_ms(self, millis: u64) -> Self {
        self.delay(Duration::from_millis(millis))
    }

    /// Convert to CSS transition string
    pub fn to_css(&self) -> String {
        let duration_ms = self.duration.as_millis();
        let delay_ms = self.delay.as_millis();

        if delay_ms > 0 {
            format!(
                "{} {}ms {} {}ms",
                self.property.to_css(),
                duration_ms,
                self.timing_function.to_css(),
                delay_ms
            )
        } else {
            format!(
                "{} {}ms {}",
                self.property.to_css(),
                duration_ms,
                self.timing_function.to_css()
            )
        }
    }
}

/// Multiple transitions combined
#[derive(Debug, Clone, Default)]
pub struct TransitionSet {
    transitions: Vec<Transition>,
}

impl TransitionSet {
    /// Create a new empty transition set
    pub fn new() -> Self {
        Self { transitions: Vec::new() }
    }

    /// Add a transition to the set
    pub fn add(mut self, transition: Transition) -> Self {
        self.transitions.push(transition);
        self
    }

    /// Build the final CSS transition value
    pub fn build(&self) -> String {
        if self.transitions.is_empty() {
            return "none".to_string();
        }
        self.transitions
            .iter()
            .map(|t| t.to_css())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

/// Predefined transition presets
pub mod presets {
    use super::*;

    /// Quick fade transition (opacity only)
    pub fn fade() -> Transition {
        Transition::new()
            .property(TransitionProperty::Opacity)
            .duration_ms(150)
            .timing_function(TimingFunction::ease_out_expo())
    }

    /// Smooth fade transition
    pub fn fade_smooth() -> Transition {
        Transition::new()
            .property(TransitionProperty::Opacity)
            .duration_ms(300)
            .timing_function(TimingFunction::ease_in_out_cubic())
    }

    /// Transform (movement/scale) transition
    pub fn transform() -> Transition {
        Transition::new()
            .property(TransitionProperty::Transform)
            .duration_ms(300)
            .timing_function(TimingFunction::ease_out_back())
    }

    /// Quick transform for micro-interactions
    pub fn transform_fast() -> Transition {
        Transition::new()
            .property(TransitionProperty::Transform)
            .duration_ms(150)
            .timing_function(TimingFunction::ease_out_expo())
    }

    /// Color transition
    pub fn color() -> Transition {
        Transition::new()
            .property(TransitionProperty::Color)
            .duration_ms(200)
            .timing_function(TimingFunction::EaseOut)
    }

    /// Background color transition
    pub fn background() -> Transition {
        Transition::new()
            .property(TransitionProperty::BackgroundColor)
            .duration_ms(200)
            .timing_function(TimingFunction::EaseOut)
    }

    /// Border color transition
    pub fn border() -> Transition {
        Transition::new()
            .property(TransitionProperty::BorderColor)
            .duration_ms(150)
            .timing_function(TimingFunction::EaseOut)
    }

    /// Shadow transition
    pub fn shadow() -> Transition {
        Transition::new()
            .property(TransitionProperty::BoxShadow)
            .duration_ms(300)
            .timing_function(TimingFunction::ease_out_expo())
    }

    /// Width/height transition
    pub fn size() -> Transition {
        Transition::new()
            .property(TransitionProperty::Width)
            .duration_ms(300)
            .timing_function(TimingFunction::ease_in_out_cubic())
    }

    /// All properties transition (use sparingly for performance)
    pub fn all() -> Transition {
        Transition::new()
            .property(TransitionProperty::All)
            .duration_ms(200)
            .timing_function(TimingFunction::EaseOut)
    }

    /// Common button transition
    pub fn button() -> TransitionSet {
        TransitionSet::new()
            .add(Transition::new()
                .property(TransitionProperty::BackgroundColor)
                .duration_ms(200)
                .timing_function(TimingFunction::EaseOut))
            .add(Transition::new()
                .property(TransitionProperty::Transform)
                .duration_ms(150)
                .timing_function(TimingFunction::ease_out_expo()))
            .add(Transition::new()
                .property(TransitionProperty::BoxShadow)
                .duration_ms(200)
                .timing_function(TimingFunction::EaseOut))
    }

    /// Common card hover transition
    pub fn card_hover() -> TransitionSet {
        TransitionSet::new()
            .add(Transition::new()
                .property(TransitionProperty::Transform)
                .duration_ms(300)
                .timing_function(TimingFunction::ease_out_expo()))
            .add(Transition::new()
                .property(TransitionProperty::BoxShadow)
                .duration_ms(300)
                .timing_function(TimingFunction::EaseOut))
    }

    /// Input focus transition
    pub fn input_focus() -> TransitionSet {
        TransitionSet::new()
            .add(Transition::new()
                .property(TransitionProperty::BorderColor)
                .duration_ms(150)
                .timing_function(TimingFunction::EaseOut))
            .add(Transition::new()
                .property(TransitionProperty::BoxShadow)
                .duration_ms(150)
                .timing_function(TimingFunction::EaseOut))
    }

    /// Modal/dialog transition
    pub fn modal() -> TransitionSet {
        TransitionSet::new()
            .add(Transition::new()
                .property(TransitionProperty::Opacity)
                .duration_ms(200)
                .timing_function(TimingFunction::ease_out_expo()))
            .add(Transition::new()
                .property(TransitionProperty::Transform)
                .duration_ms(300)
                .timing_function(TimingFunction::ease_out_back()))
    }
}

/// Transform utilities
pub mod transform {
    /// A CSS transform definition
    #[derive(Debug, Clone, Default)]
    pub struct Transform {
        operations: Vec<String>,
    }

    impl Transform {
        /// Create a new transform
        pub fn new() -> Self {
            Self { operations: Vec::new() }
        }

        /// Add a translate operation
        pub fn translate(mut self, x: impl Into<String>, y: impl Into<String>) -> Self {
            self.operations.push(format!(
                "translate({}, {})",
                x.into(),
                y.into()
            ));
            self
        }

        /// Add a translateX operation
        pub fn translate_x(mut self, x: impl Into<String>) -> Self {
            self.operations.push(format!("translateX({})", x.into()));
            self
        }

        /// Add a translateY operation
        pub fn translate_y(mut self, y: impl Into<String>) -> Self {
            self.operations.push(format!("translateY({})", y.into()));
            self
        }

        /// Add a scale operation
        pub fn scale(mut self, factor: f32) -> Self {
            self.operations.push(format!("scale({})", factor));
            self
        }

        /// Add separate X and Y scale
        pub fn scale_xy(mut self, x: f32, y: f32) -> Self {
            self.operations.push(format!("scale({}, {})", x, y));
            self
        }

        /// Add a rotate operation
        pub fn rotate(mut self, degrees: f32) -> Self {
            self.operations.push(format!("rotate({}deg)", degrees));
            self
        }

        /// Add a skew operation
        pub fn skew(mut self, x: f32, y: f32) -> Self {
            self.operations.push(format!("skew({}deg, {}deg)", x, y));
            self
        }

        /// Build the final CSS transform value
        pub fn build(&self) -> String {
            if self.operations.is_empty() {
                return "none".to_string();
            }
            self.operations.join(" ")
        }
    }

    /// Create a translate transform
    pub fn translate(x: impl Into<String>, y: impl Into<String>) -> Transform {
        Transform::new().translate(x, y)
    }

    /// Create a scale transform
    pub fn scale(factor: f32) -> Transform {
        Transform::new().scale(factor)
    }

    /// Create a rotate transform
    pub fn rotate(degrees: f32) -> Transform {
        Transform::new().rotate(degrees)
    }
}

/// Filter effects
pub mod filter {
    /// A CSS filter definition
    #[derive(Debug, Clone, Default)]
    pub struct Filter {
        effects: Vec<String>,
    }

    impl Filter {
        /// Create a new filter
        pub fn new() -> Self {
            Self { effects: Vec::new() }
        }

        /// Add a blur effect
        pub fn blur(mut self, radius: impl Into<String>) -> Self {
            self.effects.push(format!("blur({})", radius.into()));
            self
        }

        /// Add a brightness effect
        pub fn brightness(mut self, amount: f32) -> Self {
            self.effects.push(format!("brightness({})", amount));
            self
        }

        /// Add a contrast effect
        pub fn contrast(mut self, amount: f32) -> Self {
            self.effects.push(format!("contrast({}%)", amount * 100.0));
            self
        }

        /// Add a grayscale effect
        pub fn grayscale(mut self, amount: f32) -> Self {
            self.effects.push(format!("grayscale({}%)", amount * 100.0));
            self
        }

        /// Add a sepia effect
        pub fn sepia(mut self, amount: f32) -> Self {
            self.effects.push(format!("sepia({}%)", amount * 100.0));
            self
        }

        /// Add a saturate effect
        pub fn saturate(mut self, amount: f32) -> Self {
            self.effects.push(format!("saturate({}%)", amount * 100.0));
            self
        }

        /// Add a hue-rotate effect
        pub fn hue_rotate(mut self, degrees: f32) -> Self {
            self.effects.push(format!("hue-rotate({}deg)", degrees));
            self
        }

        /// Add an invert effect
        pub fn invert(mut self, amount: f32) -> Self {
            self.effects.push(format!("invert({}%)", amount * 100.0));
            self
        }

        /// Add an opacity effect (different from element opacity)
        pub fn opacity(mut self, amount: f32) -> Self {
            self.effects.push(format!("opacity({}%)", amount * 100.0));
            self
        }

        /// Add a drop-shadow effect
        pub fn drop_shadow(mut self, x: f32, y: f32, blur: f32, color: &str) -> Self {
            self.effects.push(format!(
                "drop-shadow({}px {}px {}px {})",
                x, y, blur, color
            ));
            self
        }

        /// Build the final CSS filter value
        pub fn build(&self) -> String {
            if self.effects.is_empty() {
                return "none".to_string();
            }
            self.effects.join(" ")
        }
    }
}

/// Generate CSS for common transition utilities
pub fn transition_css() -> String {
    r#"
/* Transition utilities */
.sh-transition {
  transition-property: all;
  transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
  transition-duration: 200ms;
}

.sh-transition-opacity {
  transition-property: opacity;
  transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
  transition-duration: 150ms;
}

.sh-transition-transform {
  transition-property: transform;
  transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
  transition-duration: 200ms;
}

.sh-transition-colors {
  transition-property: color, background-color, border-color, text-decoration-color, fill, stroke;
  transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
  transition-duration: 200ms;
}

.sh-transition-shadow {
  transition-property: box-shadow;
  transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
  transition-duration: 200ms;
}

/* Duration utilities */
.sh-duration-75 { transition-duration: 75ms; }
.sh-duration-100 { transition-duration: 100ms; }
.sh-duration-150 { transition-duration: 150ms; }
.sh-duration-200 { transition-duration: 200ms; }
.sh-duration-300 { transition-duration: 300ms; }
.sh-duration-500 { transition-duration: 500ms; }
.sh-duration-700 { transition-duration: 700ms; }
.sh-duration-1000 { transition-duration: 1000ms; }

/* Easing utilities */
.sh-ease-linear { transition-timing-function: linear; }
.sh-ease-in { transition-timing-function: cubic-bezier(0.4, 0, 1, 1); }
.sh-ease-out { transition-timing-function: cubic-bezier(0, 0, 0.2, 1); }
.sh-ease-in-out { transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); }

/* Transform utilities */
.sh-transform {
  transform: translateZ(0);
  will-change: transform;
}

.sh-transform-gpu {
  transform: translate3d(0, 0, 0);
  will-change: transform;
}

/* Hover lift effect */
.sh-hover-lift {
  transition: transform 200ms cubic-bezier(0.4, 0, 0.2, 1),
              box-shadow 200ms cubic-bezier(0.4, 0, 0.2, 1);
}

.sh-hover-lift:hover {
  transform: translateY(-2px);
}

/* Hover scale effect */
.sh-hover-scale {
  transition: transform 150ms cubic-bezier(0.4, 0, 0.2, 1);
}

.sh-hover-scale:hover {
  transform: scale(1.02);
}

/* Active press effect */
.sh-active-press:active {
  transform: scale(0.98);
}

/* Focus ring animation */
.sh-focus-ring-transition {
  transition: box-shadow 200ms cubic-bezier(0.4, 0, 0.2, 1),
              outline 200ms cubic-bezier(0.4, 0, 0.2, 1);
}
"#.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transition_creation() {
        let transition = Transition::new()
            .property(TransitionProperty::Opacity)
            .duration_ms(300)
            .timing_function(TimingFunction::ease_out_expo())
            .delay_ms(100);

        assert_eq!(transition.property, TransitionProperty::Opacity);
        assert_eq!(transition.duration.as_millis(), 300);
        assert_eq!(transition.delay.as_millis(), 100);
    }

    #[test]
    fn test_transition_css_output() {
        let transition = Transition::new()
            .property(TransitionProperty::Transform)
            .duration_ms(300)
            .timing_function(TimingFunction::EaseOut);

        assert_eq!(transition.to_css(), "transform 300ms ease-out");
    }

    #[test]
    fn test_transition_with_delay() {
        let transition = Transition::new()
            .property(TransitionProperty::Opacity)
            .duration_ms(200)
            .delay_ms(100);

        assert_eq!(transition.to_css(), "opacity 200ms ease-out 100ms");
    }

    #[test]
    fn test_transition_set() {
        let set = TransitionSet::new()
            .add(Transition::new().property(TransitionProperty::Opacity))
            .add(Transition::new().property(TransitionProperty::Transform));

        let css = set.build();
        assert!(css.contains("opacity"));
        assert!(css.contains("transform"));
    }

    #[test]
    fn test_timing_functions() {
        assert_eq!(TimingFunction::Linear.to_css(), "linear");
        assert_eq!(TimingFunction::EaseOut.to_css(), "ease-out");
        assert_eq!(
            TimingFunction::CubicBezier(0.25, 0.1, 0.25, 1.0).to_css(),
            "cubic-bezier(0.250, 0.100, 0.250, 1.000)"
        );
    }

    #[test]
    fn test_presets() {
        let fade = presets::fade();
        assert_eq!(fade.property, TransitionProperty::Opacity);
        assert_eq!(fade.duration.as_millis(), 150);

        let transform = presets::transform();
        assert_eq!(transform.property, TransitionProperty::Transform);
    }

    #[test]
    fn test_transform_builder() {
        let t = transform::Transform::new()
            .translate("10px", "20px")
            .scale(1.5)
            .rotate(45.0);

        assert_eq!(t.build(), "translate(10px, 20px) scale(1.5) rotate(45deg)");
    }

    #[test]
    fn test_filter_builder() {
        let f = filter::Filter::new()
            .blur("5px")
            .brightness(1.2)
            .grayscale(0.5);

        let css = f.build();
        assert!(css.contains("blur(5px)"));
        assert!(css.contains("brightness(1.2)"));
        assert!(css.contains("grayscale(50%)"));
    }
}
