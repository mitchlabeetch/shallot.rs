//! Scroll Reveal Component - Scroll-Triggered Animations
//!
//! Reveals content as it scrolls into view using CSS animations.
//! Uses Intersection Observer API via progressive enhancement (zero-JS baseline works too).
//!
//! Research Reference: Section 5.6.2 - Scroll & Parallax
//!
//! # Example
//! ```
//! use shallot_components::scroll_reveal::{ScrollReveal, RevealAnimation};
//! use maud::html;
//!
//! let reveal = ScrollReveal::new(html! {
//!     h2 { "Scroll to see me" }
//!     p { "This content animates in" }
//! })
//! .animation(RevealAnimation::FadeUp)
//! .duration(800)
//! .delay(200);
//! ```

use maud::{html, Markup};

/// Animation types for scroll reveal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RevealAnimation {
    Fade,        // Simple opacity fade
    FadeUp,      // Fade + translate up
    FadeDown,    // Fade + translate down
    FadeLeft,    // Fade + translate from left
    FadeRight,   // Fade + translate from right
    ZoomIn,      // Scale from smaller
    ZoomOut,     // Scale from larger
    FlipX,       // Rotate on X axis
    FlipY,       // Rotate on Y axis
    SlideUp,     // Slide without fade
    SlideDown,   // Slide without fade
    BlurIn,      // Blur to clear
    RevealUp,    // Clip-path reveal from bottom
    RevealDown,  // Clip-path reveal from top
    RevealLeft,  // Clip-path reveal from right
    RevealRight, // Clip-path reveal from left
}

impl RevealAnimation {
    pub fn css_class(&self) -> &'static str {
        match self {
            Self::Fade => "sh-reveal--fade",
            Self::FadeUp => "sh-reveal--fade-up",
            Self::FadeDown => "sh-reveal--fade-down",
            Self::FadeLeft => "sh-reveal--fade-left",
            Self::FadeRight => "sh-reveal--fade-right",
            Self::ZoomIn => "sh-reveal--zoom-in",
            Self::ZoomOut => "sh-reveal--zoom-out",
            Self::FlipX => "sh-reveal--flip-x",
            Self::FlipY => "sh-reveal--flip-y",
            Self::SlideUp => "sh-reveal--slide-up",
            Self::SlideDown => "sh-reveal--slide-down",
            Self::BlurIn => "sh-reveal--blur-in",
            Self::RevealUp => "sh-reveal--reveal-up",
            Self::RevealDown => "sh-reveal--reveal-down",
            Self::RevealLeft => "sh-reveal--reveal-left",
            Self::RevealRight => "sh-reveal--reveal-right",
        }
    }

    /// Whether this animation uses clip-path
    pub fn uses_clip_path(&self) -> bool {
        matches!(
            self,
            Self::RevealUp | Self::RevealDown | Self::RevealLeft | Self::RevealRight
        )
    }
}

/// Easing functions for animations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Easing {
    Linear,
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInBack,
    EaseOutBack,
    EaseInOutBack,
    Spring,
}

impl Easing {
    pub fn css_value(&self) -> &'static str {
        match self {
            Self::Linear => "linear",
            Self::Ease => "ease",
            Self::EaseIn => "ease-in",
            Self::EaseOut => "ease-out",
            Self::EaseInOut => "ease-in-out",
            Self::EaseInQuad => "cubic-bezier(0.55, 0.085, 0.68, 0.53)",
            Self::EaseOutQuad => "cubic-bezier(0.25, 0.46, 0.45, 0.94)",
            Self::EaseInOutQuad => "cubic-bezier(0.455, 0.03, 0.515, 0.955)",
            Self::EaseInCubic => "cubic-bezier(0.55, 0.055, 0.675, 0.19)",
            Self::EaseOutCubic => "cubic-bezier(0.215, 0.61, 0.355, 1)",
            Self::EaseInOutCubic => "cubic-bezier(0.645, 0.045, 0.355, 1)",
            Self::EaseInBack => "cubic-bezier(0.6, -0.28, 0.735, 0.045)",
            Self::EaseOutBack => "cubic-bezier(0.175, 0.885, 0.32, 1.275)",
            Self::EaseInOutBack => "cubic-bezier(0.68, -0.55, 0.265, 1.55)",
            Self::Spring => "cubic-bezier(0.34, 1.56, 0.64, 1)",
        }
    }
}

