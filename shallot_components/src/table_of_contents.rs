//! TableOfContents Component - Auto-generated TOC from Headings
//!
//! Displays a table of contents based on page headings.
//! Uses CSS scroll-linked highlighting for active section.

use maud::{html, Markup, Render};

/// TOC Item representing a heading
pub struct TocItem<'a> {
    pub label: &'a str,
    pub href: &'a str,
    pub level: u8,
    pub children: Vec<TocItem<'a>>,
}

impl<'a> TocItem<'a> {
    pub fn new(label: &'a str, href: &'a str, level: u8) -> Self {
        Self {
            label,
            href,
            level,
            children: Vec::new(),
        }
    }

    pub fn children(mut self, children: Vec<TocItem<'a>>) -> Self {
        self.children = children;
        self
    }
}

/// TableOfContents component
pub struct TableOfContents<'a> {
    items: Vec<TocItem<'a>>,
    title: Option<&'a str>,
    max_depth: Option<u8>,
    highlight: bool,
    class: Option<&'a str>,
}

impl<'a> TableOfContents<'a> {
    /// Create a new TableOfContents
    pub fn new(items: Vec<TocItem<'a>>) -> Self {
        Self {
            items,
            title: Some("On this page"),
            max_depth: Some(3),
            highlight: true,
            class: None,
        }
    }

    /// Set the title
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    /// Hide the title
    pub fn hide_title(mut self) -> Self {
        self.title = None;
        self
    }

    /// Set maximum depth to display
    pub fn max_depth(mut self, depth: u8) -> Self {
        self.max_depth = Some(depth);
        self
    }

    /// Disable active highlight
    pub fn highlight(mut self, highlight: bool) -> Self {
        self.highlight = highlight;
        self
    }

    /// Add custom class
    pub fn class(mut self, class: &'a str) -> Self {
        self.class = Some(class);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-toc".to_string()];
        if !self.highlight {
            classes.push("sh-toc--no-highlight".to_string());
        }
        if let Some(custom) = self.class {
            classes.push(custom.to_string());
        }
        classes.join(" ")
    }

    fn render_item(&self, item: &TocItem<'a>, depth: u8) -> Markup {
        let show_children = self.max_depth.map_or(true, |max| depth < max);
        let indent_class = match item.level {
            1 => "sh-toc__link--h1",
            2 => "sh-toc__link--h2",
            3 => "sh-toc__link--h3",
            4 => "sh-toc__link--h4",
            _ => "",
        };

        html! {
            li class="sh-toc__item" {
                a
                    class=(format!("sh-toc__link {}", indent_class))
                    href=(item.href)
                {
                    (item.label)
                }
                @if !item.children.is_empty() && show_children {
                    ul class="sh-toc__sub" role="group" {
                        @for child in &item.children {
                            (self.render_item(child, depth + 1))
                        }
                    }
                }
            }
        }
    }
}

impl<'a> Render for TableOfContents<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();

        html! {
            nav
                class=(classes)
                aria-label=(self.title.unwrap_or("Table of contents"))
                role="navigation"
            {
                @if let Some(title) = self.title {
                    h2 class="sh-toc__title" {
                        (title)
                    }
                }
                ul class="sh-toc__list" role="list" {
                    @for item in &self.items {
                        (self.render_item(item, 1))
                    }
                }
            }
        }
    }
}

/// Generate CSS for TableOfContents component
pub fn table_of_contents_css() -> String {
    r#"
.sh-toc {
    position: sticky;
    top: 1rem;
    max-width: 16rem;
    padding: 1.5rem;
    background: var(--sh-surface, #fff);
    border: 1px solid var(--sh-border, #e5e7eb);
    border-radius: var(--sh-radius-lg, 0.5rem);
    max-height: calc(100vh - 2rem);
    overflow-y: auto;
}

.sh-toc__title {
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--sh-text-muted, #6b7280);
    margin-bottom: 1rem;
}

.sh-toc__list {
    list-style: none;
    padding: 0;
    margin: 0;
}

.sh-toc__item {
    margin-bottom: 0.25rem;
}

.sh-toc__link {
    display: block;
    padding: 0.375rem 0.5rem;
    color: var(--sh-text-secondary, #4b5563);
    text-decoration: none;
    font-size: 0.875rem;
    line-height: 1.5;
    border-radius: var(--sh-radius-sm, 0.25rem);
    transition: all 0.2s ease;
}

.sh-toc__link:hover {
    color: var(--sh-text, #1f2937);
    background: var(--sh-surface-2, #f3f4f6);
}

/* Active state via CSS scroll-driven or :target */
.sh-toc__link:has(:target) {
    color: var(--sh-primary, #3b82f6);
    font-weight: 500;
    background: var(--sh-primary-bg, rgba(59, 130, 246, 0.1));
}

/* Indentation by heading level */
.sh-toc__link--h1 {
    font-weight: 500;
    color: var(--sh-text, #1f2937);
}

.sh-toc__link--h2 {
    padding-left: 1rem;
}

.sh-toc__link--h3 {
    padding-left: 1.5rem;
    font-size: 0.8125rem;
}

.sh-toc__link--h4 {
    padding-left: 2rem;
    font-size: 0.8125rem;
    color: var(--sh-text-muted, #9ca3af);
}

.sh-toc__sub {
    list-style: none;
    padding: 0;
    margin: 0.25rem 0;
}

/* No highlight mode */
.sh-toc--no-highlight .sh-toc__link:has(:target) {
    background: transparent;
    font-weight: normal;
}

/* Scrollbar styling */
.sh-toc::-webkit-scrollbar {
    width: 0.375rem;
}

.sh-toc::-webkit-scrollbar-track {
    background: transparent;
}

.sh-toc::-webkit-scrollbar-thumb {
    background: var(--sh-border, #e5e7eb);
    border-radius: 9999px;
}

.sh-toc::-webkit-scrollbar-thumb:hover {
    background: var(--sh-text-muted, #9ca3af);
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toc_creation() {
        let items = vec![TocItem::new("Introduction", "#intro", 2)];
        let toc = TableOfContents::new(items);
        assert_eq!(toc.items.len(), 1);
        assert_eq!(toc.title, Some("On this page"));
    }

    #[test]
    fn test_toc_title() {
        let items = vec![];
        let toc = TableOfContents::new(items).title("Contents");
        assert_eq!(toc.title, Some("Contents"));
    }

    #[test]
    fn test_toc_hide_title() {
        let items = vec![];
        let toc = TableOfContents::new(items).hide_title();
        assert_eq!(toc.title, None);
    }

    #[test]
    fn test_toc_max_depth() {
        let items = vec![];
        let toc = TableOfContents::new(items).max_depth(2);
        assert_eq!(toc.max_depth, Some(2));
    }

    #[test]
    fn test_toc_css() {
        let css = table_of_contents_css();
        assert!(css.contains(".sh-toc"));
        assert!(css.contains(".sh-toc__list"));
    }
}
