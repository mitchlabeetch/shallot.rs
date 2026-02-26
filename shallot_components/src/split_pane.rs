//! Split Pane - Resizable split panel layout
//! CSS-only layout with fixed ratio splits

use maud::{html, Markup, Render};

/// Split direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SplitDirection {
    #[default]
    Horizontal,
    Vertical,
}

/// Split pane component
#[derive(Debug, Clone)]
pub struct SplitPane<'a> {
    pub direction: SplitDirection,
    pub first: Markup,
    pub second: Markup,
    pub first_size: &'a str,
    pub second_size: &'a str,
    pub divider: bool,
    pub min_first: Option<&'a str>,
    pub min_second: Option<&'a str>,
}

impl<'a> SplitPane<'a> {
    pub fn new(first: Markup, second: Markup) -> Self {
        Self {
            direction: SplitDirection::default(),
            first,
            second,
            first_size: "1fr",
            second_size: "1fr",
            divider: true,
            min_first: None,
            min_second: None,
        }
    }

    pub fn direction(mut self, direction: SplitDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn first_size(mut self, size: &'a str) -> Self {
        self.first_size = size;
        self
    }

    pub fn second_size(mut self, size: &'a str) -> Self {
        self.second_size = size;
        self
    }

    pub fn divider(mut self, divider: bool) -> Self {
        self.divider = divider;
        self
    }

    pub fn min_first(mut self, min: &'a str) -> Self {
        self.min_first = Some(min);
        self
    }

    pub fn min_second(mut self, min: &'a str) -> Self {
        self.min_second = Some(min);
        self
    }

    fn build_classes(&self) -> String {
        match self.direction {
            SplitDirection::Horizontal => "sh-split-pane sh-split-pane--horizontal".to_string(),
            SplitDirection::Vertical => "sh-split-pane sh-split-pane--vertical".to_string(),
        }
    }

    fn build_style(&self) -> String {
        let mut styles = vec![
            format!("--sh-split-first: {}", self.first_size),
            format!("--sh-split-second: {}", self.second_size),
        ];

        if let Some(min) = self.min_first {
            styles.push(format!("--sh-split-min-first: {}", min));
        }

        if let Some(min) = self.min_second {
            styles.push(format!("--sh-split-min-second: {}", min));
        }

        styles.join("; ")
    }
}

impl<'a> Render for SplitPane<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let style = self.build_style();

        html! {
            div
                class=(classes)
                style=(style)
                role="application"
                aria-label="Split pane layout"
            {
                div
                    class="sh-split-pane__first"
                    role="region"
                    aria-label="First pane"
                {
                    (self.first)
                }

                @if self.divider {
                    div
                        class="sh-split-pane__divider"
                        role="separator"
                        aria-orientation=(match self.direction {
                            SplitDirection::Horizontal => "vertical",
                            SplitDirection::Vertical => "horizontal",
                        })
                    {}
                }

                div
                    class="sh-split-pane__second"
                    role="region"
                    aria-label="Second pane"
                {
                    (self.second)
                }
            }
        }
    }
}

pub fn split_pane_css() -> String {
    r#"
.sh-split-pane {
    display: grid;
    width: 100%;
    height: 100%;
    overflow: hidden;
}

.sh-split-pane--horizontal {
    grid-template-columns: var(--sh-split-first, 1fr) auto var(--sh-split-second, 1fr);
    grid-template-rows: 1fr;
}

.sh-split-pane--vertical {
    grid-template-columns: 1fr;
    grid-template-rows: var(--sh-split-first, 1fr) auto var(--sh-split-second, 1fr);
}

.sh-split-pane__first {
    min-width: var(--sh-split-min-first, 0);
    min-height: var(--sh-split-min-first, 0);
    overflow: auto;
}

.sh-split-pane__second {
    min-width: var(--sh-split-min-second, 0);
    min-height: var(--sh-split-min-second, 0);
    overflow: auto;
}

.sh-split-pane__divider {
    background: var(--sh-color-border, #e5e5e5);
    flex-shrink: 0;
}

.sh-split-pane--horizontal .sh-split-pane__divider {
    width: 1px;
    cursor: col-resize;
    margin: 0 var(--sh-spacing-xs, 0.25rem);
}

.sh-split-pane--vertical .sh-split-pane__divider {
    height: 1px;
    cursor: row-resize;
    margin: var(--sh-spacing-xs, 0.25rem) 0;
}

.sh-split-pane__divider:hover {
    background: var(--sh-color-primary, #3b82f6);
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_pane_creation() {
        let pane = SplitPane::new(html! { "First" }, html! { "Second" })
            .direction(SplitDirection::Vertical);

        assert_eq!(pane.direction, SplitDirection::Vertical);
    }

    #[test]
    fn test_split_pane_render() {
        let pane = SplitPane::new(html! { "A" }, html! { "B" });
        let html = pane.render().into_string();

        assert!(html.contains("sh-split-pane"));
        assert!(html.contains("A"));
        assert!(html.contains("B"));
    }

    #[test]
    fn test_split_pane_sizes() {
        let pane = SplitPane::new(html! {}, html! {})
            .first_size("200px")
            .second_size("1fr");

        let style = pane.build_style();
        assert!(style.contains("--sh-split-first: 200px"));
        assert!(style.contains("--sh-split-second: 1fr"));
    }

    #[test]
    fn test_split_pane_no_divider() {
        let pane = SplitPane::new(html! {}, html! {}).divider(false);
        let html = pane.render().into_string();

        assert!(!html.contains("sh-split-pane__divider"));
    }

    #[test]
    fn test_css_generation() {
        let css = split_pane_css();
        assert!(css.contains(".sh-split-pane"));
    }
}
