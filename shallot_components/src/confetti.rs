use maud::{html, Markup, Render};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConfettiVariant {
    #[default]
    Default,
    Explosion,
    Rain,
    Side,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConfettiSize {
    Sm,
    #[default]
    Md,
    Lg,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConfettiShape {
    #[default]
    Square,
    Circle,
    Strip,
    Mixed,
}

pub struct Confetti {
    pub count: u32,
    pub variant: ConfettiVariant,
    pub size: ConfettiSize,
    pub shape: ConfettiShape,
    pub duration_s: f32,
    pub colors: Vec<&'static str>,
}

impl Default for Confetti {
    fn default() -> Self {
        Self {
            count: 50,
            variant: ConfettiVariant::Default,
            size: ConfettiSize::Md,
            shape: ConfettiShape::Mixed,
            duration_s: 3.0,
            colors: vec![
                "#ff0000", "#00ff00", "#0000ff", "#ffff00", "#ff00ff", "#00ffff",
            ],
        }
    }
}

impl Confetti {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn count(mut self, count: u32) -> Self {
        self.count = count;
        self
    }

    pub fn variant(mut self, variant: ConfettiVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ConfettiSize) -> Self {
        self.size = size;
        self
    }

    pub fn shape(mut self, shape: ConfettiShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn duration(mut self, duration_s: f32) -> Self {
        self.duration_s = duration_s;
        self
    }

    pub fn colors(mut self, colors: Vec<&'static str>) -> Self {
        self.colors = colors;
        self
    }
}

impl Render for Confetti {
    fn render(&self) -> Markup {
        let variant_class = match self.variant {
            ConfettiVariant::Default => "sh-confetti",
            ConfettiVariant::Explosion => "sh-confetti sh-confetti--explosion",
            ConfettiVariant::Rain => "sh-confetti sh-confetti--rain",
            ConfettiVariant::Side => "sh-confetti sh-confetti--side",
        };

        let size_class = match self.size {
            ConfettiSize::Sm => "sh-confetti--sm",
            ConfettiSize::Md => "sh-confetti--md",
            ConfettiSize::Lg => "sh-confetti--lg",
        };

        let shape_class = match self.shape {
            ConfettiShape::Square => "sh-confetti--square",
            ConfettiShape::Circle => "sh-confetti--circle",
            ConfettiShape::Strip => "sh-confetti--strip",
            ConfettiShape::Mixed => "sh-confetti--mixed",
        };

        let style = format!("--sh-confetti-dur: {}s;", self.duration_s.max(1.0));

        html! {
            div class=(format!("{} {} {}", variant_class, size_class, shape_class)) style=(style) role="presentation" aria-hidden="true" {
                @for i in 0..self.count {
                    @let color = self.colors[i as usize % self.colors.len()];
                    @let delay = (i as f32 * 0.1) % 2.0;
                    @let x_start = (i * 7) % 100;
                    @let x_end = ((i * 13) % 100) as i32 - 50;
                    @let rotation = (i * 37) % 360;
                    @let dur = self.duration_s + ((i as f32 * 0.2) % 2.0);
                    div class="sh-confetti__piece" style=(format!("--sh-confetti-color: {}; --sh-confetti-delay: {}s; --sh-confetti-x-start: {}%; --sh-confetti-x-end: {}%; --sh-confetti-rot: {}deg; --sh-confetti-piece-dur: {}s;", color, delay, x_start, x_end, rotation, dur)) {}
                }
            }
        }
    }
}

pub fn confetti_css() -> String {
    r#"
.sh-confetti {
    --sh-confetti-dur: 3s;
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
    overflow: hidden;
    z-index: 9999;
}

.sh-confetti--sm .sh-confetti__piece {
    width: 6px;
    height: 6px;
}

.sh-confetti--md .sh-confetti__piece {
    width: 10px;
    height: 10px;
}

.sh-confetti--lg .sh-confetti__piece {
    width: 14px;
    height: 14px;
}

.sh-confetti__piece {
    --sh-confetti-color: #ff0000;
    --sh-confetti-delay: 0s;
    --sh-confetti-x-start: 50%;
    --sh-confetti-x-end: 0%;
    --sh-confetti-rot: 0deg;
    --sh-confetti-piece-dur: 3s;
    position: absolute;
    top: -20px;
    left: var(--sh-confetti-x-start);
    background-color: var(--sh-confetti-color);
    animation: confetti-fall var(--sh-confetti-piece-dur) ease-out forwards;
    animation-delay: var(--sh-confetti-delay);
}

.sh-confetti--square .sh-confetti__piece {
    border-radius: 0;
}

.sh-confetti--circle .sh-confetti__piece {
    border-radius: 50%;
}

.sh-confetti--strip .sh-confetti__piece {
    width: 8px !important;
    height: 16px !important;
    border-radius: 2px;
}

.sh-confetti--mixed .sh-confetti__piece:nth-child(3n) {
    border-radius: 50%;
}

.sh-confetti--mixed .sh-confetti__piece:nth-child(3n+1) {
    border-radius: 0;
}

.sh-confetti--mixed .sh-confetti__piece:nth-child(3n+2) {
    width: 8px !important;
    height: 16px !important;
    border-radius: 2px;
}

.sh-confetti--explosion .sh-confetti__piece {
    animation: confetti-explode var(--sh-confetti-piece-dur) ease-out forwards;
    animation-delay: var(--sh-confetti-delay);
}

.sh-confetti--rain .sh-confetti__piece {
    animation: confetti-rain var(--sh-confetti-piece-dur) linear infinite;
    animation-delay: var(--sh-confetti-delay);
}

.sh-confetti--side .sh-confetti__piece {
    animation: confetti-side var(--sh-confetti-piece-dur) ease-out forwards;
    animation-delay: var(--sh-confetti-delay);
}

@keyframes confetti-fall {
    0% {
        transform: translateY(0) translateX(0) rotate(0deg);
        opacity: 1;
    }
    100% {
        transform: translateY(100vh) translateX(calc(var(--sh-confetti-x-end) - var(--sh-confetti-x-start))) rotate(var(--sh-confetti-rot));
        opacity: 0;
    }
}

@keyframes confetti-explode {
    0% {
        transform: translate(-50%, -50%) scale(0);
        opacity: 1;
    }
    50% {
        transform: translate(calc(var(--sh-confetti-x-end) * 1px), calc(var(--sh-confetti-x-end) * 0.5px)) scale(1) rotate(var(--sh-confetti-rot));
        opacity: 1;
    }
    100% {
        transform: translate(calc(var(--sh-confetti-x-end) * 2px), 100vh) scale(0.5) rotate(calc(var(--sh-confetti-rot) * 2));
        opacity: 0;
    }
}

@keyframes confetti-rain {
    0% {
        transform: translateY(-20px) rotate(0deg);
        opacity: 1;
    }
    100% {
        transform: translateY(100vh) rotate(720deg);
        opacity: 0.5;
    }
}

@keyframes confetti-side {
    0% {
        transform: translateX(-100px) translateY(calc(var(--sh-confetti-x-start) * 1vh)) rotate(0deg);
        opacity: 1;
    }
    100% {
        transform: translateX(100vw) translateY(calc(var(--sh-confetti-x-end) * 1vh)) rotate(720deg);
        opacity: 0;
    }
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confetti_default() {
        let confetti = Confetti::new();
        let rendered = confetti.render();
        assert!(rendered.0.as_str().contains("sh-confetti"));
    }

    #[test]
    fn test_confetti_variant_explosion() {
        let confetti = Confetti::new().variant(ConfettiVariant::Explosion);
        let rendered = confetti.render();
        assert!(rendered.0.as_str().contains("sh-confetti--explosion"));
    }

    #[test]
    fn test_confetti_variant_rain() {
        let confetti = Confetti::new().variant(ConfettiVariant::Rain);
        let rendered = confetti.render();
        assert!(rendered.0.as_str().contains("sh-confetti--rain"));
    }

    #[test]
    fn test_confetti_variant_side() {
        let confetti = Confetti::new().variant(ConfettiVariant::Side);
        let rendered = confetti.render();
        assert!(rendered.0.as_str().contains("sh-confetti--side"));
    }

    #[test]
    fn test_confetti_size_sm() {
        let confetti = Confetti::new().size(ConfettiSize::Sm);
        let rendered = confetti.render();
        assert!(rendered.0.as_str().contains("sh-confetti--sm"));
    }

    #[test]
    fn test_confetti_size_lg() {
        let confetti = Confetti::new().size(ConfettiSize::Lg);
        let rendered = confetti.render();
        assert!(rendered.0.as_str().contains("sh-confetti--lg"));
    }

    #[test]
    fn test_confetti_shape_circle() {
        let confetti = Confetti::new().shape(ConfettiShape::Circle);
        let rendered = confetti.render();
        assert!(rendered.0.as_str().contains("sh-confetti--circle"));
    }

    #[test]
    fn test_confetti_shape_strip() {
        let confetti = Confetti::new().shape(ConfettiShape::Strip);
        let rendered = confetti.render();
        assert!(rendered.0.as_str().contains("sh-confetti--strip"));
    }

    #[test]
    fn test_confetti_custom_count() {
        let confetti = Confetti::new().count(10);
        let rendered = confetti.render();
        let piece_count = rendered.0.as_str().matches("sh-confetti__piece").count();
        assert_eq!(piece_count, 10);
    }

    #[test]
    fn test_confetti_custom_duration() {
        let confetti = Confetti::new().duration(5.0);
        let rendered = confetti.render();
        assert!(rendered.0.as_str().contains("--sh-confetti-dur: 5s"));
    }

    #[test]
    fn test_confetti_a11y() {
        let confetti = Confetti::new();
        let rendered = confetti.render();
        assert!(rendered.0.as_str().contains("role=\"presentation\""));
        assert!(rendered.0.as_str().contains("aria-hidden=\"true\""));
    }

    #[test]
    fn test_confetti_css_function() {
        let css = confetti_css();
        assert!(css.contains(".sh-confetti"));
        assert!(css.contains("@keyframes confetti-fall"));
    }
}
