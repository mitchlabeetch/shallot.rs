//! Truncatable Component - Text truncation with expand/collapse
//! CSS-only text truncation with line-clamp support

use maud::{html, Markup, Render};

/// Truncatable component for text that can be expanded
#[derive(Debug, Clone)]
pub struct Truncatable<'a> {
    pub text: &'a str,
    pub max_lines: u8,
    pub expand_label: Option<&'a str>,
    pub collapse_label: Option<&'a str>,
    pub id: Option<&'a str>,
}

impl<'a> Truncatable<'a> {
    /// Create a new truncatable text component
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            max_lines: 3,
            expand_label: None,
            collapse_label: None,
            id: None,
        }
    }

    /// Set the maximum number of lines before truncation
    pub fn max_lines(mut self, lines: u8) -> Self {
        self.max_lines = lines;
        self
    }

    /// Set the expand button label
    pub fn expand_label(mut self, label: &'a str) -> Self {
        self.expand_label = Some(label);
        self
    }

    /// Set the collapse button label
    pub fn collapse_label(mut self, label: &'a str) -> Self {
        self.collapse_label = Some(label);
        self
    }

    /// Set a unique ID for the component
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-truncatable".to_string()];

        let lines_class = match self.max_lines {
            1 => "sh-truncatable--1-line",
            2 => "sh-truncatable--2-lines",
            3 => "sh-truncatable--3-lines",
            4 => "sh-truncatable--4-lines",
            5 => "sh-truncatable--5-lines",
            _ => "sh-truncatable--3-lines",
        };
        classes.push(lines_class.to_string());

        classes.join(" ")
    }
}

impl<'a> Render for Truncatable<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let expand_label = self.expand_label.unwrap_or("Show more");
        let collapse_label = self.collapse_label.unwrap_or("Show less");
        let id = self.id.unwrap_or("truncatable");

        html! {
            div class=(classes) data-truncatable-id=(id) {
                div class="sh-truncatable__content" {
                    (self.text)
                }
                div class="sh-truncatable__actions" {
                    button
                        type="button"
                        class="sh-truncatable__btn sh-truncatable__btn--expand"
                        data-action="expand"
                        aria-expanded="false" {
                        (expand_label)
                    }
                    button
                        type="button"
                        class="sh-truncatable__btn sh-truncatable__btn--collapse"
                        data-action="collapse"
                        aria-expanded="true" {
                        (collapse_label)
                    }
                }
            }
        }
    }
}

/// Generate CSS for truncatable component
pub fn truncatable_css() -> String {
    r#"
.sh-truncatable {
    position: relative;
}

.sh-truncatable__content {
    overflow: hidden;
    text-overflow: ellipsis;
    display: -webkit-box;
    -webkit-box-orient: vertical;
    transition: max-height 0.3s ease;
}

.sh-truncatable--1-line .sh-truncatable__content {
    -webkit-line-clamp: 1;
}

.sh-truncatable--2-lines .sh-truncatable__content {
    -webkit-line-clamp: 2;
}

.sh-truncatable--3-lines .sh-truncatable__content {
    -webkit-line-clamp: 3;
}

.sh-truncatable--4-lines .sh-truncatable__content {
    -webkit-line-clamp: 4;
}

.sh-truncatable--5-lines .sh-truncatable__content {
    -webkit-line-clamp: 5;
}

.sh-truncatable--expanded .sh-truncatable__content {
    -webkit-line-clamp: unset;
}

.sh-truncatable__actions {
    display: flex;
    gap: var(--sh-spacing-2, 0.5rem);
    margin-top: var(--sh-spacing-2, 0.5rem);
}

.sh-truncatable__btn {
    padding: 0;
    border: none;
    background: none;
    color: var(--sh-color-primary, #3b82f6);
    font-size: var(--sh-font-size-sm, 0.875rem);
    font-weight: var(--sh-font-weight-medium, 500);
    cursor: pointer;
    text-decoration: underline;
    text-underline-offset: 2px;
}

.sh-truncatable__btn:hover {
    color: var(--sh-color-primary-hover, #2563eb);
}

.sh-truncatable__btn:focus-visible {
    outline: 2px solid var(--sh-color-primary, #3b82f6);
    outline-offset: 2px;
    border-radius: var(--sh-radius-sm, 0.25rem);
}

.sh-truncatable__btn--collapse {
    display: none;
}

.sh-truncatable--expanded .sh-truncatable__btn--expand {
    display: none;
}

.sh-truncatable--expanded .sh-truncatable__btn--collapse {
    display: inline;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncatable_creation() {
        let trunc = Truncatable::new("This is some long text");

        assert_eq!(trunc.text, "This is some long text");
        assert_eq!(trunc.max_lines, 3);
    }

    #[test]
    fn test_truncatable_render() {
        let trunc = Truncatable::new("This is a very long text that will be truncated")
            .max_lines(2)
            .expand_label("Read more")
            .collapse_label("Read less");

        let html = trunc.render().into_string();
        assert!(html.contains("sh-truncatable"));
        assert!(html.contains("sh-truncatable--2-lines"));
        assert!(html.contains("Read more"));
        assert!(html.contains("Read less"));
    }

    #[test]
    fn test_truncatable_line_variants() {
        let one = Truncatable::new("test").max_lines(1);
        let three = Truncatable::new("test").max_lines(3);
        let five = Truncatable::new("test").max_lines(5);

        assert!(one
            .render()
            .into_string()
            .contains("sh-truncatable--1-line"));
        assert!(three
            .render()
            .into_string()
            .contains("sh-truncatable--3-lines"));
        assert!(five
            .render()
            .into_string()
            .contains("sh-truncatable--5-lines"));
    }

    #[test]
    fn test_truncatable_with_id() {
        let trunc = Truncatable::new("test").id("my-text");
        let html = trunc.render().into_string();

        assert!(html.contains("data-truncatable-id=\"my-text\""));
    }

    #[test]
    fn test_truncatable_css() {
        let css = truncatable_css();
        assert!(css.contains(".sh-truncatable"));
        assert!(css.contains(".sh-truncatable__content"));
        assert!(css.contains("-webkit-line-clamp"));
    }
}
