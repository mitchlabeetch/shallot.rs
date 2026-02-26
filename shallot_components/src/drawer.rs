use maud::{html, Markup, Render};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawerSide {
    Left,
    Right,
    Bottom,
}

#[derive(Debug, Clone)]
pub struct Drawer<'a> {
    pub open: bool,
    pub side: DrawerSide,
    pub close_href: Option<&'a str>,
    pub children: Markup,
}

impl<'a> Drawer<'a> {
    pub fn new(children: Markup) -> Self {
        Self {
            open: false,
            side: DrawerSide::Left,
            close_href: None,
            children,
        }
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn side(mut self, side: DrawerSide) -> Self {
        self.side = side;
        self
    }

    pub fn close_href(mut self, href: &'a str) -> Self {
        self.close_href = Some(href);
        self
    }
}

impl<'a> Render for Drawer<'a> {
    fn render(&self) -> Markup {
        let side_class = match self.side {
            DrawerSide::Left => "",
            DrawerSide::Right => "sh-drawer--right",
            DrawerSide::Bottom => "sh-drawer--bottom",
        };
        let open_class = if self.open { "sh-drawer--open" } else { "" };
        let backdrop_class = if self.open {
            "sh-drawer-backdrop--open"
        } else {
            ""
        };

        html! {
            @if self.open {
                div class=(format!("sh-drawer-backdrop {}", backdrop_class)) {
                    @if let Some(href) = self.close_href {
                        a href=(href) style="position:absolute;inset:0;cursor:default" aria-label="Close" {}
                    }
                }
            }
            div class=(format!("sh-drawer {} {}", side_class, open_class)) {
                @if let Some(href) = self.close_href {
                    div style="display:flex;justify-content:flex-end;margin-bottom:12px" {
                        a class="sh-icon-btn" href=(href) aria-label="Close" {
                            img class="sh-icon" src="/icons/close.svg" alt="" loading="lazy";
                        }
                    }
                }
                (self.children)
            }
        }
    }
}

/// Generate CSS for drawer component
pub fn drawer_css() -> String {
    r#"
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

.sh-drawer--right {
    left: auto;
    right: 0;
    transform: translateX(100%);
}

.sh-drawer--bottom {
    top: auto;
    bottom: 0;
    left: 0;
    right: 0;
    width: 100%;
    height: auto;
    max-height: 50vh;
    transform: translateY(100%);
}

.sh-drawer--open {
    transform: translateX(0);
}

.sh-drawer--right.sh-drawer--open {
    transform: translateX(0);
}

.sh-drawer--bottom.sh-drawer--open {
    transform: translateY(0);
}

.sh-drawer-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    opacity: 0;
    transition: opacity 0.3s ease;
    z-index: 999;
}

.sh-drawer-backdrop--open {
    opacity: 1;
}

.sh-icon-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 2rem;
    height: 2rem;
    border-radius: var(--sh-radius-md, 0.375rem);
    background: var(--sh-surface-2, #f3f4f6);
    transition: background 0.2s ease;
}

.sh-icon-btn:hover {
    background: var(--sh-surface-hover, #e5e7eb);
}

.sh-icon {
    width: 1rem;
    height: 1rem;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drawer_creation() {
        let drawer = Drawer::new(html! { "Content" });
        assert!(!drawer.open);
        assert_eq!(drawer.side, DrawerSide::Left);
    }

    #[test]
    fn test_drawer_open() {
        let drawer = Drawer::new(html! {}).open(true);
        assert!(drawer.open);
    }

    #[test]
    fn test_drawer_side() {
        let drawer = Drawer::new(html! {}).side(DrawerSide::Right);
        assert_eq!(drawer.side, DrawerSide::Right);
    }

    #[test]
    fn test_drawer_css() {
        let css = drawer_css();
        assert!(css.contains(".sh-drawer"));
        assert!(css.contains(".sh-drawer--right"));
    }
}
