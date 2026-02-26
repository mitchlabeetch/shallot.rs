use maud::{html, Markup};

pub struct DockItem<'a> {
    pub href: &'a str,
    pub label: &'a str,
    pub icon_src: &'a str,
}

pub struct Dock<'a> {
    pub items: Vec<DockItem<'a>>,
}

impl<'a> Dock<'a> {
    pub fn new(items: Vec<DockItem<'a>>) -> Self {
        Self { items }
    }

    pub fn render(self) -> Markup {
        html! {
            nav class="sh-dock" aria-label="Dock" {
                @for it in self.items {
                    a class="sh-dock__item" href=(it.href) aria-label=(it.label) {
                        img class="sh-dock__icon" src=(it.icon_src) alt="" loading="lazy" {}
                    }
                }
            }
        }
    }
}

/// Generate CSS for dock component
pub fn dock_css() -> String {
    r#"
.sh-dock {
    position: fixed;
    bottom: 1rem;
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    gap: 0.5rem;
    padding: 0.5rem;
    background: var(--sh-surface, rgba(255, 255, 255, 0.9));
    backdrop-filter: blur(10px);
    border-radius: var(--sh-radius-xl, 1rem);
    box-shadow: var(--sh-shadow-lg, 0 10px 40px rgba(0, 0, 0, 0.1));
    border: 1px solid var(--sh-border, rgba(0, 0, 0, 0.1));
    z-index: 1000;
}

.sh-dock__item {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 3rem;
    height: 3rem;
    border-radius: var(--sh-radius-lg, 0.5rem);
    transition: all 0.2s ease;
    text-decoration: none;
}

.sh-dock__item:hover {
    background: var(--sh-surface-2, #f3f4f6);
    transform: scale(1.1);
}

.sh-dock__item:active {
    transform: scale(0.95);
}

.sh-dock__icon {
    width: 1.5rem;
    height: 1.5rem;
    object-fit: contain;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dock_creation() {
        let items = vec![DockItem {
            href: "/home",
            label: "Home",
            icon_src: "/home.svg",
        }];
        let dock = Dock::new(items);
        assert_eq!(dock.items.len(), 1);
    }

    #[test]
    fn test_dock_render() {
        let items = vec![DockItem {
            href: "/home",
            label: "Home",
            icon_src: "/home.svg",
        }];
        let dock = Dock::new(items);
        let html = dock.render().into_string();
        assert!(html.contains("sh-dock"));
        assert!(html.contains("aria-label=\"Dock\""));
    }

    #[test]
    fn test_dock_css() {
        let css = dock_css();
        assert!(css.contains(".sh-dock"));
        assert!(css.contains(".sh-dock__item"));
    }
}
