use maud::{html, Markup, Render};

pub struct Icon<'a> {
    pub name: &'a str,
    pub size_px: u16,
    pub alt: &'a str,
}

impl<'a> Icon<'a> {
    pub fn new(name: &'a str, alt: &'a str) -> Self {
        Self {
            name,
            size_px: 16,
            alt,
        }
    }

    pub fn size(mut self, size: u16) -> Self {
        self.size_px = size;
        self
    }
}

impl<'a> Render for Icon<'a> {
    fn render(&self) -> Markup {
        let src = format!("/icons/{}.svg", self.name);
        html! {
            img class="sh-icon" src=(src) width=(self.size_px) height=(self.size_px) alt=(self.alt) loading="lazy";
        }
    }
}

pub struct IconButton<'a> {
    pub href: &'a str,
    pub aria_label: &'a str,
    pub icon: Icon<'a>,
}

impl<'a> IconButton<'a> {
    pub fn new(href: &'a str, aria_label: &'a str, icon: Icon<'a>) -> Self {
        Self {
            href,
            aria_label,
            icon,
        }
    }
}

impl<'a> Render for IconButton<'a> {
    fn render(&self) -> Markup {
        html! {
            a class="sh-icon-btn" href=(self.href) aria-label=(self.aria_label) {
                (self.icon.render())
            }
        }
    }
}

/// Generate CSS for icon components
pub fn icon_css() -> String {
    r#"
.sh-icon {
    display: inline-block;
    vertical-align: middle;
    object-fit: contain;
}

.sh-icon-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 2.5rem;
    height: 2.5rem;
    border-radius: var(--sh-radius-md, 0.375rem);
    background: var(--sh-surface-2, #f3f4f6);
    transition: all 0.2s ease;
    text-decoration: none;
}

.sh-icon-btn:hover {
    background: var(--sh-surface-hover, #e5e7eb);
    transform: scale(1.05);
}

.sh-icon-btn:active {
    transform: scale(0.95);
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_creation() {
        let icon = Icon::new("home", "Home icon");
        assert_eq!(icon.name, "home");
        assert_eq!(icon.size_px, 16);
    }

    #[test]
    fn test_icon_size() {
        let icon = Icon::new("search", "Search").size(24);
        assert_eq!(icon.size_px, 24);
    }

    #[test]
    fn test_icon_button_creation() {
        let icon = Icon::new("close", "Close");
        let btn = IconButton::new("#", "Close button", icon);
        assert_eq!(btn.aria_label, "Close button");
    }

    #[test]
    fn test_icon_css() {
        let css = icon_css();
        assert!(css.contains(".sh-icon"));
        assert!(css.contains(".sh-icon-btn"));
    }
}
