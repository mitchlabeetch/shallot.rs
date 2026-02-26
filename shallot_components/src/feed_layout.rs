//! Feed Layout - Social media-style feed layout
//! CSS-only vertical feed with optional separators

use maud::{html, Markup, Render};

/// Feed size variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FeedSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Feed layout component
#[derive(Debug, Clone)]
pub struct FeedLayout<'a> {
    pub items: Vec<Markup>,
    pub size: FeedSize,
    pub divided: bool,
    pub centered: bool,
    pub max_width: Option<&'a str>,
}

impl<'a> FeedLayout<'a> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            size: FeedSize::default(),
            divided: true,
            centered: false,
            max_width: None,
        }
    }

    pub fn add(mut self, item: Markup) -> Self {
        self.items.push(item);
        self
    }

    pub fn size(mut self, size: FeedSize) -> Self {
        self.size = size;
        self
    }

    pub fn divided(mut self, divided: bool) -> Self {
        self.divided = divided;
        self
    }

    pub fn centered(mut self, centered: bool) -> Self {
        self.centered = centered;
        self
    }

    pub fn max_width(mut self, width: &'a str) -> Self {
        self.max_width = Some(width);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-feed"];

        match self.size {
            FeedSize::Sm => classes.push("sh-feed--sm"),
            FeedSize::Md => classes.push("sh-feed--md"),
            FeedSize::Lg => classes.push("sh-feed--lg"),
        }

        if self.divided {
            classes.push("sh-feed--divided");
        }

        if self.centered {
            classes.push("sh-feed--centered");
        }

        classes.join(" ")
    }
}

impl<'a> Default for FeedLayout<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Render for FeedLayout<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let style = self
            .max_width
            .map(|w| format!("max-width: {};", w))
            .unwrap_or_default();

        html! {
            div class=(classes) style=(style) role="feed" {
                @for item in &self.items {
                    article class="sh-feed__item" {
                        (item)
                    }
                }
            }
        }
    }
}

pub fn feed_layout_css() -> String {
    r#"
.sh-feed {
    display: flex;
    flex-direction: column;
    width: 100%;
}

.sh-feed--centered {
    margin-left: auto;
    margin-right: auto;
}

.sh-feed--sm .sh-feed__item {
    padding: var(--sh-spacing-sm, 0.5rem) var(--sh-spacing-md, 1rem);
}

.sh-feed--md .sh-feed__item {
    padding: var(--sh-spacing-md, 1rem) var(--sh-spacing-lg, 1.5rem);
}

.sh-feed--lg .sh-feed__item {
    padding: var(--sh-spacing-lg, 1.5rem) var(--sh-spacing-xl, 2rem);
}

.sh-feed--divided .sh-feed__item {
    border-bottom: 1px solid var(--sh-color-border, #e5e5e5);
}

.sh-feed--divided .sh-feed__item:last-child {
    border-bottom: none;
}

.sh-feed__item {
    width: 100%;
}

.sh-feed__item:hover {
    background: var(--sh-color-surface-hover, #f9fafb);
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feed_creation() {
        let feed = FeedLayout::new().size(FeedSize::Lg).centered(true);

        assert_eq!(feed.size, FeedSize::Lg);
        assert!(feed.centered);
    }

    #[test]
    fn test_feed_render() {
        let feed = FeedLayout::new()
            .add(html! { "Post 1" })
            .add(html! { "Post 2" });

        let html = feed.render().into_string();
        assert!(html.contains("sh-feed"));
        assert!(html.contains("Post 1"));
    }

    #[test]
    fn test_feed_divided() {
        let divided = FeedLayout::new().divided(true);
        let not_divided = FeedLayout::new().divided(false);

        assert!(divided.build_classes().contains("sh-feed--divided"));
        assert!(!not_divided.build_classes().contains("sh-feed--divided"));
    }

    #[test]
    fn test_feed_max_width() {
        let feed = FeedLayout::new().max_width("600px");
        let html = feed.render().into_string();

        assert!(html.contains("max-width: 600px"));
    }

    #[test]
    fn test_css_generation() {
        let css = feed_layout_css();
        assert!(css.contains(".sh-feed"));
        assert!(css.contains(".sh-feed__item"));
    }
}