/// Threshold for triggering animation (0.0 - 1.0)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RevealThreshold {
    Early,       // 0.1 - triggers early
    Default,     // 0.2 - standard
    Late,        // 0.5 - triggers when more visible
    Custom(f32), // Custom value
}

impl RevealThreshold {
    pub fn value(&self) -> f32 {
        match self {
            Self::Early => 0.1,
            Self::Default => 0.2,
            Self::Late => 0.5,
            Self::Custom(v) => v.clamp(0.0, 1.0),
        }
    }
}

/// Scroll reveal component
#[derive(Debug, Clone)]
pub struct ScrollReveal {
    content: Markup,
    animation: RevealAnimation,
    duration: u16, // milliseconds
    delay: u16,    // milliseconds
    easing: Easing,
    threshold: RevealThreshold,
    once: bool,                          // Only animate once
    stagger_children: Option<(u16, u8)>, // (delay_ms, child_selector_depth)
    distance: Option<u16>,               // Override default distance (px)
    class: Option<String>,
    id: Option<String>,
}

impl ScrollReveal {
    pub fn new(content: Markup) -> Self {
        Self {
            content,
            animation: RevealAnimation::FadeUp,
            duration: 600,
            delay: 0,
            easing: Easing::EaseOut,
            threshold: RevealThreshold::Default,
            once: true,
            stagger_children: None,
            distance: None,
            class: None,
            id: None,
        }
    }

    pub fn animation(mut self, animation: RevealAnimation) -> Self {
        self.animation = animation;
        self
    }

    pub fn duration(mut self, duration: u16) -> Self {
        self.duration = duration;
        self
    }

    pub fn delay(mut self, delay: u16) -> Self {
        self.delay = delay;
        self
    }

    pub fn easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }

    pub fn threshold(mut self, threshold: RevealThreshold) -> Self {
        self.threshold = threshold;
        self
    }

    /// Set whether animation only plays once
    pub fn once(mut self, once: bool) -> Self {
        self.once = once;
        self
    }

    /// Stagger children animations
    pub fn stagger_children(mut self, base_delay: u16, depth: u8) -> Self {
        self.stagger_children = Some((base_delay, depth));
        self
    }

    /// Set custom animation distance in pixels
    pub fn distance(mut self, distance: u16) -> Self {
        self.distance = Some(distance);
        self
    }

    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    fn build_styles(&self) -> String {
        let distance = self.distance.unwrap_or(30);

        format!(
            "--sh-reveal-duration:{}ms;--sh-reveal-delay:{}ms;--sh-reveal-easing:{};--sh-reveal-distance:{}px;--sh-reveal-threshold:{};",
            self.duration,
            self.delay,
            self.easing.css_value(),
            distance,
            self.threshold.value()
        )
    }

    fn build_classes(&self) -> String {
        let mut classes = vec![
            "sh-reveal".to_string(),
            self.animation.css_class().to_string(),
        ];

        if !self.once {
            classes.push("sh-reveal--repeat".to_string());
        }

        if self.stagger_children.is_some() {
            classes.push("sh-reveal--stagger".to_string());
        }

        if let Some(ref c) = self.class {
            classes.push(c.clone());
        }

        classes.join(" ")
    }

    pub fn render(self) -> Markup {
        let styles = self.build_styles();
        let classes = self.build_classes();
        let id = self.id.unwrap_or_else(|| {
            // Generate a unique-ish ID
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let mut hasher = DefaultHasher::new();
            self.animation.css_class().hash(&mut hasher);
            self.duration.hash(&mut hasher);
            format!("reveal-{}", hasher.finish() % 10000)
        });

        // Build data attributes for progressive enhancement
        let threshold_val = self.threshold.value();
        let once_val = self.once;

        html! {
            div
                class=(classes)
                id=(id)
                style=(styles)
                data-threshold=(threshold_val)
                data-once=(once_val)
                role="region"
                aria-label="Scroll reveal animation"
            {
                div class="sh-reveal-content" {
                    (self.content)
                }
            }
        }
    }
}

