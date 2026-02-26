use maud::{html, Markup, Render};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrollAreaVariant {
    #[default]
    Default,
    Bordered,
    Shadow,
    Minimal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrollAreaSize {
    Sm,
    #[default]
    Md,
    Lg,
    Custom(&'static str),
}

#[derive(Debug, Clone)]
pub struct ScrollArea<'a> {
    pub children: Markup,
    pub max_height: Option<&'a str>,
    pub variant: ScrollAreaVariant,
    pub size: ScrollAreaSize,
    pub horizontal: bool,
    pub id: Option<&'a str>,
}

impl<'a> Default for ScrollArea<'a> {
    fn default() -> Self {
        Self {
            children: html! {},
            max_height: Some("300px"),
            variant: ScrollAreaVariant::Default,
            size: ScrollAreaSize::Md,
            horizontal: false,
            id: None,
        }
    }
}

impl<'a> ScrollArea<'a> {
    pub fn new(children: Markup) -> Self {
        Self {
            children,
            ..Default::default()
        }
    }

    pub fn max_height(mut self, height: &'a str) -> Self {
        self.max_height = Some(height);
        self
    }

    pub fn variant(mut self, variant: ScrollAreaVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ScrollAreaSize) -> Self {
        self.size = size;
        self
    }

    pub fn horizontal(mut self, horizontal: bool) -> Self {
        self.horizontal = horizontal;
        self
    }

    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-scroll-area"];

        match self.variant {
            ScrollAreaVariant::Default => classes.push("sh-scroll-area--default"),
            ScrollAreaVariant::Bordered => classes.push("sh-scroll-area--bordered"),
            ScrollAreaVariant::Shadow => classes.push("sh-scroll-area--shadow"),
            ScrollAreaVariant::Minimal => classes.push("sh-scroll-area--minimal"),
        }

        match self.size {
            ScrollAreaSize::Sm => classes.push("sh-scroll-area--sm"),
            ScrollAreaSize::Md => classes.push("sh-scroll-area--md"),
            ScrollAreaSize::Lg => classes.push("sh-scroll-area--lg"),
            ScrollAreaSize::Custom(_) => {}
        }

        if self.horizontal {
            classes.push("sh-scroll-area--horizontal");
        }

        classes.join(" ")
    }
}

impl<'a> Render for ScrollArea<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();

        let style = match self.size {
            ScrollAreaSize::Custom(h) => format!("max-height: {};", h),
            _ => {
                let height = self.max_height.unwrap_or("300px");
                format!("max-height: {};", height)
            }
        };

        html! {
            div
                class=(classes)
                style=(style)
                id=(self.id.unwrap_or(""))
            {
                div class="sh-scroll-area__content" {
                    (self.children)
                }
            }
        }
    }
}

