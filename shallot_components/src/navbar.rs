//! Navbar Component - Navigation header with multiple variants
//!
//! Provides a responsive navigation bar with support for sticky, fixed,
//! and transparent styles.

use maud::{html, Markup, Render};

/// Navbar variant determining position and style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NavbarVariant {
    /// Standard navbar in document flow
    #[default]
    Static,
    /// Sticks to top when scrolling
    Sticky,
    /// Fixed at top, content flows beneath
    Fixed,
    /// Transparent background for hero sections
    Transparent,
}

/// Navbar size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NavbarSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Navbar Component
#[derive(Debug, Clone)]
pub struct Navbar<'a> {
    pub start: Markup,
    pub center: Markup,
    pub end: Markup,
    pub variant: NavbarVariant,
    pub size: NavbarSize,
    pub aria_label: Option<&'a str>,
    pub expanded: bool,
}

impl<'a> Default for Navbar<'a> {
    fn default() -> Self {
        Self {
            start: html! {},
            center: html! {},
            end: html! {},
            variant: NavbarVariant::default(),
            size: NavbarSize::default(),
            aria_label: None,
            expanded: false,
        }
    }
}

impl<'a> Navbar<'a> {
    /// Create a new navbar
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the navbar variant
    pub fn variant(mut self, variant: NavbarVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the navbar size
    pub fn size(mut self, size: NavbarSize) -> Self {
        self.size = size;
        self
    }

    /// Set the aria-label for accessibility
    pub fn aria_label(mut self, label: &'a str) -> Self {
        self.aria_label = Some(label);
        self
    }

    /// Set the start section content (logo, branding)
    pub fn start(mut self, content: Markup) -> Self {
        self.start = content;
        self
    }

    /// Set the center section content (main navigation)
    pub fn center(mut self, content: Markup) -> Self {
        self.center = content;
        self
    }

    /// Set the end section content (actions, user menu)
    pub fn end(mut self, content: Markup) -> Self {
        self.end = content;
        self
    }

    /// Set expanded state for mobile
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-navbar"];

        match self.variant {
            NavbarVariant::Static => classes.push("sh-navbar--static"),
            NavbarVariant::Sticky => classes.push("sh-navbar--sticky"),
            NavbarVariant::Fixed => classes.push("sh-navbar--fixed"),
            NavbarVariant::Transparent => classes.push("sh-navbar--transparent"),
        }

        match self.size {
            NavbarSize::Sm => classes.push("sh-navbar--sm"),
            NavbarSize::Md => classes.push("sh-navbar--md"),
            NavbarSize::Lg => classes.push("sh-navbar--lg"),
        }

        if self.expanded {
            classes.push("sh-navbar--expanded");
        }

        classes.join(" ")
    }
}

impl<'a> Render for Navbar<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();

        html! {
            header
                class=(classes)
                role="banner"
                aria-label=(self.aria_label.unwrap_or("Main navigation"))
            {
                div class="sh-navbar__start" { (self.start) }
                div class="sh-navbar__center" { (self.center) }
                div class="sh-navbar__end" { (self.end) }
            }
        }
    }
}

/// Generate CSS for navbar components
pub fn navbar_css() -> String {
    r#"
/* Navbar Component Styles */
.sh-navbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    background: var(--sh-surface);
    border-bottom: 1px solid var(--sh-border);
    z-index: 100;
}

/* Variants */
.sh-navbar--static {
    position: relative;
}

.sh-navbar--sticky {
    position: sticky;
    top: 0;
}

.sh-navbar--fixed {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
}

.sh-navbar--transparent {
    background: transparent;
    border-bottom: none;
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
}

/* Sizes */
.sh-navbar--sm {
    padding: 0.5rem 1rem;
    min-height: 48px;
}

.sh-navbar--md {
    padding: 0.75rem 1.5rem;
    min-height: 64px;
}

.sh-navbar--lg {
    padding: 1rem 2rem;
    min-height: 80px;
}

/* Sections */
.sh-navbar__start,
.sh-navbar__center,
.sh-navbar__end {
    display: flex;
    align-items: center;
    gap: 0.5rem;
}

.sh-navbar__start {
    flex: 0 0 auto;
}

.sh-navbar__center {
    flex: 1 1 auto;
    justify-content: center;
}

.sh-navbar__end {
    flex: 0 0 auto;
}

/* Expanded state (mobile) */
.sh-navbar--expanded .sh-navbar__center {
    display: flex;
}

/* Reduced motion */
@media (prefers-reduced-motion: reduce) {
    .sh-navbar {
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
    fn test_navbar_creation() {
        let navbar = Navbar::new()
            .start(html! { "Logo" })
            .center(html! { "Nav" })
            .end(html! { "Actions" });

        assert_eq!(navbar.variant, NavbarVariant::Static);
        assert_eq!(navbar.size, NavbarSize::Md);
    }

    #[test]
    fn test_navbar_variants() {
        let sticky = Navbar::new().variant(NavbarVariant::Sticky);
        assert_eq!(sticky.variant, NavbarVariant::Sticky);

        let fixed = Navbar::new().variant(NavbarVariant::Fixed);
        assert_eq!(fixed.variant, NavbarVariant::Fixed);

        let transparent = Navbar::new().variant(NavbarVariant::Transparent);
        assert_eq!(transparent.variant, NavbarVariant::Transparent);
    }

    #[test]
    fn test_navbar_sizes() {
        let sm = Navbar::new().size(NavbarSize::Sm);
        assert_eq!(sm.size, NavbarSize::Sm);

        let lg = Navbar::new().size(NavbarSize::Lg);
        assert_eq!(lg.size, NavbarSize::Lg);
    }

    #[test]
    fn test_navbar_aria_label() {
        let navbar = Navbar::new().aria_label("Main menu");
        assert_eq!(navbar.aria_label, Some("Main menu"));
    }

    #[test]
    fn test_navbar_expanded() {
        let navbar = Navbar::new().expanded(true);
        assert!(navbar.expanded);
    }

    #[test]
    fn test_navbar_css() {
        let css = navbar_css();
        assert!(css.contains(".sh-navbar"));
        assert!(css.contains(".sh-navbar--sticky"));
        assert!(css.contains(".sh-navbar--fixed"));
        assert!(css.contains(".sh-navbar--transparent"));
    }
}
