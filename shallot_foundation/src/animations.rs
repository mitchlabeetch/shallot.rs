use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EasingFunction {
    Linear,
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    CubicBezier(f32, f32, f32, f32),
}

impl EasingFunction {
    pub fn to_css(&self) -> String {
        match self {
            EasingFunction::Linear => "linear".to_string(),
            EasingFunction::Ease => "ease".to_string(),
            EasingFunction::EaseIn => "ease-in".to_string(),
            EasingFunction::EaseOut => "ease-out".to_string(),
            EasingFunction::EaseInOut => "ease-in-out".to_string(),
            EasingFunction::CubicBezier(x1, y1, x2, y2) => {
                format!("cubic-bezier({:.3}, {:.3}, {:.3}, {:.3})", x1, y1, x2, y2)
            }
        }
    }

    pub fn ease_out_expo() -> Self {
        EasingFunction::CubicBezier(0.19, 1.0, 0.22, 1.0)
    }

    pub fn ease_in_out_cubic() -> Self {
        EasingFunction::CubicBezier(0.645, 0.045, 0.355, 1.0)
    }

    pub fn ease_out_back() -> Self {
        EasingFunction::CubicBezier(0.175, 0.885, 0.32, 1.275)
    }

    pub fn ease_in_out_back() -> Self {
        EasingFunction::CubicBezier(0.68, -0.55, 0.265, 1.55)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationDirection {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

impl AnimationDirection {
    pub fn to_css(&self) -> &'static str {
        match self {
            AnimationDirection::Normal => "normal",
            AnimationDirection::Reverse => "reverse",
            AnimationDirection::Alternate => "alternate",
            AnimationDirection::AlternateReverse => "alternate-reverse",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationFillMode {
    None,
    Forwards,
    Backwards,
    Both,
}

impl AnimationFillMode {
    pub fn to_css(&self) -> &'static str {
        match self {
            AnimationFillMode::None => "none",
            AnimationFillMode::Forwards => "forwards",
            AnimationFillMode::Backwards => "backwards",
            AnimationFillMode::Both => "both",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationPlayState {
    Running,
    Paused,
}

impl AnimationPlayState {
    pub fn to_css(&self) -> &'static str {
        match self {
            AnimationPlayState::Running => "running",
            AnimationPlayState::Paused => "paused",
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnimationTiming {
    pub duration: Duration,
    pub delay: Duration,
    pub easing: EasingFunction,
    pub iterations: AnimationIterations,
    pub direction: AnimationDirection,
    pub fill_mode: AnimationFillMode,
    pub play_state: AnimationPlayState,
}

impl Default for AnimationTiming {
    fn default() -> Self {
        Self {
            duration: Duration::from_millis(300),
            delay: Duration::ZERO,
            easing: EasingFunction::EaseOut,
            iterations: AnimationIterations::Count(1),
            direction: AnimationDirection::Normal,
            fill_mode: AnimationFillMode::Both,
            play_state: AnimationPlayState::Running,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnimationIterations {
    Count(u32),
    Infinite,
}

impl AnimationIterations {
    pub fn to_css(&self) -> String {
        match self {
            AnimationIterations::Count(n) => n.to_string(),
            AnimationIterations::Infinite => "infinite".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Keyframe {
    pub percentage: f32,
    pub properties: HashMap<String, String>,
}

impl Keyframe {
    pub fn new(percentage: f32) -> Self {
        Self {
            percentage: percentage.clamp(0.0, 100.0),
            properties: HashMap::new(),
        }
    }

    pub fn property(mut self, name: &str, value: &str) -> Self {
        self.properties.insert(name.to_string(), value.to_string());
        self
    }

    pub fn transform(mut self, value: &str) -> Self {
        self.properties.insert("transform".to_string(), value.to_string());
        self
    }

    pub fn opacity(mut self, value: f32) -> Self {
        self.properties.insert("opacity".to_string(), value.to_string());
        self
    }

    pub fn scale(mut self, x: f32, y: Option<f32>) -> Self {
        let y_val = y.unwrap_or(x);
        self.properties.insert("transform".to_string(), format!("scale({:.3}, {:.3})", x, y_val));
        self
    }

    pub fn translate(mut self, x: &str, y: Option<&str>) -> Self {
        let y_val = y.unwrap_or("0");
        self.properties.insert("transform".to_string(), format!("translate({:.3}, {:.3})", x, y_val));
        self
    }

    pub fn rotate(mut self, degrees: f32) -> Self {
        self.properties.insert("transform".to_string(), format!("rotate({:.1}deg)", degrees));
        self
    }

    pub fn blur(mut self, radius: &str) -> Self {
        self.properties.insert("filter".to_string(), format!("blur({})", radius));
        self
    }
}

#[derive(Debug, Clone)]
pub struct KeyframeAnimation {
    pub name: String,
    pub keyframes: Vec<Keyframe>,
    pub timing: AnimationTiming,
}

impl KeyframeAnimation {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            keyframes: Vec::new(),
            timing: AnimationTiming::default(),
        }
    }

    pub fn keyframe(mut self, keyframe: Keyframe) -> Self {
        self.keyframes.push(keyframe);
        self
    }

    pub fn from(mut self, properties: HashMap<String, String>) -> Self {
        let keyframe = Keyframe {
            percentage: 0.0,
            properties,
        };
        self.keyframes.push(keyframe);
        self
    }

    pub fn to(mut self, properties: HashMap<String, String>) -> Self {
        let keyframe = Keyframe {
            percentage: 100.0,
            properties,
        };
        self.keyframes.push(keyframe);
        self
    }

    pub fn timing(mut self, timing: AnimationTiming) -> Self {
        self.timing = timing;
        self
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.timing.duration = duration;
        self
    }

    pub fn delay(mut self, delay: Duration) -> Self {
        self.timing.delay = delay;
        self
    }

    pub fn easing(mut self, easing: EasingFunction) -> Self {
        self.timing.easing = easing;
        self
    }

    pub fn iterations(mut self, iterations: AnimationIterations) -> Self {
        self.timing.iterations = iterations;
        self
    }

    pub fn infinite(mut self) -> Self {
        self.timing.iterations = AnimationIterations::Infinite;
        self
    }

    pub fn to_css(&self) -> String {
        let mut css = String::new();
        
        // Keyframe definition
        css.push_str(&format!("@keyframes {} {{\n", self.name));
        
        for keyframe in &self.keyframes {
            css.push_str(&format!("  {:.1}% {{\n", keyframe.percentage));
            for (prop, value) in &keyframe.properties {
                css.push_str(&format!("    {}: {};\n", prop, value));
            }
            css.push_str("  }\n");
        }
        
        css.push_str("}\n");
        
        css
    }

    pub fn to_animation_css(&self) -> String {
        format!(
            "{} {} {} {} {} {} {}",
            self.name,
            format_duration(&self.timing.duration),
            self.timing.easing.to_css(),
            format_duration(&self.timing.delay),
            self.timing.iterations.to_css(),
            self.timing.direction.to_css(),
            self.timing.fill_mode.to_css(),
        )
    }
}

fn format_duration(duration: &Duration) -> String {
    let millis = duration.as_millis();
    if millis == 0 {
        "0s".to_string()
    } else {
        format!("{}ms", millis)
    }
}

// Pre-built animation presets
pub fn fade_in() -> KeyframeAnimation {
    KeyframeAnimation::new("sh-fade-in")
        .from(HashMap::from([("opacity".to_string(), "0".to_string())]))
        .to(HashMap::from([("opacity".to_string(), "1".to_string())]))
        .duration(Duration::from_millis(300))
        .easing(EasingFunction::ease_out_expo())
}

pub fn fade_out() -> KeyframeAnimation {
    KeyframeAnimation::new("sh-fade-out")
        .from(HashMap::from([("opacity".to_string(), "1".to_string())]))
        .to(HashMap::from([("opacity".to_string(), "0".to_string())]))
        .duration(Duration::from_millis(300))
        .easing(EasingFunction::ease_in_out_cubic())
}

pub fn slide_in_up() -> KeyframeAnimation {
    KeyframeAnimation::new("sh-slide-in-up")
        .from(HashMap::from([
            ("transform".to_string(), "translateY(100%)".to_string()),
            ("opacity".to_string(), "0".to_string()),
        ]))
        .to(HashMap::from([
            ("transform".to_string(), "translateY(0)".to_string()),
            ("opacity".to_string(), "1".to_string()),
        ]))
        .duration(Duration::from_millis(400))
        .easing(EasingFunction::ease_out_back())
}

pub fn slide_in_down() -> KeyframeAnimation {
    KeyframeAnimation::new("sh-slide-in-down")
        .from(HashMap::from([
            ("transform".to_string(), "translateY(-100%)".to_string()),
            ("opacity".to_string(), "0".to_string()),
        ]))
        .to(HashMap::from([
            ("transform".to_string(), "translateY(0)".to_string()),
            ("opacity".to_string(), "1".to_string()),
        ]))
        .duration(Duration::from_millis(400))
        .easing(EasingFunction::ease_out_back())
}

pub fn scale_in() -> KeyframeAnimation {
    KeyframeAnimation::new("sh-scale-in")
        .from(HashMap::from([
            ("transform".to_string(), "scale(0.9)".to_string()),
            ("opacity".to_string(), "0".to_string()),
        ]))
        .to(HashMap::from([
            ("transform".to_string(), "scale(1)".to_string()),
            ("opacity".to_string(), "1".to_string()),
        ]))
        .duration(Duration::from_millis(300))
        .easing(EasingFunction::ease_out_expo())
}

pub fn bounce_in() -> KeyframeAnimation {
    KeyframeAnimation::new("sh-bounce-in")
        .keyframe(Keyframe::new(0.0)
            .opacity(0.0)
            .scale(0.3, None))
        .keyframe(Keyframe::new(50.0)
            .opacity(1.0)
            .scale(1.05, None))
        .keyframe(Keyframe::new(70.0)
            .scale(0.9, None))
        .keyframe(Keyframe::new(100.0)
            .scale(1.0, None))
        .duration(Duration::from_millis(600))
        .easing(EasingFunction::ease_out_back())
}

pub fn shake() -> KeyframeAnimation {
    KeyframeAnimation::new("sh-shake")
        .keyframe(Keyframe::new(0.0).transform("translateX(0)"))
        .keyframe(Keyframe::new(10.0).transform("translateX(-10px)"))
        .keyframe(Keyframe::new(20.0).transform("translateX(10px)"))
        .keyframe(Keyframe::new(30.0).transform("translateX(-10px)"))
        .keyframe(Keyframe::new(40.0).transform("translateX(10px)"))
        .keyframe(Keyframe::new(50.0).transform("translateX(-5px)"))
        .keyframe(Keyframe::new(60.0).transform("translateX(5px)"))
        .keyframe(Keyframe::new(70.0).transform("translateX(-2px)"))
        .keyframe(Keyframe::new(80.0).transform("translateX(2px)"))
        .keyframe(Keyframe::new(90.0).transform("translateX(-1px)"))
        .keyframe(Keyframe::new(100.0).transform("translateX(0)"))
        .duration(Duration::from_millis(800))
        .easing(EasingFunction::ease_in_out_cubic())
}

pub fn pulse() -> KeyframeAnimation {
    KeyframeAnimation::new("sh-pulse")
        .keyframe(Keyframe::new(0.0)
            .opacity(1.0))
        .keyframe(Keyframe::new(50.0)
            .opacity(0.5))
        .keyframe(Keyframe::new(100.0)
            .opacity(1.0))
        .duration(Duration::from_millis(2000))
        .iterations(AnimationIterations::Infinite)
        .easing(EasingFunction::ease_in_out_cubic())
}

pub fn glow() -> KeyframeAnimation {
    KeyframeAnimation::new("sh-glow")
        .keyframe(Keyframe::new(0.0)
            .property("box-shadow", "0 0 5px rgba(255,255,255,0.5)"))
        .keyframe(Keyframe::new(50.0)
            .property("box-shadow", "0 0 20px rgba(255,255,255,0.8), 0 0 30px rgba(255,255,255,0.6)"))
        .keyframe(Keyframe::new(100.0)
            .property("box-shadow", "0 0 5px rgba(255,255,255,0.5)"))
        .duration(Duration::from_millis(2000))
        .iterations(AnimationIterations::Infinite)
        .easing(EasingFunction::ease_in_out_cubic())
}

pub fn float() -> KeyframeAnimation {
    KeyframeAnimation::new("sh-float")
        .keyframe(Keyframe::new(0.0)
            .transform("translateY(0px)"))
        .keyframe(Keyframe::new(50.0)
            .transform("translateY(-10px)"))
        .keyframe(Keyframe::new(100.0)
            .transform("translateY(0px)"))
        .duration(Duration::from_millis(3000))
        .iterations(AnimationIterations::Infinite)
        .easing(EasingFunction::ease_in_out_cubic())
}

// Utility function to generate all animation CSS
pub fn generate_all_animations() -> String {
    let animations = vec![
        fade_in(),
        fade_out(),
        slide_in_up(),
        slide_in_down(),
        scale_in(),
        bounce_in(),
        shake(),
        pulse(),
        glow(),
        float(),
    ];

    let mut css = String::new();
    for animation in animations {
        css.push_str(&animation.to_css());
        css.push('\n');
    }
    css
}

// CSS classes for common animations
pub fn animation_classes() -> String {
    format!(
        r#"
/* Animation Classes */
.sh-animate-fade-in {{
  animation: {};
}}

.sh-animate-fade-out {{
  animation: {};
}}

.sh-animate-slide-in-up {{
  animation: {};
}}

.sh-animate-slide-in-down {{
  animation: {};
}}

.sh-animate-scale-in {{
  animation: {};
}}

.sh-animate-bounce-in {{
  animation: {};
}}

.sh-animate-shake {{
  animation: {};
}}

.sh-animate-pulse {{
  animation: {};
}}

.sh-animate-glow {{
  animation: {};
}}

.sh-animate-float {{
  animation: {};
}}

/* Animation utilities */
.sh-animate-none {{
  animation: none !important;
}}

.sh-animate-paused {{
  animation-play-state: paused !important;
}}

.sh-animate-running {{
  animation-play-state: running !important;
}}

/* Reduced motion support */
@media (prefers-reduced-motion: reduce) {{
  .sh-animate-fade-in,
  .sh-animate-fade-out,
  .sh-animate-slide-in-up,
  .sh-animate-slide-in-down,
  .sh-animate-scale-in,
  .sh-animate-bounce-in,
  .sh-animate-shake,
  .sh-animate-pulse,
  .sh-animate-glow,
  .sh-animate-float {{
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
  }}
}}
"#,
        fade_in().to_animation_css(),
        fade_out().to_animation_css(),
        slide_in_up().to_animation_css(),
        slide_in_down().to_animation_css(),
        scale_in().to_animation_css(),
        bounce_in().to_animation_css(),
        shake().to_animation_css(),
        pulse().to_animation_css(),
        glow().to_animation_css(),
        float().to_animation_css(),
    )
}