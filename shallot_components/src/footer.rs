use maud::{html, Markup, Render};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FooterVariant {
    #[default]
    Default,
    Dark,
    Light,
    BorderTop,
    BorderAll,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FooterSize {
    Sm,
    #[default]
    Md,
    Lg,
}

#[derive(Debug, Clone)]
pub struct FooterColumn {
    pub title: Markup,
    pub links: Vec<Markup>,
}

impl FooterColumn {
    pub fn new(title: Markup) -> Self {
        Self {
            title,
            links: Vec::new(),
        }
    }

    pub fn link(mut self, link: Markup) -> Self {
        self.links.push(link);
        self
    }

    pub fn links(mut self, links: Vec<Markup>) -> Self {
        self.links = links;
        self
    }
}

#[derive(Debug, Clone)]
pub struct Footer {
    pub columns: Vec<FooterColumn>,
    pub variant: FooterVariant,
    pub size: FooterSize,
    pub copyright: Option<Markup>,
    pub social_links: Vec<Markup>,
}

impl Default for Footer {
    fn default() -> Self {
        Self {
            columns: Vec::new(),
            variant: FooterVariant::Default,
            size: FooterSize::Md,
            copyright: None,
            social_links: Vec::new(),
        }
    }
}

impl Footer {
    pub fn new(columns: Vec<FooterColumn>) -> Self {
        Self {
            columns,
            ..Default::default()
        }
    }

    pub fn variant(mut self, variant: FooterVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: FooterSize) -> Self {
        self.size = size;
        self
    }

    pub fn copyright(mut self, copyright: Markup) -> Self {
        self.copyright = Some(copyright);
        self
    }

    pub fn social_links(mut self, links: Vec<Markup>) -> Self {
        self.social_links = links;
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-footer"];

        match self.variant {
            FooterVariant::Default => classes.push("sh-footer--default"),
            FooterVariant::Dark => classes.push("sh-footer--dark"),
            FooterVariant::Light => classes.push("sh-footer--light"),
            FooterVariant::BorderTop => classes.push("sh-footer--border-top"),
            FooterVariant::BorderAll => classes.push("sh-footer--border-all"),
        }

        match self.size {
            FooterSize::Sm => classes.push("sh-footer--sm"),
            FooterSize::Md => classes.push("sh-footer--md"),
            FooterSize::Lg => classes.push("sh-footer--lg"),
        }

        classes.join(" ")
    }
}

impl Render for Footer {
    fn render(&self) -> Markup {
        let classes = self.build_classes();

        html! {
            footer
                class=(classes)
                role="contentinfo"
            {
                div class="sh-footer__container" {
                    div class="sh-footer__columns" {
                        @for col in &self.columns {
                            div class="sh-footer__col" {
                                div class="sh-footer__title" { (col.title) }
                                div class="sh-footer__links" {
                                    @for link in &col.links {
                                        (link)
                                    }
                                }
                            }
                        }
                    }
                    @if !self.social_links.is_empty() {
                        div class="sh-footer__social" {
                            @for link in &self.social_links {
                                (link)
                            }
                        }
                    }
                    @if let Some(copyright) = &self.copyright {
                        div class="sh-footer__copyright" {
                            (copyright)
                        }
                    }
                }
            }
        }
    }
}

pub fn footer_css() -> String {
    r#"
.sh-footer {
    --footer-bg: var(--sh-surface, #fff);
    --footer-border: var(--sh-border, #e5e7eb);
    --footer-text: var(--sh-text, #1f2937);
    --footer-text-secondary: var(--sh-text-secondary, #6b7280);
    --footer-link: var(--sh-link, #3b82f6);
    --footer-link-hover: var(--sh-link-hover, #2563eb);
    
    background: var(--footer-bg);
    color: var(--footer-text);
    padding: 3rem 1.5rem;
    width: 100%;
}

/* Variants */
.sh-footer--default {
    border-top: 1px solid var(--footer-border);
}

.sh-footer--dark {
    background: #1f2937;
    color: #f9fafb;
    --footer-text: #f9fafb;
    --footer-text-secondary: #d1d5db;
    --footer-link: #60a5fa;
    --footer-link-hover: #93c5fd;
}

.sh-footer--light {
    background: #f9fafb;
}

.sh-footer--border-top {
    border-top: 2px solid var(--footer-border);
}

.sh-footer--border-all {
    border: 1px solid var(--footer-border);
}

/* Sizes */
.sh-footer--sm {
    padding: 1.5rem 1rem;
}

.sh-footer--md {
    padding: 3rem 1.5rem;
}

.sh-footer--lg {
    padding: 4rem 2rem;
}

/* Container */
.sh-footer__container {
    max-width: 1200px;
    margin: 0 auto;
    width: 100%;
}

/* Columns */
.sh-footer__columns {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 2rem;
    margin-bottom: 2rem;
}

.sh-footer__col {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
}

/* Title */
.sh-footer__title {
    font-weight: 600;
    font-size: 0.875rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--footer-text);
    margin-bottom: 0.5rem;
}

/* Links */
.sh-footer__links {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
}

.sh-footer__links a {
    color: var(--footer-text-secondary);
    text-decoration: none;
    transition: color 0.15s ease;
    font-size: 0.875rem;
}

.sh-footer__links a:hover {
    color: var(--footer-link-hover);
}

/* Social links */
.sh-footer__social {
    display: flex;
    gap: 1rem;
    justify-content: center;
    padding: 1.5rem 0;
    border-top: 1px solid var(--footer-border);
    margin-top: 1.5rem;
}

/* Copyright */
.sh-footer__copyright {
    text-align: center;
    font-size: 0.875rem;
    color: var(--footer-text-secondary);
    padding-top: 1.5rem;
    border-top: 1px solid var(--footer-border);
}

/* Reduced motion */
@media (prefers-reduced-motion: reduce) {
    .sh-footer a {
        transition: none;
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
    fn test_footer_column() {
        let col = FooterColumn::new(html! { "Column Title" })
            .link(html! { "Link 1" })
            .link(html! { "Link 2" });

        assert_eq!(col.links.len(), 2);
    }

    #[test]
    fn test_footer_creation() {
        let col = FooterColumn::new(html! { "Links" });
        let footer = Footer::new(vec![col]);

        assert_eq!(footer.columns.len(), 1);
    }

    #[test]
    fn test_footer_variants() {
        let default = Footer::new(vec![]).variant(FooterVariant::Default);
        assert_eq!(default.variant, FooterVariant::Default);

        let dark = Footer::new(vec![]).variant(FooterVariant::Dark);
        assert_eq!(dark.variant, FooterVariant::Dark);

        let light = Footer::new(vec![]).variant(FooterVariant::Light);
        assert_eq!(light.variant, FooterVariant::Light);

        let border_top = Footer::new(vec![]).variant(FooterVariant::BorderTop);
        assert_eq!(border_top.variant, FooterVariant::BorderTop);

        let border_all = Footer::new(vec![]).variant(FooterVariant::BorderAll);
        assert_eq!(border_all.variant, FooterVariant::BorderAll);
    }

    #[test]
    fn test_footer_sizes() {
        let sm = Footer::new(vec![]).size(FooterSize::Sm);
        assert_eq!(sm.size, FooterSize::Sm);

        let md = Footer::new(vec![]).size(FooterSize::Md);
        assert_eq!(md.size, FooterSize::Md);

        let lg = Footer::new(vec![]).size(FooterSize::Lg);
        assert_eq!(lg.size, FooterSize::Lg);
    }

    #[test]
    fn test_footer_copyright() {
        let footer = Footer::new(vec![]).copyright(html! { "© 2024" });
        assert!(footer.copyright.is_some());
    }

    #[test]
    fn test_footer_social_links() {
        let links = vec![html! { "Twitter" }, html! { "GitHub" }];
        let footer = Footer::new(vec![]).social_links(links);
        assert_eq!(footer.social_links.len(), 2);
    }

    #[test]
    fn test_footer_render() {
        let col = FooterColumn::new(html! { "Title" }).link(html! { "Link" });
        let footer = Footer::new(vec![col]);

        let rendered = footer.render();
        let html_str = rendered.0.as_str();

        assert!(html_str.contains("sh-footer"));
        assert!(html_str.contains("role=\"contentinfo\""));
    }

    #[test]
    fn test_footer_css() {
        let css = footer_css();
        assert!(css.contains(".sh-footer"));
        assert!(css.contains(".sh-footer--default"));
        assert!(css.contains(".sh-footer--dark"));
        assert!(css.contains(".sh-footer--light"));
        assert!(css.contains(".sh-footer--border-top"));
        assert!(css.contains(".sh-footer--border-all"));
        assert!(css.contains(".sh-footer--sm"));
        assert!(css.contains(".sh-footer--md"));
        assert!(css.contains(".sh-footer--lg"));
    }
}
