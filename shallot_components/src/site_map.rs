//! SiteMap Component - Hierarchical Site Navigation Map
//!
//! Displays a hierarchical sitemap of the website.
//! Useful for SEO and user navigation.

use maud::{html, Markup, Render};

/// SiteMap item
pub struct SiteMapItem<'a> {
    pub label: &'a str,
    pub href: &'a str,
    pub children: Vec<SiteMapItem<'a>>,
    pub new_tab: bool,
}

impl<'a> SiteMapItem<'a> {
    pub fn new(label: &'a str, href: &'a str) -> Self {
        Self {
            label,
            href,
            children: Vec::new(),
            new_tab: false,
        }
    }

    pub fn children(mut self, children: Vec<SiteMapItem<'a>>) -> Self {
        self.children = children;
        self
    }

    pub fn new_tab(mut self, new_tab: bool) -> Self {
        self.new_tab = new_tab;
        self
    }
}

/// SiteMap component
pub struct SiteMap<'a> {
    items: Vec<SiteMapItem<'a>>,
    max_depth: Option<u8>,
    collapsed: bool,
    class: Option<&'a str>,
}

impl<'a> SiteMap<'a> {
    /// Create a new SiteMap
    pub fn new(items: Vec<SiteMapItem<'a>>) -> Self {
        Self {
            items,
            max_depth: None,
            collapsed: false,
            class: None,
        }
    }

    /// Set maximum depth to display
    pub fn max_depth(mut self, depth: u8) -> Self {
        self.max_depth = Some(depth);
        self
    }

    /// Start collapsed
    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    /// Add custom class
    pub fn class(mut self, class: &'a str) -> Self {
        self.class = Some(class);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-sitemap".to_string()];
        if self.collapsed {
            classes.push("sh-sitemap--collapsed".to_string());
        }
        if let Some(custom) = self.class {
            classes.push(custom.to_string());
        }
        classes.join(" ")
    }

    fn render_item(&self, item: &SiteMapItem<'a>, depth: u8) -> Markup {
        let has_children = !item.children.is_empty();
        let show_children = self.max_depth.map_or(true, |max| depth < max);

        html! {
            li class="sh-sitemap__item" {
                a
                    class="sh-sitemap__link"
                    href=(item.href)
                    target=[if item.new_tab { Some("_blank") } else { None }]
                    rel=[if item.new_tab { Some("noopener noreferrer") } else { None }]
                {
                    (item.label)
                    @if has_children {
                        span class="sh-sitemap__indicator" aria-hidden="true" {
                            svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" {
                                polyline points="6 9 12 15 18 9";
                            }
                        }
                    }
                }
                @if has_children && show_children {
                    ul class="sh-sitemap__sub" role="group" {
                        @for child in &item.children {
                            (self.render_item(child, depth + 1))
                        }
                    }
                }
            }
        }
    }
}

impl<'a> Render for SiteMap<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();

        html! {
            nav
                class=(classes)
                aria-label="Site map"
                role="navigation"
            {
                h2 class="sh-sitemap__title" {
                    "Site Map"
                }
                ul class="sh-sitemap__list" role="tree" {
                    @for item in &self.items {
                        (self.render_item(item, 0))
                    }
                }
            }
        }
    }
}

/// Generate CSS for SiteMap component
pub fn site_map_css() -> String {
    r#"
.sh-sitemap {
    max-width: 48rem;
    padding: 1.5rem;
    background: var(--sh-surface, #fff);
    border: 1px solid var(--sh-border, #e5e7eb);
    border-radius: var(--sh-radius-lg, 0.5rem);
}

.sh-sitemap__title {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--sh-text, #1f2937);
    margin-bottom: 1.5rem;
    padding-bottom: 0.75rem;
    border-bottom: 1px solid var(--sh-border, #e5e7eb);
}

.sh-sitemap__list {
    list-style: none;
    padding: 0;
    margin: 0;
}

.sh-sitemap__item {
    margin-bottom: 0.5rem;
}

.sh-sitemap__link {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0.75rem;
    color: var(--sh-text, #1f2937);
    text-decoration: none;
    border-radius: var(--sh-radius-md, 0.375rem);
    transition: all 0.2s ease;
    font-size: 0.9375rem;
}

.sh-sitemap__link:hover {
    background: var(--sh-surface-2, #f3f4f6);
    color: var(--sh-primary, #3b82f6);
}

.sh-sitemap__indicator {
    margin-left: auto;
    color: var(--sh-text-muted, #9ca3af);
    transition: transform 0.2s ease;
}

.sh-sitemap__item:has(> .sh-sitemap__sub) .sh-sitemap__indicator {
    transform: rotate(90deg);
}

.sh-sitemap__sub {
    list-style: none;
    padding-left: 1.5rem;
    margin-top: 0.5rem;
    border-left: 2px solid var(--sh-border, #e5e7eb);
}

.sh-sitemap__sub .sh-sitemap__link {
    font-size: 0.875rem;
    color: var(--sh-text-secondary, #4b5563);
}

/* Collapsed state */
.sh-sitemap--collapsed .sh-sitemap__sub {
    display: none;
}

.sh-sitemap--collapsed .sh-sitemap__indicator {
    transform: rotate(0deg);
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sitemap_creation() {
        let items = vec![SiteMapItem::new("Home", "/")];
        let sitemap = SiteMap::new(items);
        assert_eq!(sitemap.items.len(), 1);
    }

    #[test]
    fn test_sitemap_item_children() {
        let child = SiteMapItem::new("Child", "/child");
        let parent = SiteMapItem::new("Parent", "/parent").children(vec![child]);
        assert_eq!(parent.children.len(), 1);
    }

    #[test]
    fn test_sitemap_item_new_tab() {
        let item = SiteMapItem::new("External", "https://example.com").new_tab(true);
        assert!(item.new_tab);
    }

    #[test]
    fn test_sitemap_max_depth() {
        let items = vec![];
        let sitemap = SiteMap::new(items).max_depth(2);
        assert_eq!(sitemap.max_depth, Some(2));
    }

    #[test]
    fn test_sitemap_css() {
        let css = site_map_css();
        assert!(css.contains(".sh-sitemap"));
        assert!(css.contains(".sh-sitemap__list"));
    }
}
