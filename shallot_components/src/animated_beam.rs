//! Animated Beam Component
//!
//! An animated beam/light that travels between two points.
//! Perfect for showcasing integrations, connections, or data flow.
//! Pure CSS implementation using SVG animations.

use crate::component::Component;
use maud::{html, Markup, Render};

/// Animated beam connecting two points
pub struct AnimatedBeam {
    /// Start X position (percentage 0-100)
    start_x: f32,
    /// Start Y position (percentage 0-100)
    start_y: f32,
    /// End X position (percentage 0-100)
    end_x: f32,
    /// End Y position (percentage 0-100)
    end_y: f32,
    /// Curvature of the beam (0 = straight)
    curvature: f32,
    /// Animation duration in seconds
    duration: f32,
    /// Beam color
    color: String,
    /// Beam width
    width: f32,
    /// Beam opacity
    opacity: f32,
    /// Whether to reverse the animation
    reverse: bool,
    /// Glow effect intensity
    glow: f32,
    /// Container width
    container_width: String,
    /// Container height
    container_height: String,
}

impl AnimatedBeam {
    /// Create a new animated beam
    pub fn new() -> Self {
        Self {
            start_x: 0.0,
            start_y: 50.0,
            end_x: 100.0,
            end_y: 50.0,
            curvature: 30.0,
            duration: 3.0,
            color: "var(--sh-accent)".to_string(),
            width: 2.0,
            opacity: 1.0,
            reverse: false,
            glow: 10.0,
            container_width: "100%".to_string(),
            container_height: "200px".to_string(),
        }
    }

    /// Set start point
    pub fn start(mut self, x: f32, y: f32) -> Self {
        self.start_x = x.clamp(0.0, 100.0);
        self.start_y = y.clamp(0.0, 100.0);
        self
    }

    /// Set end point
    pub fn end(mut self, x: f32, y: f32) -> Self {
        self.end_x = x.clamp(0.0, 100.0);
        self.end_y = y.clamp(0.0, 100.0);
        self
    }

    /// Set curvature (0 = straight line)
    pub fn curvature(mut self, curvature: f32) -> Self {
        self.curvature = curvature;
        self
    }

    /// Set animation duration
    pub fn duration(mut self, duration: f32) -> Self {
        self.duration = duration.max(0.1);
        self
    }

    /// Set beam color
    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = color.into();
        self
    }

    /// Set beam width
    pub fn width(mut self, width: f32) -> Self {
        self.width = width.max(0.1);
        self
    }

    /// Set beam opacity
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Reverse animation direction
    pub fn reverse(mut self, reverse: bool) -> Self {
        self.reverse = reverse;
        self
    }

    /// Set glow intensity
    pub fn glow(mut self, glow: f32) -> Self {
        self.glow = glow.max(0.0);
        self
    }

    /// Set container size
    pub fn container_size(mut self, width: impl Into<String>, height: impl Into<String>) -> Self {
        self.container_width = width.into();
        self.container_height = height.into();
        self
    }

    /// Build the SVG path for the beam
    fn build_path(&self) -> String {
        // Calculate control points for quadratic bezier curve
        let mid_x = (self.start_x + self.end_x) / 2.0;
        let mid_y = (self.start_y + self.end_y) / 2.0;

        // Offset control point for curvature
        let control_x = mid_x;
        let control_y = mid_y + self.curvature;

        format!(
            "M {},{} Q {},{} {},{}",
            self.start_x, self.start_y, control_x, control_y, self.end_x, self.end_y
        )
    }

    fn build_style(&self) -> String {
        format!(
            "--beam-duration: {}s; --beam-color: {}; --beam-width: {}; \
             --beam-opacity: {}; --beam-glow: {}px;",
            self.duration, self.color, self.width, self.opacity, self.glow
        )
    }
}

