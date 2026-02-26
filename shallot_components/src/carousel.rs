//! Carousel Component - Image/content slider with navigation

use maud::{html, Markup, Render};

/// Carousel animation type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CarouselAnimation {
    None,
    #[default]
    Slide,
    Fade,
}

/// Carousel indicator style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CarouselIndicatorStyle {
    #[default]
    Dots,
    Lines,
    Numbers,
    None,
}

/// Carousel Component
#[derive(Debug, Clone)]
pub struct Carousel<'a> {
    pub items: Vec<Markup>,
    pub animation: CarouselAnimation,
    pub indicator_style: CarouselIndicatorStyle,
    pub autoplay: bool,
    pub autoplay_interval_ms: u16,
    pub show_arrows: bool,
    pub aria_label: Option<&'a str>,
    pub active_index: usize,
}

impl<'a> Default for Carousel<'a> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            animation: CarouselAnimation::default(),
            indicator_style: CarouselIndicatorStyle::default(),
            autoplay: false,
            autoplay_interval_ms: 5000,
            show_arrows: true,
            aria_label: None,
            active_index: 0,
        }
    }
}

impl<'a> Carousel<'a> {
    pub fn new(items: Vec<Markup>) -> Self {
        Self {
            items,
            ..Self::default()
        }
    }

    pub fn animation(mut self, animation: CarouselAnimation) -> Self {
        self.animation = animation;
        self
    }

    pub fn indicator_style(mut self, style: CarouselIndicatorStyle) -> Self {
        self.indicator_style = style;
        self
    }

    pub fn autoplay(mut self, autoplay: bool) -> Self {
        self.autoplay = autoplay;
        self
    }

    pub fn autoplay_interval(mut self, interval_ms: u16) -> Self {
        self.autoplay_interval_ms = interval_ms;
        self
    }

    pub fn show_arrows(mut self, show: bool) -> Self {
        self.show_arrows = show;
        self
    }

    pub fn aria_label(mut self, label: &'a str) -> Self {
        self.aria_label = Some(label);
        self
    }

    pub fn active_index(mut self, index: usize) -> Self {
        self.active_index = index.min(self.items.len().saturating_sub(1));
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-carousel"];

        match self.animation {
            CarouselAnimation::None => classes.push("sh-carousel--no-animation"),
            CarouselAnimation::Slide => classes.push("sh-carousel--slide"),
            CarouselAnimation::Fade => classes.push("sh-carousel--fade"),
        }

        match self.indicator_style {
            CarouselIndicatorStyle::Dots => classes.push("sh-carousel--dots"),
            CarouselIndicatorStyle::Lines => classes.push("sh-carousel--lines"),
            CarouselIndicatorStyle::Numbers => classes.push("sh-carousel--numbers"),
            CarouselIndicatorStyle::None => classes.push("sh-carousel--no-indicators"),
        }

        if self.autoplay {
            classes.push("sh-carousel--autoplay");
        }

        classes.join(" ")
    }
}

