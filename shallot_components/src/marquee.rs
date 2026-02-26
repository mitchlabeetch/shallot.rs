use maud::{html, Markup, Render};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MarqueeVariant {
    #[default]
    Default,
    Seamless,
    Gradient,
    Bordered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MarqueeDirection {
    #[default]
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MarqueeSize {
    Sm,
    #[default]
    Md,
    Lg,
}

pub struct Marquee {
    pub items: Vec<Markup>,
    pub speed_s: f32,
    pub pause_on_hover: bool,
    pub variant: MarqueeVariant,
    pub direction: MarqueeDirection,
    pub size: MarqueeSize,
    pub gap: u32,
}

impl Default for Marquee {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            speed_s: 18.0,
            pause_on_hover: true,
            variant: MarqueeVariant::Default,
            direction: MarqueeDirection::Left,
            size: MarqueeSize::Md,
            gap: 16,
        }
    }
}

impl Marquee {
    pub fn new(items: Vec<Markup>) -> Self {
        Self::default().items(items)
    }

    pub fn speed(mut self, speed_s: f32) -> Self {
        self.speed_s = speed_s;
        self
    }

    pub fn pause_on_hover(mut self, pause: bool) -> Self {
        self.pause_on_hover = pause;
        self
    }

    pub fn variant(mut self, variant: MarqueeVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn direction(mut self, direction: MarqueeDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn size(mut self, size: MarqueeSize) -> Self {
        self.size = size;
        self
    }

    pub fn gap(mut self, gap: u32) -> Self {
        self.gap = gap;
        self
    }

    pub fn item(mut self, item: Markup) -> Self {
        self.items.push(item);
        self
    }

    pub fn items(mut self, items: Vec<Markup>) -> Self {
        self.items = items;
        self
    }
}

impl Render for Marquee {
    fn render(&self) -> Markup {
        let variant_class = match self.variant {
            MarqueeVariant::Default => "sh-marquee",
            MarqueeVariant::Seamless => "sh-marquee sh-marquee--seamless",
            MarqueeVariant::Gradient => "sh-marquee sh-marquee--gradient",
            MarqueeVariant::Bordered => "sh-marquee sh-marquee--bordered",
        };

        let size_class = match self.size {
            MarqueeSize::Sm => "sh-marquee--sm",
            MarqueeSize::Md => "sh-marquee--md",
            MarqueeSize::Lg => "sh-marquee--lg",
        };

        let pause_class = if self.pause_on_hover {
            "sh-marquee--pause"
        } else {
            ""
        };

        let direction_value = match self.direction {
            MarqueeDirection::Left => "normal",
            MarqueeDirection::Right => "reverse",
        };

        let direction_class = match self.direction {
            MarqueeDirection::Left => "",
            MarqueeDirection::Right => "sh-marquee--right",
        };

        let style = format!(
            "--sh-marquee-dur: {}s; --sh-marquee-gap: {}px; --sh-marquee-dir: {};",
            self.speed_s.max(4.0),
            self.gap,
            direction_value
        );

        html! {
            div class=(format!("{} {} {} {}", variant_class, size_class, pause_class, direction_class)) style=(style) role="region" aria-label="Marquee" {
                div class="sh-marquee__track" {
                    @for it in &self.items { div class="sh-marquee__item" { (it) } }
                    @for it in &self.items { div class="sh-marquee__item" { (it) } }
                }
            }
        }
    }
}

pub fn marquee_css() -> String {
    r#"
.sh-marquee {
    --sh-marquee-dur: 18s;
    --sh-marquee-gap: 16px;
    --sh-marquee-dir: normal;
    position: relative;
    overflow: hidden;
    width: 100%;
}

.sh-marquee--sm {
    height: 32px;
}

.sh-marquee--md {
    height: 48px;
}

.sh-marquee--lg {
    height: 64px;
}

.sh-marquee--bordered {
    border: 1px solid var(--sh-border-color, #e5e7eb);
    border-radius: 8px;
}

.sh-marquee--gradient .sh-marquee__track {
    mask-image: linear-gradient(to right, transparent, black 10%, black 90%, transparent);
    -webkit-mask-image: linear-gradient(to right, transparent, black 10%, black 90%, transparent);
}

.sh-marquee--seamless .sh-marquee__item {
    padding-right: 0;
}

.sh-marquee__track {
    display: flex;
    gap: var(--sh-marquee-gap);
    width: max-content;
    animation: scroll var(--sh-marquee-dur) linear infinite;
    animation-direction: var(--sh-marquee-dir);
}

.sh-marquee--pause .sh-marquee__track:hover {
    animation-play-state: paused;
}

.sh-marquee__item {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    white-space: nowrap;
}

@keyframes scroll {
    0% {
        transform: translateX(0);
    }
    100% {
        transform: translateX(-50%);
    }
}

.sh-marquee--right .sh-marquee__track {
    animation-direction: reverse;
}

.sh-marquee--right {
    direction: rtl;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marquee_default() {
        let marquee = Marquee::new(vec![html! { "Item" }]);
        let rendered = marquee.render();
        assert!(rendered.0.as_str().contains("sh-marquee"));
    }

    #[test]
    fn test_marquee_with_items() {
        let marquee = Marquee::new(vec![
            html! { "Item 1" },
            html! { "Item 2" },
            html! { "Item 3" },
        ]);
        let rendered = marquee.render();
        assert!(rendered.0.as_str().contains("Item 1"));
        assert!(rendered.0.as_str().contains("Item 2"));
        assert!(rendered.0.as_str().contains("Item 3"));
    }

    #[test]
    fn test_marquee_variant_seamless() {
        let marquee = Marquee::new(vec![html! { "Test" }]).variant(MarqueeVariant::Seamless);
        let rendered = marquee.render();
        assert!(rendered.0.as_str().contains("sh-marquee--seamless"));
    }

    #[test]
    fn test_marquee_variant_gradient() {
        let marquee = Marquee::new(vec![html! { "Test" }]).variant(MarqueeVariant::Gradient);
        let rendered = marquee.render();
        assert!(rendered.0.as_str().contains("sh-marquee--gradient"));
    }

    #[test]
    fn test_marquee_direction_right() {
        let marquee = Marquee::new(vec![html! { "Test" }]).direction(MarqueeDirection::Right);
        let rendered = marquee.render();
        assert!(rendered.0.as_str().contains("sh-marquee--right"));
    }

    #[test]
    fn test_marquee_size_sm() {
        let marquee = Marquee::new(vec![html! { "Test" }]).size(MarqueeSize::Sm);
        let rendered = marquee.render();
        assert!(rendered.0.as_str().contains("sh-marquee--sm"));
    }

    #[test]
    fn test_marquee_size_lg() {
        let marquee = Marquee::new(vec![html! { "Test" }]).size(MarqueeSize::Lg);
        let rendered = marquee.render();
        assert!(rendered.0.as_str().contains("sh-marquee--lg"));
    }

    #[test]
    fn test_marquee_custom_speed() {
        let marquee = Marquee::new(vec![html! { "Test" }]).speed(25.0);
        let rendered = marquee.render();
        assert!(rendered.0.as_str().contains("--sh-marquee-dur: 25s"));
    }

    #[test]
    fn test_marquee_custom_gap() {
        let marquee = Marquee::new(vec![html! { "Test" }]).gap(32);
        let rendered = marquee.render();
        assert!(rendered.0.as_str().contains("--sh-marquee-gap: 32px"));
    }

    #[test]
    fn test_marquee_a11y() {
        let marquee = Marquee::new(vec![html! { "Test" }]);
        let rendered = marquee.render();
        assert!(rendered.0.as_str().contains("role=\"region\""));
        assert!(rendered.0.as_str().contains("aria-label=\"Marquee\""));
    }

    #[test]
    fn test_marquee_css_function() {
        let css = marquee_css();
        assert!(css.contains(".sh-marquee"));
        assert!(css.contains("@keyframes scroll"));
    }
}
