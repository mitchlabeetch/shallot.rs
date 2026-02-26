use maud::{html, Markup, Render};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PopoverVariant {
    #[default]
    Default,
    Dark,
    Light,
    Bordered,
    Shadow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PopoverSize {
    Sm,
    #[default]
    Md,
    Lg,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PopoverPosition {
    #[default]
    Bottom,
    Top,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Popover<'a> {
    pub trigger: Markup,
    pub content: Markup,
    pub variant: PopoverVariant,
    pub size: PopoverSize,
    pub position: PopoverPosition,
    pub align_end: bool,
    pub title: Option<&'a str>,
    pub id: Option<&'a str>,
}

impl<'a> Default for Popover<'a> {
    fn default() -> Self {
        Self {
            trigger: html! { "Trigger" },
            content: html! { "Content" },
            variant: PopoverVariant::Default,
            size: PopoverSize::Md,
            position: PopoverPosition::Bottom,
            align_end: false,
            title: None,
            id: None,
        }
    }
}

impl<'a> Popover<'a> {
    pub fn new(trigger: Markup, content: Markup) -> Self {
        Self {
            trigger,
            content,
            ..Default::default()
        }
    }

    pub fn variant(mut self, variant: PopoverVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: PopoverSize) -> Self {
        self.size = size;
        self
    }

    pub fn position(mut self, position: PopoverPosition) -> Self {
        self.position = position;
        self
    }

    pub fn align_end(mut self, align_end: bool) -> Self {
        self.align_end = align_end;
        self
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-popover"];

        match self.variant {
            PopoverVariant::Default => classes.push("sh-popover--default"),
            PopoverVariant::Dark => classes.push("sh-popover--dark"),
            PopoverVariant::Light => classes.push("sh-popover--light"),
            PopoverVariant::Bordered => classes.push("sh-popover--bordered"),
            PopoverVariant::Shadow => classes.push("sh-popover--shadow"),
        }

        match self.size {
            PopoverSize::Sm => classes.push("sh-popover--sm"),
            PopoverSize::Md => classes.push("sh-popover--md"),
            PopoverSize::Lg => classes.push("sh-popover--lg"),
        }

        match self.position {
            PopoverPosition::Bottom => classes.push("sh-popover--bottom"),
            PopoverPosition::Top => classes.push("sh-popover--top"),
            PopoverPosition::Left => classes.push("sh-popover--left"),
            PopoverPosition::Right => classes.push("sh-popover--right"),
        }

        if self.align_end {
            classes.push("sh-popover--align-end");
        }

        classes.join(" ")
    }
}

impl<'a> Render for Popover<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();

        html! {
            div class=(classes) {
                details class="sh-popover__wrapper" {
                    summary
                        class="sh-popover__trigger"
                        role="button"
                        aria-expanded="false"
                        aria-haspopup="true"
                        tabindex="0"
                    { (self.trigger) }
                    div
                        class="sh-popover__panel"
                        role="tooltip"
                        id=(self.id.unwrap_or("popover-panel"))
                    {
                        @if let Some(title) = self.title {
                            div class="sh-popover__header" { (title) }
                        }
                        div class="sh-popover__content" { (self.content) }
                    }
                }
            }
        }
    }
}