impl<'a> Render for Carousel<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let style = format!("--sh-carousel-interval: {}ms;", self.autoplay_interval_ms);
        let item_count = self.items.len();

        // Pre-compute aria-hidden values
        let aria_hidden: Vec<&'static str> = self.items.iter().enumerate().map(|(idx, _)| {
            if idx == self.active_index { "false" } else { "true" }
        }).collect();

        // Pre-compute aria-selected values for indicators
        let aria_selected_indicators: Vec<&'static str> = (0..item_count).map(|idx| {
            if idx == self.active_index { "true" } else { "false" }
        }).collect();

        html! {
            div
                class=(classes)
                style=(style)
                role="region"
                aria-label=(self.aria_label.unwrap_or("Carousel"))
                aria-roledescription="carousel"
            {
                div class="sh-carousel__viewport" {
                    div class="sh-carousel__track" {
                        @for (idx, item) in self.items.iter().enumerate() {
                            div
                                class="sh-carousel__item"
                                role="group"
                                aria-roledescription="slide"
                                aria-label={(format!("Slide {} of {}", idx + 1, item_count))}
                                aria-hidden=(aria_hidden[idx])
                            {
                                (item)
                            }
                        }
                    }
                }

                @if self.show_arrows && item_count > 1 {
                    button
                        class="sh-carousel__arrow sh-carousel__arrow--prev"
                        type="button"
                        aria-label="Previous slide"
                    {
                        span class="sh-carousel__arrow-icon" { "‹" }
                    }
                    button
                        class="sh-carousel__arrow sh-carousel__arrow--next"
                        type="button"
                        aria-label="Next slide"
                    {
                        span class="sh-carousel__arrow-icon" { "›" }
                    }
                }

                @if self.indicator_style != CarouselIndicatorStyle::None && item_count > 1 {
                    div class="sh-carousel__indicators" role="tablist" {
                        @for idx in 0..item_count {
                            button
                                class="sh-carousel__indicator"
                                type="button"
                                role="tab"
                                aria-selected=(aria_selected_indicators[idx])
                                aria-label={(format!("Go to slide {}", idx + 1))}
                            {
                                @if self.indicator_style == CarouselIndicatorStyle::Numbers {
                                    (idx + 1)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Generate CSS for carousel components
pub fn carousel_css() -> String {
    r#"
/* Carousel Component Styles */
.sh-carousel {
    position: relative;
    width: 100%;
    overflow: hidden;
}

.sh-carousel__viewport {
    overflow: hidden;
    width: 100%;
}

.sh-carousel__track {
    display: flex;
    transition: transform 0.5s ease;
}

.sh-carousel--no-animation .sh-carousel__track {
    transition: none;
}

.sh-carousel--fade .sh-carousel__track {
    display: block;
}

.sh-carousel__item {
    flex: 0 0 100%;
    min-width: 100%;
}

.sh-carousel--fade .sh-carousel__item {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    opacity: 0;
    transition: opacity 0.5s ease;
}

.sh-carousel--fade .sh-carousel__item:first-child {
    position: relative;
}

/* Arrows */
.sh-carousel__arrow {
    position: absolute;
    top: 50%;
    transform: translateY(-50%);
    width: 40px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(0, 0, 0, 0.5);
    color: white;
    border: none;
    border-radius: 50%;
    cursor: pointer;
    z-index: 10;
    transition: background 0.2s;
}

.sh-carousel__arrow:hover {
    background: rgba(0, 0, 0, 0.7);
}

.sh-carousel__arrow--prev {
    left: 1rem;
}

.sh-carousel__arrow--next {
    right: 1rem;
}

.sh-carousel__arrow-icon {
    font-size: 1.5rem;
    line-height: 1;
}

/* Indicators */
.sh-carousel__indicators {
    display: flex;
    justify-content: center;
    gap: 0.5rem;
    padding: 1rem;
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
}

.sh-carousel__indicator {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.5);
    border: none;
    cursor: pointer;
    transition: background 0.2s, transform 0.2s;
}

.sh-carousel__indicator[aria-selected="true"] {
    background: white;
    transform: scale(1.2);
}

/* Lines style */
.sh-carousel--lines .sh-carousel__indicator {
    width: 24px;
    height: 4px;
    border-radius: 2px;
}

/* Numbers style */
.sh-carousel--numbers .sh-carousel__indicator {
    width: auto;
    height: auto;
    padding: 0.25rem 0.5rem;
    font-size: 0.75rem;
    background: rgba(0, 0, 0, 0.5);
    color: white;
    border-radius: 0.25rem;
}

/* Autoplay animation */
.sh-carousel--autoplay .sh-carousel__track {
    animation: carousel-slide var(--sh-carousel-interval, 5s) infinite;
}

/* Reduced motion */
@media (prefers-reduced-motion: reduce) {
    .sh-carousel__track,
    .sh-carousel--fade .sh-carousel__item,
    .sh-carousel__arrow,
    .sh-carousel__indicator {
        transition: none;
    }

    .sh-carousel--autoplay .sh-carousel__track {
        animation: none;
    }
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use maud::html;

    #[test]
    fn test_carousel_creation() {
        let carousel = Carousel::new(vec![html! { "Slide 1" }, html! { "Slide 2" }]);

        assert_eq!(carousel.items.len(), 2);
        assert_eq!(carousel.animation, CarouselAnimation::Slide);
    }

    #[test]
    fn test_carousel_animations() {
        let fade = Carousel::new(vec![]).animation(CarouselAnimation::Fade);
        assert_eq!(fade.animation, CarouselAnimation::Fade);

        let none = Carousel::new(vec![]).animation(CarouselAnimation::None);
        assert_eq!(none.animation, CarouselAnimation::None);
    }

    #[test]
    fn test_carousel_indicator_styles() {
        let lines = Carousel::new(vec![]).indicator_style(CarouselIndicatorStyle::Lines);
        assert_eq!(lines.indicator_style, CarouselIndicatorStyle::Lines);

        let numbers = Carousel::new(vec![]).indicator_style(CarouselIndicatorStyle::Numbers);
        assert_eq!(numbers.indicator_style, CarouselIndicatorStyle::Numbers);
    }

    #[test]
    fn test_carousel_autoplay() {
        let carousel = Carousel::new(vec![]).autoplay(true).autoplay_interval(3000);

        assert!(carousel.autoplay);
        assert_eq!(carousel.autoplay_interval_ms, 3000);
    }

    #[test]
    fn test_carousel_arrows() {
        let carousel = Carousel::new(vec![]).show_arrows(false);
        assert!(!carousel.show_arrows);
    }

    #[test]
    fn test_carousel_css() {
        let css = carousel_css();
        assert!(css.contains(".sh-carousel"));
        assert!(css.contains(".sh-carousel--fade"));
        assert!(css.contains(".sh-carousel__arrow"));
        assert!(css.contains(".sh-carousel__indicator"));
    }
}
