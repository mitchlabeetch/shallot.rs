//! Bottom Sheet Component - Mobile-style bottom drawer
//! CSS-only using details/summary for open/close

use maud::{html, Markup, Render};

/// Bottom sheet size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BottomSheetSize {
    #[default]
    Auto,
    Half,
    Full,
}

/// Bottom sheet component
#[derive(Debug, Clone)]
pub struct BottomSheet<'a> {
    pub id: &'a str,
    pub title: Option<&'a str>,
    pub children: Markup,
    pub size: BottomSheetSize,
    pub dismissible: bool,
    pub handle: bool,
}

impl<'a> BottomSheet<'a> {
    pub fn new(id: &'a str, children: Markup) -> Self {
        Self {
            id,
            title: None,
            children,
            size: BottomSheetSize::default(),
            dismissible: true,
            handle: true,
        }
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    pub fn size(mut self, size: BottomSheetSize) -> Self {
        self.size = size;
        self
    }

    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    pub fn handle(mut self, handle: bool) -> Self {
        self.handle = handle;
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-bottom-sheet"];

        match self.size {
            BottomSheetSize::Auto => classes.push("sh-bottom-sheet--auto"),
            BottomSheetSize::Half => classes.push("sh-bottom-sheet--half"),
            BottomSheetSize::Full => classes.push("sh-bottom-sheet--full"),
        }

        classes.join(" ")
    }
}

impl<'a> Render for BottomSheet<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let sheet_id = format!("{}-sheet", self.id);

        html! {
            div class="sh-bottom-sheet-container" {
                input type="checkbox" id=(sheet_id) class="sh-bottom-sheet__trigger" hidden;

                label for=(sheet_id) class="sh-bottom-sheet__backdrop" {}

                div class=(classes) role="dialog" aria-modal="true" aria-labelledby=(format!("{}-title", self.id)) {
                    @if self.handle {
                        div class="sh-bottom-sheet__handle" {}
                    }

                    @if self.title.is_some() || self.dismissible {
                        div class="sh-bottom-sheet__header" {
                            @if let Some(title) = self.title {
                                h2 id=(format!("{}-title", self.id)) class="sh-bottom-sheet__title" {
                                    (title)
                                }
                            }
                            @if self.dismissible {
                                label for=(sheet_id) class="sh-bottom-sheet__close" aria-label="Close" {
                                    "×"
                                }
                            }
                        }
                    }

                    div class="sh-bottom-sheet__content" {
                        (self.children)
                    }
                }
            }
        }
    }
}

pub fn bottom_sheet_css() -> String {
    r#"
.sh-bottom-sheet-container {
    position: relative;
}

.sh-bottom-sheet__trigger {
    position: absolute;
    opacity: 0;
    pointer-events: none;
}

.sh-bottom-sheet__backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    z-index: 100;
    opacity: 0;
    visibility: hidden;
    transition: opacity 0.3s ease, visibility 0.3s ease;
    cursor: pointer;
}

.sh-bottom-sheet__trigger:checked ~ .sh-bottom-sheet__backdrop {
    opacity: 1;
    visibility: visible;
}

.sh-bottom-sheet {
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    z-index: 101;
    background: var(--sh-color-surface, #ffffff);
    border-radius: var(--sh-radius-xl, 1rem) var(--sh-radius-xl, 1rem) 0 0;
    box-shadow: var(--sh-shadow-2xl, 0 25px 50px -12px rgba(0, 0, 0, 0.25));
    transform: translateY(100%);
    transition: transform 0.3s ease;
    max-height: 90vh;
    overflow: hidden;
    display: flex;
    flex-direction: column;
}

.sh-bottom-sheet__trigger:checked ~ .sh-bottom-sheet {
    transform: translateY(0);
}

.sh-bottom-sheet--auto {
    height: auto;
}

.sh-bottom-sheet--half {
    height: 50vh;
}

.sh-bottom-sheet--full {
    height: 90vh;
}

.sh-bottom-sheet__handle {
    width: 36px;
    height: 4px;
    background: var(--sh-color-surface-muted, #e5e5e5);
    border-radius: var(--sh-radius-full, 9999px);
    margin: var(--sh-spacing-sm, 0.5rem) auto;
    flex-shrink: 0;
}

.sh-bottom-sheet__header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--sh-spacing-sm, 0.5rem) var(--sh-spacing-md, 1rem);
    border-bottom: 1px solid var(--sh-color-border, #e5e5e5);
    flex-shrink: 0;
}

.sh-bottom-sheet__title {
    font-size: var(--sh-font-size-lg, 1.125rem);
    font-weight: var(--sh-font-weight-semibold, 600);
    margin: 0;
}

.sh-bottom-sheet__close {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.5rem;
    color: var(--sh-color-text-muted, #666);
    cursor: pointer;
    border-radius: var(--sh-radius-full, 9999px);
    transition: background-color 0.15s ease;
}

.sh-bottom-sheet__close:hover {
    background: var(--sh-color-surface-muted, #e5e5e5);
}

.sh-bottom-sheet__content {
    flex: 1;
    overflow-y: auto;
    padding: var(--sh-spacing-md, 1rem);
}

/* Open trigger for external use */
.sh-bottom-sheet-open {
    cursor: pointer;
}

.sh-bottom-sheet-open:checked + .sh-bottom-sheet-container .sh-bottom-sheet__backdrop,
.sh-bottom-sheet__trigger:checked ~ .sh-bottom-sheet {
    opacity: 1;
    visibility: visible;
    transform: translateY(0);
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bottom_sheet_creation() {
        let sheet = BottomSheet::new("test", html! { "Content" })
            .title("My Sheet")
            .size(BottomSheetSize::Half);

        assert_eq!(sheet.id, "test");
        assert_eq!(sheet.title, Some("My Sheet"));
        assert_eq!(sheet.size, BottomSheetSize::Half);
    }

    #[test]
    fn test_bottom_sheet_render() {
        let sheet = BottomSheet::new("sheet", html! { p { "Hello" } });
        let rendered = sheet.render();
        let html = rendered.into_string();

        assert!(html.contains("sh-bottom-sheet"));
        assert!(html.contains("Hello"));
    }

    #[test]
    fn test_bottom_sheet_sizes() {
        let auto = BottomSheet::new("auto", html! {}).size(BottomSheetSize::Auto);
        let full = BottomSheet::new("full", html! {}).size(BottomSheetSize::Full);

        assert!(auto.build_classes().contains("sh-bottom-sheet--auto"));
        assert!(full.build_classes().contains("sh-bottom-sheet--full"));
    }

    #[test]
    fn test_bottom_sheet_handle() {
        let with_handle = BottomSheet::new("h1", html! {}).handle(true);
        let without_handle = BottomSheet::new("h2", html! {}).handle(false);

        let html_with = with_handle.render().into_string();
        let html_without = without_handle.render().into_string();

        assert!(html_with.contains("sh-bottom-sheet__handle"));
        assert!(!html_without.contains("sh-bottom-sheet__handle"));
    }

    #[test]
    fn test_css_generation() {
        let css = bottom_sheet_css();
        assert!(css.contains(".sh-bottom-sheet"));
        assert!(css.contains(".sh-bottom-sheet__content"));
    }
}
