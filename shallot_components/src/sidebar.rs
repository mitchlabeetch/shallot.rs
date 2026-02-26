//! Sidebar Component - Side navigation panel with multiple variants

use maud::{html, Markup, Render};

/// Sidebar variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SidebarVariant {
    /// Standard sidebar
    #[default]
    Default,
    /// Collapsible sidebar
    Collapsible,
    /// Mini sidebar with icons only
    Mini,
    /// Responsive sidebar (mobile-friendly)
    Responsive,
}

/// Sidebar position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SidebarPosition {
    #[default]
    Left,
    Right,
}

/// Sidebar Component
#[derive(Debug, Clone)]
pub struct Sidebar<'a> {
    pub header: Markup,
    pub content: Markup,
    pub footer: Markup,
    pub variant: SidebarVariant,
    pub position: SidebarPosition,
    pub width_px: u16,
    pub aria_label: Option<&'a str>,
    pub collapsed: bool,
}

impl<'a> Default for Sidebar<'a> {
    fn default() -> Self {
        Self {
            header: html! {},
            content: html! {},
            footer: html! {},
            variant: SidebarVariant::default(),
            position: SidebarPosition::default(),
            width_px: 280,
            aria_label: None,
            collapsed: false,
        }
    }
}

impl<'a> Sidebar<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn variant(mut self, variant: SidebarVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn position(mut self, position: SidebarPosition) -> Self {
        self.position = position;
        self
    }

    pub fn width(mut self, width_px: u16) -> Self {
        self.width_px = width_px;
        self
    }

    pub fn aria_label(mut self, label: &'a str) -> Self {
        self.aria_label = Some(label);
        self
    }

    pub fn header(mut self, content: Markup) -> Self {
        self.header = content;
        self
    }

    pub fn content(mut self, content: Markup) -> Self {
        self.content = content;
        self
    }

    pub fn footer(mut self, content: Markup) -> Self {
        self.footer = content;
        self
    }

    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-sidebar"];

        match self.variant {
            SidebarVariant::Default => classes.push("sh-sidebar--default"),
            SidebarVariant::Collapsible => classes.push("sh-sidebar--collapsible"),
            SidebarVariant::Mini => classes.push("sh-sidebar--mini"),
            SidebarVariant::Responsive => classes.push("sh-sidebar--responsive"),
        }

        match self.position {
            SidebarPosition::Left => classes.push("sh-sidebar--left"),
            SidebarPosition::Right => classes.push("sh-sidebar--right"),
        }

        if self.collapsed {
            classes.push("sh-sidebar--collapsed");
        }

        classes.join(" ")
    }
}

impl<'a> Render for Sidebar<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let style = format!("--sh-sidebar-w: {}px;", self.width_px);

        html! {
            aside
                class=(classes)
                style=(style)
                role="complementary"
                aria-label=(self.aria_label.unwrap_or("Sidebar"))
            {
                div class="sh-sidebar__header" { (self.header) }
                div class="sh-sidebar__content" { (self.content) }
                div class="sh-sidebar__footer" { (self.footer) }
            }
        }
    }
}

/// Generate CSS for sidebar components
pub fn sidebar_css() -> String {
    r#"
/* Sidebar Component Styles */
.sh-sidebar {
    display: flex;
    flex-direction: column;
    width: var(--sh-sidebar-w, 280px);
    height: 100vh;
    background: var(--sh-surface);
    border-right: 1px solid var(--sh-border);
    overflow: hidden;
}

/* Position variants */
.sh-sidebar--left {
    border-right: 1px solid var(--sh-border);
    border-left: none;
}

.sh-sidebar--right {
    border-left: 1px solid var(--sh-border);
    border-right: none;
}

/* Style variants */
.sh-sidebar--default {
    flex-shrink: 0;
}

.sh-sidebar--collapsible {
    transition: width 0.3s ease;
}

.sh-sidebar--mini {
    width: 64px;
    --sh-sidebar-w: 64px;
}

.sh-sidebar--mini .sh-sidebar__header,
.sh-sidebar--mini .sh-sidebar__footer {
    padding: 0.5rem;
}

.sh-sidebar--responsive {
    position: fixed;
    top: 0;
    left: 0;
    height: 100vh;
    z-index: 200;
}

/* Collapsed state */
.sh-sidebar--collapsed {
    width: 64px;
    --sh-sidebar-w: 64px;
}

/* Sections */
.sh-sidebar__header {
    flex: 0 0 auto;
    padding: 1rem;
    border-bottom: 1px solid var(--sh-border);
}

.sh-sidebar__content {
    flex: 1 1 auto;
    overflow-y: auto;
    padding: 1rem 0;
}

.sh-sidebar__footer {
    flex: 0 0 auto;
    padding: 1rem;
    border-top: 1px solid var(--sh-border);
}

/* Reduced motion */
@media (prefers-reduced-motion: reduce) {
    .sh-sidebar,
    .sh-sidebar--collapsible {
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
    fn test_sidebar_creation() {
        let sidebar = Sidebar::new()
            .header(html! { "Header" })
            .content(html! { "Content" })
            .footer(html! { "Footer" });

        assert_eq!(sidebar.variant, SidebarVariant::Default);
        assert_eq!(sidebar.width_px, 280);
    }

    #[test]
    fn test_sidebar_variants() {
        let collapsible = Sidebar::new().variant(SidebarVariant::Collapsible);
        assert_eq!(collapsible.variant, SidebarVariant::Collapsible);

        let mini = Sidebar::new().variant(SidebarVariant::Mini);
        assert_eq!(mini.variant, SidebarVariant::Mini);

        let responsive = Sidebar::new().variant(SidebarVariant::Responsive);
        assert_eq!(responsive.variant, SidebarVariant::Responsive);
    }

    #[test]
    fn test_sidebar_position() {
        let right = Sidebar::new().position(SidebarPosition::Right);
        assert_eq!(right.position, SidebarPosition::Right);
    }

    #[test]
    fn test_sidebar_width() {
        let sidebar = Sidebar::new().width(320);
        assert_eq!(sidebar.width_px, 320);
    }

    #[test]
    fn test_sidebar_collapsed() {
        let sidebar = Sidebar::new().collapsed(true);
        assert!(sidebar.collapsed);
    }

    #[test]
    fn test_sidebar_css() {
        let css = sidebar_css();
        assert!(css.contains(".sh-sidebar"));
        assert!(css.contains(".sh-sidebar--collapsible"));
        assert!(css.contains(".sh-sidebar--mini"));
        assert!(css.contains(".sh-sidebar--collapsed"));
    }
}
