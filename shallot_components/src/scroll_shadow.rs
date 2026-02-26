//! Scroll Shadow Component - Visual indicator for scrollable content
//! CSS-only using gradient overlays

use maud::{html, Markup, Render};

/// Scroll shadow direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrollShadowDirection {
    #[default]
    Vertical,
    Horizontal,
    Both,
}

/// Scroll shadow component
#[derive(Debug, Clone)]
pub struct ScrollShadow<'a> {
    pub content: &'a str,
    pub direction: ScrollShadowDirection,
    pub max_height: Option<&'a str>,
    pub max_width: Option<&'a str>,
}

impl<'a> ScrollShadow<'a> {
    /// Create a new scroll shadow container
    pub fn new(content: &'a str) -> Self {
        Self {
            content,
            direction: ScrollShadowDirection::default(),
            max_height: None,
            max_width: None,
        }
    }

    /// Set the scroll direction
    pub fn direction(mut self, direction: ScrollShadowDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Set the maximum height
    pub fn max_height(mut self, height: &'a str) -> Self {
        self.max_height = Some(height);
        self
    }

    /// Set the maximum width
    pub fn max_width(mut self, width: &'a str) -> Self {
        self.max_width = Some(width);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-scroll-shadow".to_string()];

        let dir_class = match self.direction {
            ScrollShadowDirection::Vertical => "sh-scroll-shadow--vertical",
            ScrollShadowDirection::Horizontal => "sh-scroll-shadow--horizontal",
            ScrollShadowDirection::Both => "sh-scroll-shadow--both",
        };
        classes.push(dir_class.to_string());

        classes.join(" ")
    }

    fn build_styles(&self) -> String {
        let mut styles = Vec::new();

        if let Some(h) = self.max_height {
            styles.push(format!("max-height: {};", h));
        }

        if let Some(w) = self.max_width {
            styles.push(format!("max-width: {};", w));
        }

        styles.join(" ")
    }
}

impl<'a> Render for ScrollShadow<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let styles = self.build_styles();

        html! {
            div
                class=(classes)
                role="region"
                aria-label="Scrollable content with shadow indicators"
            {
                div class="sh-scroll-shadow__top" aria-hidden="true" {}
                div class="sh-scroll-shadow__bottom" aria-hidden="true" {}
                div class="sh-scroll-shadow__left" aria-hidden="true" {}
                div class="sh-scroll-shadow__right" aria-hidden="true" {}
                div
                    class="sh-scroll-shadow__content"
                    style=(styles)
                    tabindex="0"
                {
                    (maud::PreEscaped(self.content))
                }
            }
        }
    }
}

/// Generate CSS for scroll shadow component
pub fn scroll_shadow_css() -> String {
    r#"
.sh-scroll-shadow {
    position: relative;
}

.sh-scroll-shadow__content {
    overflow: auto;
    scrollbar-width: thin;
}

.sh-scroll-shadow__top,
.sh-scroll-shadow__bottom,
.sh-scroll-shadow__left,
.sh-scroll-shadow__right {
    position: absolute;
    pointer-events: none;
    opacity: 0;
    transition: opacity 0.2s ease;
}

.sh-scroll-shadow--vertical .sh-scroll-shadow__top,
.sh-scroll-shadow--both .sh-scroll-shadow__top {
    top: 0;
    left: 0;
    right: 0;
    height: 16px;
    background: linear-gradient(to bottom, var(--sh-color-background, #ffffff), transparent);
}

.sh-scroll-shadow--vertical .sh-scroll-shadow__bottom,
.sh-scroll-shadow--both .sh-scroll-shadow__bottom {
    bottom: 0;
    left: 0;
    right: 0;
    height: 16px;
    background: linear-gradient(to top, var(--sh-color-background, #ffffff), transparent);
}

.sh-scroll-shadow--horizontal .sh-scroll-shadow__left,
.sh-scroll-shadow--both .sh-scroll-shadow__left {
    top: 0;
    left: 0;
    bottom: 0;
    width: 16px;
    background: linear-gradient(to right, var(--sh-color-background, #ffffff), transparent);
}

.sh-scroll-shadow--horizontal .sh-scroll-shadow__right,
.sh-scroll-shadow--both .sh-scroll-shadow__right {
    top: 0;
    right: 0;
    bottom: 0;
    width: 16px;
    background: linear-gradient(to left, var(--sh-color-background, #ffffff), transparent);
}

/* Show shadows based on scroll position */
.sh-scroll-shadow--has-top .sh-scroll-shadow__top,
.sh-scroll-shadow--has-bottom .sh-scroll-shadow__bottom,
.sh-scroll-shadow--has-left .sh-scroll-shadow__left,
.sh-scroll-shadow--has-right .sh-scroll-shadow__right {
    opacity: 1;
}

.sh-scroll-shadow--vertical .sh-scroll-shadow__top,
.sh-scroll-shadow--vertical .sh-scroll-shadow__bottom {
    display: block;
}

.sh-scroll-shadow--vertical .sh-scroll-shadow__left,
.sh-scroll-shadow--vertical .sh-scroll-shadow__right {
    display: none;
}

.sh-scroll-shadow--horizontal .sh-scroll-shadow__top,
.sh-scroll-shadow--horizontal .sh-scroll-shadow__bottom {
    display: none;
}

.sh-scroll-shadow--horizontal .sh-scroll-shadow__left,
.sh-scroll-shadow--horizontal .sh-scroll-shadow__right {
    display: block;
}

.sh-scroll-shadow--both .sh-scroll-shadow__top,
.sh-scroll-shadow--both .sh-scroll-shadow__bottom,
.sh-scroll-shadow--both .sh-scroll-shadow__left,
.sh-scroll-shadow--both .sh-scroll-shadow__right {
    display: block;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scroll_shadow_creation() {
        let scroll = ScrollShadow::new("Content");

        assert_eq!(scroll.content, "Content");
        assert_eq!(scroll.direction, ScrollShadowDirection::Vertical);
    }

    #[test]
    fn test_scroll_shadow_render() {
        let scroll = ScrollShadow::new("Test content");
        let html = scroll.render().into_string();

        assert!(html.contains("sh-scroll-shadow"));
        assert!(html.contains("sh-scroll-shadow__content"));
        assert!(html.contains("Test content"));
    }

    #[test]
    fn test_scroll_shadow_directions() {
        let vertical = ScrollShadow::new("test").direction(ScrollShadowDirection::Vertical);
        let horizontal = ScrollShadow::new("test").direction(ScrollShadowDirection::Horizontal);
        let both = ScrollShadow::new("test").direction(ScrollShadowDirection::Both);

        assert!(vertical
            .render()
            .into_string()
            .contains("sh-scroll-shadow--vertical"));
        assert!(horizontal
            .render()
            .into_string()
            .contains("sh-scroll-shadow--horizontal"));
        assert!(both
            .render()
            .into_string()
            .contains("sh-scroll-shadow--both"));
    }

    #[test]
    fn test_scroll_shadow_dimensions() {
        let scroll = ScrollShadow::new("test")
            .max_height("200px")
            .max_width("400px");
        let html = scroll.render().into_string();

        assert!(html.contains("max-height: 200px;"));
        assert!(html.contains("max-width: 400px;"));
    }

    #[test]
    fn test_scroll_shadow_css() {
        let css = scroll_shadow_css();
        assert!(css.contains(".sh-scroll-shadow"));
        assert!(css.contains("linear-gradient"));
    }
}