impl Default for AnimatedBeam {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for AnimatedBeam {
    fn render(&self) -> Markup {
        let path = self.build_path();
        let style = self.build_style();
        let animation_direction = if self.reverse { "reverse" } else { "normal" };

        html! {
            div
                class="sh-animated-beam"
                style=(format!("{} width: {}; height: {};", style, self.container_width, self.container_height))
                role="img"
                aria-label="Animated connection beam"
            {
                svg
                    class="sh-animated-beam__svg"
                    viewBox=(format!("0 0 100 100"))
                    preserveAspectRatio="none"
                    aria-hidden="true"
                    focusable="false"
                {
                    // Gradient definition
                    defs {
                        linearGradient id="beam-gradient" x1="0%" y1="0%" x2="100%" y2="0%" {
                            stop offset="0%" style=(format!("stop-color: {}; stop-opacity: 0", self.color)) {}
                            stop offset="50%" style=(format!("stop-color: {}; stop-opacity: {}", self.color, self.opacity)) {}
                            stop offset="100%" style=(format!("stop-color: {}; stop-opacity: 0", self.color)) {}
                        }

                        filter id="beam-glow" {
                            feGaussianBlur stdDeviation=(format!("{}", self.glow / 10.0)) result="coloredBlur" {}
                            feMerge {
                                feMergeNode in="coloredBlur" {}
                                feMergeNode in="SourceGraphic" {}
                            }
                        }
                    }

                    // Base path (dimmed)
                    path
                        class="sh-animated-beam__path-base"
                        d=(path.clone())
                        stroke=(self.color.clone())
                        stroke-width=(format!("{}", self.width))
                        fill="none"
                        stroke-linecap="round"
                        opacity="0.2"
                    {}

                    // Animated path
                    path
                        class=(format!("sh-animated-beam__path-animated sh-animated-beam__path-animated--{}", animation_direction))
                        d=(path)
                        stroke="url(#beam-gradient)"
                        stroke-width=(format!("{}", self.width * 1.5))
                        fill="none"
                        stroke-linecap="round"
                        filter="url(#beam-glow)"
                    {}

                    // Start point marker
                    circle
                        class="sh-animated-beam__marker"
                        cx=(format!("{}", self.start_x))
                        cy=(format!("{}", self.start_y))
                        r="3"
                        fill=(self.color.clone())
                    {}

                    // End point marker
                    circle
                        class=(format!("sh-animated-beam__marker sh-animated-beam__marker--end sh-animated-beam__marker--{}", animation_direction))
                        cx=(format!("{}", self.end_x))
                        cy=(format!("{}", self.end_y))
                        r="3"
                        fill=(self.color.clone())
                    {}
                }
            }
        }
    }
}

impl Component for AnimatedBeam {
    fn classes(&self) -> String {
        "sh-animated-beam".to_string()
    }
}

/// Multiple beams container
pub struct AnimatedBeamGroup {
    beams: Vec<AnimatedBeam>,
}

impl AnimatedBeamGroup {
    pub fn new(beams: Vec<AnimatedBeam>) -> Self {
        Self { beams }
    }
}

impl Render for AnimatedBeamGroup {
    fn render(&self) -> Markup {
        html! {
            div class="sh-animated-beam-group" {
                @for beam in &self.beams {
                    (beam.render())
                }
            }
        }
    }
}

/// Predefined beam presets
pub mod beam_presets {
    use super::*;

    /// Horizontal beam (left to right)
    pub fn horizontal() -> AnimatedBeam {
        AnimatedBeam::new()
            .start(0.0, 50.0)
            .end(100.0, 50.0)
            .curvature(0.0)
    }

    /// Curved beam (left to right with curve)
    pub fn curved() -> AnimatedBeam {
        AnimatedBeam::new()
            .start(0.0, 50.0)
            .end(100.0, 50.0)
            .curvature(30.0)
    }

    /// Diagonal beam (top-left to bottom-right)
    pub fn diagonal_down() -> AnimatedBeam {
        AnimatedBeam::new()
            .start(0.0, 0.0)
            .end(100.0, 100.0)
            .curvature(20.0)
    }

    /// Diagonal beam (bottom-left to top-right)
    pub fn diagonal_up() -> AnimatedBeam {
        AnimatedBeam::new()
            .start(0.0, 100.0)
            .end(100.0, 0.0)
            .curvature(20.0)
    }

