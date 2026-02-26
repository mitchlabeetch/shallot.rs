use maud::{html, Markup, Render};

pub struct Link<'a> {
    pub href: &'a str,
    pub label: &'a str,
}

impl<'a> Link<'a> {
    pub fn new(href: &'a str, label: &'a str) -> Self {
        Self { href, label }
    }
}

impl<'a> Render for Link<'a> {
    fn render(&self) -> Markup {
        html! {
            a class="sh-link" href=(self.href) { (self.label) }
        }
    }
}

pub struct MenuItem<'a> {
    pub label: &'a str,
    pub href: Option<&'a str>,
    pub icon: Option<Markup>,
    pub active: bool,
}

impl<'a> MenuItem<'a> {
    pub fn new(label: &'a str, href: Option<&'a str>) -> Self {
        Self {
            label,
            href,
            icon: None,
            active: false,
        }
    }

    pub fn icon(mut self, icon: Markup) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
}

pub struct Menu<'a> {
    pub items: Vec<MenuItem<'a>>,
}

impl<'a> Menu<'a> {
    pub fn new(items: Vec<MenuItem<'a>>) -> Self {
        Self { items }
    }
}

impl<'a> Render for Menu<'a> {
    fn render(&self) -> Markup {
        html! {
            div class="sh-menu" role="menu" {
                @for item in &self.items {
                    a class={(if item.active { "sh-menu-item active" } else { "sh-menu-item" })} href=[item.href] role="menuitem" {
                        @if let Some(icon) = &item.icon {
                            (icon)
                        }
                        (item.label)
                    }
                }
            }
        }
    }
}

pub struct Drawer {
    pub open: bool,
    pub children: Markup,
}

impl Drawer {
    pub fn new(children: Markup) -> Self {
        Self {
            open: false,
            children,
        }
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
}

impl Render for Drawer {
    fn render(&self) -> Markup {
        // Simple CSS-only drawer logic is tricky without :target or checkbox hack.
        // For now, we render it based on 'open' prop (server-side state).
        if self.open {
            html! {
                div class="sh-drawer-backdrop" style="display: block;" {}
                div class="sh-drawer" style="transform: translateX(0);" {
                    (self.children)
                }
            }
        } else {
            html! {
                div class="sh-drawer" {
                    (self.children)
                }
            }
        }
    }
}

/// Generate CSS for navigation components
pub fn navigation_css() -> String {
    r#"
/* Link */
.sh-link {
    color: var(--sh-primary, #3b82f6);
    text-decoration: none;
    transition: color 0.2s ease;
}

.sh-link:hover {
    color: var(--sh-primary-hover, #2563eb);
    text-decoration: underline;
}

/* Menu */
.sh-menu {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
}

.sh-menu-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem 1rem;
    color: var(--sh-text, #1f2937);
    text-decoration: none;
    border-radius: var(--sh-radius-md, 0.375rem);
    transition: background 0.2s ease;
}

.sh-menu-item:hover {
    background: var(--sh-surface-2, #f3f4f6);
}

.sh-menu-item.active {
    background: var(--sh-primary, #3b82f6);
    color: white;
}

/* Drawer */
.sh-drawer-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    z-index: 999;
    display: none;
}

.sh-drawer {
    position: fixed;
    top: 0;
    left: 0;
    width: 18rem;
    height: 100vh;
    background: var(--sh-surface, #fff);
    box-shadow: var(--sh-shadow-xl, 0 20px 40px rgba(0, 0, 0, 0.15));
    z-index: 1000;
    padding: 1rem;
    transition: transform 0.3s ease;
    transform: translateX(-100%);
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
        assert_eq!(link.label, "Home");
    }

    #[test]
    fn test_menu_item_creation() {
        let item = MenuItem::new("Dashboard", Some("/dash"));
        assert!(!item.active);
        assert_eq!(item.label, "Dashboard");
    }

    #[test]
    fn test_menu_creation() {
        let items = vec![MenuItem::new("Home", Some("/"))];
        let menu = Menu::new(items);
        assert_eq!(menu.items.len(), 1);
    }

    #[test]
    fn test_drawer_creation() {
        let drawer = Drawer::new(html! { "Content" });
        assert!(!drawer.open);
    }

    #[test]
    fn test_navigation_css() {
        let css = navigation_css();
        assert!(css.contains(".sh-link"));
        assert!(css.contains(".sh-menu"));
    }
}
