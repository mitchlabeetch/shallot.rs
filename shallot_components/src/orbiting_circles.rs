use maud::{html, Markup, Render};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OrbitingCirclesVariant {
    #[default]
    Default,
    Primary,
    Secondary,
    Accent,
    Gradient,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OrbitingCirclesSize {
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OrbitDirection {
    #[default]
    Clockwise,
    CounterClockwise,
}

pub struct OrbitingCircle {
    pub icon: &'static str,
    pub radius: u32,
    pub delay_s: f32,
}

impl OrbitingCircle {
    pub fn new(icon: &'static str, radius: u32) -> Self {
        Self {
            icon,
            radius,
            delay_s: 0.0,
        }
    }

    pub fn delay(mut self, delay_s: f32) -> Self {
        self.delay_s = delay_s;
        self
    }
}

pub struct OrbitingCircles {
    pub circles: Vec<OrbitingCircle>,
    pub duration_s: f32,
    pub variant: OrbitingCirclesVariant,
    pub size: OrbitingCirclesSize,
    pub direction: OrbitDirection,
    pub center_icon: Option<&'static str>,
}

impl Default for OrbitingCircles {
    fn default() -> Self {
        Self {
            circles: Vec::new(),
            duration_s: 20.0,
            variant: OrbitingCirclesVariant::Default,
            size: OrbitingCirclesSize::Md,
            direction: OrbitDirection::Clockwise,
            center_icon: None,
        }
    }
}

impl OrbitingCircles {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn circle(mut self, circle: OrbitingCircle) -> Self {
        self.circles.push(circle);
        self
    }

    pub fn circles(mut self, circles: Vec<OrbitingCircle>) -> Self {
        self.circles = circles;
        self
    }

    pub fn duration(mut self, duration_s: f32) -> Self {
        self.duration_s = duration_s;
        self
    }

    pub fn variant(mut self, variant: OrbitingCirclesVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: OrbitingCirclesSize) -> Self {
        self.size = size;
        self
    }

    pub fn direction(mut self, direction: OrbitDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn center_icon(mut self, icon: &'static str) -> Self {
        self.center_icon = Some(icon);
        self
    }
}

impl Render for OrbitingCircles {
    fn render(&self) -> Markup {
        let variant_class = match self.variant {
            OrbitingCirclesVariant::Default => "sh-orbit",
            OrbitingCirclesVariant::Primary => "sh-orbit sh-orbit--primary",
            OrbitingCirclesVariant::Secondary => "sh-orbit sh-orbit--secondary",
            OrbitingCirclesVariant::Accent => "sh-orbit sh-orbit--accent",
            OrbitingCirclesVariant::Gradient => "sh-orbit sh-orbit--gradient",
        };

        let size_class = match self.size {
            OrbitingCirclesSize::Sm => "sh-orbit--sm",
            OrbitingCirclesSize::Md => "sh-orbit--md",
            OrbitingCirclesSize::Lg => "sh-orbit--lg",
            OrbitingCirclesSize::Xl => "sh-orbit--xl",
        };

        let dir_value = match self.direction {
            OrbitDirection::Clockwise => "normal",
            OrbitDirection::CounterClockwise => "reverse",
        };

        let style = format!(
            "--sh-orbit-dur: {}s; --sh-orbit-dir: {};",
            self.duration_s.max(5.0),
            dir_value
        );

        html! {
            div class=(format!("{} {}", variant_class, size_class)) style=(style) role="img" aria-label="Orbiting icons animation" {
                @if let Some(center) = self.center_icon {
                    div class="sh-orbit__center" {
                        img src=(center) alt="" loading="lazy";
                    }
                }
                @for circle in &self.circles {
                    div class="sh-orbit__circle" style=(format!("--sh-orbit-r: {}px; --sh-orbit-delay: {}s;", circle.radius, circle.delay_s)) {
                        div class="sh-orbit__icon" {
                            img src=(circle.icon) alt="" loading="lazy";
                        }
                    }
                }
            }
        }
    }
}

pub fn orbiting_circles_css() -> String {
    r#"
.sh-orbit {
    --sh-orbit-dur: 20s;
    --sh-orbit-dir: normal;
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
}

.sh-orbit--sm {
    min-width: 120px;
    min-height: 120px;
}

.sh-orbit--md {
    min-width: 200px;
    min-height: 200px;
}

.sh-orbit--lg {
    min-width: 300px;
    min-height: 300px;
}

.sh-orbit--xl {
    min-width: 400px;
    min-height: 400px;
}

.sh-orbit__center {
    position: absolute;
    z-index: 10;
    width: 48px;
    height: 48px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--sh-surface, #ffffff);
    border-radius: 50%;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.sh-orbit__center img {
    width: 32px;
    height: 32px;
    object-fit: contain;
}

.sh-orbit__circle {
    --sh-orbit-r: 80px;
    --sh-orbit-delay: 0s;
    position: absolute;
    width: var(--sh-orbit-r);
    height: var(--sh-orbit-r);
    animation: orbit-spin var(--sh-orbit-dur) linear infinite;
    animation-direction: var(--sh-orbit-dir);
    animation-delay: var(--sh-orbit-delay);
}

.sh-orbit__icon {
    position: absolute;
    top: 0;
    left: 50%;
    transform: translateX(-50%);
    width: 40px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--sh-surface, #ffffff);
    border-radius: 50%;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
    animation: orbit-counter-spin var(--sh-orbit-dur) linear infinite;
    animation-direction: var(--sh-orbit-dir);
    animation-delay: var(--sh-orbit-delay);
}

.sh-orbit__icon img {
    width: 24px;
    height: 24px;
    object-fit: contain;
}

@keyframes orbit-spin {
    from {
        transform: rotate(0deg);
    }
    to {
        transform: rotate(360deg);
    }
}

@keyframes orbit-counter-spin {
    from {
        transform: translateX(-50%) rotate(0deg);
    }
    to {
        transform: translateX(-50%) rotate(-360deg);
    }
}

.sh-orbit--primary .sh-orbit__icon {
    background: var(--sh-primary, #3b82f6);
}

.sh-orbit--primary .sh-orbit__icon img {
    filter: brightness(0) invert(1);
}

.sh-orbit--secondary .sh-orbit__icon {
    background: var(--sh-secondary, #6b7280);
}

.sh-orbit--secondary .sh-orbit__icon img {
    filter: brightness(0) invert(1);
}

.sh-orbit--accent .sh-orbit__icon {
    background: var(--sh-accent, #10b981);
}

.sh-orbit--accent .sh-orbit__icon img {
    filter: brightness(0) invert(1);
}

.sh-orbit--gradient .sh-orbit__icon {
    background: linear-gradient(135deg, var(--sh-primary, #3b82f6), var(--sh-accent, #10b981));
}

.sh-orbit--gradient .sh-orbit__icon img {
    filter: brightness(0) invert(1);
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orbiting_circles_default() {
        let orbit = OrbitingCircles::new();
        let rendered = orbit.render();
        assert!(rendered.0.as_str().contains("sh-orbit"));
    }

    #[test]
    fn test_orbiting_circles_with_circle() {
        let orbit = OrbitingCircles::new().circle(OrbitingCircle::new("/icon.svg", 80));
        let rendered = orbit.render();
        assert!(rendered.0.as_str().contains("sh-orbit__circle"));
        assert!(rendered.0.as_str().contains("/icon.svg"));
    }

    #[test]
    fn test_orbiting_circles_variant_primary() {
        let orbit = OrbitingCircles::new().variant(OrbitingCirclesVariant::Primary);
        let rendered = orbit.render();
        assert!(rendered.0.as_str().contains("sh-orbit--primary"));
    }

    #[test]
    fn test_orbiting_circles_variant_secondary() {
        let orbit = OrbitingCircles::new().variant(OrbitingCirclesVariant::Secondary);
        let rendered = orbit.render();
        assert!(rendered.0.as_str().contains("sh-orbit--secondary"));
    }

    #[test]
    fn test_orbiting_circles_variant_accent() {
        let orbit = OrbitingCircles::new().variant(OrbitingCirclesVariant::Accent);
        let rendered = orbit.render();
        assert!(rendered.0.as_str().contains("sh-orbit--accent"));
    }

    #[test]
    fn test_orbiting_circles_variant_gradient() {
        let orbit = OrbitingCircles::new().variant(OrbitingCirclesVariant::Gradient);
        let rendered = orbit.render();
        assert!(rendered.0.as_str().contains("sh-orbit--gradient"));
    }

    #[test]
    fn test_orbiting_circles_size_sm() {
        let orbit = OrbitingCircles::new().size(OrbitingCirclesSize::Sm);
        let rendered = orbit.render();
        assert!(rendered.0.as_str().contains("sh-orbit--sm"));
    }

    #[test]
    fn test_orbiting_circles_size_lg() {
        let orbit = OrbitingCircles::new().size(OrbitingCirclesSize::Lg);
        let rendered = orbit.render();
        assert!(rendered.0.as_str().contains("sh-orbit--lg"));
    }

    #[test]
    fn test_orbiting_circles_direction_counter() {
        let orbit = OrbitingCircles::new().direction(OrbitDirection::CounterClockwise);
        let rendered = orbit.render();
        assert!(rendered.0.as_str().contains("--sh-orbit-dir: reverse"));
    }

    #[test]
    fn test_orbiting_circles_custom_duration() {
        let orbit = OrbitingCircles::new().duration(30.0);
        let rendered = orbit.render();
        assert!(rendered.0.as_str().contains("--sh-orbit-dur: 30s"));
    }

    #[test]
    fn test_orbiting_circles_center_icon() {
        let orbit = OrbitingCircles::new().center_icon("/center.svg");
        let rendered = orbit.render();
        assert!(rendered.0.as_str().contains("sh-orbit__center"));
        assert!(rendered.0.as_str().contains("/center.svg"));
    }

    #[test]
    fn test_orbiting_circles_a11y() {
        let orbit = OrbitingCircles::new();
        let rendered = orbit.render();
        assert!(rendered.0.as_str().contains("role=\"img\""));
        assert!(rendered.0.as_str().contains("aria-label"));
    }

    #[test]
    fn test_orbiting_circles_css_function() {
        let css = orbiting_circles_css();
        assert!(css.contains(".sh-orbit"));
        assert!(css.contains("@keyframes orbit-spin"));
    }
}
