use maud::{html, Markup, Render};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SurfaceVariant {
    #[default]
    Default,
    Elevated,
    Bordered,
    Glass,
    Flat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SurfaceSize {
    #[default]
    Md,
    Sm,
    Lg,
    XL,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SurfaceRadius {
    #[default]
    Md,
    None,
    Sm,
    Lg,
    Full,
}

pub struct Surface {
    pub children: Markup,
    pub variant: SurfaceVariant,
    pub size: SurfaceSize,
    pub radius: SurfaceRadius,
    pub padding: Option<u32>,
}

impl Default for Surface {
    fn default() -> Self {
        Self {
            children: html! {},
            variant: SurfaceVariant::Default,
            size: SurfaceSize::Md,
            radius: SurfaceRadius::Md,
            padding: None,
        }
    }
}

impl Surface {
    pub fn new(children: Markup) -> Self {
        Self::default().children(children)
    }

    pub fn variant(mut self, variant: SurfaceVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: SurfaceSize) -> Self {
        self.size = size;
        self
    }

    pub fn radius(mut self, radius: SurfaceRadius) -> Self {
        self.radius = radius;
        self
    }

    pub fn padding(mut self, padding: u32) -> Self {
        self.padding = Some(padding);
        self
    }

    pub fn children(mut self, children: Markup) -> Self {
        self.children = children;
        self
    }
}

impl Render for Surface {
    fn render(&self) -> Markup {
        let variant_class = match self.variant {
            SurfaceVariant::Default => "sh-surface",
            SurfaceVariant::Elevated => "sh-surface sh-surface--elevated",
            SurfaceVariant::Bordered => "sh-surface sh-surface--bordered",
            SurfaceVariant::Glass => "sh-surface sh-surface--glass",
            SurfaceVariant::Flat => "sh-surface sh-surface--flat",
        };

        let size_class = match self.size {
            SurfaceSize::Sm => "sh-surface--sm",
            SurfaceSize::Md => "sh-surface--md",
            SurfaceSize::Lg => "sh-surface--lg",
            SurfaceSize::XL => "sh-surface--xl",
        };

        let radius_class = match self.radius {
            SurfaceRadius::None => "sh-surface--radius-none",
            SurfaceRadius::Sm => "sh-surface--radius-sm",
            SurfaceRadius::Md => "sh-surface--radius-md",
            SurfaceRadius::Lg => "sh-surface--radius-lg",
            SurfaceRadius::Full => "sh-surface--radius-full",
        };

        let padding_style = self
            .padding
            .map(|p| format!("padding: {}px;", p))
            .unwrap_or_default();

        html! {
            div
                class=(format!("{} {} {}", variant_class, size_class, radius_class))
                style=(padding_style)
            {
                (self.children)
            }
        }
    }
}

pub struct Paper {
    pub children: Markup,
}

impl Paper {
    pub fn new(children: Markup) -> Self {
        Self { children }
    }
}

impl Render for Paper {
    fn render(&self) -> Markup {
        html! {
            div class="sh-paper" {
                (self.children)
            }
        }
    }
}

pub struct AccordionItem<'a> {
    pub title: &'a str,
    pub content: Markup,
    pub open: bool,
}

pub struct Accordion<'a> {
    pub items: Vec<AccordionItem<'a>>,
}

impl<'a> Accordion<'a> {
    pub fn new(items: Vec<AccordionItem<'a>>) -> Self {
        Self { items }
    }
}

impl<'a> Render for Accordion<'a> {
    fn render(&self) -> Markup {
        html! {
            div class="sh-paper" style="padding: 0" {
                @for item in &self.items {
                    details class="sh-accordion-item" open?[item.open] {
                        summary class="sh-accordion-trigger" {
                            (item.title)
                            span style="font-size: 10px" { "▼" }
                        }
                        div class="sh-accordion-content" {
                            (item.content)
                        }
                    }
                }
            }
        }
    }
}

