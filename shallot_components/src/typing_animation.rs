use maud::{html, Markup, Render};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TypingAnimationVariant {
    #[default]
    Default,
    Cursor,
    Blinking,
    Fade,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TypingSpeed {
    Slow,
    #[default]
    Normal,
    Fast,
}

pub struct TypingAnimation {
    pub text: &'static str,
    pub variant: TypingAnimationVariant,
    pub speed: TypingSpeed,
    pub duration_s: f32,
    pub cursor_char: &'static str,
    pub loop_animation: bool,
}

impl Default for TypingAnimation {
    fn default() -> Self {
        Self {
            text: "",
            variant: TypingAnimationVariant::Default,
            speed: TypingSpeed::Normal,
            duration_s: 2.0,
            cursor_char: "|",
            loop_animation: false,
        }
    }
}

impl TypingAnimation {
    pub fn new(text: &'static str) -> Self {
        Self::default().text(text)
    }

    pub fn text(mut self, text: &'static str) -> Self {
        self.text = text;
        self
    }

    pub fn variant(mut self, variant: TypingAnimationVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn speed(mut self, speed: TypingSpeed) -> Self {
        self.speed = speed;
        self
    }

    pub fn duration(mut self, duration_s: f32) -> Self {
        self.duration_s = duration_s;
        self
    }

    pub fn cursor(mut self, cursor_char: &'static str) -> Self {
        self.cursor_char = cursor_char;
        self
    }

    pub fn r#loop(mut self) -> Self {
        self.loop_animation = true;
        self
    }
}

impl Render for TypingAnimation {
    fn render(&self) -> Markup {
        let text_len = self.text.len().max(1);
        let variant_class = match self.variant {
            TypingAnimationVariant::Default => "sh-typing",
            TypingAnimationVariant::Cursor => "sh-typing sh-typing--cursor",
            TypingAnimationVariant::Blinking => "sh-typing sh-typing--blinking",
            TypingAnimationVariant::Fade => "sh-typing sh-typing--fade",
        };

        let speed_class = match self.speed {
            TypingSpeed::Slow => "sh-typing--slow",
            TypingSpeed::Normal => "sh-typing--normal",
            TypingSpeed::Fast => "sh-typing--fast",
        };

        let loop_class = if self.loop_animation {
            "sh-typing--loop"
        } else {
            ""
        };

        let style = format!(
            "--sh-typing-len: {}; --sh-typing-dur: {}s; --sh-typing-cursor: '{}';",
            text_len,
            self.duration_s.max(0.5),
            self.cursor_char
        );

        html! {
            span class=(format!("{} {} {}", variant_class, speed_class, loop_class)) style=(style) role="text" aria-label=(self.text) aria-live="polite" {
                span class="sh-typing__text" {
                    (self.text)
                }
                span class="sh-typing__cursor" aria-hidden="true" {}
            }
        }
    }
}

pub fn typing_animation_css() -> String {
    r#"
.sh-typing {
    --sh-typing-len: 10;
    --sh-typing-dur: 2s;
    --sh-typing-cursor: '|';
    display: inline-flex;
    align-items: baseline;
    font-family: monospace;
}

.sh-typing--slow {
    --sh-typing-char-dur: 0.15s;
}

.sh-typing--normal {
    --sh-typing-char-dur: 0.1s;
}

.sh-typing--fast {
    --sh-typing-char-dur: 0.05s;
}

.sh-typing__text {
    overflow: hidden;
    white-space: nowrap;
    border-right: 0 solid transparent;
    animation: typing-text var(--sh-typing-dur) steps(var(--sh-typing-len)) forwards;
}

.sh-typing__cursor {
    display: inline-block;
    width: 0;
}

.sh-typing__cursor::after {
    content: var(--sh-typing-cursor);
    animation: typing-cursor-blink 0.7s infinite;
}

.sh-typing--cursor .sh-typing__cursor::after {
    animation: typing-cursor-blink 0.7s infinite;
}

.sh-typing--blinking .sh-typing__cursor::after {
    animation: typing-cursor-blink-fast 0.3s infinite;
}

.sh-typing--fade .sh-typing__text {
    animation: typing-text-fade var(--sh-typing-dur) steps(var(--sh-typing-len)) forwards;
}

.sh-typing--loop .sh-typing__text {
    animation: typing-text-loop var(--sh-typing-dur) steps(var(--sh-typing-len)) infinite;
}

@keyframes typing-text {
    from {
        width: 0;
    }
    to {
        width: 100%;
    }
}

@keyframes typing-text-fade {
    from {
        width: 0;
        opacity: 0.5;
    }
    to {
        width: 100%;
        opacity: 1;
    }
}

@keyframes typing-text-loop {
    0%, 100% {
        width: 0;
    }
    50% {
        width: 100%;
    }
}

@keyframes typing-cursor-blink {
    0%, 100% {
        opacity: 1;
    }
    50% {
        opacity: 0;
    }
}

@keyframes typing-cursor-blink-fast {
    0%, 100% {
        opacity: 1;
    }
    50% {
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
    fn test_typing_animation_default() {
        let typing = TypingAnimation::new("Hello");
        let rendered = typing.render();
        assert!(rendered.0.as_str().contains("sh-typing"));
    }

    #[test]
    fn test_typing_animation_text() {
        let typing = TypingAnimation::new("Hello World");
        let rendered = typing.render();
        assert!(rendered.0.as_str().contains("Hello World"));
    }

    #[test]
    fn test_typing_animation_variant_cursor() {
        let typing = TypingAnimation::new("Test").variant(TypingAnimationVariant::Cursor);
        let rendered = typing.render();
        assert!(rendered.0.as_str().contains("sh-typing--cursor"));
    }

    #[test]
    fn test_typing_animation_variant_blinking() {
        let typing = TypingAnimation::new("Test").variant(TypingAnimationVariant::Blinking);
        let rendered = typing.render();
        assert!(rendered.0.as_str().contains("sh-typing--blinking"));
    }

    #[test]
    fn test_typing_animation_variant_fade() {
        let typing = TypingAnimation::new("Test").variant(TypingAnimationVariant::Fade);
        let rendered = typing.render();
        assert!(rendered.0.as_str().contains("sh-typing--fade"));
    }

    #[test]
    fn test_typing_animation_speed_slow() {
        let typing = TypingAnimation::new("Test").speed(TypingSpeed::Slow);
        let rendered = typing.render();
        assert!(rendered.0.as_str().contains("sh-typing--slow"));
    }

    #[test]
    fn test_typing_animation_speed_fast() {
        let typing = TypingAnimation::new("Test").speed(TypingSpeed::Fast);
        let rendered = typing.render();
        assert!(rendered.0.as_str().contains("sh-typing--fast"));
    }

    #[test]
    fn test_typing_animation_custom_duration() {
        let typing = TypingAnimation::new("Test").duration(5.0);
        let rendered = typing.render();
        assert!(rendered.0.as_str().contains("--sh-typing-dur: 5s"));
    }

    #[test]
    fn test_typing_animation_custom_cursor() {
        let typing = TypingAnimation::new("Test").cursor("_");
        let rendered = typing.render();
        assert!(rendered.0.as_str().contains("--sh-typing-cursor: '_'"));
    }

    #[test]
    fn test_typing_animation_loop() {
        let typing = TypingAnimation::new("Test").r#loop();
        let rendered = typing.render();
        assert!(rendered.0.as_str().contains("sh-typing--loop"));
    }

    #[test]
    fn test_typing_animation_a11y() {
        let typing = TypingAnimation::new("Hello");
        let rendered = typing.render();
        assert!(rendered.0.as_str().contains("role=\"text\""));
        assert!(rendered.0.as_str().contains("aria-label=\"Hello\""));
        assert!(rendered.0.as_str().contains("aria-live=\"polite\""));
    }

    #[test]
    fn test_typing_animation_css_function() {
        let css = typing_animation_css();
        assert!(css.contains(".sh-typing"));
        assert!(css.contains("@keyframes typing-text"));
    }
}