/// Stagger container for animating children sequentially
#[derive(Debug, Clone)]
pub struct StaggerContainer {
    children: Vec<Markup>,
    base_delay: u16,
    stagger_delay: u16,
    animation: RevealAnimation,
    duration: u16,
}

impl StaggerContainer {
    pub fn new(children: Vec<Markup>) -> Self {
        Self {
            children,
            base_delay: 0,
            stagger_delay: 100,
            animation: RevealAnimation::FadeUp,
            duration: 600,
        }
    }

    pub fn base_delay(mut self, delay: u16) -> Self {
        self.base_delay = delay;
        self
    }

    pub fn stagger_delay(mut self, delay: u16) -> Self {
        self.stagger_delay = delay;
        self
    }

    pub fn animation(mut self, animation: RevealAnimation) -> Self {
        self.animation = animation;
        self
    }

    pub fn duration(mut self, duration: u16) -> Self {
        self.duration = duration;
        self
    }

    pub fn render(self) -> Markup {
        html! {
            div class="sh-stagger-container" {
                @for (i, child) in self.children.into_iter().enumerate() {
                    @let delay = self.base_delay + (i as u16 * self.stagger_delay);
                    (ScrollReveal::new(child)
                        .animation(self.animation)
                        .duration(self.duration)
                        .delay(delay)
                        .render())
                }
            }
        }
    }
}

/// Parallax container for scroll-based parallax effects
#[derive(Debug, Clone)]
pub struct ParallaxContainer {
    layers: Vec<(Markup, f32)>, // (content, speed)
    height: Option<u16>,
    class: Option<String>,
}

impl ParallaxContainer {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            height: None,
            class: None,
        }
    }

    /// Add a layer with speed (0.0 = no movement, 0.5 = half speed, 1.0 = normal)
    pub fn add_layer(mut self, content: Markup, speed: f32) -> Self {
        self.layers.push((content, speed.clamp(0.0, 2.0)));
        self
    }

    pub fn height(mut self, height: u16) -> Self {
        self.height = Some(height);
        self
    }

    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }

    pub fn render(self) -> Markup {
        let height_style = self
            .height
            .map(|h| format!("height:{}px;", h))
            .unwrap_or_default();

        let mut classes = vec!["sh-parallax".to_string()];
        if let Some(ref c) = self.class {
            classes.push(c.clone());
        }
        let class = classes.join(" ");

        html! {
            div class=(class) style=(height_style) {
                @for (i, (content, speed)) in self.layers.into_iter().enumerate() {
                    div
                        class="sh-parallax-layer"
                        style=(format!("--parallax-speed:{};z-index:{};", speed, i))
                        data-speed=(speed)
                    {
                        (content)
                    }
                }
            }
        }
    }
}

