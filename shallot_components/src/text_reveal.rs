use maud::{html, Markup, Render};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextRevealVariant {
    #[default]
    Default,
    Fade,
    Slide,
    Scale,
    Blur,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextRevealSpeed {
    Slow,
    #[default]
    Normal,
    Fast,
}

pub struct TextReveal {
    pub text: &'static str,
    pub variant: TextRevealVariant,
    pub speed: TextRevealSpeed,
    pub delay_ms: u64,
    pub stagger_ms: u64,
}

impl Default for TextReveal {
    fn default() -> Self {
        Self {
            text: "",
            variant: TextRevealVariant::Default,
            speed: TextRevealSpeed::Normal,
            delay_ms: 0,
            stagger_ms: 100,
        }
    }
}

impl TextReveal {
    pub fn new(text: &'static str) -> Self {
        Self::default().text(text)
    }

    pub fn text(mut self, text: &'static str) -> Self {
        self.text = text;
        self
    }

    pub fn variant(mut self, variant: TextRevealVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn speed(mut self, speed: TextRevealSpeed) -> Self {
        self.speed = speed;
        self
    }

    pub fn delay(mut self, delay_ms: u64) -> Self {
        self.delay_ms = delay_ms;
        self
    }

    pub fn stagger(mut self, stagger_ms: u64) -> Self {
        self.stagger_ms = stagger_ms;
        self
    }
}

impl Render for TextReveal {
    fn render(&self) -> Markup {
        let variant_class = match self.variant {
            TextRevealVariant::Default => "sh-text-reveal",
            TextRevealVariant::Fade => "sh-text-reveal sh-text-reveal--fade",
            TextRevealVariant::Slide => "sh-text-reveal sh-text-reveal--slide",
            TextRevealVariant::Scale => "sh-text-reveal sh-text-reveal--scale",
            TextRevealVariant::Blur => "sh-text-reveal sh-text-reveal--blur",
        };

        let speed_class = match self.speed {
            TextRevealSpeed::Slow => "sh-text-reveal--slow",
            TextRevealSpeed::Normal => "sh-text-reveal--normal",
            TextRevealSpeed::Fast => "sh-text-reveal--fast",
        };

        let words: Vec<&str> = self.text.split_whitespace().collect();
        let style = format!(
            "--sh-reveal-delay: {}ms; --sh-reveal-stagger: {}ms;",
            self.delay_ms, self.stagger_ms
        );

        html! {
            span class=(format!("{} {}", variant_class, speed_class)) style=(style) role="text" aria-label=(self.text) {
                @for (i, word) in words.iter().enumerate() {
                    span class="sh-text-reveal__word" style=(format!("--sh-reveal-word-delay: {}", i as u64 * self.stagger_ms)) {
                        (word) " "
                    }
                }
            }
        }
    }
}

pub fn text_reveal_css() -> String {
    r#"
.sh-text-reveal {
    --sh-reveal-delay: 0ms;
    --sh-reveal-stagger: 100ms;
    --sh-reveal-duration: 0.6s;
    display: inline;
}

.sh-text-reveal--slow {
    --sh-reveal-duration: 1s;
}

.sh-text-reveal--normal {
    --sh-reveal-duration: 0.6s;
}

.sh-text-reveal--fast {
    --sh-reveal-duration: 0.3s;
}

.sh-text-reveal__word {
    display: inline-block;
    opacity: 0;
    animation: reveal-default var(--sh-reveal-duration) ease-out forwards;
    animation-delay: calc(var(--sh-reveal-delay) + var(--sh-reveal-word-delay) * 1ms);
}

.sh-text-reveal--fade .sh-text-reveal__word {
    animation: reveal-fade var(--sh-reveal-duration) ease-out forwards;
    animation-delay: calc(var(--sh-reveal-delay) + var(--sh-reveal-word-delay) * 1ms);
}

.sh-text-reveal--slide .sh-text-reveal__word {
    animation: reveal-slide var(--sh-reveal-duration) ease-out forwards;
    animation-delay: calc(var(--sh-reveal-delay) + var(--sh-reveal-word-delay) * 1ms);
}

.sh-text-reveal--scale .sh-text-reveal__word {
    animation: reveal-scale var(--sh-reveal-duration) ease-out forwards;
    animation-delay: calc(var(--sh-reveal-delay) + var(--sh-reveal-word-delay) * 1ms);
}

.sh-text-reveal--blur .sh-text-reveal__word {
    animation: reveal-blur var(--sh-reveal-duration) ease-out forwards;
    animation-delay: calc(var(--sh-reveal-delay) + var(--sh-reveal-word-delay) * 1ms);
}

@keyframes reveal-default {
    0% {
        opacity: 0;
        transform: translateY(20px);
    }
    100% {
        opacity: 1;
        transform: translateY(0);
    }
}

@keyframes reveal-fade {
    0% {
        opacity: 0;
    }
    100% {
        opacity: 1;
    }
}

@keyframes reveal-slide {
    0% {
        opacity: 0;
        transform: translateX(-20px);
    }
    100% {
        opacity: 1;
        transform: translateX(0);
    }
}

@keyframes reveal-scale {
    0% {
        opacity: 0;
        transform: scale(0.5);
    }
    100% {
        opacity: 1;
        transform: scale(1);
    }
}

@keyframes reveal-blur {
    0% {
        opacity: 0;
        filter: blur(10px);
    }
    100% {
        opacity: 1;
        filter: blur(0);
    }
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_reveal_default() {
        let reveal = TextReveal::new("Hello World");
        let rendered = reveal.render();
        assert!(rendered.0.as_str().contains("sh-text-reveal"));
    }

    #[test]
    fn test_text_reveal_words() {
        let reveal = TextReveal::new("Hello World Test");
        let rendered = reveal.render();
        assert!(rendered.0.as_str().contains("Hello"));
        assert!(rendered.0.as_str().contains("World"));
        assert!(rendered.0.as_str().contains("Test"));
    }

    #[test]
    fn test_text_reveal_variant_fade() {
        let reveal = TextReveal::new("Test").variant(TextRevealVariant::Fade);
        let rendered = reveal.render();
        assert!(rendered.0.as_str().contains("sh-text-reveal--fade"));
    }

    #[test]
    fn test_text_reveal_variant_slide() {
        let reveal = TextReveal::new("Test").variant(TextRevealVariant::Slide);
        let rendered = reveal.render();
        assert!(rendered.0.as_str().contains("sh-text-reveal--slide"));
    }

    #[test]
    fn test_text_reveal_variant_scale() {
        let reveal = TextReveal::new("Test").variant(TextRevealVariant::Scale);
        let rendered = reveal.render();
        assert!(rendered.0.as_str().contains("sh-text-reveal--scale"));
    }

    #[test]
    fn test_text_reveal_variant_blur() {
        let reveal = TextReveal::new("Test").variant(TextRevealVariant::Blur);
        let rendered = reveal.render();
        assert!(rendered.0.as_str().contains("sh-text-reveal--blur"));
    }

    #[test]
    fn test_text_reveal_speed_slow() {
        let reveal = TextReveal::new("Test").speed(TextRevealSpeed::Slow);
        let rendered = reveal.render();
        assert!(rendered.0.as_str().contains("sh-text-reveal--slow"));
    }

    #[test]
    fn test_text_reveal_speed_fast() {
        let reveal = TextReveal::new("Test").speed(TextRevealSpeed::Fast);
        let rendered = reveal.render();
        assert!(rendered.0.as_str().contains("sh-text-reveal--fast"));
    }

    #[test]
    fn test_text_reveal_custom_delay() {
        let reveal = TextReveal::new("Test").delay(500);
        let rendered = reveal.render();
        assert!(rendered.0.as_str().contains("--sh-reveal-delay: 500ms"));
    }

    #[test]
    fn test_text_reveal_custom_stagger() {
        let reveal = TextReveal::new("Test").stagger(200);
        let rendered = reveal.render();
        assert!(rendered.0.as_str().contains("--sh-reveal-stagger: 200ms"));
    }

    #[test]
    fn test_text_reveal_a11y() {
        let reveal = TextReveal::new("Hello World");
        let rendered = reveal.render();
        assert!(rendered.0.as_str().contains("role=\"text\""));
        assert!(rendered.0.as_str().contains("aria-label=\"Hello World\""));
    }

    #[test]
    fn test_text_reveal_css_function() {
        let css = text_reveal_css();
        assert!(css.contains(".sh-text-reveal"));
        assert!(css.contains("@keyframes reveal-default"));
    }
}