    /// Bidirectional beam (two beams going opposite directions)
    pub fn bidirectional() -> Vec<AnimatedBeam> {
        vec![
            AnimatedBeam::new()
                .start(0.0, 40.0)
                .end(100.0, 40.0)
                .curvature(10.0)
                .color("#9E7AFF"),
            AnimatedBeam::new()
                .start(100.0, 60.0)
                .end(0.0, 60.0)
                .curvature(10.0)
                .color("#FE8BBB")
                .reverse(true),
        ]
    }
}

/// Generate CSS for animated beam
pub fn animated_beam_css() -> String {
    r#"
/* Animated Beam */
.sh-animated-beam {
  position: relative;
  display: block;
}

.sh-animated-beam__svg {
  width: 100%;
  height: 100%;
  overflow: visible;
}

/* Base path styling */
.sh-animated-beam__path-base {
  vector-effect: non-scaling-stroke;
}

/* Animated path */
.sh-animated-beam__path-animated {
  vector-effect: non-scaling-stroke;
  stroke-dasharray: 20, 200;
  stroke-dashoffset: 0;
  animation: beam-travel var(--beam-duration, 3s) linear infinite;
}

.sh-animated-beam__path-animated--reverse {
  animation-direction: reverse;
}

@keyframes beam-travel {
  0% {
    stroke-dashoffset: 220;
  }
  100% {
    stroke-dashoffset: 0;
  }
}

/* Markers */
.sh-animated-beam__marker {
  animation: beam-pulse var(--beam-duration, 3s) ease-in-out infinite;
}

.sh-animated-beam__marker--end {
  animation-delay: calc(var(--beam-duration, 3s) * 0.5);
}

.sh-animated-beam__marker--reverse {
  animation-direction: reverse;
}

@keyframes beam-pulse {
  0%, 100% {
    transform: scale(1);
    opacity: 1;
  }
  50% {
    transform: scale(1.5);
    opacity: 0.5;
  }
}

/* Beam Group */
.sh-animated-beam-group {
  position: relative;
}

.sh-animated-beam-group .sh-animated-beam {
  position: absolute;
  inset: 0;
}

/* Preset: Integration flow */
.sh-animated-beam--integration {
  --beam-color: #6366f1;
  --beam-glow: 15px;
}

/* Preset: Data flow */
.sh-animated-beam--data {
  --beam-color: #10b981;
  --beam-glow: 12px;
}

/* Preset: Energy/Laser */
.sh-animated-beam--energy {
  --beam-color: #f59e0b;
  --beam-glow: 20px;
  --beam-width: 3px;
}

/* Responsive */
@media (max-width: 640px) {
  .sh-animated-beam {
    --beam-glow: calc(var(--beam-glow) * 0.5);
  }
}

/* Reduced motion */
@media (prefers-reduced-motion: reduce) {
  .sh-animated-beam__path-animated {
    animation: none;
    stroke-dasharray: none;
    opacity: 0.5;
  }

  .sh-animated-beam__marker {
    animation: none;
  }
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::beam_presets::*;
    use super::*;

    #[test]
    fn test_animated_beam_creation() {
        let beam = AnimatedBeam::new()
            .start(10.0, 20.0)
            .end(90.0, 80.0)
            .curvature(40.0)
            .duration(2.0)
            .color("#ff0000")
            .width(3.0)
            .glow(15.0);

        assert_eq!(beam.start_x, 10.0);
        assert_eq!(beam.end_x, 90.0);
        assert_eq!(beam.curvature, 40.0);
        assert_eq!(beam.duration, 2.0);
    }

    #[test]
    fn test_beam_path_generation() {
        let beam = AnimatedBeam::new()
            .start(0.0, 50.0)
            .end(100.0, 50.0)
            .curvature(30.0);

        let path = beam.build_path();
        assert!(path.starts_with("M"));
        assert!(path.contains("Q"));
    }

    #[test]
    fn test_preset_horizontal() {
        let beam = horizontal();
        assert_eq!(beam.curvature, 0.0);
    }

    #[test]
    fn test_preset_bidirectional() {
        let beams = bidirectional();
        assert_eq!(beams.len(), 2);
        assert!(beams[1].reverse);
    }
}