/// Generate CSS for scroll reveal and parallax components
pub fn scroll_reveal_css() -> String {
    r#"
/* Scroll Reveal Styles */

.sh-reveal {
    --sh-reveal-duration: 600ms;
    --sh-reveal-delay: 0ms;
    --sh-reveal-easing: ease-out;
    --sh-reveal-distance: 30px;
    --sh-reveal-threshold: 0.2;
}

.sh-reveal-content {
    opacity: 0;
    animation-duration: var(--sh-reveal-duration);
    animation-delay: var(--sh-reveal-delay);
    animation-timing-function: var(--sh-reveal-easing);
    animation-fill-mode: forwards;
    will-change: opacity, transform;
}

/* Animation keyframes */
@keyframes reveal-fade {
    from { opacity: 0; }
    to { opacity: 1; }
}

@keyframes reveal-fade-up {
    from {
        opacity: 0;
        transform: translateY(var(--sh-reveal-distance));
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}

@keyframes reveal-fade-down {
    from {
        opacity: 0;
        transform: translateY(calc(var(--sh-reveal-distance) * -1));
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}

@keyframes reveal-fade-left {
    from {
        opacity: 0;
        transform: translateX(var(--sh-reveal-distance));
    }
    to {
        opacity: 1;
        transform: translateX(0);
    }
}

@keyframes reveal-fade-right {
    from {
        opacity: 0;
        transform: translateX(calc(var(--sh-reveal-distance) * -1));
    }
    to {
        opacity: 1;
        transform: translateX(0);
    }
}

@keyframes reveal-zoom-in {
    from {
        opacity: 0;
        transform: scale(0.9);
    }
    to {
        opacity: 1;
        transform: scale(1);
    }
}

@keyframes reveal-zoom-out {
    from {
        opacity: 0;
        transform: scale(1.1);
    }
    to {
        opacity: 1;
        transform: scale(1);
    }
}

@keyframes reveal-flip-x {
    from {
        opacity: 0;
        transform: perspective(400px) rotateX(90deg);
    }
    to {
        opacity: 1;
        transform: perspective(400px) rotateX(0);
    }
}

@keyframes reveal-flip-y {
    from {
        opacity: 0;
        transform: perspective(400px) rotateY(90deg);
    }
    to {
        opacity: 1;
        transform: perspective(400px) rotateY(0);
    }
}

@keyframes reveal-slide-up {
    from {
        transform: translateY(var(--sh-reveal-distance));
    }
    to {
        transform: translateY(0);
    }
}

@keyframes reveal-slide-down {
    from {
        transform: translateY(calc(var(--sh-reveal-distance) * -1));
    }
    to {
        transform: translateY(0);
    }
}

@keyframes reveal-blur-in {
    from {
        opacity: 0;
        filter: blur(10px);
    }
    to {
        opacity: 1;
        filter: blur(0);
    }
}

@keyframes reveal-reveal-up {
    from {
        clip-path: inset(100% 0 0 0);
    }
    to {
        clip-path: inset(0 0 0 0);
    }
}

@keyframes reveal-reveal-down {
    from {
        clip-path: inset(0 0 100% 0);
    }
    to {
        clip-path: inset(0 0 0 0);
    }
}

@keyframes reveal-reveal-left {
    from {
        clip-path: inset(0 0 0 100%);
    }
    to {
        clip-path: inset(0 0 0 0);
    }
}

@keyframes reveal-reveal-right {
    from {
        clip-path: inset(0 100% 0 0);
    }
    to {
        clip-path: inset(0 0 0 0);
    }
}

/* Animation class assignments */
.sh-reveal--fade .sh-reveal-content {
    animation-name: reveal-fade;
}

.sh-reveal--fade-up .sh-reveal-content {
    animation-name: reveal-fade-up;
}

.sh-reveal--fade-down .sh-reveal-content {
    animation-name: reveal-fade-down;
}

.sh-reveal--fade-left .sh-reveal-content {
    animation-name: reveal-fade-left;
}

.sh-reveal--fade-right .sh-reveal-content {
    animation-name: reveal-fade-right;
}

.sh-reveal--zoom-in .sh-reveal-content {
    animation-name: reveal-zoom-in;
}

.sh-reveal--zoom-out .sh-reveal-content {
    animation-name: reveal-zoom-out;
}

.sh-reveal--flip-x .sh-reveal-content {
    animation-name: reveal-flip-x;
}

.sh-reveal--flip-y .sh-reveal-content {
    animation-name: reveal-flip-y;
}

.sh-reveal--slide-up .sh-reveal-content {
    animation-name: reveal-slide-up;
    opacity: 1;
}

.sh-reveal--slide-down .sh-reveal-content {
    animation-name: reveal-slide-down;
    opacity: 1;
}

.sh-reveal--blur-in .sh-reveal-content {
    animation-name: reveal-blur-in;
}

.sh-reveal--reveal-up .sh-reveal-content {
    animation-name: reveal-reveal-up;
    opacity: 1;
}

.sh-reveal--reveal-down .sh-reveal-content {
    animation-name: reveal-reveal-down;
    opacity: 1;
}

.sh-reveal--reveal-left .sh-reveal-content {
    animation-name: reveal-reveal-left;
    opacity: 1;
}

.sh-reveal--reveal-right .sh-reveal-content {
    animation-name: reveal-reveal-right;
    opacity: 1;
}

/* Zero-JS baseline: content is visible without animation */
@media (prefers-reduced-motion: reduce) {
    .sh-reveal-content {
        opacity: 1;
        animation: none;
        transform: none;
        filter: none;
        clip-path: none;
    }
}

/* Stagger container */
.sh-stagger-container {
    display: contents;
}

/* Parallax container */
.sh-parallax {
    position: relative;
    overflow: hidden;
}

.sh-parallax-layer {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    will-change: transform;
}

/* Progressive enhancement: only animate when JS adds visible class */
.sh-reveal:not(.sh-reveal--visible) .sh-reveal-content {
    animation-play-state: paused;
}

.sh-reveal.sh-reveal--visible .sh-reveal-content {
    animation-play-state: running;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reveal_animation_classes() {
        assert_eq!(RevealAnimation::FadeUp.css_class(), "sh-reveal--fade-up");
        assert_eq!(RevealAnimation::ZoomIn.css_class(), "sh-reveal--zoom-in");
        assert_eq!(RevealAnimation::FlipX.css_class(), "sh-reveal--flip-x");
    }

    #[test]
    fn test_reveal_uses_clip_path() {
        assert!(RevealAnimation::RevealUp.uses_clip_path());
        assert!(RevealAnimation::RevealLeft.uses_clip_path());
        assert!(!RevealAnimation::FadeUp.uses_clip_path());
        assert!(!RevealAnimation::ZoomIn.uses_clip_path());
    }

    #[test]
    fn test_easing_functions() {
        assert_eq!(Easing::Linear.css_value(), "linear");
        assert_eq!(
            Easing::EaseOutBack.css_value(),
            "cubic-bezier(0.175, 0.885, 0.32, 1.275)"
        );
        assert_eq!(
            Easing::Spring.css_value(),
            "cubic-bezier(0.34, 1.56, 0.64, 1)"
        );
    }

    #[test]
    fn test_threshold_values() {
        assert_eq!(RevealThreshold::Early.value(), 0.1);
        assert_eq!(RevealThreshold::Default.value(), 0.2);
        assert_eq!(RevealThreshold::Late.value(), 0.5);
        assert_eq!(RevealThreshold::Custom(0.75).value(), 0.75);
    }

    #[test]
    fn test_scroll_reveal_builder() {
        let reveal = ScrollReveal::new(html! { "Test" })
            .animation(RevealAnimation::FadeLeft)
            .duration(1000)
            .delay(200)
            .easing(Easing::Spring)
            .once(false);

        assert_eq!(reveal.animation, RevealAnimation::FadeLeft);
        assert_eq!(reveal.duration, 1000);
        assert_eq!(reveal.delay, 200);
        assert!(!reveal.once);
    }

    #[test]
    fn test_stagger_container() {
        let children = vec![html! { "Item 1" }, html! { "Item 2" }, html! { "Item 3" }];

        let stagger = StaggerContainer::new(children)
            .base_delay(100)
            .stagger_delay(50)
            .animation(RevealAnimation::ZoomIn);

        assert_eq!(stagger.children.len(), 3);
        assert_eq!(stagger.base_delay, 100);
        assert_eq!(stagger.stagger_delay, 50);
    }
}
