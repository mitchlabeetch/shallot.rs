//! Aspect Ratio Container - Maintains consistent aspect ratios
//! CSS-only using padding-bottom technique

use maud::{html, Markup, Render};

/// Aspect ratio variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum AspectRatioValue {
    #[default]
    Square, // 1:1
    Video,     // 16:9
    Wide,      // 21:9
    Classic,   // 4:3
    Portrait,  // 3:4
    UltraWide, // 32:9
    Custom(f32, f32),
}

impl AspectRatioValue {
    fn padding_bottom(&self) -> String {
        match self {
            AspectRatioValue::Square => "100%".to_string(),
            AspectRatioValue::Video => "56.25%".to_string(),
            AspectRatioValue::Wide => "42.86%".to_string(),
            AspectRatioValue::Classic => "75%".to_string(),
            AspectRatioValue::Portrait => "133.33%".to_string(),
            AspectRatioValue::UltraWide => "28.125%".to_string(),
            AspectRatioValue::Custom(w, h) => format!("{}%", (h / w) * 100.0),
        }
    }
}

/// Aspect Ratio container component
#[derive(Debug, Clone)]
pub struct AspectRatioContainer<'a> {
    pub ratio: AspectRatioValue,
    pub children: Markup,
    pub class: Option<&'a str>,
}

impl<'a> AspectRatioContainer<'a> {
    pub fn new(ratio: AspectRatioValue, children: Markup) -> Self {
        Self {
            ratio,
            children,
            class: None,
        }
    }

    pub fn class(mut self, class: &'a str) -> Self {
        self.class = Some(class);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-aspect-ratio"];
        if let Some(custom) = self.class {
            classes.push(custom);
        }
        classes.join(" ")
    }
}

impl<'a> Render for AspectRatioContainer<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let padding = self.ratio.padding_bottom();

        html! {
            div
                class=(classes)
                style=(format!("--sh-aspect-ratio: {}", padding))
                role="group"
                aria-label="Aspect ratio container"
            {
                div class="sh-aspect-ratio__content" {
                    (self.children)
                }
            }
        }
    }
}

pub fn aspect_ratio_css() -> String {
    r#"
.sh-aspect-ratio {
    position: relative;
    width: 100%;
    padding-bottom: var(--sh-aspect-ratio, 100%);
}

.sh-aspect-ratio__content {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    overflow: hidden;
}

.sh-aspect-ratio__content > * {
    width: 100%;
    height: 100%;
    object-fit: cover;
}

.sh-aspect-ratio__content > img,
.sh-aspect-ratio__content > video {
    object-fit: cover;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aspect_ratio_creation() {
        let container = AspectRatioContainer::new(AspectRatioValue::Video, html! { "Content" });
        assert_eq!(container.ratio, AspectRatioValue::Video);
    }

    #[test]
    fn test_aspect_ratio_render() {
        let container = AspectRatioContainer::new(AspectRatioValue::Square, html! { "Test" });
        let html = container.render().into_string();
        assert!(html.contains("sh-aspect-ratio"));
    }

    #[test]
    fn test_padding_bottom_values() {
        assert_eq!(AspectRatioValue::Square.padding_bottom(), "100%");
        assert_eq!(AspectRatioValue::Video.padding_bottom(), "56.25%");
        assert_eq!(AspectRatioValue::Wide.padding_bottom(), "42.86%");
        assert_eq!(AspectRatioValue::Classic.padding_bottom(), "75%");
    }

    #[test]
    fn test_custom_ratio() {
        let custom = AspectRatioValue::Custom(2.0, 3.0);
        assert_eq!(custom.padding_bottom(), "150%");
    }

    #[test]
    fn test_css_generation() {
        let css = aspect_ratio_css();
        assert!(css.contains(".sh-aspect-ratio"));
    }
}