pub fn scroll_area_css() -> String {
    r#"
.sh-scroll-area {
    --scroll-area-bg: var(--sh-surface, #fff);
    --scroll-area-border: var(--sh-border, #e5e7eb);
    --scroll-bar-bg: #e5e7eb;
    --scroll-bar-hover: #d1d5db;
    
    overflow: auto;
    background: var(--scroll-area-bg);
}

/* Variants */
.sh-scroll-area--default {
    border: 1px solid var(--scroll-area-border);
    border-radius: 0.375rem;
}

.sh-scroll-area--bordered {
    border: 2px solid var(--scroll-area-border);
    border-radius: 0.5rem;
}

.sh-scroll-area--shadow {
    border-radius: 0.5rem;
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
}

.sh-scroll-area--minimal {
    border-radius: 0.25rem;
}

/* Sizes */
.sh-scroll-area--sm {
    max-height: 150px;
}

.sh-scroll-area--md {
    max-height: 300px;
}

.sh-scroll-area--lg {
    max-height: 500px;
}

/* Horizontal */
.sh-scroll-area--horizontal {
    overflow-x: auto;
    overflow-y: hidden;
}

/* Content */
.sh-scroll-area__content {
    min-height: 100%;
}

/* Custom scrollbar */
.sh-scroll-area::-webkit-scrollbar {
    width: 8px;
    height: 8px;
}

.sh-scroll-area::-webkit-scrollbar-track {
    background: transparent;
}

.sh-scroll-area::-webkit-scrollbar-thumb {
    background: var(--scroll-bar-bg);
    border-radius: 4px;
}

.sh-scroll-area::-webkit-scrollbar-thumb:hover {
    background: var(--scroll-bar-hover);
}

/* Firefox */
.sh-scroll-area {
    scrollbar-width: thin;
    scrollbar-color: var(--scroll-bar-bg) transparent;
}

/* Reduced motion */
@media (prefers-reduced-motion: reduce) {
    .sh-scroll-area {
        scroll-behavior: auto;
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
    fn test_scroll_area_creation() {
        let area = ScrollArea::new(html! { "Content" });
        assert!(area.children.0.contains("Content"));
    }

    #[test]
    fn test_scroll_area_max_height() {
        let area = ScrollArea::new(html! {}).max_height("200px");
        assert_eq!(area.max_height, Some("200px"));
    }

    #[test]
    fn test_scroll_area_horizontal() {
        let area = ScrollArea::new(html! {}).horizontal(true);
        assert!(area.horizontal);
    }

    #[test]
    fn test_scroll_area_id() {
        let area = ScrollArea::new(html! {}).id("my-scroll");
        assert_eq!(area.id, Some("my-scroll"));
    }

    #[test]
    fn test_scroll_area_variants() {
        let default = ScrollArea::new(html! {}).variant(ScrollAreaVariant::Default);
        assert_eq!(default.variant, ScrollAreaVariant::Default);

        let bordered = ScrollArea::new(html! {}).variant(ScrollAreaVariant::Bordered);
        assert_eq!(bordered.variant, ScrollAreaVariant::Bordered);

        let shadow = ScrollArea::new(html! {}).variant(ScrollAreaVariant::Shadow);
        assert_eq!(shadow.variant, ScrollAreaVariant::Shadow);

        let minimal = ScrollArea::new(html! {}).variant(ScrollAreaVariant::Minimal);
        assert_eq!(minimal.variant, ScrollAreaVariant::Minimal);
    }

    #[test]
    fn test_scroll_area_sizes() {
        let sm = ScrollArea::new(html! {}).size(ScrollAreaSize::Sm);
        assert_eq!(sm.size, ScrollAreaSize::Sm);

        let md = ScrollArea::new(html! {}).size(ScrollAreaSize::Md);
        assert_eq!(md.size, ScrollAreaSize::Md);

        let lg = ScrollArea::new(html! {}).size(ScrollAreaSize::Lg);
        assert_eq!(lg.size, ScrollAreaSize::Lg);

        let custom = ScrollArea::new(html! {}).size(ScrollAreaSize::Custom("400px"));
        assert_eq!(custom.size, ScrollAreaSize::Custom("400px"));
    }

    #[test]
    fn test_scroll_area_render() {
        let area = ScrollArea::new(html! { "Scrollable content" });

        let rendered = area.render();
        let html_str = rendered.0.as_str();

        assert!(html_str.contains("sh-scroll-area"));
        assert!(html_str.contains("Scrollable content"));
    }

    #[test]
    fn test_scroll_area_css() {
        let css = scroll_area_css();
        assert!(css.contains(".sh-scroll-area"));
        assert!(css.contains(".sh-scroll-area--default"));
        assert!(css.contains(".sh-scroll-area--bordered"));
        assert!(css.contains(".sh-scroll-area--shadow"));
        assert!(css.contains(".sh-scroll-area--minimal"));
        assert!(css.contains(".sh-scroll-area--sm"));
        assert!(css.contains(".sh-scroll-area--md"));
        assert!(css.contains(".sh-scroll-area--lg"));
    }
}
