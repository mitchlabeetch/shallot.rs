use maud::{html, Markup, Render};

pub struct Dropdown<'a> {
    pub trigger: Markup,
    pub menu: Markup,
    pub open: bool,
    pub align_end: bool,
    pub aria_label: &'a str,
}

impl<'a> Dropdown<'a> {
    pub fn new(trigger: Markup, menu: Markup, aria_label: &'a str) -> Self {
        Self {
            trigger,
            menu,
            open: false,
            align_end: false,
            aria_label,
        }
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn align_end(mut self, align_end: bool) -> Self {
        self.align_end = align_end;
        self
    }
}

impl<'a> Render for Dropdown<'a> {
    fn render(&self) -> Markup {
        let class = if self.align_end {
            "sh-dropdown sh-dropdown--end"
        } else {
            "sh-dropdown sh-dropdown--start"
        };

        html! {
            details class=(class) open?[self.open] {
                summary class="sh-dropdown__trigger" aria-label=(self.aria_label) {
                    (self.trigger)
                }
                div class="sh-dropdown__menu" {
                    (self.menu)
                }
            }
        }
    }
}

/// Generate CSS for dropdown component
pub fn dropdown_css() -> String {
    r#"
.sh-dropdown {
    position: relative;
    display: inline-block;
}

.sh-dropdown__trigger {
    list-style: none;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0.75rem;
    border-radius: var(--sh-radius-md, 0.375rem);
    transition: background 0.2s ease;
}

.sh-dropdown__trigger:hover {
    background: var(--sh-surface-2, #f3f4f6);
}

.sh-dropdown__trigger::-webkit-details-marker {
    display: none;
}

.sh-dropdown__menu {
    position: absolute;
    top: 100%;
    left: 0;
    min-width: 12rem;
    background: var(--sh-surface, #fff);
    border: 1px solid var(--sh-border, #e5e7eb);
    border-radius: var(--sh-radius-lg, 0.5rem);
    box-shadow: var(--sh-shadow-lg, 0 10px 40px rgba(0, 0, 0, 0.1));
    z-index: 1000;
    animation: dropdown-fade 0.2s ease;
}

.sh-dropdown--end .sh-dropdown__menu {
    left: auto;
    right: 0;
}

@keyframes dropdown-fade {
    from {
        opacity: 0;
        transform: translateY(-4px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dropdown_creation() {
        let dd = Dropdown::new(html! { "Menu" }, html! { "Items" }, "Dropdown");
        assert!(!dd.open);
        assert!(!dd.align_end);
    }

    #[test]
    fn test_dropdown_open() {
        let dd = Dropdown::new(html! {}, html! {}, "Test").open(true);
        assert!(dd.open);
    }

    #[test]
    fn test_dropdown_align_end() {
        let dd = Dropdown::new(html! {}, html! {}, "Test").align_end(true);
        assert!(dd.align_end);
    }

    #[test]
    fn test_dropdown_css() {
        let css = dropdown_css();
        assert!(css.contains(".sh-dropdown"));
        assert!(css.contains(".sh-dropdown__menu"));
    }
}
