//! Link Component - Styled link with variants
//! CSS-only styling with hover and focus states

use maud::{html, Markup, Render};

/// Link size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LinkSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Link variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LinkVariant {
    #[default]
    Primary,
    Secondary,
    Muted,
    Unstyled,
}

/// Link component
#[derive(Debug, Clone)]
pub struct Link<'a> {
    pub href: &'a str,
    pub text: &'a str,
    pub size: LinkSize,
    pub variant: LinkVariant,
    pub external: bool,
    pub disabled: bool,
    pub underline: bool,
}

impl<'a> Link<'a> {
    /// Create a new link
    pub fn new(href: &'a str, text: &'a str) -> Self {
        Self {
            href,
            text,
            size: LinkSize::default(),
            variant: LinkVariant::default(),
            external: false,
            disabled: false,
            underline: true,
        }
    }

    /// Set the size
    pub fn size(mut self, size: LinkSize) -> Self {
        self.size = size;
        self
    }

    /// Set the variant
    pub fn variant(mut self, variant: LinkVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Mark as external link (opens in new tab)
    pub fn external(mut self, external: bool) -> Self {
        self.external = external;
        self
    }

    /// Set the disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set underline visibility
    pub fn underline(mut self, underline: bool) -> Self {
        self.underline = underline;
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-link".to_string()];

        let size_class = match self.size {
            LinkSize::Sm => "sh-link--sm",
            LinkSize::Md => "sh-link--md",
            LinkSize::Lg => "sh-link--lg",
        };
        classes.push(size_class.to_string());

        let variant_class = match self.variant {
            LinkVariant::Primary => "sh-link--primary",
            LinkVariant::Secondary => "sh-link--secondary",
            LinkVariant::Muted => "sh-link--muted",
            LinkVariant::Unstyled => "sh-link--unstyled",
        };
        classes.push(variant_class.to_string());

        if self.disabled {
            classes.push("sh-link--disabled".to_string());
        }

        if !self.underline && self.variant != LinkVariant::Unstyled {
            classes.push("sh-link--no-underline".to_string());
        }

        classes.join(" ")
    }
}

impl<'a> Render for Link<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();

        if self.disabled {
            html! {
                span class=(classes) aria-disabled="true" {
                    (self.text)
                }
            }
        } else if self.external {
            html! {
                a
                    href=(self.href)
                    class=(classes)
                    target="_blank"
                    rel="noopener noreferrer" {
                    (self.text)
                    svg
                        class="sh-link__icon"
                        xmlns="http://www.w3.org/2000/svg"
                        width="16"
                        height="16"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round" {
                        path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" {}
                        polyline points="15 3 21 3 21 9" {}
                        line x1="10" y1="14" x2="21" y2="3" {}
                    }
                }
            }
        } else {
            html! {
                a href=(self.href) class=(classes) {
                    (self.text)
                }
            }
        }
    }
}

/// Generate CSS for link component
pub fn link_css() -> String {
    r#"
.sh-link {
    display: inline-flex;
    align-items: center;
    gap: var(--sh-spacing-1, 0.25rem);
    color: var(--sh-color-primary, #3b82f6);
    text-decoration: underline;
    text-underline-offset: 2px;
    cursor: pointer;
    transition: color 0.15s ease;
}

.sh-link:hover:not(.sh-link--disabled) {
    color: var(--sh-color-primary-hover, #2563eb);
}

.sh-link:focus-visible {
    outline: 2px solid var(--sh-color-primary, #3b82f6);
    outline-offset: 2px;
    border-radius: var(--sh-radius-sm, 0.25rem);
}

.sh-link--sm {
    font-size: var(--sh-font-size-sm, 0.875rem);
}

.sh-link--md {
    font-size: var(--sh-font-size-base, 1rem);
}

.sh-link--lg {
    font-size: var(--sh-font-size-lg, 1.125rem);
}

.sh-link--primary {
    color: var(--sh-color-primary, #3b82f6);
}

.sh-link--primary:hover:not(.sh-link--disabled) {
    color: var(--sh-color-primary-hover, #2563eb);
}

.sh-link--secondary {
    color: var(--sh-color-secondary, #6b7280);
}

.sh-link--secondary:hover:not(.sh-link--disabled) {
    color: var(--sh-color-secondary-hover, #4b5563);
}

.sh-link--muted {
    color: var(--sh-color-muted-foreground, #9ca3af);
}

.sh-link--muted:hover:not(.sh-link--disabled) {
    color: var(--sh-color-foreground, #1f2937);
}

.sh-link--unstyled {
    color: inherit;
    text-decoration: none;
}

.sh-link--unstyled:hover:not(.sh-link--disabled) {
    color: inherit;
    text-decoration: underline;
}

.sh-link--no-underline {
    text-decoration: none;
}

.sh-link--no-underline:hover:not(.sh-link--disabled) {
    text-decoration: underline;
}

.sh-link--disabled {
    color: var(--sh-color-muted-foreground, #9ca3af);
    cursor: not-allowed;
    text-decoration: none;
}

.sh-link__icon {
    width: 1em;
    height: 1em;
    flex-shrink: 0;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_creation() {
        let link = Link::new("/home", "Home");

        assert_eq!(link.href, "/home");
        assert_eq!(link.text, "Home");
        assert_eq!(link.size, LinkSize::Md);
        assert_eq!(link.variant, LinkVariant::Primary);
    }

    #[test]
    fn test_link_render() {
        let link = Link::new("/about", "About Us");
        let html = link.render().into_string();

        assert!(html.contains("sh-link"));
        assert!(html.contains("href=\"/about\""));
        assert!(html.contains("About Us"));
    }

    #[test]
    fn test_link_sizes() {
        let sm = Link::new("/", "Home").size(LinkSize::Sm);
        let md = Link::new("/", "Home").size(LinkSize::Md);
        let lg = Link::new("/", "Home").size(LinkSize::Lg);

        assert!(sm.render().into_string().contains("sh-link--sm"));
        assert!(md.render().into_string().contains("sh-link--md"));
        assert!(lg.render().into_string().contains("sh-link--lg"));
    }

    #[test]
    fn test_link_variants() {
        let primary = Link::new("/", "Home").variant(LinkVariant::Primary);
        let secondary = Link::new("/", "Home").variant(LinkVariant::Secondary);
        let muted = Link::new("/", "Home").variant(LinkVariant::Muted);

        assert!(primary.render().into_string().contains("sh-link--primary"));
        assert!(secondary
            .render()
            .into_string()
            .contains("sh-link--secondary"));
        assert!(muted.render().into_string().contains("sh-link--muted"));
    }

    #[test]
    fn test_link_external() {
        let link = Link::new("https://example.com", "External").external(true);
        let html = link.render().into_string();

        assert!(html.contains("target=\"_blank\""));
        assert!(html.contains("rel=\"noopener noreferrer\""));
        assert!(html.contains("sh-link__icon"));
    }

    #[test]
    fn test_link_disabled() {
        let link = Link::new("/", "Home").disabled(true);
        let html = link.render().into_string();

        assert!(html.contains("sh-link--disabled"));
        assert!(html.contains("aria-disabled=\"true\""));
        assert!(!html.contains("<a"));
    }

    #[test]
    fn test_link_no_underline() {
        let link = Link::new("/", "Home").underline(false);
        let html = link.render().into_string();

        assert!(html.contains("sh-link--no-underline"));
    }

    #[test]
    fn test_link_css() {
        let css = link_css();
        assert!(css.contains(".sh-link"));
        assert!(css.contains(".sh-link--disabled"));
    }
}