pub fn popover_css() -> String {
    r#"
.sh-popover {
    --popover-bg: var(--sh-surface, #fff);
    --popover-border: var(--sh-border, #e5e7eb);
    --popover-text: var(--sh-text, #1f2937);
    --popover-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -4px rgba(0, 0, 0, 0.1);
    
    display: inline-block;
    position: relative;
}

.sh-popover__wrapper {
    display: inline-block;
    position: relative;
    list-style: none;
}

.sh-popover__wrapper summary {
    list-style: none;
    cursor: pointer;
}

.sh-popover__wrapper summary::-webkit-details-marker {
    display: none;
}

/* Trigger */
.sh-popover__trigger {
    cursor: pointer;
    display: inline-block;
}

/* Panel */
.sh-popover__panel {
    position: absolute;
    z-index: 50;
    min-width: 200px;
    background: var(--popover-bg);
    border-radius: 0.5rem;
    padding: 0.75rem;
    box-shadow: var(--popover-shadow);
    opacity: 0;
    visibility: hidden;
    transform: translateY(-8px);
    transition: all 0.2s ease;
}

.sh-popover__wrapper[open] .sh-popover__panel {
    opacity: 1;
    visibility: visible;
    transform: translateY(0);
}

/* Variants */
.sh-popover--default .sh-popover__panel {
    border: 1px solid var(--popover-border);
}

.sh-popover--dark .sh-popover__panel {
    background: #1f2937;
    border: 1px solid #374151;
    color: #f9fafb;
}

.sh-popover--light .sh-popover__panel {
    background: #f9fafb;
    border: 1px solid #e5e7eb;
}

.sh-popover--bordered .sh-popover__panel {
    border: 2px solid var(--popover-border);
}

.sh-popover--shadow .sh-popover__panel {
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 8px 10px -6px rgba(0, 0, 0, 0.1);
}

/* Sizes */
.sh-popover--sm .sh-popover__panel {
    min-width: 150px;
    padding: 0.5rem;
    font-size: 0.75rem;
}

.sh-popover--md .sh-popover__panel {
    min-width: 200px;
    padding: 0.75rem;
    font-size: 0.875rem;
}

.sh-popover--lg .sh-popover__panel {
    min-width: 300px;
    padding: 1rem;
    font-size: 1rem;
}

/* Positions */
.sh-popover--bottom .sh-popover__panel {
    top: 100%;
    margin-top: 0.5rem;
}

.sh-popover--top .sh-popover__panel {
    bottom: 100%;
    margin-bottom: 0.5rem;
}

.sh-popover--left .sh-popover__panel {
    right: 100%;
    margin-right: 0.5rem;
}

.sh-popover--right .sh-popover__panel {
    left: 100%;
    margin-left: 0.5rem;
}

/* Align end */
.sh-popover--align-end .sh-popover__panel {
    right: 0;
}

/* Header */
.sh-popover__header {
    font-weight: 600;
    color: var(--popover-text);
    padding-bottom: 0.5rem;
    margin-bottom: 0.5rem;
    border-bottom: 1px solid var(--popover-border);
}

/* Content */
.sh-popover__content {
    color: var(--popover-text);
}

/* Reduced motion */
@media (prefers-reduced-motion: reduce) {
    .sh-popover__panel {
        transition: none;
    }
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use maud::html;

    #[test]
    fn test_popover_creation() {
        let popover = Popover::new(html! { "Click me" }, html! { "Popover content" });
        let html_str = popover.trigger.0.as_str();
        assert!(html_str.contains("Click me"));
    }

    #[test]
    fn test_popover_title() {
        let popover = Popover::new(html! { "Trigger" }, html! { "Content" }).title("Title");

        assert_eq!(popover.title, Some("Title"));
    }

    #[test]
    fn test_popover_id() {
        let popover = Popover::new(html! { "Trigger" }, html! { "Content" }).id("my-popover");

        assert_eq!(popover.id, Some("my-popover"));
    }

    #[test]
    fn test_popover_variants() {
        let default = Popover::new(html! {}, html! {}).variant(PopoverVariant::Default);
        assert_eq!(default.variant, PopoverVariant::Default);

        let dark = Popover::new(html! {}, html! {}).variant(PopoverVariant::Dark);
        assert_eq!(dark.variant, PopoverVariant::Dark);

        let light = Popover::new(html! {}, html! {}).variant(PopoverVariant::Light);
        assert_eq!(light.variant, PopoverVariant::Light);

        let bordered = Popover::new(html! {}, html! {}).variant(PopoverVariant::Bordered);
        assert_eq!(bordered.variant, PopoverVariant::Bordered);

        let shadow = Popover::new(html! {}, html! {}).variant(PopoverVariant::Shadow);
        assert_eq!(shadow.variant, PopoverVariant::Shadow);
    }

    #[test]
    fn test_popover_sizes() {
        let sm = Popover::new(html! {}, html! {}).size(PopoverSize::Sm);
        assert_eq!(sm.size, PopoverSize::Sm);

        let md = Popover::new(html! {}, html! {}).size(PopoverSize::Md);
        assert_eq!(md.size, PopoverSize::Md);

        let lg = Popover::new(html! {}, html! {}).size(PopoverSize::Lg);
        assert_eq!(lg.size, PopoverSize::Lg);
    }

    #[test]
    fn test_popover_positions() {
        let bottom = Popover::new(html! {}, html! {}).position(PopoverPosition::Bottom);
        assert_eq!(bottom.position, PopoverPosition::Bottom);

        let top = Popover::new(html! {}, html! {}).position(PopoverPosition::Top);
        assert_eq!(top.position, PopoverPosition::Top);

        let left = Popover::new(html! {}, html! {}).position(PopoverPosition::Left);
        assert_eq!(left.position, PopoverPosition::Left);

        let right = Popover::new(html! {}, html! {}).position(PopoverPosition::Right);
        assert_eq!(right.position, PopoverPosition::Right);
    }

    #[test]
    fn test_popover_align_end() {
        let popover = Popover::new(html! {}, html! {}).align_end(true);
        assert!(popover.align_end);
    }

    #[test]
    fn test_popover_render() {
        let popover = Popover::new(html! { "Trigger" }, html! { "Content" }).title("Popover Title");

        let rendered = popover.render();
        let html_str = rendered.0.as_str();

        assert!(html_str.contains("sh-popover"));
        assert!(html_str.contains("Trigger"));
        assert!(html_str.contains("Content"));
    }

    #[test]
    fn test_popover_aria() {
        let popover = Popover::new(html! { "Trigger" }, html! { "Content" });

        let rendered = popover.render();
        let html_str = rendered.0.as_str();

        assert!(html_str.contains("role=\"button\""));
        assert!(html_str.contains("role=\"tooltip\""));
    }

    #[test]
    fn test_popover_css() {
        let css = popover_css();
        assert!(css.contains(".sh-popover"));
        assert!(css.contains(".sh-popover--default"));
        assert!(css.contains(".sh-popover--dark"));
        assert!(css.contains(".sh-popover--light"));
        assert!(css.contains(".sh-popover--bordered"));
        assert!(css.contains(".sh-popover--shadow"));
        assert!(css.contains(".sh-popover--sm"));
        assert!(css.contains(".sh-popover--md"));
        assert!(css.contains(".sh-popover--lg"));
        assert!(css.contains(".sh-popover--bottom"));
        assert!(css.contains(".sh-popover--top"));
        assert!(css.contains(".sh-popover--left"));
        assert!(css.contains(".sh-popover--right"));
    }
}