pub fn surface_css() -> String {
    r#"
.sh-surface {
    background: var(--sh-bg-primary, #ffffff);
    transition: all 0.2s ease;
}

.sh-surface--sm {
    padding: 0.5rem;
}

.sh-surface--md {
    padding: 1rem;
}

.sh-surface--lg {
    padding: 1.5rem;
}

.sh-surface--xl {
    padding: 2rem;
}

.sh-surface--elevated {
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.08);
}

.sh-surface--bordered {
    border: 1px solid var(--sh-border-color, #e5e7eb);
}

.sh-surface--glass {
    background: rgba(255, 255, 255, 0.7);
    backdrop-filter: blur(12px);
    border: 1px solid rgba(255, 255, 255, 0.3);
}

.sh-surface--flat {
    background: transparent;
}

.sh-surface--radius-none {
    border-radius: 0;
}

.sh-surface--radius-sm {
    border-radius: 4px;
}

.sh-surface--radius-md {
    border-radius: 8px;
}

.sh-surface--radius-lg {
    border-radius: 16px;
}

.sh-surface--radius-full {
    border-radius: 9999px;
}

.sh-paper {
    background: var(--sh-bg-primary, #ffffff);
    border-radius: 8px;
    padding: 1rem;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.sh-accordion-item {
    border-bottom: 1px solid var(--sh-border-color, #e5e7eb);
}

.sh-accordion-item:last-child {
    border-bottom: none;
}

.sh-accordion-trigger {
    padding: 0.75rem 1rem;
    cursor: pointer;
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-weight: 500;
}

.sh-accordion-trigger:hover {
    background: var(--sh-bg-secondary, #f9fafb);
}

.sh-accordion-content {
    padding: 0.75rem 1rem;
    color: var(--sh-text-secondary, #6b7280);
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surface_default() {
        let surface = Surface::new(html! { "Content" });
        let rendered = surface.render();
        assert!(rendered.0.as_str().contains("sh-surface"));
        assert!(rendered.0.as_str().contains("Content"));
    }

    #[test]
    fn test_surface_variant_elevated() {
        let surface = Surface::new(html! { "Test" }).variant(SurfaceVariant::Elevated);
        let rendered = surface.render();
        assert!(rendered.0.as_str().contains("sh-surface--elevated"));
    }

    #[test]
    fn test_surface_variant_glass() {
        let surface = Surface::new(html! { "Test" }).variant(SurfaceVariant::Glass);
        let rendered = surface.render();
        assert!(rendered.0.as_str().contains("sh-surface--glass"));
    }

    #[test]
    fn test_surface_variant_bordered() {
        let surface = Surface::new(html! { "Test" }).variant(SurfaceVariant::Bordered);
        let rendered = surface.render();
        assert!(rendered.0.as_str().contains("sh-surface--bordered"));
    }

    #[test]
    fn test_surface_size_sm() {
        let surface = Surface::new(html! { "Test" }).size(SurfaceSize::Sm);
        let rendered = surface.render();
        assert!(rendered.0.as_str().contains("sh-surface--sm"));
    }

    #[test]
    fn test_surface_size_lg() {
        let surface = Surface::new(html! { "Test" }).size(SurfaceSize::Lg);
        let rendered = surface.render();
        assert!(rendered.0.as_str().contains("sh-surface--lg"));
    }

    #[test]
    fn test_surface_radius_full() {
        let surface = Surface::new(html! { "Test" }).radius(SurfaceRadius::Full);
        let rendered = surface.render();
        assert!(rendered.0.as_str().contains("sh-surface--radius-full"));
    }

    #[test]
    fn test_surface_custom_padding() {
        let surface = Surface::new(html! { "Test" }).padding(32);
        let rendered = surface.render();
        assert!(rendered.0.as_str().contains("padding: 32px"));
    }

    #[test]
    fn test_paper() {
        let paper = Paper::new(html! { "Paper content" });
        let rendered = paper.render();
        assert!(rendered.0.as_str().contains("sh-paper"));
        assert!(rendered.0.as_str().contains("Paper content"));
    }

    #[test]
    fn test_accordion() {
        let accordion = Accordion::new(vec![
            AccordionItem {
                title: "Item 1",
                content: html! { "Content 1" },
                open: true,
            },
            AccordionItem {
                title: "Item 2",
                content: html! { "Content 2" },
                open: false,
            },
        ]);
        let rendered = accordion.render();
        assert!(rendered.0.as_str().contains("Item 1"));
        assert!(rendered.0.as_str().contains("Item 2"));
    }

    #[test]
    fn test_surface_css() {
        let css = surface_css();
        assert!(css.contains(".sh-surface"));
        assert!(css.contains(".sh-paper"));
    }
}
