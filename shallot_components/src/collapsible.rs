use maud::{html, Markup, Render};

pub struct Collapsible<'a> {
    pub summary: Markup,
    pub content: Markup,
    pub open: bool,
    pub class: Option<&'a str>,
}

impl<'a> Collapsible<'a> {
    pub fn new(summary: Markup, content: Markup) -> Self {
        Self {
            summary,
            content,
            open: false,
            class: None,
        }
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn class(mut self, class: &'a str) -> Self {
        self.class = Some(class);
        self
    }
}

impl<'a> Render for Collapsible<'a> {
    fn render(&self) -> Markup {
        let class = if let Some(extra) = self.class {
            format!("sh-collapsible {}", extra)
        } else {
            "sh-collapsible".to_string()
        };

        html! {
            details class=(class) open?[self.open] {
                summary class="sh-collapsible__summary" {
                    (self.summary)
                    span class="sh-collapsible__chev" aria-hidden="true" { "▾" }
                }
                div class="sh-collapsible__content" { (self.content) }
            }
        }
    }
}

/// Generate CSS for collapsible component
pub fn collapsible_css() -> String {
    r#"
.sh-collapsible {
    border: 1px solid var(--sh-border, #e5e7eb);
    border-radius: var(--sh-radius-md, 0.375rem);
    overflow: hidden;
}

.sh-collapsible__summary {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 1rem;
    background: var(--sh-surface-2, #f9fafb);
    cursor: pointer;
    font-weight: 500;
    list-style: none;
    transition: background 0.2s ease;
}

.sh-collapsible__summary:hover {
    background: var(--sh-surface-hover, #f3f4f6);
}

.sh-collapsible__summary::-webkit-details-marker {
    display: none;
}

.sh-collapsible__chev {
    transition: transform 0.2s ease;
    color: var(--sh-text-muted, #9ca3af);
}

.sh-collapsible[open] .sh-collapsible__chev {
    transform: rotate(180deg);
}

.sh-collapsible__content {
    padding: 1rem;
    border-top: 1px solid var(--sh-border, #e5e7eb);
    animation: collapsible-slide 0.2s ease;
}

@keyframes collapsible-slide {
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
    fn test_collapsible_creation() {
        let coll = Collapsible::new(html! { "Summary" }, html! { "Content" });
        assert!(!coll.open);
        assert_eq!(coll.class, None);
    }

    #[test]
    fn test_collapsible_open() {
        let coll = Collapsible::new(html! {}, html! {}).open(true);
        assert!(coll.open);
    }

    #[test]
    fn test_collapsible_class() {
        let coll = Collapsible::new(html! {}, html! {}).class("custom");
        assert_eq!(coll.class, Some("custom"));
    }

    #[test]
    fn test_collapsible_css() {
        let css = collapsible_css();
        assert!(css.contains(".sh-collapsible"));
        assert!(css.contains(".sh-collapsible__summary"));
    }
}
