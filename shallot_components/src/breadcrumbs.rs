use maud::{html, Markup};

pub struct BreadcrumbItem<'a> {
    pub label: &'a str,
    pub href: Option<&'a str>,
}

pub struct Breadcrumbs<'a> {
    pub items: Vec<BreadcrumbItem<'a>>,
}

impl<'a> Breadcrumbs<'a> {
    pub fn new(items: Vec<BreadcrumbItem<'a>>) -> Self {
        Self { items }
    }

    pub fn render(self) -> Markup {
        html! {
            nav aria-label="Breadcrumb" {
                ol class="sh-bc" {
                    @for (idx, it) in self.items.iter().enumerate() {
                        li class="sh-bc__item" {
                            @if let Some(href) = it.href {
                                a href=(href) { (it.label) }
                            } @else {
                                span aria-current="page" { (it.label) }
                            }
                            @if idx + 1 != self.items.len() {
                                span class="sh-bc__sep" aria-hidden="true" { "/" }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Generate CSS for breadcrumbs component
pub fn breadcrumbs_css() -> String {
    r#"
.sh-bc {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    list-style: none;
    padding: 0;
    margin: 0;
    gap: 0.5rem;
    font-size: 0.875rem;
}

.sh-bc__item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
}

.sh-bc__item a {
    color: var(--sh-primary, #3b82f6);
    text-decoration: none;
    transition: color 0.2s ease;
}

.sh-bc__item a:hover {
    color: var(--sh-primary-hover, #2563eb);
    text-decoration: underline;
}

.sh-bc__item span[aria-current="page"] {
    color: var(--sh-text-muted, #6b7280);
    font-weight: 500;
}

.sh-bc__sep {
    color: var(--sh-text-muted, #9ca3af);
    user-select: none;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breadcrumbs_creation() {
        let items = vec![
            BreadcrumbItem {
                label: "Home",
                href: Some("/"),
            },
            BreadcrumbItem {
                label: "Current",
                href: None,
            },
        ];
        let bc = Breadcrumbs::new(items);
        assert_eq!(bc.items.len(), 2);
    }

    #[test]
    fn test_breadcrumbs_render() {
        let items = vec![
            BreadcrumbItem {
                label: "Home",
                href: Some("/"),
            },
            BreadcrumbItem {
                label: "Page",
                href: None,
            },
        ];
        let bc = Breadcrumbs::new(items);
        let html = bc.render().into_string();
        assert!(html.contains("sh-bc"));
        assert!(html.contains("aria-label=\"Breadcrumb\""));
        assert!(html.contains("aria-current=\"page\""));
    }

    #[test]
    fn test_breadcrumbs_css() {
        let css = breadcrumbs_css();
        assert!(css.contains(".sh-bc"));
        assert!(css.contains(".sh-bc__item"));
    }
}
